//! glTF/GLB export with GPU instancing support
//!
//! Uses EXT_mesh_gpu_instancing extension to efficiently render
//! millions of identical blocks without duplicating geometry.

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
    #[serde(rename = "extensionsUsed")]
    extensions_used: Vec<String>,
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
    extensions: Option<GltfNodeExtensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Serialize)]
struct GltfNodeExtensions {
    #[serde(rename = "EXT_mesh_gpu_instancing")]
    instancing: GltfInstancing,
}

#[derive(Serialize)]
struct GltfInstancing {
    attributes: GltfInstancingAttributes,
}

#[derive(Serialize)]
struct GltfInstancingAttributes {
    #[serde(rename = "TRANSLATION")]
    translation: usize, // accessor index
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
}

// ============ Constants ============

const GLTF_FLOAT: u32 = 5126;
#[allow(dead_code)]
const GLTF_UNSIGNED_SHORT: u32 = 5123;
const GLTF_UNSIGNED_INT: u32 = 5125;
const GLTF_ARRAY_BUFFER: u32 = 34962;
const GLTF_ELEMENT_ARRAY_BUFFER: u32 = 34963;

// ============ Block mesh data ============

/// Mesh data for a unique block type
struct BlockMesh {
    /// Vertex positions (x, y, z) relative to block origin
    positions: Vec<f32>,
    /// Vertex normals
    normals: Vec<f32>,
    /// UV coordinates
    uvs: Vec<f32>,
    /// Triangle indices
    indices: Vec<u32>,
    /// Material name
    material: String,
}

/// Check if block at (x, y, z) has any neighbor that is air or transparent
fn is_exposed(schematic: &UnifiedSchematic, x: usize, y: usize, z: usize, w: usize, h: usize, l: usize) -> bool {
    // Edge blocks are always exposed
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

/// Generate a simple cube mesh (1x1x1 at origin)
fn generate_cube_mesh(material: &str) -> BlockMesh {
    // 8 vertices of a unit cube
    let positions = vec![
        // Front face (z+)
        0.0, 0.0, 1.0,  1.0, 0.0, 1.0,  1.0, 1.0, 1.0,  0.0, 1.0, 1.0,
        // Back face (z-)
        1.0, 0.0, 0.0,  0.0, 0.0, 0.0,  0.0, 1.0, 0.0,  1.0, 1.0, 0.0,
        // Top face (y+)
        0.0, 1.0, 1.0,  1.0, 1.0, 1.0,  1.0, 1.0, 0.0,  0.0, 1.0, 0.0,
        // Bottom face (y-)
        0.0, 0.0, 0.0,  1.0, 0.0, 0.0,  1.0, 0.0, 1.0,  0.0, 0.0, 1.0,
        // Right face (x+)
        1.0, 0.0, 1.0,  1.0, 0.0, 0.0,  1.0, 1.0, 0.0,  1.0, 1.0, 1.0,
        // Left face (x-)
        0.0, 0.0, 0.0,  0.0, 0.0, 1.0,  0.0, 1.0, 1.0,  0.0, 1.0, 0.0,
    ];

    let normals = vec![
        // Front
        0.0, 0.0, 1.0,  0.0, 0.0, 1.0,  0.0, 0.0, 1.0,  0.0, 0.0, 1.0,
        // Back
        0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0,
        // Top
        0.0, 1.0, 0.0,  0.0, 1.0, 0.0,  0.0, 1.0, 0.0,  0.0, 1.0, 0.0,
        // Bottom
        0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0,
        // Right
        1.0, 0.0, 0.0,  1.0, 0.0, 0.0,  1.0, 0.0, 0.0,  1.0, 0.0, 0.0,
        // Left
        -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0,
    ];

    let uvs = vec![
        // Each face gets 0-1 UV mapping
        0.0, 0.0,  1.0, 0.0,  1.0, 1.0,  0.0, 1.0, // Front
        0.0, 0.0,  1.0, 0.0,  1.0, 1.0,  0.0, 1.0, // Back
        0.0, 0.0,  1.0, 0.0,  1.0, 1.0,  0.0, 1.0, // Top
        0.0, 0.0,  1.0, 0.0,  1.0, 1.0,  0.0, 1.0, // Bottom
        0.0, 0.0,  1.0, 0.0,  1.0, 1.0,  0.0, 1.0, // Right
        0.0, 0.0,  1.0, 0.0,  1.0, 1.0,  0.0, 1.0, // Left
    ];

    // Two triangles per face, 6 faces
    let indices = vec![
        0, 1, 2, 0, 2, 3,       // Front
        4, 5, 6, 4, 6, 7,       // Back
        8, 9, 10, 8, 10, 11,    // Top
        12, 13, 14, 12, 14, 15, // Bottom
        16, 17, 18, 16, 18, 19, // Right
        20, 21, 22, 20, 22, 23, // Left
    ];

    BlockMesh {
        positions,
        normals,
        uvs,
        indices,
        material: material.to_string(),
    }
}

/// Generate mesh from quads (for JSON model blocks)
fn generate_mesh_from_quads(quads: &[GeneratedQuad], material: &str) -> BlockMesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for quad in quads {
        let base_idx = (positions.len() / 3) as u32;

        // Add 4 vertices
        for (i, v) in quad.vertices.iter().enumerate() {
            positions.extend_from_slice(&[v.0, v.1, v.2]);

            // Compute normal from first 3 vertices
            if i == 0 {
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
                // Add normal 4 times (once per vertex)
                for _ in 0..4 {
                    normals.extend_from_slice(&[normal.0, normal.1, normal.2]);
                }
            }

            uvs.extend_from_slice(&[quad.uv_coords[i].0, 1.0 - quad.uv_coords[i].1]);
        }

        // Two triangles for quad
        indices.extend_from_slice(&[
            base_idx, base_idx + 1, base_idx + 2,
            base_idx, base_idx + 2, base_idx + 3,
        ]);
    }

    BlockMesh {
        positions,
        normals,
        uvs,
        indices,
        material: material.to_string(),
    }
}

