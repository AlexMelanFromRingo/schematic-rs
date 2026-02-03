//! 3D export functionality for schematics
//!
//! Supports exporting to OBJ format with MTL materials and optional textures
//! Includes greedy meshing algorithm for dramatically reduced polygon counts

use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};
use crate::UnifiedSchematic;
use crate::textures::TextureManager;

/// Block color mapping (approximate Minecraft colors)
pub fn get_block_color(name: &str) -> (f32, f32, f32) {
    let name = name.strip_prefix("minecraft:").unwrap_or(name);

    match name {
        // Stone variants
        "stone" => (0.5, 0.5, 0.5),
        "cobblestone" | "mossy_cobblestone" => (0.45, 0.45, 0.45),
        "granite" | "polished_granite" => (0.6, 0.4, 0.35),
        "diorite" | "polished_diorite" => (0.75, 0.75, 0.75),
        "andesite" | "polished_andesite" => (0.55, 0.55, 0.55),
        "deepslate" | "cobbled_deepslate" => (0.3, 0.3, 0.35),
        "polished_deepslate" => (0.28, 0.28, 0.32),
        "deepslate_bricks" | "cracked_deepslate_bricks" => (0.25, 0.25, 0.3),
        "deepslate_tiles" | "cracked_deepslate_tiles" => (0.22, 0.22, 0.27),
        "chiseled_deepslate" => (0.27, 0.27, 0.32),
        "tuff" => (0.45, 0.47, 0.43),
        "polished_tuff" | "tuff_bricks" => (0.48, 0.5, 0.46),
        "calcite" => (0.9, 0.9, 0.88),
        "dripstone_block" => (0.55, 0.45, 0.4),
        "blackstone" | "gilded_blackstone" => (0.15, 0.13, 0.15),
        "polished_blackstone" => (0.12, 0.1, 0.12),
        "polished_blackstone_bricks" | "cracked_polished_blackstone_bricks" => (0.13, 0.11, 0.13),
        "chiseled_polished_blackstone" => (0.14, 0.12, 0.14),
        "basalt" | "polished_basalt" => (0.3, 0.3, 0.32),
        "smooth_basalt" => (0.25, 0.25, 0.27),
        "dirt" | "coarse_dirt" | "rooted_dirt" => (0.55, 0.4, 0.3),
        "grass_block" => (0.4, 0.6, 0.3),
        "podzol" => (0.45, 0.35, 0.25),
        "mycelium" => (0.5, 0.45, 0.5),
        "mud" => (0.35, 0.3, 0.35),
        "packed_mud" => (0.5, 0.4, 0.35),
        "mud_bricks" => (0.55, 0.45, 0.4),
        "sand" => (0.85, 0.8, 0.6),
        "red_sand" => (0.75, 0.45, 0.25),
        "gravel" => (0.55, 0.52, 0.5),
        "clay" => (0.6, 0.62, 0.68),
        "sandstone" | "cut_sandstone" | "smooth_sandstone" | "chiseled_sandstone" => (0.85, 0.78, 0.55),
        "red_sandstone" | "cut_red_sandstone" | "smooth_red_sandstone" => (0.7, 0.4, 0.2),
        n if n.contains("oak") && n.contains("log") => (0.45, 0.35, 0.2),
        n if n.contains("oak") && n.contains("plank") => (0.6, 0.5, 0.3),
        n if n.contains("spruce") => (0.35, 0.25, 0.15),
        n if n.contains("birch") => (0.8, 0.75, 0.6),
        n if n.contains("jungle") => (0.55, 0.4, 0.25),
        n if n.contains("acacia") => (0.7, 0.4, 0.25),
        n if n.contains("dark_oak") => (0.25, 0.18, 0.1),
        n if n.contains("mangrove") => (0.45, 0.2, 0.15),
        n if n.contains("cherry") => (0.75, 0.55, 0.55),
        n if n.contains("bamboo") => (0.7, 0.65, 0.4),
        n if n.contains("crimson") => (0.5, 0.2, 0.25),
        n if n.contains("warped") => (0.2, 0.45, 0.45),
        n if n.contains("log") || n.contains("wood") => (0.45, 0.35, 0.2),
        n if n.contains("plank") => (0.6, 0.5, 0.3),
        n if n.contains("leaves") => (0.25, 0.5, 0.2),
        "bricks" | "brick_stairs" | "brick_slab" => (0.6, 0.35, 0.3),
        "stone_bricks" | "mossy_stone_bricks" | "cracked_stone_bricks" | "chiseled_stone_bricks" => (0.48, 0.48, 0.48),
        "nether_bricks" | "cracked_nether_bricks" | "chiseled_nether_bricks" => (0.25, 0.15, 0.2),
        "red_nether_bricks" => (0.35, 0.12, 0.12),
        "end_stone_bricks" => (0.85, 0.85, 0.7),
        "prismarine_bricks" => (0.4, 0.6, 0.55),
        "iron_block" => (0.75, 0.75, 0.75),
        "gold_block" => (0.9, 0.75, 0.2),
        "diamond_block" => (0.4, 0.8, 0.8),
        "emerald_block" => (0.3, 0.7, 0.35),
        "lapis_block" => (0.2, 0.3, 0.7),
        "redstone_block" => (0.7, 0.15, 0.1),
        "coal_block" => (0.15, 0.15, 0.15),
        "copper_block" | "cut_copper" => (0.7, 0.45, 0.35),
        "netherite_block" => (0.25, 0.22, 0.25),
        n if n.contains("ore") => (0.5, 0.5, 0.5),
        "glass" => (0.85, 0.9, 0.95),
        "white_stained_glass" => (0.95, 0.95, 0.95),
        "red_stained_glass" => (0.8, 0.2, 0.2),
        "orange_stained_glass" => (0.9, 0.5, 0.15),
        "yellow_stained_glass" => (0.9, 0.85, 0.2),
        "lime_stained_glass" => (0.5, 0.8, 0.2),
        "green_stained_glass" => (0.3, 0.5, 0.2),
        "cyan_stained_glass" => (0.2, 0.6, 0.65),
        "light_blue_stained_glass" => (0.5, 0.7, 0.9),
        "blue_stained_glass" => (0.2, 0.3, 0.8),
        "purple_stained_glass" => (0.5, 0.25, 0.7),
        "magenta_stained_glass" => (0.7, 0.3, 0.65),
        "pink_stained_glass" => (0.85, 0.55, 0.65),
        "brown_stained_glass" => (0.45, 0.3, 0.2),
        "gray_stained_glass" => (0.4, 0.4, 0.4),
        "light_gray_stained_glass" => (0.6, 0.6, 0.6),
        "black_stained_glass" => (0.15, 0.15, 0.18),
        "white_wool" => (0.95, 0.95, 0.95),
        "red_wool" => (0.7, 0.2, 0.2),
        "orange_wool" => (0.85, 0.5, 0.15),
        "yellow_wool" => (0.9, 0.85, 0.25),
        "lime_wool" => (0.5, 0.75, 0.2),
        "green_wool" => (0.35, 0.5, 0.2),
        "cyan_wool" => (0.2, 0.55, 0.6),
        "light_blue_wool" => (0.5, 0.7, 0.85),
        "blue_wool" => (0.25, 0.3, 0.7),
        "purple_wool" => (0.5, 0.25, 0.65),
        "magenta_wool" => (0.65, 0.3, 0.6),
        "pink_wool" => (0.85, 0.55, 0.65),
        "brown_wool" => (0.45, 0.3, 0.2),
        "gray_wool" => (0.35, 0.35, 0.35),
        "light_gray_wool" => (0.6, 0.6, 0.6),
        "black_wool" => (0.12, 0.12, 0.15),
        "white_concrete" => (0.95, 0.95, 0.95),
        "red_concrete" => (0.6, 0.15, 0.15),
        "orange_concrete" => (0.85, 0.45, 0.1),
        "yellow_concrete" => (0.9, 0.8, 0.15),
        "lime_concrete" => (0.45, 0.7, 0.15),
        "green_concrete" => (0.3, 0.45, 0.2),
        "cyan_concrete" => (0.15, 0.5, 0.55),
        "light_blue_concrete" => (0.4, 0.6, 0.8),
        "blue_concrete" => (0.25, 0.3, 0.65),
        "purple_concrete" => (0.45, 0.2, 0.6),
        "magenta_concrete" => (0.6, 0.25, 0.55),
        "pink_concrete" => (0.8, 0.5, 0.6),
        "brown_concrete" => (0.4, 0.28, 0.18),
        "gray_concrete" => (0.3, 0.3, 0.32),
        "light_gray_concrete" => (0.55, 0.55, 0.55),
        "black_concrete" => (0.08, 0.08, 0.1),
        "terracotta" => (0.6, 0.45, 0.38),
        "white_terracotta" => (0.82, 0.72, 0.68),
        "red_terracotta" => (0.55, 0.25, 0.2),
        "orange_terracotta" => (0.65, 0.38, 0.22),
        "yellow_terracotta" => (0.7, 0.55, 0.25),
        "lime_terracotta" => (0.45, 0.5, 0.28),
        "green_terracotta" => (0.35, 0.42, 0.3),
        "cyan_terracotta" => (0.35, 0.45, 0.45),
        "light_blue_terracotta" => (0.48, 0.52, 0.6),
        "blue_terracotta" => (0.3, 0.32, 0.52),
        "purple_terracotta" => (0.45, 0.32, 0.42),
        "magenta_terracotta" => (0.58, 0.38, 0.45),
        "pink_terracotta" => (0.65, 0.45, 0.45),
        "brown_terracotta" => (0.35, 0.25, 0.2),
        "gray_terracotta" => (0.32, 0.28, 0.28),
        "light_gray_terracotta" => (0.52, 0.45, 0.42),
        "black_terracotta" => (0.18, 0.12, 0.12),
        "netherrack" => (0.5, 0.25, 0.25),
        "soul_sand" => (0.35, 0.28, 0.22),
        "soul_soil" => (0.32, 0.25, 0.2),
        "glowstone" => (0.85, 0.7, 0.4),
        "magma_block" => (0.55, 0.25, 0.1),
        "nether_wart_block" => (0.5, 0.15, 0.15),
        "warped_wart_block" => (0.1, 0.5, 0.5),
        "shroomlight" => (0.9, 0.6, 0.4),
        "end_stone" => (0.85, 0.85, 0.7),
        "purpur_block" | "purpur_pillar" => (0.6, 0.45, 0.6),
        "quartz_block" | "smooth_quartz" | "quartz_bricks" | "chiseled_quartz_block" | "quartz_pillar" => (0.9, 0.88, 0.85),
        "prismarine" => (0.4, 0.55, 0.5),
        "dark_prismarine" => (0.25, 0.4, 0.38),
        "sea_lantern" => (0.7, 0.85, 0.85),
        "obsidian" | "crying_obsidian" => (0.15, 0.1, 0.2),
        "bedrock" => (0.3, 0.3, 0.3),
        "ice" | "packed_ice" | "blue_ice" => (0.6, 0.75, 0.9),
        "snow_block" | "powder_snow" => (0.95, 0.97, 1.0),
        "hay_block" => (0.75, 0.65, 0.25),
        "bone_block" => (0.85, 0.82, 0.75),
        "slime_block" => (0.45, 0.7, 0.4),
        "honey_block" => (0.85, 0.6, 0.2),
        "bookshelf" | "chiseled_bookshelf" => (0.55, 0.45, 0.3),
        "tnt" => (0.7, 0.3, 0.25),
        "sponge" | "wet_sponge" => (0.75, 0.75, 0.35),
        "melon" => (0.5, 0.65, 0.3),
        "pumpkin" | "carved_pumpkin" | "jack_o_lantern" => (0.8, 0.5, 0.15),
        "redstone_lamp" => (0.55, 0.35, 0.2),
        "redstone_wire" | "redstone_torch" => (0.6, 0.15, 0.1),
        n if n.contains("piston") => (0.55, 0.45, 0.35),
        "observer" | "dropper" | "dispenser" => (0.45, 0.45, 0.45),
        "hopper" => (0.4, 0.4, 0.45),
        "water" => (0.2, 0.4, 0.8),
        "lava" => (0.9, 0.45, 0.1),
        _ => (0.5, 0.5, 0.5),
    }
}

