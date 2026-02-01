//! Legacy .schematic format (MCEdit)
//!
//! Structure:
//! - Width, Height, Length: short
//! - Blocks: byte array (block IDs)
//! - Data: byte array (block data/damage values)
//! - Entities: list of entity compounds
//! - TileEntities: list of tile entity compounds

use serde::{Deserialize, Serialize};
use crate::{
    Block, BlockState, BlockEntity, Entity, Metadata,
    SchematicFormat, UnifiedSchematic,
    block::{legacy_id_to_name, legacy_data_to_state},
};
use std::collections::HashMap;

/// Legacy MCEdit schematic format
#[derive(Debug, Deserialize, Serialize)]
pub struct Schematic {
    #[serde(rename = "Width")]
    pub width: i16,

    #[serde(rename = "Height")]
    pub height: i16,

    #[serde(rename = "Length")]
    pub length: i16,

    #[serde(rename = "Materials", default)]
    pub materials: Option<String>,

    #[serde(rename = "Blocks")]
    pub blocks: fastnbt::ByteArray,

    #[serde(rename = "Data")]
    pub data: fastnbt::ByteArray,

    #[serde(rename = "AddBlocks", default)]
    pub add_blocks: Option<fastnbt::ByteArray>,

    #[serde(rename = "Entities", default)]
    pub entities: Vec<LegacyEntity>,

    #[serde(rename = "TileEntities", default)]
    pub tile_entities: Vec<LegacyTileEntity>,

    // Optional offset
    #[serde(rename = "WEOffsetX", default)]
    pub we_offset_x: Option<i32>,

    #[serde(rename = "WEOffsetY", default)]
    pub we_offset_y: Option<i32>,

    #[serde(rename = "WEOffsetZ", default)]
    pub we_offset_z: Option<i32>,

    // Schematica specific
    #[serde(rename = "SchematicaMapping", default)]
    pub schematica_mapping: Option<HashMap<String, i16>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LegacyEntity {
    pub id: Option<String>,
    #[serde(rename = "Pos", default)]
    pub pos: Option<Vec<f64>>,
    #[serde(flatten)]
    pub extra: HashMap<String, fastnbt::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LegacyTileEntity {
    pub id: Option<String>,
    #[serde(rename = "Id", default)]
    pub id_alt: Option<String>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub z: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, fastnbt::Value>,
}

impl Schematic {
    /// Get block ID at position (supports AddBlocks for IDs > 255)
    fn get_block_id(&self, index: usize) -> u16 {
        let base_id = self.blocks.get(index).copied().unwrap_or(0) as u16;

        if let Some(ref add_blocks) = self.add_blocks {
            // AddBlocks uses nibbles for upper 4 bits
            let add_index = index / 2;
            if let Some(&add_byte) = add_blocks.get(add_index) {
                let nibble = if index % 2 == 0 {
                    add_byte & 0x0F
                } else {
                    (add_byte >> 4) & 0x0F
                };
                return base_id | ((nibble as u16) << 8);
            }
        }

        base_id
    }

    /// Convert to unified format
    pub fn to_unified(&self) -> UnifiedSchematic {
        let width = self.width as u16;
        let height = self.height as u16;
        let length = self.length as u16;

        // Build reverse mapping if Schematica mapping exists
        let id_to_name: Option<HashMap<i16, String>> = self.schematica_mapping.as_ref().map(|m| {
            m.iter().map(|(name, id)| (*id, name.clone())).collect()
        });

        // Parse blocks
        let mut blocks = Vec::with_capacity(self.blocks.len());
        for y in 0..height {
            for z in 0..length {
                for x in 0..width {
                    let index = (y as usize * length as usize + z as usize) * width as usize + x as usize;
                    let block_id = self.get_block_id(index);
                    let data_value = self.data.get(index).copied().unwrap_or(0) as u8;

                    let (name, state) = if let Some(ref mapping) = id_to_name {
                        // Use Schematica mapping
                        if let Some(mapped_name) = mapping.get(&(block_id as i16)) {
                            (mapped_name.clone(), BlockState::default())
                        } else {
                            (legacy_id_to_name(block_id as u8, data_value), legacy_data_to_state(block_id as u8, data_value))
                        }
                    } else {
                        // Use legacy ID mapping
                        (legacy_id_to_name(block_id as u8, data_value), legacy_data_to_state(block_id as u8, data_value))
                    };

                    blocks.push(Block::with_state(name, state));
                }
            }
        }

        // Parse tile entities
        let block_entities: Vec<BlockEntity> = self.tile_entities.iter().map(|te| {
            let id = te.id.clone()
                .or_else(|| te.id_alt.clone())
                .unwrap_or_else(|| "unknown".to_string());

            let pos = (
                te.x.unwrap_or(0),
                te.y.unwrap_or(0),
                te.z.unwrap_or(0),
            );

            let mut data = HashMap::new();
            for (key, value) in &te.extra {
                data.insert(key.clone(), format!("{:?}", value));
            }

            BlockEntity { id, pos, data }
        }).collect();

        // Parse entities
        let entities: Vec<Entity> = self.entities.iter().filter_map(|e| {
            let id = e.id.clone()?;
            let pos_vec = e.pos.as_ref()?;
            if pos_vec.len() < 3 {
                return None;
            }

            let pos = (pos_vec[0], pos_vec[1], pos_vec[2]);

            let mut data = HashMap::new();
            for (key, value) in &e.extra {
                data.insert(key.clone(), format!("{:?}", value));
            }

            Some(Entity { id, pos, data })
        }).collect();

        UnifiedSchematic {
            format: SchematicFormat::Legacy,
            width,
            height,
            length,
            blocks,
            block_entities,
            entities,
            metadata: Metadata::default(),
        }
    }
}

impl From<Schematic> for UnifiedSchematic {
    fn from(schematic: Schematic) -> Self {
        schematic.to_unified()
    }
}
