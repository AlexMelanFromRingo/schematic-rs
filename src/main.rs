use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use colored::Colorize;
use schem_tool::UnifiedSchematic;
use std::path::PathBuf;
use tabled::{Table, Tabled, settings::Style};

/// Format Unix timestamp (milliseconds) to human-readable date
fn format_timestamp(millis: i64) -> String {
    DateTime::from_timestamp_millis(millis)
        .map(|dt: DateTime<Utc>| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| format!("{} (invalid)", millis))
}

#[derive(Parser)]
#[command(name = "schem-tool")]
#[command(about = "Minecraft schematic file parser and analyzer", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show general information about the schematic
    Info {
        /// Path to the schematic file (.schematic or .schem)
        file: PathBuf,
    },

    /// List all blocks with counts
    Blocks {
        /// Path to the schematic file
        file: PathBuf,

        /// Show only non-air blocks
        #[arg(short, long)]
        no_air: bool,

        /// Sort by count (descending)
        #[arg(short, long)]
        sort: bool,

        /// Limit number of results
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// List unique block types with their states
    Palette {
        /// Path to the schematic file
        file: PathBuf,
    },

    /// List block entities (chests, signs, etc.)
    BlockEntities {
        /// Path to the schematic file
        file: PathBuf,

        /// Filter by entity type
        #[arg(short = 't', long)]
        entity_type: Option<String>,

        /// Show full data
        #[arg(short, long)]
        verbose: bool,
    },

    /// List entities (mobs, items, etc.)
    Entities {
        /// Path to the schematic file
        file: PathBuf,

        /// Show full data
        #[arg(short, long)]
        verbose: bool,
    },

    /// List signs with their text content
    Signs {
        /// Path to the schematic file
        file: PathBuf,
    },

    /// Show metadata
    Metadata {
        /// Path to the schematic file
        file: PathBuf,
    },

    /// Get block at specific position
    GetBlock {
        /// Path to the schematic file
        file: PathBuf,

        /// X coordinate
        #[arg(short)]
        x: u16,

        /// Y coordinate
        #[arg(short)]
        y: u16,

        /// Z coordinate
        #[arg(short)]
        z: u16,
    },

    /// Search for blocks by name
    Search {
        /// Path to the schematic file
        file: PathBuf,

        /// Block name pattern (partial match)
        pattern: String,

        /// Show positions
        #[arg(short, long)]
        positions: bool,

        /// Limit number of results
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// Export block list to CSV
    Export {
        /// Path to the schematic file
        file: PathBuf,

        /// Output CSV file
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Calculate raw materials needed (break down crafted items)
    Materials {
        /// Path to the schematic file
        file: PathBuf,

        /// Sort by count (descending)
        #[arg(short, long)]
        sort: bool,

        /// Show intermediate crafting steps
        #[arg(short, long)]
        verbose: bool,

        /// Limit number of results
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// Show layer-by-layer view (2D slice)
    Layer {
        /// Path to the schematic file
        file: PathBuf,

        /// Y level to show
        #[arg(short)]
        y: u16,

        /// Use simple ASCII characters
        #[arg(short, long)]
        ascii: bool,
    },

    /// Export to OBJ 3D model (viewable in Blender, Windows 3D Viewer, etc.)
    RenderObj {
        /// Path to the schematic file
        file: PathBuf,

        /// Output OBJ file path
        #[arg(short, long)]
        output: PathBuf,

        /// Only export visible (exposed) blocks
        #[arg(long)]
        hollow: bool,

        /// Use greedy meshing to reduce polygon count (10-100x smaller files)
        #[arg(short, long)]
        greedy: bool,

        /// Use Minecraft JSON models for accurate block geometry
        #[arg(long)]
        models: bool,

        /// Extract and apply textures from Minecraft installation
        #[arg(short, long)]
        textures: bool,

        /// Path to Minecraft directory or client.jar (e.g., ~/.minecraft or client.jar)
        #[arg(short, long)]
        minecraft: Option<PathBuf>,
    },

    /// Export to interactive HTML viewer (Three.js)
    RenderHtml {
        /// Path to the schematic file
        file: PathBuf,

        /// Output HTML file path
        #[arg(short, long)]
        output: PathBuf,

        /// Maximum blocks to render (default: 100000)
        #[arg(short, long, default_value = "100000")]
        max_blocks: usize,
    },

    /// Dump raw NBT structure for debugging
    Debug {
        /// Path to the schematic file
        file: PathBuf,
    },
}

#[derive(Tabled)]
struct BlockCount {
    #[tabled(rename = "Block")]
    name: String,
    #[tabled(rename = "Count")]
    count: usize,
    #[tabled(rename = "%")]
    percent: String,
}

#[derive(Tabled)]
struct BlockEntityRow {
    #[tabled(rename = "Type")]
    entity_type: String,
    #[tabled(rename = "Position")]
    position: String,
    #[tabled(rename = "Data")]
    data: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { file } => cmd_info(&file)?,
        Commands::Blocks { file, no_air, sort, limit } => cmd_blocks(&file, no_air, sort, limit)?,
        Commands::Palette { file } => cmd_palette(&file)?,
        Commands::BlockEntities { file, entity_type, verbose } => cmd_block_entities(&file, entity_type, verbose)?,
        Commands::Entities { file, verbose } => cmd_entities(&file, verbose)?,
        Commands::Signs { file } => cmd_signs(&file)?,
        Commands::Metadata { file } => cmd_metadata(&file)?,
        Commands::GetBlock { file, x, y, z } => cmd_get_block(&file, x, y, z)?,
        Commands::Search { file, pattern, positions, limit } => cmd_search(&file, &pattern, positions, limit)?,
        Commands::Export { file, output } => cmd_export(&file, &output)?,
        Commands::Materials { file, sort, verbose, limit } => cmd_materials(&file, sort, verbose, limit)?,
        Commands::Layer { file, y, ascii } => cmd_layer(&file, y, ascii)?,
        Commands::RenderObj { file, output, hollow, greedy, models, textures, minecraft } => cmd_render_obj(&file, &output, hollow, greedy, models, textures, minecraft.as_deref())?,
        Commands::RenderHtml { file, output, max_blocks } => cmd_render_html(&file, &output, max_blocks)?,
        Commands::Debug { file } => cmd_debug(&file)?,
    }

    Ok(())
}

fn cmd_info(file: &PathBuf) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;

    println!("{}", "=== Schematic Info ===".bold().cyan());
    println!();

    println!("{}  {}", "File:".bold(), file.display());
    println!("{}  {:?}", "Format:".bold(), schem.format);
    println!();

    println!("{}", "--- Dimensions ---".yellow());
    println!("  Width (X):  {}", schem.width);
    println!("  Height (Y): {}", schem.height);
    println!("  Length (Z): {}", schem.length);
    println!("  Volume:     {} blocks", schem.volume());
    println!();

    println!("{}", "--- Contents ---".yellow());
    println!("  Total blocks:    {}", schem.blocks.len());
    println!("  Solid blocks:    {}", schem.solid_blocks());
    println!("  Unique types:    {}", schem.block_counts().len());
    println!("  Block entities:  {}", schem.block_entities.len());
    println!("  Entities:        {}", schem.entities.len());
    println!();

    if schem.metadata.name.is_some() || schem.metadata.author.is_some() || schem.metadata.date.is_some() {
        println!("{}", "--- Metadata ---".yellow());
        if let Some(ref name) = schem.metadata.name {
            println!("  Name:   {}", name);
        }
        if let Some(ref author) = schem.metadata.author {
            println!("  Author: {}", author);
        }
        if let Some(date) = schem.metadata.date {
            println!("  Date:   {}", format_timestamp(date));
        }
        if !schem.metadata.required_mods.is_empty() {
            println!("  Mods:   {}", schem.metadata.required_mods.join(", "));
        }
    }

    Ok(())
}

fn cmd_blocks(file: &PathBuf, no_air: bool, sort: bool, limit: Option<usize>) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;
    let mut counts: Vec<(String, usize)> = schem.block_counts().into_iter().collect();

    if no_air {
        counts.retain(|(name, _)| {
            !matches!(name.as_str(), "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air" | "air")
        });
    }

    if sort {
        counts.sort_by(|a, b| b.1.cmp(&a.1));
    } else {
        counts.sort_by(|a, b| a.0.cmp(&b.0));
    }

    let total: usize = counts.iter().map(|(_, c)| c).sum();

    let rows: Vec<BlockCount> = counts.iter()
        .take(limit.unwrap_or(usize::MAX))
        .map(|(name, count)| {
            let percent = if total > 0 {
                format!("{:.1}", (*count as f64 / total as f64) * 100.0)
            } else {
                "0.0".to_string()
            };
            BlockCount {
                name: name.clone(),
                count: *count,
                percent,
            }
        })
        .collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", table);

    println!("\nTotal: {} blocks ({} types)", total, counts.len());

    Ok(())
}