/// Create a progress bar with consistent styling
fn create_progress_bar(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {elapsed_precise}")
            .unwrap()
            .progress_chars("=>-")
    );
    pb.set_message(message.to_string());
    pb
}

/// Face direction for greedy meshing
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FaceDir {
    XNeg, XPos,  // -X, +X
    YNeg, YPos,  // -Y (bottom), +Y (top)
    ZNeg, ZPos,  // -Z, +Z
}

impl FaceDir {
    fn all() -> [FaceDir; 6] {
        [FaceDir::XNeg, FaceDir::XPos, FaceDir::YNeg, FaceDir::YPos, FaceDir::ZNeg, FaceDir::ZPos]
    }
}

/// A merged quad from greedy meshing
#[derive(Debug)]
struct GreedyQuad {
    /// Material name for this quad
    material: String,
    /// Four corner vertices (counter-clockwise)
    vertices: [(f32, f32, f32); 4],
    /// Size in blocks (width, height) for texture tiling
    size: (usize, usize),
}

/// Generate OBJ file from schematic (simple per-block cubes)
pub fn export_obj<P: AsRef<Path>>(
    schematic: &UnifiedSchematic,
    obj_path: P,
    hollow: bool,
    skip_air: bool,
) -> std::io::Result<()> {
    export_obj_internal(schematic, obj_path, hollow, skip_air, None, false)
}

