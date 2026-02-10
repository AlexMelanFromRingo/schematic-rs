//! glTF/GLB export with explicit geometry (same approach as OBJ export)
//!
//! Generates all block geometry at actual world positions, grouped by material.
//! Supports Minecraft JSON models and embedded textures.

use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};
use serde::Serialize;

use crate::mc_models::{ModelManager, GeneratedQuad};
use crate::textures::TextureManager;
use crate::UnifiedSchematic;

/// Create a progress bar with consistent style
fn create_progress_bar(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {eta}")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message(message.to_string());
    pb
}

// ============ glTF JSON structures ============

#[derive(Serialize)]
struct GltfRoot {
    asset: GltfAsset,
    scene: usize,
    scenes: Vec<GltfScene>,
    nodes: Vec<GltfNode>,
    meshes: Vec<GltfMesh>,
    accessors: Vec<GltfAccessor>,
    #[serde(rename = "bufferViews")]
    buffer_views: Vec<GltfBufferView>,
    buffers: Vec<GltfBuffer>,
    materials: Vec<GltfMaterial>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    images: Vec<GltfImage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    samplers: Vec<GltfSampler>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    textures: Vec<GltfTexture>,
}

#[derive(Serialize)]
struct GltfAsset {
    version: String,
    generator: String,
}

#[derive(Serialize)]
struct GltfScene {
    nodes: Vec<usize>,
}

#[derive(Serialize)]
struct GltfNode {
    mesh: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Serialize)]
struct GltfMesh {
    primitives: Vec<GltfPrimitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Serialize)]
struct GltfPrimitive {
    attributes: GltfAttributes,
    #[serde(skip_serializing_if = "Option::is_none")]
    indices: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<usize>,
}

#[derive(Serialize)]
struct GltfAttributes {
    #[serde(rename = "POSITION")]
    position: usize,
    #[serde(rename = "NORMAL", skip_serializing_if = "Option::is_none")]
    normal: Option<usize>,
    #[serde(rename = "TEXCOORD_0", skip_serializing_if = "Option::is_none")]
    texcoord: Option<usize>,
}

#[derive(Serialize)]
struct GltfAccessor {
    #[serde(rename = "bufferView")]
    buffer_view: usize,
    #[serde(rename = "byteOffset")]
    byte_offset: usize,
    #[serde(rename = "componentType")]
    component_type: u32,
    count: usize,
    #[serde(rename = "type")]
    accessor_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Vec<f32>>,
}

#[derive(Serialize)]
struct GltfBufferView {
    buffer: usize,
    #[serde(rename = "byteOffset")]
    byte_offset: usize,
    #[serde(rename = "byteLength")]
    byte_length: usize,
    #[serde(rename = "byteStride", skip_serializing_if = "Option::is_none")]
    byte_stride: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<u32>,
}

#[derive(Serialize)]
struct GltfBuffer {
    #[serde(rename = "byteLength")]
    byte_length: usize,
}

#[derive(Serialize)]
struct GltfMaterial {
    name: String,
    #[serde(rename = "pbrMetallicRoughness")]
    pbr: GltfPbr,
    #[serde(rename = "alphaMode", skip_serializing_if = "Option::is_none")]
    alpha_mode: Option<String>,
    #[serde(rename = "alphaCutoff", skip_serializing_if = "Option::is_none")]
    alpha_cutoff: Option<f32>,
    #[serde(rename = "doubleSided")]
    double_sided: bool,
}

#[derive(Serialize)]
struct GltfPbr {
    #[serde(rename = "baseColorFactor")]
    base_color_factor: [f32; 4],
    #[serde(rename = "metallicFactor")]
    metallic_factor: f32,
    #[serde(rename = "roughnessFactor")]
    roughness_factor: f32,
    #[serde(rename = "baseColorTexture", skip_serializing_if = "Option::is_none")]
    base_color_texture: Option<GltfTextureInfo>,
}

#[derive(Serialize)]
struct GltfImage {
    #[serde(rename = "bufferView")]
    buffer_view: usize,
    #[serde(rename = "mimeType")]
    mime_type: String,
}

#[derive(Serialize)]
struct GltfSampler {
    #[serde(rename = "magFilter")]
    mag_filter: u32,
    #[serde(rename = "minFilter")]
    min_filter: u32,
    #[serde(rename = "wrapS")]
    wrap_s: u32,
    #[serde(rename = "wrapT")]
    wrap_t: u32,
}

#[derive(Serialize)]
struct GltfTexture {
    source: usize,
    sampler: usize,
}