fn cmd_palette(file: &PathBuf) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;

    println!("{}", "=== Block Palette ===".bold().cyan());
    println!();

    let unique = schem.unique_blocks();
    for block in &unique {
        if block.state.properties.is_empty() {
            println!("  {}", block.name);
        } else {
            println!("  {}", block.full_name().green());
            for (key, value) in &block.state.properties {
                println!("    {} = {}", key.yellow(), value);
            }
        }
    }

    println!("\nTotal: {} unique block states", unique.len());

    Ok(())
}

fn cmd_block_entities(file: &PathBuf, filter_type: Option<String>, verbose: bool) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;

    let entities: Vec<_> = schem.block_entities.iter()
        .filter(|be| {
            if let Some(ref filter) = filter_type {
                be.id.to_lowercase().contains(&filter.to_lowercase())
            } else {
                true
            }
        })
        .collect();

    if entities.is_empty() {
        println!("No block entities found.");
        return Ok(());
    }

    let rows: Vec<BlockEntityRow> = entities.iter().map(|be| {
        let data = if verbose {
            be.data.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            format!("{} fields", be.data.len())
        };

        BlockEntityRow {
            entity_type: be.id.clone(),
            position: format!("{}, {}, {}", be.pos.0, be.pos.1, be.pos.2),
            data,
        }
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", table);

    println!("\nTotal: {} block entities", entities.len());

    Ok(())
}

fn cmd_entities(file: &PathBuf, verbose: bool) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;

    if schem.entities.is_empty() {
        println!("No entities found.");
        return Ok(());
    }

    println!("{}", "=== Entities ===".bold().cyan());
    println!();

    for entity in &schem.entities {
        println!("  {} at ({:.1}, {:.1}, {:.1})",
            entity.id.green(),
            entity.pos.0, entity.pos.1, entity.pos.2
        );
        if verbose {
            for (key, value) in &entity.data {
                println!("    {}: {}", key.yellow(), value);
            }
        }
    }

    println!("\nTotal: {} entities", schem.entities.len());

    Ok(())
}