/// Generate OBJ file from schematic with optional textures
pub fn export_obj_with_textures<P: AsRef<Path>>(
    schematic: &UnifiedSchematic,
    obj_path: P,
    hollow: bool,
    skip_air: bool,
    textures: Option<&TextureManager>,
) -> std::io::Result<()> {
    export_obj_internal(schematic, obj_path, hollow, skip_air, textures, false)
}

/// Generate OBJ file with greedy meshing (dramatically reduced polygon count)
pub fn export_obj_greedy<P: AsRef<Path>>(
    schematic: &UnifiedSchematic,
    obj_path: P,
    textures: Option<&TextureManager>,
) -> std::io::Result<()> {
    export_obj_internal(schematic, obj_path, true, true, textures, true)
}

/// Internal function for OBJ export with all options
fn export_obj_internal<P: AsRef<Path>>(
    schematic: &UnifiedSchematic,
    obj_path: P,
    hollow: bool,
    skip_air: bool,
    textures: Option<&TextureManager>,
    greedy: bool,
) -> std::io::Result<()> {
    let obj_path = obj_path.as_ref();
    let mtl_path = obj_path.with_extension("mtl");
    let use_textures = textures.map(|t| t.has_textures()).unwrap_or(false);

    // Create textures subdirectory if using textures
    let tex_dir = if use_textures {
        let dir = obj_path.parent().unwrap_or(Path::new(".")).join("textures");
        std::fs::create_dir_all(&dir)?;
        Some(dir)
    } else {
        None
    };

    // Use BufWriter for much faster I/O
    let mut obj_file = BufWriter::with_capacity(1024 * 1024, std::fs::File::create(obj_path)?);
    let mut mtl_file = BufWriter::with_capacity(64 * 1024, std::fs::File::create(&mtl_path)?);

    // Write headers
    writeln!(obj_file, "# Minecraft Schematic Export")?;
    writeln!(obj_file, "# Generated by schem-tool{}", if greedy { " (greedy meshing)" } else { "" })?;
    writeln!(obj_file, "# Dimensions: {}x{}x{}", schematic.width, schematic.height, schematic.length)?;
    writeln!(obj_file, "mtllib {}", mtl_path.file_name().unwrap().to_string_lossy())?;
    writeln!(obj_file)?;

    // Note: For greedy meshing, UV coordinates are written per-quad
    // For naive mode, we use fixed 0-1 coordinates
    if use_textures && !greedy {
        writeln!(obj_file, "# Texture coordinates")?;
        writeln!(obj_file, "vt 0 0")?;
        writeln!(obj_file, "vt 1 0")?;
        writeln!(obj_file, "vt 1 1")?;
        writeln!(obj_file, "vt 0 1")?;
        writeln!(obj_file)?;
    }

    writeln!(mtl_file, "# Minecraft Block Materials")?;
    writeln!(mtl_file)?;

    // Collect materials
    let total_positions = schematic.width as u64 * schematic.height as u64 * schematic.length as u64;
    let pb = create_progress_bar(total_positions, "Collecting materials");

    let mut materials: HashMap<String, (f32, f32, f32, Option<String>)> = HashMap::new();
    let mut processed = 0u64;

    for y in 0..schematic.height {
        for z in 0..schematic.length {
            for x in 0..schematic.width {
                processed += 1;
                if processed % 100_000 == 0 {
                    pb.set_position(processed);
                }
                if let Some(block) = schematic.get_block(x, y, z) {
                    if skip_air && block.is_air() { continue; }
                    let mat_name = block.display_name().replace([':', '[', ']', '=', ','], "_");
                    if !materials.contains_key(&mat_name) {
                        let color = get_block_color(&block.name);
                        let texture_file = if let (Some(tex_mgr), Some(tex_out_dir)) = (textures, &tex_dir) {
                            if let Some(tex_path) = tex_mgr.get_texture(&block.name) {
                                let tex_name = format!("{}.png", mat_name);
                                let dest = tex_out_dir.join(&tex_name);
                                if std::fs::copy(tex_path, &dest).is_ok() {
                                    Some(format!("textures/{}", tex_name))
                                } else { None }
                            } else { None }
                        } else { None };
                        materials.insert(mat_name.clone(), (color.0, color.1, color.2, texture_file));
                    }
                }
            }
        }
    }
    pb.finish_with_message(format!("Found {} unique materials", materials.len()));

    // Write materials
    for (name, (r, g, b, tex_file)) in &materials {
        writeln!(mtl_file, "newmtl {}", name)?;
        writeln!(mtl_file, "Kd {} {} {}", r, g, b)?;
        writeln!(mtl_file, "Ka 0.2 0.2 0.2")?;
        if tex_file.is_some() {
            writeln!(mtl_file, "Ks 0.1 0.1 0.1")?;
            writeln!(mtl_file, "Ns 50.0")?;
        } else {
            writeln!(mtl_file, "Ks 0.0 0.0 0.0")?;
            writeln!(mtl_file, "Ns 10.0")?;
        }
        writeln!(mtl_file, "d 1.0")?;
        writeln!(mtl_file, "illum 2")?;
        if let Some(tex) = tex_file {
            writeln!(mtl_file, "map_Kd {}", tex)?;
        }
        writeln!(mtl_file)?;
    }
    mtl_file.flush()?;

    // Generate geometry
    if greedy {
        generate_greedy_geometry(schematic, &mut obj_file, use_textures)?;
    } else {
        generate_naive_geometry(schematic, &mut obj_file, hollow, skip_air, use_textures)?;
    }

    obj_file.flush()?;
    Ok(())
}

