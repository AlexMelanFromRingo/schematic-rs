//! Sponge Schematic format (.schem) v2 and v3
//!
//! v2 Structure:
//! - Version: int (2)
//! - DataVersion: int (Minecraft data version)
//! - Width, Height, Length: short
//! - Offset: int array [x, y, z]
//! - Palette: compound {block_state_string -> varint_id}
//! - BlockData: byte array (varint encoded palette indices)
//! - BlockEntities: list
//! - Entities: list (optional)
//! - Metadata: compound (optional)
//!
//! v3 Structure:
//! - Root compound with "Schematic" field containing v3 data
//! - Blocks compound with Palette and Data

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{
    Block, BlockState, BlockEntity, Entity, Metadata,
    SchematicFormat, UnifiedSchematic,
};

/// Wrapper for v3 format where root NBT has "Schematic" compound
#[derive(Debug, Deserialize, Serialize)]
pub struct SchemWrapper {
    #[serde(rename = "Schematic")]
    pub schematic: Schem,
}

/// Sponge Schematic format (v2 and v3)
#[derive(Debug, Deserialize, Serialize)]
pub struct Schem {
    #[serde(rename = "Version", alias = "version")]
    pub version: i32,

    #[serde(rename = "DataVersion", alias = "dataVersion", default)]
    pub data_version: Option<i32>,

    #[serde(rename = "Width", alias = "width", default)]
    pub width: Option<i16>,

    #[serde(rename = "Height", alias = "height", default)]
    pub height: Option<i16>,

    #[serde(rename = "Length", alias = "length", default)]
    pub length: Option<i16>,

    #[serde(rename = "Offset", default)]
    pub offset: Option<fastnbt::IntArray>,

    #[serde(rename = "Palette", default)]
    pub palette: Option<HashMap<String, i32>>,

    #[serde(rename = "PaletteMax", default)]
    pub palette_max: Option<i32>,

    #[serde(rename = "BlockData", default)]
    pub block_data: Option<fastnbt::ByteArray>,

    #[serde(rename = "BlockEntities", default)]
    pub block_entities: Vec<SchemBlockEntity>,

    #[serde(rename = "TileEntities", default)]
    pub tile_entities: Vec<SchemBlockEntity>,

    #[serde(rename = "Entities", default)]
    pub entities: Vec<SchemEntity>,

    #[serde(rename = "Metadata", default)]
    pub metadata: Option<SchemMetadata>,

    // v3 specific - nested structure
    #[serde(rename = "Schematic", default)]
    pub schematic: Option<Box<Schem>>,

    #[serde(rename = "Blocks", default)]
    pub blocks: Option<SchemBlocks>,

    // v3 biomes
    #[serde(rename = "Biomes", default)]
    pub biomes: Option<SchemBiomes>,
}

/// v3 Blocks compound
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SchemBlocks {
    #[serde(rename = "Palette", default)]
    pub palette: HashMap<String, i32>,

    #[serde(rename = "Data", default)]
    pub data: Option<fastnbt::ByteArray>,

    #[serde(rename = "BlockEntities", default)]
    pub block_entities: Vec<SchemBlockEntity>,
}

/// v3 Biomes compound
#[derive(Debug, Deserialize, Serialize)]
pub struct SchemBiomes {
    #[serde(rename = "Palette", default)]
    pub palette: HashMap<String, i32>,