fn cmd_signs(file: &PathBuf) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;
    let signs = schem.get_signs();

    if signs.is_empty() {
        println!("No signs with text found.");
        return Ok(());
    }

    println!("{}", "=== Signs ===".bold().cyan());
    println!();

    for (i, (block_entity, text)) in signs.iter().enumerate() {
        let pos = block_entity.pos;
        println!("{}. Sign at ({}, {}, {})", (i + 1).to_string().bold(), pos.0, pos.1, pos.2);

        if !text.front.is_empty() {
            let has_content = text.front.iter().any(|s| !s.is_empty());
            if has_content {
                println!("   {}:", "Front".yellow());
                for line in &text.front {
                    if !line.is_empty() {
                        println!("     \"{}\"", line.green());
                    }
                }
            }
        }

        if !text.back.is_empty() {
            let has_content = text.back.iter().any(|s| !s.is_empty());
            if has_content {
                println!("   {}:", "Back".yellow());
                for line in &text.back {
                    if !line.is_empty() {
                        println!("     \"{}\"", line.green());
                    }
                }
            }
        }

        println!();
    }

    println!("Total: {} signs", signs.len());

    Ok(())
}

fn cmd_metadata(file: &PathBuf) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;
    let meta = &schem.metadata;

    println!("{}", "=== Metadata ===".bold().cyan());
    println!();

    println!("  Name:   {}", meta.name.as_deref().unwrap_or("(not set)"));
    println!("  Author: {}", meta.author.as_deref().unwrap_or("(not set)"));

    if let Some(date) = meta.date {
        println!("  Date:   {}", format_timestamp(date));
    } else {
        println!("  Date:   (not set)");
    }

    if meta.required_mods.is_empty() {
        println!("  Mods:   (none)");
    } else {
        println!("  Mods:");
        for mod_name in &meta.required_mods {
            println!("    - {}", mod_name);
        }
    }

    if !meta.extra.is_empty() {
        println!();
        println!("  Extra fields:");
        for (key, value) in &meta.extra {
            println!("    {}: {}", key.yellow(), value);
        }
    }

    Ok(())
}