/// Generate geometry using naive per-block approach
fn generate_naive_geometry<W: Write>(
    schematic: &UnifiedSchematic,
    obj_file: &mut W,
    hollow: bool,
    skip_air: bool,
    use_textures: bool,
) -> std::io::Result<()> {
    let total_positions = schematic.width as u64 * schematic.height as u64 * schematic.length as u64;
    let pb = create_progress_bar(total_positions, "Generating geometry");

    let mut vertex_index = 1u32;
    let mut current_material = String::new();
    let mut blocks_written = 0u64;
    let mut processed = 0u64;
    let (w, h, l) = (schematic.width, schematic.height, schematic.length);

    for y in 0..h {
        for z in 0..l {
            for x in 0..w {
                processed += 1;
                if processed % 100_000 == 0 {
                    pb.set_position(processed);
                }

                if let Some(block) = schematic.get_block(x, y, z) {
                    if skip_air && block.is_air() { continue; }
                    if hollow && !is_exposed_fast(schematic, x, y, z, w, h, l) { continue; }

                    let mat_name = block.display_name().replace([':', '[', ']', '=', ','], "_");
                    if mat_name != current_material {
                        writeln!(obj_file, "usemtl {}", mat_name)?;
                        current_material = mat_name;
                    }

                    write_cube(obj_file, x as f32, y as f32, z as f32, vertex_index, use_textures)?;
                    vertex_index += 8;
                    blocks_written += 1;
                }
            }
        }
    }

    pb.finish_with_message(format!("Written {} blocks ({} vertices)", blocks_written, vertex_index - 1));
    Ok(())
}

