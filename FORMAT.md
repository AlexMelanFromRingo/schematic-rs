# Minecraft Schematic Format Documentation

This document explains the binary formats used for Minecraft schematics and how to parse them from scratch.

## Table of Contents

1. [NBT (Named Binary Tag)](#nbt-named-binary-tag)
2. [Legacy .schematic Format](#legacy-schematic-format)
3. [Sponge Schematic v2/v3 (.schem)](#sponge-schematic-v2v3-schem)
4. [Litematica Format (.litematic)](#litematica-format-litematic)
5. [Implementation Guide](#implementation-guide)

---

## NBT (Named Binary Tag)

NBT is the foundation of all Minecraft data formats. It's a binary format similar to JSON but more compact.

### Structure Overview

NBT files are typically GZIP compressed. After decompression, the structure is:

```
[Root Compound Tag]
  └── Named tags...
```

### Tag Types

| ID | Type | Payload |
|----|------|---------|
| 0 | TAG_End | None (marks end of compound) |
| 1 | TAG_Byte | 1 byte signed integer |
| 2 | TAG_Short | 2 bytes signed integer (big-endian) |
| 3 | TAG_Int | 4 bytes signed integer (big-endian) |
| 4 | TAG_Long | 8 bytes signed integer (big-endian) |
| 5 | TAG_Float | 4 bytes IEEE 754 float (big-endian) |
| 6 | TAG_Double | 8 bytes IEEE 754 double (big-endian) |
| 7 | TAG_Byte_Array | Int length + bytes |
| 8 | TAG_String | Short length + UTF-8 bytes |
| 9 | TAG_List | Byte type + Int length + payloads |
| 10 | TAG_Compound | Named tags until TAG_End |
| 11 | TAG_Int_Array | Int length + ints |
| 12 | TAG_Long_Array | Int length + longs |

### Binary Layout

**Named Tag (inside Compound):**
```
[1 byte: tag type]
[2 bytes: name length (big-endian)]
[N bytes: name (UTF-8)]
[payload...]
```

**Unnamed Tag (inside List):**
```
[payload only - type is inherited from list]
```

### Example: Parsing a Compound Tag

```
0A              <- TAG_Compound (10)
00 04           <- Name length: 4
72 6F 6F 74     <- Name: "root"
  03            <- TAG_Int (3)
  00 05         <- Name length: 5
  76 61 6C 75 65 <- Name: "value"
  00 00 00 2A   <- Int payload: 42
  00            <- TAG_End
```

This represents: `{root: {value: 42}}`

### Pseudocode: NBT Parser

```python
def read_nbt(stream):
    tag_type = read_byte(stream)
    name_len = read_short(stream)  # big-endian
    name = read_string(stream, name_len)
    payload = read_payload(stream, tag_type)
    return (name, payload)

def read_payload(stream, tag_type):
    match tag_type:
        case 0: return None  # End
        case 1: return read_byte(stream)
        case 2: return read_short(stream)
        case 3: return read_int(stream)
        case 4: return read_long(stream)
        case 5: return read_float(stream)
        case 6: return read_double(stream)
        case 7:
            length = read_int(stream)
            return [read_byte(stream) for _ in range(length)]
        case 8:
            length = read_short(stream)
            return read_string(stream, length)
        case 9:
            item_type = read_byte(stream)
            length = read_int(stream)
            return [read_payload(stream, item_type) for _ in range(length)]
        case 10:
            compound = {}
            while True:
                child_type = read_byte(stream)
                if child_type == 0:
                    break
                child_name_len = read_short(stream)
                child_name = read_string(stream, child_name_len)
                compound[child_name] = read_payload(stream, child_type)
            return compound
        case 11:
            length = read_int(stream)
            return [read_int(stream) for _ in range(length)]
        case 12:
            length = read_int(stream)
            return [read_long(stream) for _ in range(length)]
```

---

## Legacy .schematic Format

The original format created by MCEdit. Uses numeric block IDs (pre-1.13 style).

### Structure

```
{
    Width: short           // X dimension
    Height: short          // Y dimension
    Length: short          // Z dimension
    Materials: string      // Usually "Alpha"
    Blocks: byte[]         // Block IDs (0-255)
    Data: byte[]           // Block data/damage values
    AddBlocks: byte[]      // Optional: upper 4 bits for IDs > 255
    Entities: list         // Entity NBT data
    TileEntities: list     // Block entity NBT data
    WEOffsetX: int         // Optional: WorldEdit offset
    WEOffsetY: int
    WEOffsetZ: int
}
```

### Block Storage

Blocks are stored in YZX order (Y changes slowest, X fastest):

```python
def get_index(x, y, z, width, length):
    return (y * length + z) * width + x

def get_block(x, y, z):
    index = get_index(x, y, z, width, length)
    block_id = blocks[index]

    # Handle AddBlocks for IDs > 255
    if add_blocks:
        nibble_index = index // 2
        if index % 2 == 0:
            block_id |= (add_blocks[nibble_index] & 0x0F) << 8
        else:
            block_id |= (add_blocks[nibble_index] & 0xF0) << 4

    data_value = data[index]
    return (block_id, data_value)
```

### Numeric ID to Modern Name

Pre-1.13 Minecraft used numeric IDs. You need a mapping table:

| ID | Data | Modern Name |
|----|------|-------------|
| 0 | * | minecraft:air |
| 1 | 0 | minecraft:stone |
| 1 | 1 | minecraft:granite |
| 1 | 2 | minecraft:polished_granite |
| 4 | 0 | minecraft:cobblestone |
| 35 | 0 | minecraft:white_wool |
| 35 | 14 | minecraft:red_wool |
| ... | ... | ... |

The data value encodes variants (wool colors, wood types, etc.) and block states (facing direction, etc.).

### TileEntities Structure

```
{
    id: string      // "Chest", "Sign", etc.
    x: int          // Absolute or relative position
    y: int
    z: int
    ... // Entity-specific data
}
```

---

## Sponge Schematic v2/v3 (.schem)

The modern format used by WorldEdit. Uses string-based block states.

### Version 2 Structure

```
{
    Version: int              // 2
    DataVersion: int          // Minecraft data version
    Width: short
    Height: short
    Length: short
    Offset: int[3]            // [x, y, z] offset
    PaletteMax: int           // Number of palette entries
    Palette: {                // Block state -> palette ID
        "minecraft:air": 0,
        "minecraft:stone": 1,
        "minecraft:chest[facing=north]": 2,
        ...
    }
    BlockData: byte[]         // Varint-encoded palette indices
    BlockEntities: [...]      // Block entity data
    Entities: [...]           // Entity data (optional)
    Metadata: {...}           // Author, date, etc. (optional)
}
```

### Version 3 Structure

Version 3 wraps everything in a "Schematic" compound and separates blocks:

```
{
    Schematic: {
        Version: int          // 3
        DataVersion: int
        Width: short
        Height: short
        Length: short
        Offset: int[3]
        Metadata: {...}
        Blocks: {
            Palette: {...}
            Data: byte[]
            BlockEntities: [...]
        }
        Biomes: {             // Optional
            Palette: {...}
            Data: byte[]
        }
        Entities: [...]
    }
}
```

### Block State Strings

Modern format uses full block state strings:

```
minecraft:stone                           // Simple block
minecraft:oak_stairs[facing=north,half=bottom,shape=straight,waterlogged=false]
minecraft:redstone_wire[power=15,north=side,south=side,east=none,west=none]
minecraft:chest[facing=west,type=single,waterlogged=false]
```

### Varint Encoding

BlockData uses variable-length integers to compress the data:

```python
def read_varint(data, offset):
    result = 0
    shift = 0

    while True:
        byte = data[offset]
        offset += 1

        result |= (byte & 0x7F) << shift

        if (byte & 0x80) == 0:
            break

        shift += 7
        if shift >= 32:
            raise Error("Varint too long")

    return (result, offset)

def decode_block_data(data, volume):
    blocks = []
    offset = 0

    while len(blocks) < volume:
        palette_id, offset = read_varint(data, offset)
        blocks.append(palette_id)

    return blocks
```

### Block Entity Structure (v3)

```
{
    Id: string              // "minecraft:chest"
    Pos: int[3]             // [x, y, z] relative to schematic
    Data: {                 // Entity-specific NBT
        Items: [...]
        Lock: string
        ...
    }
}
```

### Metadata Structure

```
{
    Name: string            // Schematic name
    Author: string          // Creator
    Date: long              // Unix timestamp (milliseconds)
    RequiredMods: [string]  // Required mod IDs
    WorldEdit: {            // WorldEdit-specific
        Version: string
        EditingPlatform: string
        Origin: int[3]
    }
}
```

---

## Litematica Format (.litematic)

Litematica is a popular mod for creating and managing schematics. It uses a more compact storage format with packed bit arrays.

### Structure

```
{
    Version: int                    // Format version (4-6)
    MinecraftDataVersion: int       // MC data version
    Metadata: {
        Name: string
        Author: string
        Description: string
        RegionCount: int
        TotalBlocks: long
        TotalVolume: long
        TimeCreated: long           // Unix timestamp (ms)
        TimeModified: long
        EnclosingSize: {x, y, z}    // Bounding box
    }
    Regions: {
        [region_name]: {
            Position: {x, y, z}
            Size: {x, y, z}         // Can be negative!
            BlockStatePalette: [    // List of block states
                {Name: "minecraft:air"},
                {Name: "minecraft:stone"},
                {Name: "minecraft:oak_stairs", Properties: {facing: "north", half: "bottom"}}
            ]
            BlockStates: LongArray  // Packed bit storage
            TileEntities: [...]
            Entities: [...]
            PendingBlockTicks: [...]
        }
    }
}
```

### Packed Bit Storage

Unlike Sponge format (varint), Litematica uses packed bits in a LongArray:

1. Calculate bits per block: `ceil(log2(palette_size))`
2. Each block index is stored using exactly that many bits
3. Bits are packed sequentially into 64-bit longs

```python
def calculate_bits_per_block(palette_size):
    if palette_size <= 1:
        return 1
    return ceil(log2(palette_size))

def decode_packed_array(long_array, bits_per_block, count):
    result = []
    mask = (1 << bits_per_block) - 1
    bit_offset = 0

    for _ in range(count):
        long_index = bit_offset // 64
        bit_in_long = bit_offset % 64

        if bit_in_long + bits_per_block <= 64:
            # Value fits in single long
            value = (long_array[long_index] >> bit_in_long) & mask
        else:
            # Value spans two longs
            bits_in_first = 64 - bit_in_long
            bits_in_second = bits_per_block - bits_in_first

            first_part = long_array[long_index] >> bit_in_long
            second_part = (long_array[long_index + 1] & ((1 << bits_in_second) - 1)) << bits_in_first

            value = (first_part | second_part) & mask

        result.append(value)
        bit_offset += bits_per_block

    return result
```

### Block Order

Blocks are stored in YZX order (same as other formats):
- Index = y * (length * width) + z * width + x

### Negative Sizes

Region sizes can be negative, indicating direction:
- Positive: blocks go from Position to Position + Size
- Negative: blocks go from Position + Size + 1 to Position

```python
def get_global_pos(region_pos, region_size, local_x, local_y, local_z):
    if region_size.x < 0:
        gx = region_pos.x + region_size.x + 1 + local_x
    else:
        gx = region_pos.x + local_x
    # Same for y and z
    return (gx, gy, gz)
```

---

## Implementation Guide

### Step 1: GZIP Decompression

```rust
use flate2::read::GzDecoder;

fn decompress(data: &[u8]) -> Vec<u8> {
    if data.starts_with(&[0x1f, 0x8b]) {
        // GZIP magic bytes
        let mut decoder = GzDecoder::new(data);
        let mut result = Vec::new();
        decoder.read_to_end(&mut result).unwrap();
        result
    } else {
        data.to_vec()
    }
}
```

### Step 2: NBT Parsing

Use a library like `fastnbt` (Rust) or `nbtlib` (Python), or implement from scratch using the pseudocode above.

### Step 3: Format Detection

```rust
fn detect_format(nbt: &Value) -> Format {
    // Check for v3 wrapper
    if nbt.contains_key("Schematic") {
        return Format::SpongeV3;
    }

    // Check for Version field (v2)
    if let Some(version) = nbt.get("Version") {
        return Format::SpongeV2;
    }

    // Check for legacy format markers
    if nbt.contains_key("Blocks") && nbt.contains_key("Data") {
        return Format::Legacy;
    }

    Format::Unknown
}
```

### Step 4: Block Extraction

**Legacy Format:**
```rust
fn extract_legacy_blocks(nbt: &Value) -> Vec<Block> {
    let width = nbt["Width"].as_i16();
    let height = nbt["Height"].as_i16();
    let length = nbt["Length"].as_i16();
    let blocks_data = nbt["Blocks"].as_byte_array();
    let data_values = nbt["Data"].as_byte_array();

    let mut blocks = Vec::new();

    for y in 0..height {
        for z in 0..length {
            for x in 0..width {
                let idx = (y * length + z) * width + x;
                let id = blocks_data[idx];
                let data = data_values[idx];
                blocks.push(legacy_id_to_block(id, data));
            }
        }
    }

    blocks
}
```

**Sponge Format:**
```rust
fn extract_sponge_blocks(nbt: &Value) -> Vec<Block> {
    let palette = &nbt["Palette"];  // or nbt["Blocks"]["Palette"] for v3
    let block_data = &nbt["BlockData"];  // or nbt["Blocks"]["Data"]

    // Build reverse palette
    let mut id_to_block = HashMap::new();
    for (state_str, id) in palette {
        id_to_block.insert(id, parse_block_state(state_str));
    }

    // Decode varints
    let mut blocks = Vec::new();
    let mut offset = 0;

    while offset < block_data.len() {
        let (palette_id, new_offset) = read_varint(block_data, offset);
        offset = new_offset;
        blocks.push(id_to_block[&palette_id].clone());
    }

    blocks
}
```

### Step 5: Block State Parsing

```rust
fn parse_block_state(state_str: &str) -> Block {
    // "minecraft:chest[facing=north,waterlogged=false]"

    if let Some(bracket_pos) = state_str.find('[') {
        let name = &state_str[..bracket_pos];
        let props_str = &state_str[bracket_pos + 1..state_str.len() - 1];

        let mut properties = HashMap::new();
        for prop in props_str.split(',') {
            let parts: Vec<&str> = prop.split('=').collect();
            properties.insert(parts[0].to_string(), parts[1].to_string());
        }

        Block { name, properties }
    } else {
        Block { name: state_str, properties: HashMap::new() }
    }
}
```

---

## Useful Resources

- [NBT Specification](https://wiki.vg/NBT)
- [Sponge Schematic Specification](https://github.com/SpongePowered/Schematic-Specification)
- [Minecraft Data Values (Legacy)](https://minecraft.wiki/w/Data_values/Pre-flattening)
- [Block States](https://minecraft.wiki/w/Block_states)

## Libraries

**Rust:**
- `fastnbt` - NBT parsing with serde support
- `hematite-nbt` - Alternative NBT library
- `flate2` - GZIP compression

**Python:**
- `nbtlib` - NBT parsing
- `mcschematic` - Schematic handling
- `litemapy` - Litematica format

**Java:**
- WorldEdit source code (reference implementation)