#[derive(Serialize)]
struct GltfTextureInfo {
    index: usize,
}

// ============ Constants ============

const GLTF_FLOAT: u32 = 5126;
const GLTF_UNSIGNED_INT: u32 = 5125;
const GLTF_ARRAY_BUFFER: u32 = 34962;
const GLTF_ELEMENT_ARRAY_BUFFER: u32 = 34963;
const GLTF_NEAREST: u32 = 9728;
const GLTF_REPEAT: u32 = 10497;

// ============ Per-material geometry accumulator ============

/// Accumulated geometry for one material
struct MaterialGeometry {
    positions: Vec<f32>,
    normals: Vec<f32>,
    uvs: Vec<f32>,
    indices: Vec<u32>,
}

impl MaterialGeometry {
    fn new() -> Self {
        Self {
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Append a quad (4 vertices, 2 triangles) to this geometry
    fn append_quad(&mut self, quad: &GeneratedQuad) {
        let base_idx = (self.positions.len() / 3) as u32;

        // Compute normal from first 3 vertices
        let v0 = quad.vertices[0];
        let v1 = quad.vertices[1];
        let v2 = quad.vertices[2];
        let e1 = (v1.0 - v0.0, v1.1 - v0.1, v1.2 - v0.2);
        let e2 = (v2.0 - v0.0, v2.1 - v0.1, v2.2 - v0.2);
        let n = (
            e1.1 * e2.2 - e1.2 * e2.1,
            e1.2 * e2.0 - e1.0 * e2.2,
            e1.0 * e2.1 - e1.1 * e2.0,
        );
        let len = (n.0 * n.0 + n.1 * n.1 + n.2 * n.2).sqrt();
        let normal = if len > 0.0 {
            (n.0 / len, n.1 / len, n.2 / len)
        } else {
            (0.0, 1.0, 0.0)
        };

        for (i, v) in quad.vertices.iter().enumerate() {
            self.positions.extend_from_slice(&[v.0, v.1, v.2]);
            self.normals.extend_from_slice(&[normal.0, normal.1, normal.2]);
            self.uvs.extend_from_slice(&[quad.uv_coords[i].0, quad.uv_coords[i].1]);
        }

        self.indices.extend_from_slice(&[
            base_idx, base_idx + 1, base_idx + 2,
            base_idx, base_idx + 2, base_idx + 3,
        ]);
    }
}

// ============ Helpers ============

/// Generate 6 face quads for a unit cube at world position (x, y, z)
fn generate_cube_quads(x: f32, y: f32, z: f32, texture: &str) -> Vec<GeneratedQuad> {
    let uv = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    vec![
        // Front (z+)
        GeneratedQuad {
            vertices: [(x, y, z+1.0), (x+1.0, y, z+1.0), (x+1.0, y+1.0, z+1.0), (x, y+1.0, z+1.0)],
            uv_coords: uv,
            texture: texture.to_string(),
            face_dir: crate::mc_models::FaceDirection::South,
            tint_index: -1,
        },
        // Back (z-)
        GeneratedQuad {
            vertices: [(x+1.0, y, z), (x, y, z), (x, y+1.0, z), (x+1.0, y+1.0, z)],
            uv_coords: uv,
            texture: texture.to_string(),
            face_dir: crate::mc_models::FaceDirection::North,
            tint_index: -1,
        },
        // Top (y+)
        GeneratedQuad {
            vertices: [(x, y+1.0, z+1.0), (x+1.0, y+1.0, z+1.0), (x+1.0, y+1.0, z), (x, y+1.0, z)],
            uv_coords: uv,
            texture: texture.to_string(),
            face_dir: crate::mc_models::FaceDirection::Up,
            tint_index: -1,
        },
        // Bottom (y-)
        GeneratedQuad {
            vertices: [(x, y, z), (x+1.0, y, z), (x+1.0, y, z+1.0), (x, y, z+1.0)],
            uv_coords: uv,
            texture: texture.to_string(),
            face_dir: crate::mc_models::FaceDirection::Down,
            tint_index: -1,
        },
        // Right (x+)
        GeneratedQuad {
            vertices: [(x+1.0, y, z+1.0), (x+1.0, y, z), (x+1.0, y+1.0, z), (x+1.0, y+1.0, z+1.0)],
            uv_coords: uv,
            texture: texture.to_string(),
            face_dir: crate::mc_models::FaceDirection::East,
            tint_index: -1,
        },
        // Left (x-)
        GeneratedQuad {
            vertices: [(x, y, z), (x, y, z+1.0), (x, y+1.0, z+1.0), (x, y+1.0, z)],
            uv_coords: uv,
            texture: texture.to_string(),
            face_dir: crate::mc_models::FaceDirection::West,
            tint_index: -1,
        },
    ]
}

/// Check if block at (x, y, z) has any neighbor that is air or transparent
fn is_exposed(schematic: &UnifiedSchematic, x: usize, y: usize, z: usize, w: usize, h: usize, l: usize) -> bool {
    if x == 0 || x == w - 1 || y == 0 || y == h - 1 || z == 0 || z == l - 1 {
        return true;
    }
    let neighbors = [
        schematic.get_block((x - 1) as u16, y as u16, z as u16),
        schematic.get_block((x + 1) as u16, y as u16, z as u16),
        schematic.get_block(x as u16, (y - 1) as u16, z as u16),
        schematic.get_block(x as u16, (y + 1) as u16, z as u16),
        schematic.get_block(x as u16, y as u16, (z - 1) as u16),
        schematic.get_block(x as u16, y as u16, (z + 1) as u16),
    ];
    for n in &neighbors {
        match n {
            None => return true,
            Some(b) if b.is_air() => return true,
            Some(b) => {
                let name = b.name.strip_prefix("minecraft:").unwrap_or(&b.name);
                if name.contains("glass") || name.contains("leaves") || name.contains("water")
                    || name.contains("lava") || name.contains("ice") {
                    return true;
                }
            }
        }
    }
    false
}

/// Get block color for material (returns [r, g, b, a])
fn get_block_color(name: &str) -> [f32; 4] {
    let name = name.strip_prefix("minecraft:").unwrap_or(name);

    let (r, g, b) = if name.contains("white") {
        (0.95, 0.95, 0.95)
    } else if name.contains("orange") {
        (0.85, 0.45, 0.1)
    } else if name.contains("magenta") {
        (0.7, 0.25, 0.7)
    } else if name.contains("light_blue") {
        (0.4, 0.6, 0.9)
    } else if name.contains("yellow") {
        (0.95, 0.9, 0.2)
    } else if name.contains("lime") {
        (0.5, 0.8, 0.1)
    } else if name.contains("pink") {
        (0.9, 0.5, 0.65)
    } else if name.contains("gray") && !name.contains("light") {
        (0.35, 0.35, 0.4)
    } else if name.contains("light_gray") {
        (0.6, 0.6, 0.6)
    } else if name.contains("cyan") {
        (0.1, 0.55, 0.55)
    } else if name.contains("purple") {
        (0.45, 0.2, 0.7)
    } else if name.contains("blue") && !name.contains("light") {
        (0.2, 0.25, 0.7)
    } else if name.contains("brown") {
        (0.45, 0.3, 0.15)
    } else if name.contains("green") && !name.contains("lime") {
        (0.3, 0.4, 0.15)
    } else if name.contains("red") && !name.contains("warped") {
        (0.7, 0.2, 0.2)
    } else if name.contains("black") {
        (0.1, 0.1, 0.12)
    } else if name.contains("stone") || name.contains("cobble") {
        (0.5, 0.5, 0.5)
    } else if name.contains("deepslate") {
        (0.25, 0.25, 0.3)
    } else if name.contains("dirt") || name.contains("mud") {
        (0.55, 0.4, 0.3)
    } else if name.contains("grass") {
        (0.4, 0.6, 0.3)
    } else if name.contains("sand") && !name.contains("stone") {
        (0.85, 0.8, 0.6)
    } else if name.contains("terracotta") {
        (0.6, 0.4, 0.35)
    } else if name.contains("glass") {
        (0.8, 0.9, 0.95)
    } else if name.contains("water") {
        (0.2, 0.4, 0.8)
    } else if name.contains("lava") {
        (0.9, 0.45, 0.1)
    } else {
        (0.6, 0.6, 0.6)
    };

    let a = if name.contains("glass") || name.contains("water") || name.contains("ice") {
        0.6
    } else {
        1.0
    };

    [r, g, b, a]
}

/// Sanitize texture path to material name (same as OBJ export)
fn texture_to_mat_name(texture: &str) -> String {
    let s = texture.strip_prefix("minecraft:").unwrap_or(texture);
    let s = s.strip_prefix("block/").unwrap_or(s);
    s.replace(['/', ':'], "_")
}

/// Apply color tint to PNG image bytes in memory
fn apply_tint_in_memory(png_bytes: &[u8], tint: (f32, f32, f32)) -> Option<Vec<u8>> {
    use image::{ImageFormat, GenericImageView};

    let img = image::load_from_memory_with_format(png_bytes, ImageFormat::Png).ok()?;
    let (w, h) = img.dimensions();
    let mut buf = image::ImageBuffer::new(w, h);

    for (x, y, pixel) in img.pixels() {
        let r = (pixel[0] as f32 * tint.0).min(255.0) as u8;
        let g = (pixel[1] as f32 * tint.1).min(255.0) as u8;
        let b = (pixel[2] as f32 * tint.2).min(255.0) as u8;
        buf.put_pixel(x, y, image::Rgba([r, g, b, pixel[3]]));
    }

    let mut out = std::io::Cursor::new(Vec::new());
    buf.write_to(&mut out, ImageFormat::Png).ok()?;
    Some(out.into_inner())
}

/// Check if a texture name needs foliage/grass tinting
fn needs_tint(name: &str) -> Option<(f32, f32, f32)> {
    let grass_tint = (0.44, 0.64, 0.22);
    let foliage_tint = (0.38, 0.60, 0.18);

    if name.contains("grass") && !name.contains("dead") {
        Some(grass_tint)
    } else if name.contains("fern") && !name.contains("dead") {
        Some(grass_tint)
    } else if name.ends_with("_leaves") || name == "leaves" {
        if name.contains("spruce") {
            Some((0.38, 0.51, 0.38))
        } else if name.contains("birch") {
            Some((0.50, 0.63, 0.33))
        } else {
            Some(foliage_tint)
        }
    } else {
        None
    }
}

/// Check if material represents a translucent block (smooth alpha blending)
/// vs cutout (binary alpha from texture)
fn is_translucent_material(name: &str) -> bool {
    name.contains("glass") || name.contains("water") || name.contains("ice")
        || name.contains("slime") || name.contains("honey")
}

/// Export schematic to GLB format with explicit geometry (like OBJ export)
pub fn export_glb<P: AsRef<Path>>(
    schematic: &UnifiedSchematic,
    output_path: P,
    jar_path: Option<&Path>,
    textures: Option<&TextureManager>,
    hollow: bool,
    resource_pack: Option<&Path>,
) -> std::io::Result<()> {
    let output_path = output_path.as_ref();

    // Warn if output path doesn't have .glb extension
    match output_path.extension().and_then(|e| e.to_str()) {
        Some("glb") => {}
        Some(ext) => {
            eprintln!("Warning: Output file has .{} extension, but GLB format requires .glb", ext);
            eprintln!("  Consider: --output {}.glb", output_path.file_stem().unwrap_or_default().to_string_lossy());
        }
        None => {
            eprintln!("Warning: Output file has no extension. GLB files should use .glb extension.");
        }
    }

    let (w, h, l) = (schematic.width as usize, schematic.height as usize, schematic.length as usize);

    // Load model manager if jar provided
    let mut model_manager = jar_path.and_then(|p| {
        match ModelManager::from_jar_with_resource_pack(p, resource_pack) {
            Ok(mm) => Some(mm),
            Err(e) => {
                eprintln!("Warning: Failed to load models from jar: {}", e);
                eprintln!("  Falling back to simple cube geometry.");
                None
            }
        }
    });

    // Phase 1: Generate all geometry at actual world positions, grouped by material
    // Process in Y-layer chunks to limit peak memory (same as OBJ export)
    const CHUNK_SIZE: usize = 16;
    let num_chunks = (h + CHUNK_SIZE - 1) / CHUNK_SIZE;
    let pb = create_progress_bar(num_chunks as u64, "Generating geometry");

    // material_name -> accumulated geometry
    let mut material_geom: HashMap<String, MaterialGeometry> = HashMap::new();
    // material_name -> (color, texture_lookup_key for TextureManager)
    // texture_lookup_key is the RAW name (e.g. "oak_planks"), NOT sanitized with _ replacements
    let mut material_info: HashMap<String, ([f32; 4], Option<String>)> = HashMap::new();
    let mut total_quads = 0usize;
    let mut skipped_no_model = 0usize;
    let mut skipped_resolve_fail = 0usize;

    // Helper: add a quad to a material's geometry
    let add_quad = |mat_name: &str, tex_lookup: Option<&str>, block_name: &str,
                    quad: &GeneratedQuad,
                    material_geom: &mut HashMap<String, MaterialGeometry>,
                    material_info: &mut HashMap<String, ([f32; 4], Option<String>)>,
                    total_quads: &mut usize| {
        material_info.entry(mat_name.to_string()).or_insert_with(|| {
            let color = get_block_color(block_name);
            (color, tex_lookup.map(|s| s.to_string()))
        });
        let geom = material_geom.entry(mat_name.to_string()).or_insert_with(MaterialGeometry::new);
        geom.append_quad(quad);
        *total_quads += 1;
    };

    for chunk_idx in 0..num_chunks {
        pb.set_position(chunk_idx as u64);

        let y_start = chunk_idx * CHUNK_SIZE;
        let y_end = ((chunk_idx + 1) * CHUNK_SIZE).min(h);

        for y in y_start..y_end {
            for z in 0..l {
                for x in 0..w {
                    let Some(block) = schematic.get_block(x as u16, y as u16, z as u16) else { continue };
                    if block.is_air() { continue; }

                    let xf = x as f32;
                    let yf = y as f32;
                    let zf = z as f32;

                    // === Water/lava handling (matches OBJ exactly) ===
                    let is_water_block = block.name == "minecraft:water" || block.name == "water";
                    let is_lava_block = block.name == "minecraft:lava" || block.name == "lava";
                    let is_water_cauldron = block.name == "minecraft:water_cauldron";
                    let is_lava_cauldron = block.name == "minecraft:lava_cauldron";

                    // Register water material if needed
                    if is_water_block || is_water_cauldron || crate::export3d::is_waterlogged(&block.state.properties) {
                        material_info.entry("water_still".to_string()).or_insert_with(|| {
                            ([0.2, 0.4, 0.8, 0.6], Some("water_still".to_string()))
                        });
                    }
                    if is_lava_block || is_lava_cauldron {
                        material_info.entry("lava_still".to_string()).or_insert_with(|| {
                            ([0.9, 0.45, 0.1, 0.95], Some("lava_still".to_string()))
                        });
                    }

                    // Generate water block geometry
                    if is_water_block {
                        let water_quads = crate::export3d::generate_water_quads_culled(x, y, z, schematic, w, h, l);
                        for quad in &water_quads {
                            let geom = material_geom.entry("water_still".to_string()).or_insert_with(MaterialGeometry::new);
                            geom.append_quad(quad);
                            total_quads += 1;
                        }
                        continue;
                    }

                    // Generate lava block geometry
                    if is_lava_block {
                        let lava_quads = crate::export3d::generate_lava_quads_culled(x, y, z, schematic, w, h, l);
                        for quad in &lava_quads {
                            let geom = material_geom.entry("lava_still".to_string()).or_insert_with(MaterialGeometry::new);
                            geom.append_quad(quad);
                            total_quads += 1;
                        }
                        continue;
                    }

                    // Handle cauldrons with liquids
                    if is_water_cauldron || is_lava_cauldron {
                        let level: u8 = block.state.properties
                            .get("level")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(3);
                        if level > 0 {
                            let liquid_quads = crate::export3d::generate_cauldron_liquid_quads(
                                xf, yf, zf, level, is_lava_cauldron,
                            );
                            let mat_name = if is_lava_cauldron { "lava_still" } else { "water_still" };
                            for quad in &liquid_quads {
                                let geom = material_geom.entry(mat_name.to_string()).or_insert_with(MaterialGeometry::new);
                                geom.append_quad(quad);
                                total_quads += 1;
                            }
                        }
                        // Fall through to render the cauldron model itself
                    }

                    // === Model-based rendering ===
                    if let Some(ref mut mm) = model_manager {
                        let model_refs = mm.get_models_for_block(&block.name, &block.state.properties);

                        if model_refs.is_empty() {
                            skipped_no_model += 1;
                            continue;
                        }

                        for (model_ref, _) in &model_refs {
                            let Some(resolved) = mm.resolve_model(&model_ref.model) else {
                                skipped_resolve_fail += 1;
                                continue;
                            };

                            let quads = crate::mc_models::generate_model_quads(
                                &resolved,
                                model_ref.x,
                                model_ref.y,
                                xf, yf, zf,
                            );

                            for quad in &quads {
                                let mat_name = texture_to_mat_name(&quad.texture);
                                // Use ORIGINAL texture path for TextureManager lookup (not sanitized)
                                let s = quad.texture.strip_prefix("minecraft:").unwrap_or(&quad.texture);
                                let tex_lookup = s.strip_prefix("block/").unwrap_or(s);

                                add_quad(&mat_name, Some(tex_lookup), &block.name, quad,
                                         &mut material_geom, &mut material_info, &mut total_quads);
                            }
                        }

                        // Waterlogged blocks: add water overlay (matches OBJ)
                        if crate::export3d::is_waterlogged(&block.state.properties) {
                            let water_quads = crate::export3d::generate_water_quads_culled(x, y, z, schematic, w, h, l);
                            for quad in &water_quads {
                                let geom = material_geom.entry("water_still".to_string()).or_insert_with(MaterialGeometry::new);
                                geom.append_quad(quad);
                                total_quads += 1;
                            }
                        }
                    } else {
                        // No model manager — all cubes (hollow only applies here, like OBJ)
                        if hollow && !is_exposed(schematic, x, y, z, w, h, l) {
                            continue;
                        }
                        let mat_name = block.display_name().replace([':', '[', ']', '=', ','], "_");
                        let tex_lookup_key = textures.and_then(|tm| {
                            let lookup = block.name.strip_prefix("minecraft:").unwrap_or(&block.name);
                            tm.get_texture(lookup)
                                .map(|p| p.file_stem().unwrap().to_string_lossy().to_string())
                        });

                        material_info.entry(mat_name.clone()).or_insert_with(|| {
                            let color = get_block_color(&block.name);
                            (color, tex_lookup_key.clone())
                        });

                        let cube_quads = generate_cube_quads(xf, yf, zf, &mat_name);
                        let geom = material_geom.entry(mat_name).or_insert_with(MaterialGeometry::new);
                        for quad in &cube_quads {
                            geom.append_quad(quad);
                            total_quads += 1;
                        }
                    }
                }
            }
        }
    }
    pb.finish_with_message(format!("Generated {} quads, {} materials", total_quads, material_geom.len()));
    if skipped_no_model > 0 {
        eprintln!("  Note: {} blocks had no model definition (skipped)", skipped_no_model);
    }
    if skipped_resolve_fail > 0 {
        eprintln!("  Warning: {} model references failed to resolve", skipped_resolve_fail);
    }

    // Phase 2: Build binary buffer — embed textures first, then geometry
    let mut binary_data: Vec<u8> = Vec::new();
    let mut buffer_views: Vec<GltfBufferView> = Vec::new();
    let mut accessors: Vec<GltfAccessor> = Vec::new();
    let mut gltf_images: Vec<GltfImage> = Vec::new();
    let mut gltf_samplers: Vec<GltfSampler> = Vec::new();
    let mut gltf_textures: Vec<GltfTexture> = Vec::new();
    let mut texture_name_to_tex_idx: HashMap<String, usize> = HashMap::new();

    if textures.is_some() {
        // Collect unique texture names
        let mut unique_tex: Vec<String> = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for (_, (_, tex_name)) in &material_info {
            if let Some(tn) = tex_name {
                if seen.insert(tn.clone()) {
                    unique_tex.push(tn.clone());
                }
            }
        }

        if !unique_tex.is_empty() {
            let tm = textures.unwrap();
            eprintln!("Embedding {} textures...", unique_tex.len());

            gltf_samplers.push(GltfSampler {
                mag_filter: GLTF_NEAREST,
                min_filter: GLTF_NEAREST,
                wrap_s: GLTF_REPEAT,
                wrap_t: GLTF_REPEAT,
            });

            let mut missing_textures: Vec<String> = Vec::new();
            for tex_name in &unique_tex {
                let png_path = tm.get_texture(tex_name);
                if png_path.is_none() {
                    missing_textures.push(tex_name.clone());
                }
                let png_bytes = png_path.and_then(|p| std::fs::read(p).ok());

                if let Some(mut bytes) = png_bytes {
                    if let Some(tint) = needs_tint(tex_name) {
                        if let Some(tinted) = apply_tint_in_memory(&bytes, tint) {
                            bytes = tinted;
                        }
                    }

                    let start = binary_data.len();
                    let byte_length = bytes.len();
                    binary_data.extend_from_slice(&bytes);
                    while binary_data.len() % 4 != 0 { binary_data.push(0); }

                    let bv_idx = buffer_views.len();
                    buffer_views.push(GltfBufferView {
                        buffer: 0, byte_offset: start, byte_length,
                        byte_stride: None, target: None,
                    });

                    let img_idx = gltf_images.len();
                    gltf_images.push(GltfImage {
                        buffer_view: bv_idx,
                        mime_type: "image/png".to_string(),
                    });

                    let tex_idx = gltf_textures.len();
                    gltf_textures.push(GltfTexture { source: img_idx, sampler: 0 });

                    texture_name_to_tex_idx.insert(tex_name.clone(), tex_idx);
                }
            }
            eprintln!("  Embedded {} textures into GLB", texture_name_to_tex_idx.len());
            if !missing_textures.is_empty() {
                eprintln!("  Warning: {} textures not found:", missing_textures.len());
                for name in missing_textures.iter().take(20) {
                    eprintln!("    - {}", name);
                }
                if missing_textures.len() > 20 {
                    eprintln!("    ... and {} more", missing_textures.len() - 20);
                }
            }
        }
    }

    // Phase 3: Write geometry per material, create glTF meshes
    let pb = create_progress_bar(material_geom.len() as u64, "Building GLB");

    let mut meshes: Vec<GltfMesh> = Vec::new();
    let mut nodes: Vec<GltfNode> = Vec::new();
    let mut materials_gltf: Vec<GltfMaterial> = Vec::new();

    let mut sorted_materials: Vec<_> = material_geom.into_iter().collect();
    sorted_materials.sort_by(|a, b| a.0.cmp(&b.0));

    for (i, (mat_name, geom)) in sorted_materials.into_iter().enumerate() {
        pb.set_position(i as u64);

        if geom.positions.is_empty() { continue; }

        // Determine color and texture for this material
        let (color, tex_name) = material_info.get(&mat_name)
            .cloned()
            .unwrap_or(([0.6, 0.6, 0.6, 1.0], None));

        let base_color_texture = tex_name.as_ref()
            .and_then(|tn| texture_name_to_tex_idx.get(tn))
            .map(|&idx| GltfTextureInfo { index: idx });

        let base_color_factor = if base_color_texture.is_some() {
            [1.0, 1.0, 1.0, color[3]]
        } else {
            color
        };

        // Determine alpha mode:
        // - Textured glass/water/ice → BLEND (smooth transparency)
        // - Other textured blocks → MASK (cutout for flowers, rails, etc.)
        // - Non-textured glass/water/ice → BLEND
        // - Fully opaque → no alpha mode (OPAQUE)
        let has_texture = base_color_texture.is_some();
        let is_translucent = is_translucent_material(&mat_name);
        let (alpha_mode, alpha_cutoff) = if is_translucent {
            (Some("BLEND".to_string()), None)
        } else if has_texture {
            // Use MASK for all textured non-translucent blocks
            // Fully opaque textures pass cutoff test on all pixels (alpha=1.0 > 0.5)
            // Flowers/plants/etc get transparent pixels clipped (alpha=0.0 < 0.5)
            (Some("MASK".to_string()), Some(0.5))
        } else if color[3] < 1.0 {
            (Some("BLEND".to_string()), None)
        } else {
            (None, None)
        };

        let material_idx = materials_gltf.len();
        materials_gltf.push(GltfMaterial {
            name: mat_name.clone(),
            pbr: GltfPbr {
                base_color_factor,
                metallic_factor: 0.0,
                roughness_factor: 0.8,
                base_color_texture,
            },
            alpha_mode,
            alpha_cutoff,
            double_sided: true,
        });

        // Write positions
        let pos_start = binary_data.len();
        for &v in &geom.positions { binary_data.extend_from_slice(&v.to_le_bytes()); }
        while binary_data.len() % 4 != 0 { binary_data.push(0); }
        let pos_len = binary_data.len() - pos_start;

        // Write normals
        let norm_start = binary_data.len();
        for &n in &geom.normals { binary_data.extend_from_slice(&n.to_le_bytes()); }
        while binary_data.len() % 4 != 0 { binary_data.push(0); }
        let norm_len = binary_data.len() - norm_start;

        // Write UVs
        let uv_start = binary_data.len();
        for &uv in &geom.uvs { binary_data.extend_from_slice(&uv.to_le_bytes()); }
        while binary_data.len() % 4 != 0 { binary_data.push(0); }
        let uv_len = binary_data.len() - uv_start;

        // Write indices
        let idx_start = binary_data.len();
        for &idx in &geom.indices { binary_data.extend_from_slice(&idx.to_le_bytes()); }
        while binary_data.len() % 4 != 0 { binary_data.push(0); }
        let idx_len = binary_data.len() - idx_start;

        // Position bounds
        let mut min_pos = [f32::MAX; 3];
        let mut max_pos = [f32::MIN; 3];
        for chunk in geom.positions.chunks(3) {
            for j in 0..3 {
                min_pos[j] = min_pos[j].min(chunk[j]);
                max_pos[j] = max_pos[j].max(chunk[j]);
            }
        }

        // Buffer views
        let pos_bv = buffer_views.len();
        buffer_views.push(GltfBufferView {
            buffer: 0, byte_offset: pos_start, byte_length: pos_len,
            byte_stride: Some(12), target: Some(GLTF_ARRAY_BUFFER),
        });
        let norm_bv = buffer_views.len();
        buffer_views.push(GltfBufferView {
            buffer: 0, byte_offset: norm_start, byte_length: norm_len,
            byte_stride: Some(12), target: Some(GLTF_ARRAY_BUFFER),
        });
        let uv_bv = buffer_views.len();
        buffer_views.push(GltfBufferView {
            buffer: 0, byte_offset: uv_start, byte_length: uv_len,
            byte_stride: Some(8), target: Some(GLTF_ARRAY_BUFFER),
        });
        let idx_bv = buffer_views.len();
        buffer_views.push(GltfBufferView {
            buffer: 0, byte_offset: idx_start, byte_length: idx_len,
            byte_stride: None, target: Some(GLTF_ELEMENT_ARRAY_BUFFER),
        });

        // Accessors
        let pos_acc = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: pos_bv, byte_offset: 0, component_type: GLTF_FLOAT,
            count: geom.positions.len() / 3, accessor_type: "VEC3".to_string(),
            min: Some(min_pos.to_vec()), max: Some(max_pos.to_vec()),
        });
        let norm_acc = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: norm_bv, byte_offset: 0, component_type: GLTF_FLOAT,
            count: geom.normals.len() / 3, accessor_type: "VEC3".to_string(),
            min: None, max: None,
        });
        let uv_acc = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: uv_bv, byte_offset: 0, component_type: GLTF_FLOAT,
            count: geom.uvs.len() / 2, accessor_type: "VEC2".to_string(),
            min: None, max: None,
        });
        let idx_acc = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: idx_bv, byte_offset: 0, component_type: GLTF_UNSIGNED_INT,
            count: geom.indices.len(), accessor_type: "SCALAR".to_string(),
            min: None, max: None,
        });

        // Create mesh + node
        let mesh_idx = meshes.len();
        meshes.push(GltfMesh {
            primitives: vec![GltfPrimitive {
                attributes: GltfAttributes {
                    position: pos_acc,
                    normal: Some(norm_acc),
                    texcoord: Some(uv_acc),
                },
                indices: Some(idx_acc),
                material: Some(material_idx),
            }],
            name: Some(mat_name),
        });

        nodes.push(GltfNode {
            mesh: Some(mesh_idx),
            name: None,
        });
    }
    pb.finish_with_message(format!("Created {} meshes", meshes.len()));

    // Build root glTF object
    let scene_nodes: Vec<usize> = (0..nodes.len()).collect();
    let gltf = GltfRoot {
        asset: GltfAsset {
            version: "2.0".to_string(),
            generator: "schem-tool".to_string(),
        },
        scene: 0,
        scenes: vec![GltfScene { nodes: scene_nodes }],
        nodes,
        meshes,
        accessors,
        buffer_views,
        buffers: vec![GltfBuffer {
            byte_length: binary_data.len(),
        }],
        materials: materials_gltf,
        images: gltf_images,
        samplers: gltf_samplers,
        textures: gltf_textures,
    };

    // Serialize JSON
    let json_str = serde_json::to_string(&gltf)?;
    let json_bytes = json_str.as_bytes();

    // Pad JSON to 4-byte boundary
    let json_padding = (4 - (json_bytes.len() % 4)) % 4;
    let json_chunk_len = json_bytes.len() + json_padding;

    // Pad binary to 4-byte boundary
    let bin_padding = (4 - (binary_data.len() % 4)) % 4;
    let bin_chunk_len = binary_data.len() + bin_padding;

    // Calculate total file size
    let total_size = 12 + 8 + json_chunk_len + 8 + bin_chunk_len;

    // Write GLB file
    eprintln!("Writing GLB file ({:.1} MB)...", total_size as f64 / 1024.0 / 1024.0);
    let mut file = BufWriter::with_capacity(4 * 1024 * 1024, std::fs::File::create(output_path)?);

    // GLB header
    file.write_all(b"glTF")?;
    file.write_all(&2u32.to_le_bytes())?;
    file.write_all(&(total_size as u32).to_le_bytes())?;

    // JSON chunk
    file.write_all(&(json_chunk_len as u32).to_le_bytes())?;
    file.write_all(&0x4E4F534Au32.to_le_bytes())?;
    file.write_all(json_bytes)?;
    for _ in 0..json_padding { file.write_all(b" ")?; }

    // BIN chunk
    file.write_all(&(bin_chunk_len as u32).to_le_bytes())?;
    file.write_all(&0x004E4942u32.to_le_bytes())?;
    file.write_all(&binary_data)?;
    for _ in 0..bin_padding { file.write_all(&[0u8])?; }

    file.flush()?;

    eprintln!("Exported to: {}", output_path.display());

    Ok(())
}