/// Generate geometry using greedy meshing algorithm
fn generate_greedy_geometry<W: Write>(
    schematic: &UnifiedSchematic,
    obj_file: &mut W,
    use_textures: bool,
) -> std::io::Result<()> {
    let (w, h, l) = (schematic.width as usize, schematic.height as usize, schematic.length as usize);

    // Collect all quads using greedy meshing
    let mut all_quads: Vec<GreedyQuad> = Vec::new();

    let total_slices = (w + h + l) * 2; // Approximate for progress
    let pb = create_progress_bar(total_slices as u64, "Greedy meshing");
    let mut slice_count = 0u64;

    // Process each face direction
    for dir in FaceDir::all() {
        let quads = greedy_mesh_direction(schematic, dir, w, h, l, &pb, &mut slice_count);
        all_quads.extend(quads);
    }

    pb.finish_with_message(format!("Generated {} merged quads", all_quads.len()));

    // Sort quads by material for efficient rendering
    all_quads.sort_by(|a, b| a.material.cmp(&b.material));

    // Write quads to OBJ
    let pb = create_progress_bar(all_quads.len() as u64, "Writing OBJ");

    let mut vertex_index = 1u32;
    let mut vt_index = 1u32;
    let mut current_material = String::new();

    for (i, quad) in all_quads.iter().enumerate() {
        if i % 10_000 == 0 {
            pb.set_position(i as u64);
        }

        if quad.material != current_material {
            writeln!(obj_file, "usemtl {}", quad.material)?;
            current_material = quad.material.clone();
        }

        // Write 4 vertices
        for v in &quad.vertices {
            writeln!(obj_file, "v {} {} {}", v.0, v.1, v.2)?;
        }

        // Write face with UV coordinates
        if use_textures {
            // Write UV coordinates for this quad - tile texture based on quad size
            let (w, h) = (quad.size.0 as f32, quad.size.1 as f32);
            writeln!(obj_file, "vt 0 0")?;
            writeln!(obj_file, "vt {} 0", w)?;
            writeln!(obj_file, "vt {} {}", w, h)?;
            writeln!(obj_file, "vt 0 {}", h)?;

            writeln!(obj_file, "f {}/{} {}/{} {}/{} {}/{}",
                vertex_index, vt_index,
                vertex_index + 1, vt_index + 1,
                vertex_index + 2, vt_index + 2,
                vertex_index + 3, vt_index + 3)?;
            vt_index += 4;
        } else {
            writeln!(obj_file, "f {} {} {} {}",
                vertex_index, vertex_index + 1, vertex_index + 2, vertex_index + 3)?;
        }
        vertex_index += 4;
    }

    pb.finish_with_message(format!("Written {} quads ({} vertices)", all_quads.len(), vertex_index - 1));
    Ok(())
}