fn cmd_get_block(file: &PathBuf, x: u16, y: u16, z: u16) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;

    if let Some(block) = schem.get_block(x, y, z) {
        println!("Block at ({}, {}, {}): {}", x, y, z, block.full_name().green());

        if !block.state.properties.is_empty() {
            println!();
            println!("Properties:");
            for (key, value) in &block.state.properties {
                println!("  {} = {}", key.yellow(), value);
            }
        }
    } else {
        println!("Position ({}, {}, {}) is out of bounds", x, y, z);
        println!("Schematic dimensions: {}x{}x{}", schem.width, schem.height, schem.length);
    }

    Ok(())
}

fn cmd_search(file: &PathBuf, pattern: &str, show_positions: bool, limit: Option<usize>) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;
    let pattern_lower = pattern.to_lowercase();

    let mut matches: Vec<(u16, u16, u16, &schem_tool::Block)> = Vec::new();

    for y in 0..schem.height {
        for z in 0..schem.length {
            for x in 0..schem.width {
                if let Some(block) = schem.get_block(x, y, z) {
                    if block.name.to_lowercase().contains(&pattern_lower) {
                        matches.push((x, y, z, block));
                    }
                }
            }
        }
    }

    if matches.is_empty() {
        println!("No blocks matching '{}' found.", pattern);
        return Ok(());
    }

    let display_count = limit.unwrap_or(matches.len()).min(matches.len());

    println!("Found {} blocks matching '{}':", matches.len(), pattern);
    println!();

    if show_positions {
        for (x, y, z, block) in matches.iter().take(display_count) {
            println!("  ({:3}, {:3}, {:3}): {}", x, y, z, block.full_name());
        }
    } else {
        // Group by block type
        let mut by_type: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for (_, _, _, block) in &matches {
            *by_type.entry(block.full_name()).or_insert(0) += 1;
        }

        let mut sorted: Vec<_> = by_type.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        for (name, count) in sorted.iter().take(display_count) {
            println!("  {} x{}", name, count);
        }
    }

    if display_count < matches.len() {
        println!("\n... and {} more", matches.len() - display_count);
    }

    Ok(())
}

