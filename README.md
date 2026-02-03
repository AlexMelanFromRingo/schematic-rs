# schematic-rs

Fast Minecraft schematic file parser and analyzer written in Rust.

Supports multiple formats:
- `.schematic` — Legacy MCEdit format
- `.schem` — Sponge/WorldEdit v2/v3
- `.litematic` — Litematica mod format

## Features

- **Parse** both legacy and modern schematic formats
- **Analyze** block composition, dimensions, metadata
- **Search** for specific blocks by name
- **Calculate** raw materials needed (breaks down crafted items)
- **Extract** sign text and block entity data
- **Visualize** layer-by-layer ASCII/Unicode view
- **Export** to OBJ 3D model with textures support
- **Greedy meshing** for dramatically smaller OBJ files (10-100x reduction)
- **Texture extraction** from Minecraft installation with proper tiling
- **Interactive HTML viewer** using Three.js
- **Debug** raw NBT structure

## Installation

```bash
cargo install --path .
```

Or build manually:

```bash
cargo build --release
./target/release/schem-tool --help
```

## Usage

### Basic Info
```bash
schem-tool info my_build.schem
```

Output:
```
=== Schematic Info ===

File:  my_build.schem
Format:  SpongeV3

--- Dimensions ---
  Width (X):  100
  Height (Y): 50
  Length (Z): 100
  Volume:     500000 blocks

--- Contents ---
  Total blocks:    500000
  Solid blocks:    125000
  Unique types:    45
  Block entities:  12
  Entities:        0
```

### Block List
```bash
# All blocks sorted by count
schem-tool blocks -ns my_build.schem

# Limit to top 10
schem-tool blocks -ns -l 10 my_build.schem
```

### Calculate Materials
```bash
schem-tool materials -s my_build.schem
```

Breaks down crafted items into raw materials:
```
=== Raw Materials Needed ===

╭────────────────┬──────────┬───────────────╮
│ Material       │ Count    │ Stacks        │
├────────────────┼──────────┼───────────────┤
│ cobblestone    │ 15000    │ 234 + 24      │
│ iron_ingot     │ 2500     │ 39 + 4        │
│ sand           │ 8000     │ 125 stacks    │
│ red_dye        │ 1000     │ 15 + 40       │
╰────────────────┴──────────┴───────────────╯
```

### Search Blocks
```bash
# Find all redstone components
schem-tool search my_build.schem redstone

# With positions
schem-tool search my_build.schem chest -p
```

### Layer View
```bash
# ASCII visualization of Y=10 slice
schem-tool layer my_build.schem -y 10 --ascii
```

### 3D Export
```bash
# Export to OBJ (for Blender, Windows 3D Viewer, etc.)
schem-tool render-obj my_build.schem -o model.obj --hollow

# Export with greedy meshing (10-100x smaller files, recommended)
schem-tool render-obj my_build.schem -o model.obj --greedy

# Export with textures from Minecraft installation
schem-tool render-obj my_build.schem -o model.obj --greedy --textures

# Specify custom Minecraft path or client.jar
schem-tool render-obj my_build.schem -o model.obj --greedy --textures -m ~/.minecraft
schem-tool render-obj my_build.schem -o model.obj --textures -m /path/to/client.jar

# Export to interactive HTML viewer
schem-tool render-html my_build.schem -o view.html -m 100000
```

#### Greedy Meshing

The `--greedy` flag enables greedy meshing algorithm that merges adjacent faces of the same block type into larger quads. This dramatically reduces file size and polygon count (typically 10-100x smaller), making models much easier to work with in 3D software.

#### Partial Block Support

Non-full blocks are rendered with correct geometry:
- **Slabs** — half-height blocks (top/bottom)
- **Stairs** — proper L-shaped geometry
- **Doors, trapdoors** — thin panels with correct orientation
- **Fences, walls** — posts and connections
- **Torches, lanterns, candles** — small decorative shapes
- **Beds, carpets** — flat/low-profile shapes
- **Chests, hoppers, cauldrons** — special container shapes
- **And many more** — buttons, levers, rails, signs, etc.

Full blocks use efficient greedy meshing, while partial blocks are rendered individually with proper face culling.

#### Transparency

Materials with transparency are exported with correct opacity values:
- **Glass** — 30% opacity
- **Leaves** — 90% opacity
- **Ice** (not packed/blue) — 70% opacity
- **Water** — 40% opacity

This allows proper rendering in Blender and other 3D software when using transparency-aware materials.

#### Textures

When using `--textures`, the tool extracts block textures from your Minecraft installation:
- Auto-detects Minecraft directory on Windows, macOS, and Linux
- Textures are cached for faster subsequent exports
- Tiled properly even with greedy meshing (textures repeat instead of stretching)
- UV coordinates scaled correctly for partial blocks (half-texture for slabs, etc.)
- Creates a `textures/` folder next to the OBJ file

### Other Commands
```bash
# Block palette with states
schem-tool palette my_build.schem

# Block entities (chests, signs, etc.)
schem-tool block-entities my_build.schem -v

# Extract sign text
schem-tool signs my_build.schem

# Metadata (author, date, etc.)
schem-tool metadata my_build.schem

# Get block at position
schem-tool get-block my_build.schem -x 10 -y 5 -z 20

# Export to CSV
schem-tool export my_build.schem -o blocks.csv

# Debug NBT structure
schem-tool debug my_build.schem
```

## Supported Formats

| Format | Extension | Description |
|--------|-----------|-------------|
| Legacy | `.schematic` | MCEdit format, numeric block IDs |
| Sponge v2 | `.schem` | WorldEdit format, string block states |
| Sponge v3 | `.schem` | Latest WorldEdit format |
| Litematica | `.litematic` | Litematica mod format, packed bit storage |

Format is auto-detected from file content, not extension.

## Format Documentation

See [FORMAT.md](FORMAT.md) for detailed documentation on NBT, `.schematic`, `.schem`, and `.litematic` formats. Useful if you want to understand how to parse these formats from scratch.

## Library Usage

```rust
use schem_tool::UnifiedSchematic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schem = UnifiedSchematic::load("my_build.schem")?;

    println!("Dimensions: {}x{}x{}", schem.width, schem.height, schem.length);
    println!("Solid blocks: {}", schem.solid_blocks());

    // Get block at position
    if let Some(block) = schem.get_block(10, 5, 20) {
        println!("Block: {}", block.full_name());
    }

    // Count blocks
    for (name, count) in schem.block_counts() {
        println!("{}: {}", name, count);
    }

    Ok(())
}
```

## Performance

Tested on a 491x384x551 schematic (~104 million blocks):

- Load & parse: ~2 seconds
- Block count: instant
- Materials calculation: ~1 second
- HTML export (50k blocks): instant

## Dependencies

- `fastnbt` - NBT parsing
- `flate2` - GZIP decompression
- `clap` - CLI argument parsing
- `serde` - Serialization
- `chrono` - Date formatting
- `colored` - Terminal colors
- `tabled` - Table formatting
- `indicatif` - Progress bars
- `zip` - Texture extraction from Minecraft JAR
- `dirs` - OS-specific directory detection

## License

MIT