/// Greedy mesh one direction (e.g., all +Y faces)
fn greedy_mesh_direction(
    schematic: &UnifiedSchematic,
    dir: FaceDir,
    w: usize, h: usize, l: usize,
    pb: &ProgressBar,
    slice_count: &mut u64,
) -> Vec<GreedyQuad> {
    let mut quads = Vec::new();

    // Determine iteration order based on direction
    let (d1_size, d2_size, slice_count_total) = match dir {
        FaceDir::XNeg | FaceDir::XPos => (h, l, w),
        FaceDir::YNeg | FaceDir::YPos => (w, l, h),
        FaceDir::ZNeg | FaceDir::ZPos => (w, h, l),
    };

    // Process each slice
    for slice_idx in 0..slice_count_total {
        *slice_count += 1;
        if *slice_count % 10 == 0 {
            pb.set_position(*slice_count);
        }

        // Build mask of exposed faces for this slice
        // mask[d1][d2] = Some(material_name) if face is visible, None otherwise
        let mut mask: Vec<Vec<Option<String>>> = vec![vec![None; d2_size]; d1_size];

        for d1 in 0..d1_size {
            for d2 in 0..d2_size {
                let (x, y, z) = match dir {
                    FaceDir::XNeg => (slice_idx, d1, d2),
                    FaceDir::XPos => (slice_idx, d1, d2),
                    FaceDir::YNeg => (d1, slice_idx, d2),
                    FaceDir::YPos => (d1, slice_idx, d2),
                    FaceDir::ZNeg => (d1, d2, slice_idx),
                    FaceDir::ZPos => (d1, d2, slice_idx),
                };

                if x >= w || y >= h || z >= l { continue; }

                if let Some(block) = schematic.get_block(x as u16, y as u16, z as u16) {
                    if block.is_air() { continue; }

                    // Check if this face is exposed
                    let neighbor = match dir {
                        FaceDir::XNeg => if x == 0 { None } else { schematic.get_block((x - 1) as u16, y as u16, z as u16) },
                        FaceDir::XPos => schematic.get_block((x + 1) as u16, y as u16, z as u16),
                        FaceDir::YNeg => if y == 0 { None } else { schematic.get_block(x as u16, (y - 1) as u16, z as u16) },
                        FaceDir::YPos => schematic.get_block(x as u16, (y + 1) as u16, z as u16),
                        FaceDir::ZNeg => if z == 0 { None } else { schematic.get_block(x as u16, y as u16, (z - 1) as u16) },
                        FaceDir::ZPos => schematic.get_block(x as u16, y as u16, (z + 1) as u16),
                    };

                    let is_exposed = match neighbor {
                        None => true,
                        Some(n) => n.is_air(),
                    };

                    if is_exposed {
                        let mat_name = block.display_name().replace([':', '[', ']', '=', ','], "_");
                        mask[d1][d2] = Some(mat_name);
                    }
                }
            }
        }

        // Greedy mesh the mask
        let slice_quads = greedy_mesh_2d(&mask, d1_size, d2_size, slice_idx, dir, w, h, l);
        quads.extend(slice_quads);
    }

    quads
}

/// Greedy mesh a 2D mask into rectangles
fn greedy_mesh_2d(
    mask: &[Vec<Option<String>>],
    d1_size: usize,
    d2_size: usize,
    slice_idx: usize,
    dir: FaceDir,
    w: usize, h: usize, l: usize,
) -> Vec<GreedyQuad> {
    let mut quads = Vec::new();
    let mut used = vec![vec![false; d2_size]; d1_size];

    for d1 in 0..d1_size {
        for d2 in 0..d2_size {
            if used[d1][d2] { continue; }

            let material = match &mask[d1][d2] {
                Some(m) => m.clone(),
                None => continue,
            };

            // Find maximum width (d2 direction)
            let mut width = 1;
            while d2 + width < d2_size
                && !used[d1][d2 + width]
                && mask[d1][d2 + width].as_ref() == Some(&material)
            {
                width += 1;
            }

            // Find maximum height (d1 direction)
            let mut height = 1;
            'outer: while d1 + height < d1_size {
                for dw in 0..width {
                    if used[d1 + height][d2 + dw]
                        || mask[d1 + height][d2 + dw].as_ref() != Some(&material)
                    {
                        break 'outer;
                    }
                }
                height += 1;
            }

            // Mark as used
            for dh in 0..height {
                for dw in 0..width {
                    used[d1 + dh][d2 + dw] = true;
                }
            }

            // Create quad with proper vertices
            let vertices = create_quad_vertices(
                slice_idx, d1, d2, width, height, dir, w, h, l
            );

            quads.push(GreedyQuad { material, vertices, size: (width, height) });
        }
    }

    quads
}