/// Get block color for material
fn get_block_color(name: &str) -> [f32; 4] {
    let name = name.strip_prefix("minecraft:").unwrap_or(name);

    // Extract color from block name
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

    // Alpha for transparent blocks
    let a = if name.contains("glass") || name.contains("water") || name.contains("ice") {
        0.6
    } else {
        1.0
    };

    [r, g, b, a]
}

/// Export schematic to GLB format with instancing
pub fn export_glb<P: AsRef<Path>>(
    schematic: &UnifiedSchematic,
    output_path: P,
    jar_path: Option<&Path>,
    _textures: Option<&TextureManager>,
    hollow: bool,
    resource_pack: Option<&Path>,
) -> std::io::Result<()> {
    let output_path = output_path.as_ref();

    let (w, h, l) = (schematic.width as usize, schematic.height as usize, schematic.length as usize);
    let total_blocks = (w * h * l) as u64;

    // Phase 1: Collect unique block types and their positions
    let pb = create_progress_bar(total_blocks, "Collecting blocks");

    // Map: block_key -> (mesh_data, positions)
    let mut block_types: HashMap<String, (Option<BlockMesh>, Vec<[f32; 3]>)> = HashMap::new();

    // Load model manager if jar provided
    let mut model_manager = jar_path.and_then(|p| {
        ModelManager::from_jar_with_resource_pack(p, resource_pack).ok()
    });

    let mut processed = 0u64;
    for y in 0..h {
        for z in 0..l {
            for x in 0..w {
                processed += 1;
                if processed % 100_000 == 0 {
                    pb.set_position(processed);
                }

                let Some(block) = schematic.get_block(x as u16, y as u16, z as u16) else { continue };
                if block.is_air() { continue; }

                // Skip unexposed blocks if hollow mode
                if hollow && !is_exposed(schematic, x, y, z, w, h, l) {
                    continue;
                }

                // Create key from block name + relevant properties
                let key = block.display_name();
                let position = [x as f32, y as f32, z as f32];

                let entry = block_types.entry(key.to_string()).or_insert_with(|| (None, Vec::new()));
                entry.1.push(position);

                // Generate mesh on first encounter
                if entry.0.is_none() {
                    let mesh = if let Some(ref mut mm) = model_manager {
                        // Try to get model from JSON
                        let model_refs = mm.get_models_for_block(&block.name, &block.state.properties);
                        if !model_refs.is_empty() {
                            let mut all_quads = Vec::new();
                            for (model_ref, _) in &model_refs {
                                if let Some(resolved) = mm.resolve_model(&model_ref.model) {
                                    let quads = crate::mc_models::generate_model_quads(
                                        &resolved,
                                        model_ref.x,
                                        model_ref.y,
                                        0.0, 0.0, 0.0, // Relative to origin
                                    );
                                    all_quads.extend(quads);
                                }
                            }
                            if !all_quads.is_empty() {
                                Some(generate_mesh_from_quads(&all_quads, &key))
                            } else {
                                Some(generate_cube_mesh(&key))
                            }
                        } else {
                            Some(generate_cube_mesh(&key))
                        }
                    } else {
                        Some(generate_cube_mesh(&key))
                    };
                    entry.0 = mesh;
                }
            }
        }
    }
    pb.finish_with_message(format!("Found {} unique block types", block_types.len()));

    // Phase 2: Build glTF structure
    let pb = create_progress_bar(block_types.len() as u64, "Building GLB");

    let mut binary_data: Vec<u8> = Vec::new();
    let mut buffer_views: Vec<GltfBufferView> = Vec::new();
    let mut accessors: Vec<GltfAccessor> = Vec::new();
    let mut meshes: Vec<GltfMesh> = Vec::new();
    let mut nodes: Vec<GltfNode> = Vec::new();
    let mut materials: Vec<GltfMaterial> = Vec::new();
    let mut material_map: HashMap<String, usize> = HashMap::new();

    let mut mesh_idx = 0usize;
    for (i, (key, (mesh_opt, positions))) in block_types.iter().enumerate() {
        pb.set_position(i as u64);

        let Some(mesh) = mesh_opt else { continue };
        if positions.is_empty() { continue; }

        // Get or create material
        let material_idx = if let Some(&idx) = material_map.get(&mesh.material) {
            idx
        } else {
            let color = get_block_color(&mesh.material);
            let alpha_mode = if color[3] < 1.0 { Some("BLEND".to_string()) } else { None };
            let mat = GltfMaterial {
                name: mesh.material.clone(),
                pbr: GltfPbr {
                    base_color_factor: color,
                    metallic_factor: 0.0,
                    roughness_factor: 0.8,
                },
                alpha_mode,
                double_sided: true,
            };
            let idx = materials.len();
            materials.push(mat);
            material_map.insert(mesh.material.clone(), idx);
            idx
        };

        // Write mesh data to binary buffer
        let positions_start = binary_data.len();
        for &v in &mesh.positions {
            binary_data.extend_from_slice(&v.to_le_bytes());
        }
        // Pad to 4-byte boundary
        while binary_data.len() % 4 != 0 {
            binary_data.push(0);
        }
        let positions_len = binary_data.len() - positions_start;

        let normals_start = binary_data.len();
        for &n in &mesh.normals {
            binary_data.extend_from_slice(&n.to_le_bytes());
        }
        while binary_data.len() % 4 != 0 {
            binary_data.push(0);
        }
        let normals_len = binary_data.len() - normals_start;

        let uvs_start = binary_data.len();
        for &uv in &mesh.uvs {
            binary_data.extend_from_slice(&uv.to_le_bytes());
        }
        while binary_data.len() % 4 != 0 {
            binary_data.push(0);
        }
        let uvs_len = binary_data.len() - uvs_start;

        let indices_start = binary_data.len();
        for &idx in &mesh.indices {
            binary_data.extend_from_slice(&idx.to_le_bytes());
        }
        while binary_data.len() % 4 != 0 {
            binary_data.push(0);
        }
        let indices_len = binary_data.len() - indices_start;

        // Compute position bounds
        let mut min_pos = [f32::MAX; 3];
        let mut max_pos = [f32::MIN; 3];
        for chunk in mesh.positions.chunks(3) {
            for i in 0..3 {
                min_pos[i] = min_pos[i].min(chunk[i]);
                max_pos[i] = max_pos[i].max(chunk[i]);
            }
        }

        // Create buffer views and accessors for mesh data
        let pos_view_idx = buffer_views.len();
        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset: positions_start,
            byte_length: positions_len,
            byte_stride: Some(12),
            target: Some(GLTF_ARRAY_BUFFER),
        });

        let pos_accessor_idx = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: pos_view_idx,
            byte_offset: 0,
            component_type: GLTF_FLOAT,
            count: mesh.positions.len() / 3,
            accessor_type: "VEC3".to_string(),
            min: Some(min_pos.to_vec()),
            max: Some(max_pos.to_vec()),
        });

        let norm_view_idx = buffer_views.len();
        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset: normals_start,
            byte_length: normals_len,
            byte_stride: Some(12),
            target: Some(GLTF_ARRAY_BUFFER),
        });

        let norm_accessor_idx = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: norm_view_idx,
            byte_offset: 0,
            component_type: GLTF_FLOAT,
            count: mesh.normals.len() / 3,
            accessor_type: "VEC3".to_string(),
            min: None,
            max: None,
        });

        let uv_view_idx = buffer_views.len();
        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset: uvs_start,
            byte_length: uvs_len,
            byte_stride: Some(8),
            target: Some(GLTF_ARRAY_BUFFER),
        });

        let uv_accessor_idx = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: uv_view_idx,
            byte_offset: 0,
            component_type: GLTF_FLOAT,
            count: mesh.uvs.len() / 2,
            accessor_type: "VEC2".to_string(),
            min: None,
            max: None,
        });

        let idx_view_idx = buffer_views.len();
        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset: indices_start,
            byte_length: indices_len,
            byte_stride: None,
            target: Some(GLTF_ELEMENT_ARRAY_BUFFER),
        });

        let idx_accessor_idx = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: idx_view_idx,
            byte_offset: 0,
            component_type: GLTF_UNSIGNED_INT,
            count: mesh.indices.len(),
            accessor_type: "SCALAR".to_string(),
            min: None,
            max: None,
        });

        // Create mesh
        meshes.push(GltfMesh {
            primitives: vec![GltfPrimitive {
                attributes: GltfAttributes {
                    position: pos_accessor_idx,
                    normal: Some(norm_accessor_idx),
                    texcoord: Some(uv_accessor_idx),
                },
                indices: Some(idx_accessor_idx),
                material: Some(material_idx),
            }],
            name: Some(key.clone()),
        });

        // Write instance translations
        let translations_start = binary_data.len();
        let mut min_trans = [f32::MAX; 3];
        let mut max_trans = [f32::MIN; 3];
        for pos in positions {
            for i in 0..3 {
                min_trans[i] = min_trans[i].min(pos[i]);
                max_trans[i] = max_trans[i].max(pos[i]);
            }
            binary_data.extend_from_slice(&pos[0].to_le_bytes());
            binary_data.extend_from_slice(&pos[1].to_le_bytes());
            binary_data.extend_from_slice(&pos[2].to_le_bytes());
        }
        while binary_data.len() % 4 != 0 {
            binary_data.push(0);
        }
        let translations_len = binary_data.len() - translations_start;

        let trans_view_idx = buffer_views.len();
        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset: translations_start,
            byte_length: translations_len,
            byte_stride: Some(12),
            target: None, // Not a standard attribute target
        });

        let trans_accessor_idx = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: trans_view_idx,
            byte_offset: 0,
            component_type: GLTF_FLOAT,
            count: positions.len(),
            accessor_type: "VEC3".to_string(),
            min: Some(min_trans.to_vec()),
            max: Some(max_trans.to_vec()),
        });

        // Create node with instancing extension
        nodes.push(GltfNode {
            mesh: Some(mesh_idx),
            extensions: Some(GltfNodeExtensions {
                instancing: GltfInstancing {
                    attributes: GltfInstancingAttributes {
                        translation: trans_accessor_idx,
                    },
                },
            }),
            name: Some(key.clone()),
        });

        mesh_idx += 1;
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
        materials,
        extensions_used: vec!["EXT_mesh_gpu_instancing".to_string()],
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
    let total_size = 12 + // GLB header
        8 + json_chunk_len + // JSON chunk header + data
        8 + bin_chunk_len; // BIN chunk header + data

    // Write GLB file
    eprintln!("Writing GLB file ({:.1} MB)...", total_size as f64 / 1024.0 / 1024.0);
    let mut file = BufWriter::with_capacity(4 * 1024 * 1024, std::fs::File::create(output_path)?);

    // GLB header
    file.write_all(b"glTF")?; // Magic
    file.write_all(&2u32.to_le_bytes())?; // Version
    file.write_all(&(total_size as u32).to_le_bytes())?; // Total length

    // JSON chunk
    file.write_all(&(json_chunk_len as u32).to_le_bytes())?; // Chunk length
    file.write_all(&0x4E4F534Au32.to_le_bytes())?; // Chunk type: JSON
    file.write_all(json_bytes)?;
    for _ in 0..json_padding {
        file.write_all(b" ")?; // JSON padding must be spaces
    }

    // BIN chunk
    file.write_all(&(bin_chunk_len as u32).to_le_bytes())?; // Chunk length
    file.write_all(&0x004E4942u32.to_le_bytes())?; // Chunk type: BIN
    file.write_all(&binary_data)?;
    for _ in 0..bin_padding {
        file.write_all(&[0u8])?; // Binary padding must be zeros
    }

    file.flush()?;

    eprintln!("Exported to: {}", output_path.display());

    Ok(())
}