fn cmd_export(file: &PathBuf, output: &PathBuf) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;

    let mut csv = String::from("block,count,percent\n");

    let counts = schem.block_counts();
    let total: usize = counts.values().sum();

    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    for (name, count) in sorted {
        let percent = if total > 0 {
            (count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        csv.push_str(&format!("\"{}\",{},{:.2}\n", name, count, percent));
    }

    std::fs::write(output, csv)?;
    println!("Exported block list to: {}", output.display());

    Ok(())
}

fn cmd_materials(file: &PathBuf, sort: bool, verbose: bool, limit: Option<usize>) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;
    let block_counts = schem.block_counts();

    if verbose {
        println!("{}", "=== Original Blocks ===".bold().cyan());
        let mut original: Vec<_> = block_counts.iter()
            .filter(|(name, _)| !name.contains("air"))
            .collect();
        original.sort_by(|a, b| b.1.cmp(a.1));

        for (name, count) in original.iter().take(20) {
            println!("  {:>10} x {}", count, name);
        }
        if original.len() > 20 {
            println!("  ... and {} more types", original.len() - 20);
        }
        println!();
    }

    println!("{}", "=== Raw Materials Needed ===".bold().cyan());
    println!();

    let materials = schem_tool::recipes::calculate_materials(&block_counts);

    let mut sorted: Vec<_> = materials.into_iter().collect();
    if sort {
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
    }

    let display_limit = limit.unwrap_or(usize::MAX);

    #[derive(tabled::Tabled)]
    struct MaterialRow {
        #[tabled(rename = "Material")]
        name: String,
        #[tabled(rename = "Count")]
        count: String,
        #[tabled(rename = "Stacks")]
        stacks: String,
    }

    let rows: Vec<MaterialRow> = sorted.iter()
        .take(display_limit)
        .map(|(name, count)| {
            let rounded = count.ceil() as u64;
            let stacks = rounded / 64;
            let remainder = rounded % 64;
            let stacks_str = if stacks > 0 {
                if remainder > 0 {
                    format!("{} + {}", stacks, remainder)
                } else {
                    format!("{} stacks", stacks)
                }
            } else {
                format!("{}", remainder)
            };

            MaterialRow {
                name: name.strip_prefix("minecraft:").unwrap_or(name).to_string(),
                count: format!("{}", rounded),
                stacks: stacks_str,
            }
        })
        .collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", table);

    if sorted.len() > display_limit {
        println!("\n... and {} more materials", sorted.len() - display_limit);
    }

    // Summary
    let total_items: f64 = sorted.iter().map(|(_, c)| c).sum();
    let total_stacks = (total_items / 64.0).ceil() as u64;
    println!("\n{}: ~{} items (~{} stacks)", "Total".bold(), total_items.ceil() as u64, total_stacks);

    Ok(())
}

fn cmd_layer(file: &PathBuf, y: u16, ascii: bool) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;

    if y >= schem.height {
        println!("Y level {} is out of bounds (max: {})", y, schem.height - 1);
        return Ok(());
    }

    println!("Layer at Y={} ({}x{})", y, schem.width, schem.length);
    println!();

    // Simple character mapping
    let get_char = |block: &schem_tool::Block| -> char {
        let name = block.display_name();

        if block.is_air() {
            return if ascii { '.' } else { ' ' };
        }

        if ascii {
            // ASCII mode
            if name.contains("stone") { return '#' }
            if name.contains("dirt") || name.contains("grass") { return '~' }
            if name.contains("wood") || name.contains("log") || name.contains("plank") { return '=' }
            if name.contains("glass") { return 'o' }
            if name.contains("water") { return 'w' }
            if name.contains("lava") { return 'L' }
            if name.contains("ore") { return '*' }
            if name.contains("chest") { return 'C' }
            if name.contains("door") { return 'D' }
            if name.contains("torch") { return 'i' }
            if name.contains("redstone") { return 'r' }
            if name.contains("wool") || name.contains("concrete") { return '@' }
            if name.contains("brick") { return 'B' }
            if name.contains("iron") { return 'I' }
            if name.contains("gold") { return 'G' }
            if name.contains("diamond") { return '$' }
            '#'
        } else {
            // Unicode mode
            if name.contains("stone") { return '\u{2588}' } // █
            if name.contains("dirt") || name.contains("grass") { return '\u{2593}' } // ▓
            if name.contains("wood") || name.contains("log") || name.contains("plank") { return '\u{2592}' } // ▒
            if name.contains("glass") { return '\u{25A1}' } // □
            if name.contains("water") { return '\u{2248}' } // ≈
            if name.contains("lava") { return '\u{2234}' } // ∴
            if name.contains("ore") { return '\u{25C6}' } // ◆
            if name.contains("chest") { return '\u{25A0}' } // ■
            if name.contains("door") { return '\u{25AF}' } // ▯
            if name.contains("torch") { return '\u{2020}' } // †
            if name.contains("redstone") { return '\u{00B7}' } // ·
            '\u{2591}' // ░
        }
    };

    // Print grid
    for z in 0..schem.length {
        for x in 0..schem.width {
            if let Some(block) = schem.get_block(x, y, z) {
                print!("{}", get_char(block));
            } else {
                print!("?");
            }
        }
        println!();
    }

    println!();
    println!("Legend ({}mode):", if ascii { "ASCII " } else { "Unicode " });
    if ascii {
        println!("  . = air, # = stone, ~ = dirt/grass, = = wood");
        println!("  o = glass, w = water, L = lava, * = ore");
        println!("  C = chest, D = door, i = torch, r = redstone");
    } else {
        println!("  █ = stone, ▓ = dirt/grass, ▒ = wood, □ = glass");
        println!("  ≈ = water, ∴ = lava, ◆ = ore, ■ = chest");
        println!("  † = torch, · = redstone, ░ = other solid");
    }

    Ok(())
}