/// Create 4 vertices for a quad based on direction and position
fn create_quad_vertices(
    slice: usize,
    d1: usize,
    d2: usize,
    width: usize,
    height: usize,
    dir: FaceDir,
    _w: usize, _h: usize, _l: usize,
) -> [(f32, f32, f32); 4] {
    let s = slice as f32;
    let (d1f, d2f) = (d1 as f32, d2 as f32);
    let (wf, hf) = (width as f32, height as f32);

    match dir {
        FaceDir::XNeg => [
            (s, d1f, d2f),
            (s, d1f, d2f + wf),
            (s, d1f + hf, d2f + wf),
            (s, d1f + hf, d2f),
        ],
        FaceDir::XPos => [
            (s + 1.0, d1f, d2f + wf),
            (s + 1.0, d1f, d2f),
            (s + 1.0, d1f + hf, d2f),
            (s + 1.0, d1f + hf, d2f + wf),
        ],
        FaceDir::YNeg => [
            (d1f, s, d2f + wf),
            (d1f, s, d2f),
            (d1f + hf, s, d2f),
            (d1f + hf, s, d2f + wf),
        ],
        FaceDir::YPos => [
            (d1f, s + 1.0, d2f),
            (d1f, s + 1.0, d2f + wf),
            (d1f + hf, s + 1.0, d2f + wf),
            (d1f + hf, s + 1.0, d2f),
        ],
        FaceDir::ZNeg => [
            (d1f + hf, d2f, s),
            (d1f, d2f, s),
            (d1f, d2f + wf, s),
            (d1f + hf, d2f + wf, s),
        ],
        FaceDir::ZPos => [
            (d1f, d2f, s + 1.0),
            (d1f + hf, d2f, s + 1.0),
            (d1f + hf, d2f + wf, s + 1.0),
            (d1f, d2f + wf, s + 1.0),
        ],
    }
}

#[inline]
fn is_exposed_fast(schematic: &UnifiedSchematic, x: u16, y: u16, z: u16, w: u16, h: u16, l: u16) -> bool {
    if x == 0 || x == w - 1 || y == 0 || y == h - 1 || z == 0 || z == l - 1 {
        return true;
    }
    if let Some(block) = schematic.get_block(x - 1, y, z) { if block.is_air() { return true; } } else { return true; }
    if let Some(block) = schematic.get_block(x + 1, y, z) { if block.is_air() { return true; } } else { return true; }
    if let Some(block) = schematic.get_block(x, y - 1, z) { if block.is_air() { return true; } } else { return true; }
    if let Some(block) = schematic.get_block(x, y + 1, z) { if block.is_air() { return true; } } else { return true; }
    if let Some(block) = schematic.get_block(x, y, z - 1) { if block.is_air() { return true; } } else { return true; }
    if let Some(block) = schematic.get_block(x, y, z + 1) { if block.is_air() { return true; } } else { return true; }
    false
}

#[inline]
fn write_cube<W: Write>(file: &mut W, x: f32, y: f32, z: f32, vi: u32, use_textures: bool) -> std::io::Result<()> {
    let x1 = x + 1.0;
    let y1 = y + 1.0;
    let z1 = z + 1.0;

    write!(file, "v {} {} {}\nv {} {} {}\nv {} {} {}\nv {} {} {}\nv {} {} {}\nv {} {} {}\nv {} {} {}\nv {} {} {}\n",
        x, y, z, x1, y, z, x1, y1, z, x, y1, z, x, y, z1, x1, y, z1, x1, y1, z1, x, y1, z1)?;

    if use_textures {
        write!(file,
            "f {}/1 {}/2 {}/3 {}/4\nf {}/1 {}/2 {}/3 {}/4\nf {}/1 {}/2 {}/3 {}/4\nf {}/1 {}/2 {}/3 {}/4\nf {}/1 {}/2 {}/3 {}/4\nf {}/1 {}/2 {}/3 {}/4\n",
            vi, vi + 1, vi + 2, vi + 3, vi + 5, vi + 4, vi + 7, vi + 6,
            vi + 4, vi, vi + 3, vi + 7, vi + 1, vi + 5, vi + 6, vi + 2,
            vi + 4, vi + 5, vi + 1, vi, vi + 3, vi + 2, vi + 6, vi + 7)?;
    } else {
        write!(file,
            "f {} {} {} {}\nf {} {} {} {}\nf {} {} {} {}\nf {} {} {} {}\nf {} {} {} {}\nf {} {} {} {}\n",
            vi, vi + 1, vi + 2, vi + 3, vi + 5, vi + 4, vi + 7, vi + 6,
            vi + 4, vi, vi + 3, vi + 7, vi + 1, vi + 5, vi + 6, vi + 2,
            vi + 4, vi + 5, vi + 1, vi, vi + 3, vi + 2, vi + 6, vi + 7)?;
    }
    Ok(())
}

