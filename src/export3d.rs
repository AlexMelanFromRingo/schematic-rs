//! 3D export functionality for schematics
//!
//! Supports exporting to OBJ format with MTL materials and optional textures

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use crate::UnifiedSchematic;
use crate::textures::TextureManager;

/// Block color mapping (approximate Minecraft colors)
pub fn get_block_color(name: &str) -> (f32, f32, f32) {
    let name = name.strip_prefix("minecraft:").unwrap_or(name);

    // Color mapping (R, G, B in 0.0-1.0 range)
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

        // Blackstone
        "blackstone" | "gilded_blackstone" => (0.15, 0.13, 0.15),
        "polished_blackstone" => (0.12, 0.1, 0.12),
        "polished_blackstone_bricks" | "cracked_polished_blackstone_bricks" => (0.13, 0.11, 0.13),
        "chiseled_polished_blackstone" => (0.14, 0.12, 0.14),

        // Basalt
        "basalt" | "polished_basalt" => (0.3, 0.3, 0.32),
        "smooth_basalt" => (0.25, 0.25, 0.27),

        // Dirt/grass
        "dirt" | "coarse_dirt" | "rooted_dirt" => (0.55, 0.4, 0.3),
        "grass_block" => (0.4, 0.6, 0.3),
        "podzol" => (0.45, 0.35, 0.25),
        "mycelium" => (0.5, 0.45, 0.5),
        "mud" => (0.35, 0.3, 0.35),
        "packed_mud" => (0.5, 0.4, 0.35),
        "mud_bricks" => (0.55, 0.45, 0.4),

        // Sand/gravel
        "sand" => (0.85, 0.8, 0.6),
        "red_sand" => (0.75, 0.45, 0.25),
        "gravel" => (0.55, 0.52, 0.5),
        "clay" => (0.6, 0.62, 0.68),

        // Sandstone
        "sandstone" | "cut_sandstone" | "smooth_sandstone" | "chiseled_sandstone" => (0.85, 0.78, 0.55),
        "red_sandstone" | "cut_red_sandstone" | "smooth_red_sandstone" => (0.7, 0.4, 0.2),

        // Wood (generic brown tones)
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

        // Leaves
        n if n.contains("leaves") => (0.25, 0.5, 0.2),

        // Bricks
        "bricks" | "brick_stairs" | "brick_slab" => (0.6, 0.35, 0.3),
        "stone_bricks" | "mossy_stone_bricks" | "cracked_stone_bricks" | "chiseled_stone_bricks" => (0.48, 0.48, 0.48),
        "nether_bricks" | "cracked_nether_bricks" | "chiseled_nether_bricks" => (0.25, 0.15, 0.2),
        "red_nether_bricks" => (0.35, 0.12, 0.12),
        "end_stone_bricks" => (0.85, 0.85, 0.7),
        "prismarine_bricks" => (0.4, 0.6, 0.55),

        // Metals
        "iron_block" => (0.75, 0.75, 0.75),
        "gold_block" => (0.9, 0.75, 0.2),
        "diamond_block" => (0.4, 0.8, 0.8),
        "emerald_block" => (0.3, 0.7, 0.35),
        "lapis_block" => (0.2, 0.3, 0.7),
        "redstone_block" => (0.7, 0.15, 0.1),
        "coal_block" => (0.15, 0.15, 0.15),
        "copper_block" | "cut_copper" => (0.7, 0.45, 0.35),
        "netherite_block" => (0.25, 0.22, 0.25),

        // Ores
        n if n.contains("ore") => (0.5, 0.5, 0.5),

        // Glass
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

        // Wool
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

        // Concrete
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

        // Terracotta
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

        // Nether
        "netherrack" => (0.5, 0.25, 0.25),
        "soul_sand" => (0.35, 0.28, 0.22),
        "soul_soil" => (0.32, 0.25, 0.2),
        "glowstone" => (0.85, 0.7, 0.4),
        "magma_block" => (0.55, 0.25, 0.1),
        "nether_wart_block" => (0.5, 0.15, 0.15),
        "warped_wart_block" => (0.1, 0.5, 0.5),
        "shroomlight" => (0.9, 0.6, 0.4),

        // End
        "end_stone" => (0.85, 0.85, 0.7),
        "purpur_block" | "purpur_pillar" => (0.6, 0.45, 0.6),

        // Quartz
        "quartz_block" | "smooth_quartz" | "quartz_bricks" | "chiseled_quartz_block" | "quartz_pillar" => (0.9, 0.88, 0.85),

        // Prismarine
        "prismarine" => (0.4, 0.55, 0.5),
        "dark_prismarine" => (0.25, 0.4, 0.38),
        "sea_lantern" => (0.7, 0.85, 0.85),

        // Misc
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

        // Redstone
        "redstone_lamp" => (0.55, 0.35, 0.2),
        "redstone_wire" | "redstone_torch" => (0.6, 0.15, 0.1),
        n if n.contains("piston") => (0.55, 0.45, 0.35),
        "observer" | "dropper" | "dispenser" => (0.45, 0.45, 0.45),
        "hopper" => (0.4, 0.4, 0.45),

        // Water/lava
        "water" => (0.2, 0.4, 0.8),
        "lava" => (0.9, 0.45, 0.1),

        // Default
        _ => (0.5, 0.5, 0.5),
    }
}