fn cmd_render_obj(file: &PathBuf, output: &PathBuf, hollow: bool, greedy: bool, use_models: bool, use_textures: bool, minecraft_path: Option<&std::path::Path>) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;

    println!("{}", "=== Exporting to OBJ ===".bold().cyan());
    println!();
    println!("  Schematic: {}x{}x{}", schem.width, schem.height, schem.length);
    println!("  Solid blocks: {}", schem.solid_blocks());

    if use_models {
        println!("  Mode: {} (accurate Minecraft geometry)", "JSON models".green());
    } else if greedy {
        println!("  Mode: {} (optimized polygon count)", "greedy meshing".green());
    } else {
        println!("  Hollow mode: {}", if hollow { "yes (only visible faces)" } else { "no (all blocks)" });
    }

    // Try to load textures if requested
    let textures = if use_textures {
        println!("  Textures: {}", "loading...".yellow());
        let tm = schem_tool::textures::TextureManager::from_minecraft_with_path(minecraft_path);
        match tm {
            Some(tm) => {
                println!("  Textures: {} textures loaded", tm.texture_count().to_string().green());
                Some(tm)
            }
            None => {
                println!("  Textures: {} (Minecraft not found, using colors)", "unavailable".red());
                if minecraft_path.is_none() {
                    println!("  {}: Use --minecraft <path> to specify Minecraft directory or client.jar", "Hint".yellow());
                }
                None
            }
        }
    } else {
        println!("  Textures: disabled (use --textures to enable)");
        None
    };
    println!();

    if use_models {
        // Find Minecraft jar for models
        let jar_path = if let Some(mc_path) = minecraft_path {
            if mc_path.extension().map(|e| e == "jar").unwrap_or(false) {
                mc_path.to_path_buf()
            } else {
                schem_tool::textures::find_client_jar(mc_path)
                    .ok_or_else(|| anyhow::anyhow!("Could not find Minecraft client.jar in {}", mc_path.display()))?
            }
        } else {
            let mc_dir = schem_tool::textures::get_minecraft_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not find Minecraft directory"))?;
            schem_tool::textures::find_client_jar(&mc_dir)
                .ok_or_else(|| anyhow::anyhow!("Could not find Minecraft client.jar"))?
        };
        println!("  Using models from: {}", jar_path.display());
        schem_tool::export3d::export_obj_with_models(&schem, output, &jar_path, textures.as_ref())?;
    } else if greedy {
        schem_tool::export3d::export_obj_greedy(&schem, output, textures.as_ref())?;
    } else {
        schem_tool::export3d::export_obj_with_textures(&schem, output, hollow, true, textures.as_ref())?;
    }

    let mtl_path = output.with_extension("mtl");
    println!();
    println!("{}:", "Exported files".green());
    println!("  OBJ: {}", output.display());
    println!("  MTL: {}", mtl_path.display());

    if textures.is_some() {
        let tex_dir = output.parent().unwrap_or(std::path::Path::new(".")).join("textures");
        println!("  Textures: {}", tex_dir.display());
    }

    println!();
    println!("Open in: Blender, Windows 3D Viewer, online viewers, etc.");
    if textures.is_some() {
        println!("{}: In Blender, ensure the textures folder is in the same directory as the OBJ file.", "Tip".yellow());
    }

    Ok(())
}