/// Generate HTML viewer
pub fn export_html<P: AsRef<Path>>(
    schematic: &UnifiedSchematic,
    html_path: P,
    max_blocks: usize,
) -> std::io::Result<()> {
    let pb = create_progress_bar(max_blocks as u64, "Building HTML data");

    let mut blocks_json = String::with_capacity(max_blocks * 20);
    blocks_json.push('[');
    let mut count = 0u64;
    let (w, h, l) = (schematic.width, schematic.height, schematic.length);

    'outer: for y in 0..h {
        for z in 0..l {
            for x in 0..w {
                if let Some(block) = schematic.get_block(x, y, z) {
                    if block.is_air() { continue; }
                    if !is_exposed_fast(schematic, x, y, z, w, h, l) { continue; }
                    if count >= max_blocks as u64 { break 'outer; }

                    let (r, g, b) = get_block_color(&block.name);
                    let color = ((r * 255.0) as u32) << 16 | ((g * 255.0) as u32) << 8 | (b * 255.0) as u32;

                    if count > 0 { blocks_json.push(','); }
                    blocks_json.push_str(&format!("[{},{},{},{}]", x, y, z, color));
                    count += 1;
                    if count % 10_000 == 0 { pb.set_position(count); }
                }
            }
        }
    }
    blocks_json.push(']');
    pb.finish_with_message(format!("Included {} blocks", count));

    let mut file = BufWriter::new(std::fs::File::create(html_path)?);
    let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Schematic Viewer - {w}x{h}x{l}</title>
    <style>
        body {{ margin: 0; overflow: hidden; }}
        #info {{ position: absolute; top: 10px; left: 10px; color: white; font-family: monospace; background: rgba(0,0,0,0.5); padding: 10px; border-radius: 5px; }}
    </style>
</head>
<body>
    <div id="info">Schematic: {w}x{h}x{l}<br>Blocks shown: {count}<br>Drag to rotate, scroll to zoom</div>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/controls/OrbitControls.js"></script>
    <script>
        const blocks = {blocks};
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0x1a1a2e);
        const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 10000);
        camera.position.set({cx}, {cy}, {cz});
        const renderer = new THREE.WebGLRenderer({{ antialias: true }});
        renderer.setSize(window.innerWidth, window.innerHeight);
        document.body.appendChild(renderer.domElement);
        const controls = new THREE.OrbitControls(camera, renderer.domElement);
        controls.target.set({tx}, {ty}, {tz});
        controls.update();
        scene.add(new THREE.AmbientLight(0x404040, 0.5));
        const dl = new THREE.DirectionalLight(0xffffff, 0.8);
        dl.position.set(1, 1, 1);
        scene.add(dl);
        const geometry = new THREE.BoxGeometry(1, 1, 1);
        const colorGroups = {{}};
        blocks.forEach(([x, y, z, color]) => {{ if (!colorGroups[color]) colorGroups[color] = []; colorGroups[color].push([x, y, z]); }});
        Object.entries(colorGroups).forEach(([color, positions]) => {{
            const mat = new THREE.MeshLambertMaterial({{ color: parseInt(color) }});
            const mesh = new THREE.InstancedMesh(geometry, mat, positions.length);
            const matrix = new THREE.Matrix4();
            positions.forEach(([x, y, z], i) => {{ matrix.setPosition(x, y, z); mesh.setMatrixAt(i, matrix); }});
            scene.add(mesh);
        }});
        const grid = new THREE.GridHelper({grid}, 10);
        grid.position.y = -0.5;
        scene.add(grid);
        function animate() {{ requestAnimationFrame(animate); controls.update(); renderer.render(scene, camera); }}
        animate();
        window.addEventListener('resize', () => {{ camera.aspect = window.innerWidth / window.innerHeight; camera.updateProjectionMatrix(); renderer.setSize(window.innerWidth, window.innerHeight); }});
    </script>
</body>
</html>"#,
        w = w, h = h, l = l, count = count, blocks = blocks_json,
        cx = w as f32 * 1.5, cy = h as f32 * 1.2, cz = l as f32 * 1.5,
        tx = w as f32 / 2.0, ty = h as f32 / 2.0, tz = l as f32 / 2.0,
        grid = w.max(l) as f32 * 1.5,
    );
    file.write_all(html.as_bytes())?;
    file.flush()?;
    Ok(())
}