    #[serde(rename = "Data", default)]
    pub data: Option<fastnbt::ByteArray>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SchemBlockEntity {
    #[serde(rename = "Id", alias = "id", default)]
    pub id: Option<String>,

    #[serde(rename = "Pos", default)]
    pub pos: Option<fastnbt::IntArray>,

    // Alternative position format
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub z: Option<i32>,

    #[serde(flatten)]
    pub extra: HashMap<String, fastnbt::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SchemEntity {
    #[serde(rename = "Id", alias = "id", default)]
    pub id: Option<String>,

    #[serde(rename = "Pos", default)]
    pub pos: Option<Vec<f64>>,

    #[serde(flatten)]
    pub extra: HashMap<String, fastnbt::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SchemMetadata {
    #[serde(rename = "Name", alias = "name", default)]
    pub name: Option<String>,

    #[serde(rename = "Author", alias = "author", default)]
    pub author: Option<String>,

    #[serde(rename = "Date", alias = "date", default)]
    pub date: Option<i64>,

    #[serde(rename = "RequiredMods", default)]
    pub required_mods: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: HashMap<String, fastnbt::Value>,
}

impl Schem {
    /// Read variable-length integer from byte array
    fn read_varint(data: &[i8], offset: &mut usize) -> Option<i32> {
        let mut result: i32 = 0;
        let mut shift = 0;

        loop {
            if *offset >= data.len() {
                return None;
            }

            let byte = (data[*offset] as u8) as i32;
            *offset += 1;

            result |= (byte & 0x7F) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift >= 32 {
                return None;
            }
        }

        Some(result)
    }

    /// Parse block state string like "minecraft:chest[facing=north,waterlogged=false]"
    fn parse_block_state(state_str: &str) -> Block {
        if let Some(bracket_pos) = state_str.find('[') {
            let name = &state_str[..bracket_pos];
            let props_str = &state_str[bracket_pos + 1..state_str.len() - 1];

            let mut properties = HashMap::new();
            for prop in props_str.split(',') {
                if let Some(eq_pos) = prop.find('=') {
                    let key = prop[..eq_pos].to_string();
                    let value = prop[eq_pos + 1..].to_string();
                    properties.insert(key, value);
                }
            }

            Block::with_state(name, BlockState { properties })
        } else {
            Block::new(state_str)
        }
    }

    /// Get the effective structure (handles v3 nested Schematic)
    fn get_effective(&self) -> &Schem {
        self.schematic.as_deref().unwrap_or(self)
    }

    /// Convert to unified format
    pub fn to_unified(&self) -> UnifiedSchematic {
        let eff = self.get_effective();

        let version = eff.version;
        let format = if version >= 3 {
            SchematicFormat::SpongeV3
        } else {
            SchematicFormat::SpongeV2
        };

        let width = eff.width.unwrap_or(0) as u16;
        let height = eff.height.unwrap_or(0) as u16;
        let length = eff.length.unwrap_or(0) as u16;

        // Get palette and data based on version
        let empty_palette = HashMap::new();
        let empty_block_entities = Vec::new();

        let (palette, block_data, block_entities_raw) = if version >= 3 {
            // v3: blocks are in Blocks compound
            if let Some(ref blocks) = eff.blocks {
                (
                    &blocks.palette,
                    blocks.data.as_ref(),
                    &blocks.block_entities,
                )
            } else {
                // Fallback to v2 style
                (
                    eff.palette.as_ref().unwrap_or(&empty_palette),
                    eff.block_data.as_ref(),
                    if !eff.block_entities.is_empty() {
                        &eff.block_entities
                    } else {
                        &empty_block_entities
                    },
                )
            }
        } else {
            // v2
            (
                eff.palette.as_ref().unwrap_or(&empty_palette),
                eff.block_data.as_ref(),
                if !eff.block_entities.is_empty() {
                    &eff.block_entities
                } else {
                    &eff.tile_entities
                },
            )
        };

        // Build reverse palette (id -> block state string)
        let mut reverse_palette: Vec<Block> = vec![Block::air(); palette.len().max(1)];
        for (state_str, &id) in palette {
            if id >= 0 && (id as usize) < reverse_palette.len() {
                reverse_palette[id as usize] = Self::parse_block_state(state_str);
            }
        }

        // Parse block data (varint encoded)
        let volume = width as usize * height as usize * length as usize;
        let mut blocks = Vec::with_capacity(volume);

        if let Some(data) = block_data {
            let mut offset = 0;
            while blocks.len() < volume {
                if let Some(palette_id) = Self::read_varint(data.as_ref(), &mut offset) {
                    let block = reverse_palette
                        .get(palette_id as usize)
                        .cloned()
                        .unwrap_or_else(Block::air);
                    blocks.push(block);
                } else {
                    // Padding with air if data is incomplete
                    blocks.push(Block::air());
                }
            }
        } else {
            // No block data, fill with air
            blocks.resize(volume, Block::air());
        }

        // Parse block entities
        let block_entities: Vec<BlockEntity> = block_entities_raw.iter().map(|be| {
            let id = be.id.clone().unwrap_or_else(|| "unknown".to_string());

            let pos = if let Some(ref pos_arr) = be.pos {
                (
                    pos_arr.get(0).copied().unwrap_or(0),
                    pos_arr.get(1).copied().unwrap_or(0),
                    pos_arr.get(2).copied().unwrap_or(0),
                )
            } else {
                (
                    be.x.unwrap_or(0),
                    be.y.unwrap_or(0),
                    be.z.unwrap_or(0),
                )
            };

            let mut data = HashMap::new();
            for (key, value) in &be.extra {
                data.insert(key.clone(), format_nbt_value(value));
            }

            BlockEntity { id, pos, data }
        }).collect();

        // Parse entities
        let entities: Vec<Entity> = eff.entities.iter().filter_map(|e| {
            let id = e.id.clone()?;
            let pos_vec = e.pos.as_ref()?;
            if pos_vec.len() < 3 {
                return None;
            }

            let pos = (pos_vec[0], pos_vec[1], pos_vec[2]);

            let mut data = HashMap::new();
            for (key, value) in &e.extra {
                data.insert(key.clone(), format_nbt_value(value));
            }

            Some(Entity { id, pos, data })
        }).collect();

        // Parse metadata
        let metadata = eff.metadata.as_ref().map(|m| {
            let mut extra = HashMap::new();
            for (key, value) in &m.extra {
                extra.insert(key.clone(), format_nbt_value(value));
            }

            Metadata {
                name: m.name.clone(),
                author: m.author.clone(),
                date: m.date,
                required_mods: m.required_mods.clone().unwrap_or_default(),
                extra,
            }
        }).unwrap_or_default();

        UnifiedSchematic {
            format,
            width,
            height,
            length,
            blocks,
            block_entities,
            entities,
            metadata,
        }
    }
}

impl From<Schem> for UnifiedSchematic {
    fn from(schem: Schem) -> Self {
        schem.to_unified()
    }
}

/// Format NBT value for display
fn format_nbt_value(value: &fastnbt::Value) -> String {
    match value {
        fastnbt::Value::Byte(b) => b.to_string(),
        fastnbt::Value::Short(s) => s.to_string(),
        fastnbt::Value::Int(i) => i.to_string(),
        fastnbt::Value::Long(l) => l.to_string(),
        fastnbt::Value::Float(f) => f.to_string(),
        fastnbt::Value::Double(d) => d.to_string(),
        fastnbt::Value::String(s) => s.clone(),
        fastnbt::Value::ByteArray(arr) => format!("[{} bytes]", arr.len()),
        fastnbt::Value::IntArray(arr) => format!("[{} ints]", arr.len()),
        fastnbt::Value::LongArray(arr) => format!("[{} longs]", arr.len()),
        fastnbt::Value::List(list) => format!("[{} items]", list.len()),
        fastnbt::Value::Compound(map) => format!("{{{} entries}}", map.len()),
    }
}