fn cmd_render_html(file: &PathBuf, output: &PathBuf, max_blocks: usize) -> Result<()> {
    let schem = UnifiedSchematic::load(file)?;

    println!("{}", "=== Exporting to HTML Viewer ===".bold().cyan());
    println!();
    println!("  Schematic: {}x{}x{}", schem.width, schem.height, schem.length);
    println!("  Max blocks to render: {}", max_blocks);
    println!();

    schem_tool::export3d::export_html(&schem, output, max_blocks)?;

    println!("{}:", "Exported".green());
    println!("  HTML: {}", output.display());
    println!();
    println!("Open in any web browser for interactive 3D view.");
    println!("Controls: drag to rotate, scroll to zoom.");

    Ok(())
}

fn cmd_debug(file: &PathBuf) -> Result<()> {
    use std::io::Read;
    use flate2::read::GzDecoder;

    let mut f = std::fs::File::open(file)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    let data = if buf.starts_with(&[0x1f, 0x8b]) {
        let mut decoder = GzDecoder::new(&buf[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        decompressed
    } else {
        buf
    };

    // Parse as generic NBT value
    let nbt: fastnbt::Value = fastnbt::from_bytes(&data)?;

    println!("{}", "=== Raw NBT Structure ===".bold().cyan());
    println!();
    print_nbt_value(&nbt, 0);

    Ok(())
}

fn print_nbt_value(value: &fastnbt::Value, indent: usize) {
    let pad = "  ".repeat(indent);

    match value {
        fastnbt::Value::Byte(b) => println!("{}Byte: {}", pad, b),
        fastnbt::Value::Short(s) => println!("{}Short: {}", pad, s),
        fastnbt::Value::Int(i) => println!("{}Int: {}", pad, i),
        fastnbt::Value::Long(l) => println!("{}Long: {}", pad, l),
        fastnbt::Value::Float(f) => println!("{}Float: {}", pad, f),
        fastnbt::Value::Double(d) => println!("{}Double: {}", pad, d),
        fastnbt::Value::String(s) => println!("{}String: \"{}\"", pad, s),
        fastnbt::Value::ByteArray(arr) => println!("{}ByteArray[{}]", pad, arr.len()),
        fastnbt::Value::IntArray(arr) => println!("{}IntArray[{}]", pad, arr.len()),
        fastnbt::Value::LongArray(arr) => println!("{}LongArray[{}]", pad, arr.len()),
        fastnbt::Value::List(list) => {
            println!("{}List[{}]:", pad, list.len());
            for (i, item) in list.iter().enumerate().take(5) {
                println!("{}  [{}]:", pad, i);
                print_nbt_value(item, indent + 2);
            }
            if list.len() > 5 {
                println!("{}  ... and {} more", pad, list.len() - 5);
            }
        }
        fastnbt::Value::Compound(map) => {
            println!("{}Compound {{", pad);
            for (key, val) in map {
                print!("{}  {}: ", pad, key.yellow());
                match val {
                    fastnbt::Value::Compound(_) | fastnbt::Value::List(_) => {
                        println!();
                        print_nbt_value(val, indent + 2);
                    }
                    _ => {
                        // Print simple values on same line
                        match val {
                            fastnbt::Value::Byte(b) => println!("{}", b),
                            fastnbt::Value::Short(s) => println!("{}", s),
                            fastnbt::Value::Int(i) => println!("{}", i),
                            fastnbt::Value::Long(l) => println!("{}", l),
                            fastnbt::Value::Float(f) => println!("{}", f),
                            fastnbt::Value::Double(d) => println!("{}", d),
                            fastnbt::Value::String(s) => println!("\"{}\"", s),
                            fastnbt::Value::ByteArray(arr) => println!("ByteArray[{}]", arr.len()),
                            fastnbt::Value::IntArray(arr) => println!("IntArray[{}]", arr.len()),
                            fastnbt::Value::LongArray(arr) => println!("LongArray[{}]", arr.len()),
                            _ => println!("{:?}", val),
                        }
                    }
                }
            }
            println!("{}}}", pad);
        }
    }
}