/// Generate OBJ file from schematic
pub fn export_obj<P: AsRef<Path>>(
    schematic: &UnifiedSchematic,
    obj_path: P,
    hollow: bool,
    skip_air: bool,
) -> std::io::Result<()> {
    export_obj_with_textures(schematic, obj_path, hollow, skip_air, None)
}

/// Generate OBJ file from schematic with optional textures
pub fn export_obj_with_textures<P: AsRef<Path>>(
    schematic: &UnifiedSchematic,
    obj_path: P,
    hollow: bool,
    skip_air: bool,
    textures: Option<&TextureManager>,
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

    let mut obj_file = std::fs::File::create(obj_path)?;
    let mut mtl_file = std::fs::File::create(&mtl_path)?;

    // Write OBJ header
    writeln!(obj_file, "# Minecraft Schematic Export")?;
    writeln!(obj_file, "# Generated by schem-tool")?;
    writeln!(obj_file, "# Dimensions: {}x{}x{}", schematic.width, schematic.height, schematic.length)?;
    writeln!(obj_file, "mtllib {}", mtl_path.file_name().unwrap().to_string_lossy())?;
    writeln!(obj_file)?;

    // Write UV coordinates for textured cubes (shared for all cubes)
    if use_textures {
        writeln!(obj_file, "# Texture coordinates")?;
        writeln!(obj_file, "vt 0 0")?;
        writeln!(obj_file, "vt 1 0")?;
        writeln!(obj_file, "vt 1 1")?;
        writeln!(obj_file, "vt 0 1")?;
        writeln!(obj_file)?;
    }

    // Write MTL header
    writeln!(mtl_file, "# Minecraft Block Materials")?;
    writeln!(mtl_file)?;

    // Collect unique materials
    let mut materials: HashMap<String, (f32, f32, f32, Option<String>)> = HashMap::new();

    for y in 0..schematic.height {
        for z in 0..schematic.length {
            for x in 0..schematic.width {
                if let Some(block) = schematic.get_block(x, y, z) {
                    if skip_air && block.is_air() {
                        continue;
                    }
                    let mat_name = block.display_name().replace([':', '[', ']', '=', ','], "_");
                    if !materials.contains_key(&mat_name) {
                        let color = get_block_color(&block.name);

                        // Try to find texture
                        let texture_file = if let (Some(tex_mgr), Some(tex_out_dir)) = (textures, &tex_dir) {
                            if let Some(tex_path) = tex_mgr.get_texture(&block.name) {
                                // Copy texture to output directory
                                let tex_name = format!("{}.png", mat_name);
                                let dest = tex_out_dir.join(&tex_name);
                                if std::fs::copy(tex_path, &dest).is_ok() {
                                    Some(format!("textures/{}", tex_name))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        materials.insert(mat_name.clone(), (color.0, color.1, color.2, texture_file));
                    }
                }
            }
        }
    }

    // Write materials to MTL
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

    // Generate geometry
    let mut vertex_index = 1u32;
    let mut current_material = String::new();

    for y in 0..schematic.height {
        for z in 0..schematic.length {
            for x in 0..schematic.width {
                if let Some(block) = schematic.get_block(x, y, z) {
                    if skip_air && block.is_air() {
                        continue;
                    }

                    // Check if block is visible (for hollow mode)
                    if hollow && !is_exposed(schematic, x, y, z) {
                        continue;
                    }

                    let mat_name = block.display_name().replace([':', '[', ']', '=', ','], "_");

                    // Switch material if needed
                    if mat_name != current_material {
                        writeln!(obj_file, "usemtl {}", mat_name)?;
                        current_material = mat_name;
                    }

                    // Write cube vertices and faces
                    let (x, y, z) = (x as f32, y as f32, z as f32);
                    write_cube(&mut obj_file, x, y, z, vertex_index, use_textures)?;
                    vertex_index += 8;
                }
            }
        }
    }

    Ok(())
}

/// Check if a block is exposed (has at least one air neighbor)
fn is_exposed(schematic: &UnifiedSchematic, x: u16, y: u16, z: u16) -> bool {
    let neighbors = [
        (x.wrapping_sub(1), y, z),
        (x + 1, y, z),
        (x, y.wrapping_sub(1), z),
        (x, y + 1, z),
        (x, y, z.wrapping_sub(1)),
        (x, y, z + 1),
    ];

    for (nx, ny, nz) in neighbors {
        if nx >= schematic.width || ny >= schematic.height || nz >= schematic.length {
            return true; // Edge of schematic
        }
        if let Some(neighbor) = schematic.get_block(nx, ny, nz) {
            if neighbor.is_air() {
                return true;
            }
        } else {
            return true;
        }
    }

    false
}

/// Write a cube to OBJ file
fn write_cube<W: Write>(file: &mut W, x: f32, y: f32, z: f32, vi: u32, use_textures: bool) -> std::io::Result<()> {
    // 8 vertices of a unit cube
    writeln!(file, "v {} {} {}", x, y, z)?;
    writeln!(file, "v {} {} {}", x + 1.0, y, z)?;
    writeln!(file, "v {} {} {}", x + 1.0, y + 1.0, z)?;
    writeln!(file, "v {} {} {}", x, y + 1.0, z)?;
    writeln!(file, "v {} {} {}", x, y, z + 1.0)?;
    writeln!(file, "v {} {} {}", x + 1.0, y, z + 1.0)?;
    writeln!(file, "v {} {} {}", x + 1.0, y + 1.0, z + 1.0)?;
    writeln!(file, "v {} {} {}", x, y + 1.0, z + 1.0)?;

    if use_textures {
        // Faces with texture coordinates (vt indices are 1-4)
        // Front (z-)
        writeln!(file, "f {}/1 {}/2 {}/3 {}/4", vi, vi + 1, vi + 2, vi + 3)?;
        // Back (z+)
        writeln!(file, "f {}/1 {}/2 {}/3 {}/4", vi + 5, vi + 4, vi + 7, vi + 6)?;
        // Left (x-)
        writeln!(file, "f {}/1 {}/2 {}/3 {}/4", vi + 4, vi, vi + 3, vi + 7)?;
        // Right (x+)
        writeln!(file, "f {}/1 {}/2 {}/3 {}/4", vi + 1, vi + 5, vi + 6, vi + 2)?;
        // Bottom (y-)
        writeln!(file, "f {}/1 {}/2 {}/3 {}/4", vi + 4, vi + 5, vi + 1, vi)?;
        // Top (y+)
        writeln!(file, "f {}/1 {}/2 {}/3 {}/4", vi + 3, vi + 2, vi + 6, vi + 7)?;
    } else {
        // 6 faces (quads) without textures
        // Front (z-)
        writeln!(file, "f {} {} {} {}", vi, vi + 1, vi + 2, vi + 3)?;
        // Back (z+)
        writeln!(file, "f {} {} {} {}", vi + 5, vi + 4, vi + 7, vi + 6)?;
        // Left (x-)
        writeln!(file, "f {} {} {} {}", vi + 4, vi, vi + 3, vi + 7)?;
        // Right (x+)
        writeln!(file, "f {} {} {} {}", vi + 1, vi + 5, vi + 6, vi + 2)?;
        // Bottom (y-)
        writeln!(file, "f {} {} {} {}", vi + 4, vi + 5, vi + 1, vi)?;
        // Top (y+)
        writeln!(file, "f {} {} {} {}", vi + 3, vi + 2, vi + 6, vi + 7)?;
    }

    Ok(())
}

/// Generate a simple HTML viewer with Three.js
pub fn export_html<P: AsRef<Path>>(
    schematic: &UnifiedSchematic,
    html_path: P,
    max_blocks: usize,
) -> std::io::Result<()> {
    let mut file = std::fs::File::create(html_path)?;

    // Build block data
    let mut blocks_json = String::from("[");
    let mut count = 0;

    for y in 0..schematic.height {
        for z in 0..schematic.length {
            for x in 0..schematic.width {
                if let Some(block) = schematic.get_block(x, y, z) {
                    if block.is_air() {
                        continue;
                    }

                    // Only include exposed blocks
                    if !is_exposed(schematic, x, y, z) {
                        continue;
                    }

                    if count >= max_blocks {
                        break;
                    }

                    let (r, g, b) = get_block_color(&block.name);
                    let color = ((r * 255.0) as u32) << 16
                        | ((g * 255.0) as u32) << 8
                        | (b * 255.0) as u32;

                    if count > 0 {
                        blocks_json.push(',');
                    }
                    blocks_json.push_str(&format!("[{},{},{},{}]", x, y, z, color));
                    count += 1;
                }
            }
            if count >= max_blocks {
                break;
            }
        }
        if count >= max_blocks {
            break;
        }
    }
    blocks_json.push(']');

    let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Schematic Viewer - {w}x{h}x{l}</title>
    <style>
        body {{ margin: 0; overflow: hidden; }}
        #info {{
            position: absolute;
            top: 10px;
            left: 10px;
            color: white;
            font-family: monospace;
            background: rgba(0,0,0,0.5);
            padding: 10px;
            border-radius: 5px;
        }}
    </style>
</head>
<body>
    <div id="info">
        Schematic: {w}x{h}x{l}<br>
        Blocks shown: {count}<br>
        Drag to rotate, scroll to zoom
    </div>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/controls/OrbitControls.js"></script>
    <script>
        const blocks = {blocks};

        // Scene setup
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

        // Lighting
        const ambientLight = new THREE.AmbientLight(0x404040, 0.5);
        scene.add(ambientLight);

        const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
        directionalLight.position.set(1, 1, 1);
        scene.add(directionalLight);

        const directionalLight2 = new THREE.DirectionalLight(0xffffff, 0.3);
        directionalLight2.position.set(-1, 0.5, -1);
        scene.add(directionalLight2);

        // Create instanced mesh for performance
        const geometry = new THREE.BoxGeometry(1, 1, 1);
        const material = new THREE.MeshLambertMaterial({{ vertexColors: true }});

        // Group blocks by color for better performance
        const colorGroups = {{}};
        blocks.forEach(([x, y, z, color]) => {{
            if (!colorGroups[color]) colorGroups[color] = [];
            colorGroups[color].push([x, y, z]);
        }});

        Object.entries(colorGroups).forEach(([color, positions]) => {{
            const mat = new THREE.MeshLambertMaterial({{ color: parseInt(color) }});
            const mesh = new THREE.InstancedMesh(geometry, mat, positions.length);

            const matrix = new THREE.Matrix4();
            positions.forEach(([x, y, z], i) => {{
                matrix.setPosition(x, y, z);
                mesh.setMatrixAt(i, matrix);
            }});

            scene.add(mesh);
        }});

        // Grid helper
        const gridHelper = new THREE.GridHelper({grid}, 10);
        gridHelper.position.y = -0.5;
        scene.add(gridHelper);

        // Animation loop
        function animate() {{
            requestAnimationFrame(animate);
            controls.update();
            renderer.render(scene, camera);
        }}
        animate();

        // Handle resize
        window.addEventListener('resize', () => {{
            camera.aspect = window.innerWidth / window.innerHeight;
            camera.updateProjectionMatrix();
            renderer.setSize(window.innerWidth, window.innerHeight);
        }});
    </script>
</body>
</html>"#,
        w = schematic.width,
        h = schematic.height,
        l = schematic.length,
        count = count,
        blocks = blocks_json,
        cx = schematic.width as f32 * 1.5,
        cy = schematic.height as f32 * 1.2,
        cz = schematic.length as f32 * 1.5,
        tx = schematic.width as f32 / 2.0,
        ty = schematic.height as f32 / 2.0,
        tz = schematic.length as f32 / 2.0,
        grid = schematic.width.max(schematic.length) as f32 * 1.5,
    );

    file.write_all(html.as_bytes())?;
    Ok(())
}
