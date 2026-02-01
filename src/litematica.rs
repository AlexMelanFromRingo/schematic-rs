//! Litematica format (.litematic)
//!
//! Structure:
//! - Version: int (4-6)
//! - MinecraftDataVersion: int
//! - Metadata: compound
//!   - Name, Author, Description
//!   - RegionCount, TotalBlocks, TotalVolume
//!   - TimeCreated, TimeModified
//!   - EnclosingSize: {x, y, z}
//! - Regions: compound
//!   - [region_name]: compound
//!     - Position: {x, y, z}
//!     - Size: {x, y, z}
//!     - BlockStatePalette: list of block state compounds
//!     - BlockStates: LongArray (packed bit storage)
//!     - TileEntities: list
//!     - Entities: list

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{
    Block, BlockState, BlockEntity, Entity, Metadata,
    SchematicFormat, UnifiedSchematic,
};

/// Litematica format
#[derive(Debug, Deserialize, Serialize)]
pub struct Litematica {
    #[serde(rename = "Version")]
    pub version: i32,

    #[serde(rename = "MinecraftDataVersion", default)]
    pub minecraft_data_version: Option<i32>,

    #[serde(rename = "Metadata")]
    pub metadata: LitematicaMetadata,

    #[serde(rename = "Regions")]
    pub regions: HashMap<String, LitematicaRegion>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LitematicaMetadata {
    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "Author", default)]
    pub author: Option<String>,

    #[serde(rename = "Description", default)]
    pub description: Option<String>,

    #[serde(rename = "RegionCount", default)]
    pub region_count: Option<i32>,

    #[serde(rename = "TotalBlocks", default)]
    pub total_blocks: Option<i64>,

    #[serde(rename = "TotalVolume", default)]
    pub total_volume: Option<i64>,

    #[serde(rename = "TimeCreated", default)]
    pub time_created: Option<i64>,

    #[serde(rename = "TimeModified", default)]
    pub time_modified: Option<i64>,

    #[serde(rename = "EnclosingSize", default)]
    pub enclosing_size: Option<LitematicaSize>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LitematicaSize {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LitematicaRegion {
    #[serde(rename = "Position", default)]
    pub position: Option<LitematicaSize>,

    #[serde(rename = "Size", default)]
    pub size: Option<LitematicaSize>,

    #[serde(rename = "BlockStatePalette", default)]
    pub block_state_palette: Vec<LitematicaBlockState>,

    #[serde(rename = "BlockStates", default)]
    pub block_states: Option<fastnbt::LongArray>,

    #[serde(rename = "TileEntities", default)]
    pub tile_entities: Vec<LitematicaTileEntity>,

    #[serde(rename = "Entities", default)]
    pub entities: Vec<LitematicaEntity>,

    #[serde(rename = "PendingBlockTicks", default)]
    pub pending_block_ticks: Vec<fastnbt::Value>,

    #[serde(rename = "PendingFluidTicks", default)]
    pub pending_fluid_ticks: Vec<fastnbt::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LitematicaBlockState {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Properties", default)]
    pub properties: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LitematicaTileEntity {
    pub id: Option<String>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub z: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, fastnbt::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LitematicaEntity {
    pub id: Option<String>,
    #[serde(rename = "Pos", default)]
    pub pos: Option<Vec<f64>>,
    #[serde(flatten)]
    pub extra: HashMap<String, fastnbt::Value>,
}

impl Litematica {
    /// Convert to unified format
    pub fn to_unified(&self) -> UnifiedSchematic {
        // Get enclosing size from metadata
        let (width, height, length) = if let Some(ref size) = self.metadata.enclosing_size {
            (size.x.unsigned_abs() as u16, size.y.unsigned_abs() as u16, size.z.unsigned_abs() as u16)
        } else {
            // Calculate from regions
            let mut max_x = 0i32;
            let mut max_y = 0i32;
            let mut max_z = 0i32;

            for region in self.regions.values() {
                if let (Some(pos), Some(size)) = (&region.position, &region.size) {
                    max_x = max_x.max(pos.x.abs() + size.x.abs());
                    max_y = max_y.max(pos.y.abs() + size.y.abs());
                    max_z = max_z.max(pos.z.abs() + size.z.abs());
                }
            }
            (max_x as u16, max_y as u16, max_z as u16)
        };

        let volume = width as usize * height as usize * length as usize;
        let mut blocks = vec![Block::air(); volume];
        let mut block_entities = Vec::new();
        let mut entities = Vec::new();

        // Process each region
        for region in self.regions.values() {
            let region_size = region.size.as_ref().map(|s| (s.x, s.y, s.z)).unwrap_or((0, 0, 0));
            let region_pos = region.position.as_ref().map(|p| (p.x, p.y, p.z)).unwrap_or((0, 0, 0));

            // Build palette
            let palette: Vec<Block> = region.block_state_palette.iter().map(|bs| {
                let state = BlockState {
                    properties: bs.properties.clone().unwrap_or_default(),
                };
                Block::with_state(&bs.name, state)
            }).collect();

            if palette.is_empty() {
                continue;
            }

            // Decode packed block states
            if let Some(ref block_states) = region.block_states {
                let bits_per_block = calculate_bits_per_block(palette.len());
                let region_width = region_size.0.unsigned_abs() as usize;
                let region_height = region_size.1.unsigned_abs() as usize;
                let region_length = region_size.2.unsigned_abs() as usize;
                let region_volume = region_width * region_height * region_length;

                // Decode blocks
                let decoded = decode_packed_array(block_states, bits_per_block, region_volume);

                // Place blocks in the unified grid
                for (i, &palette_idx) in decoded.iter().enumerate() {
                    if palette_idx >= palette.len() {
                        continue;
                    }

                    // Litematica uses YZX order
                    let ry = i / (region_length * region_width);
                    let rz = (i / region_width) % region_length;
                    let rx = i % region_width;

                    // Apply region offset (handle negative sizes)
                    let gx = if region_size.0 < 0 {
                        region_pos.0 + region_size.0 + 1 + rx as i32
                    } else {
                        region_pos.0 + rx as i32
                    };
                    let gy = if region_size.1 < 0 {
                        region_pos.1 + region_size.1 + 1 + ry as i32
                    } else {
                        region_pos.1 + ry as i32
                    };
                    let gz = if region_size.2 < 0 {
                        region_pos.2 + region_size.2 + 1 + rz as i32
                    } else {
                        region_pos.2 + rz as i32
                    };

                    if gx >= 0 && gy >= 0 && gz >= 0 {
                        let gx = gx as u16;
                        let gy = gy as u16;
                        let gz = gz as u16;

                        if gx < width && gy < height && gz < length {
                            let idx = (gy as usize * length as usize + gz as usize) * width as usize + gx as usize;
                            if idx < blocks.len() {
                                blocks[idx] = palette[palette_idx].clone();
                            }
                        }
                    }
                }
            }

            // Process tile entities
            for te in &region.tile_entities {
                let id = te.id.clone().unwrap_or_else(|| "unknown".to_string());
                let pos = (
                    te.x.unwrap_or(0) + region_pos.0,
                    te.y.unwrap_or(0) + region_pos.1,
                    te.z.unwrap_or(0) + region_pos.2,
                );
                let mut data = HashMap::new();
                for (key, value) in &te.extra {
                    data.insert(key.clone(), format!("{:?}", value));
                }
                block_entities.push(BlockEntity { id, pos, data });
            }

            // Process entities
            for e in &region.entities {
                if let Some(ref id) = e.id {
                    if let Some(ref pos_vec) = e.pos {
                        if pos_vec.len() >= 3 {
                            let pos = (
                                pos_vec[0] + region_pos.0 as f64,
                                pos_vec[1] + region_pos.1 as f64,
                                pos_vec[2] + region_pos.2 as f64,
                            );
                            let mut data = HashMap::new();
                            for (key, value) in &e.extra {
                                data.insert(key.clone(), format!("{:?}", value));
                            }
                            entities.push(Entity { id: id.clone(), pos, data });
                        }
                    }
                }
            }
        }

        // Build metadata
        let metadata = Metadata {
            name: self.metadata.name.clone(),
            author: self.metadata.author.clone(),
            date: self.metadata.time_created,
            required_mods: Vec::new(),
            extra: HashMap::new(),
        };

        UnifiedSchematic {
            format: SchematicFormat::Litematica,
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

impl From<Litematica> for UnifiedSchematic {
    fn from(lit: Litematica) -> Self {
        lit.to_unified()
    }
}

/// Calculate bits per block based on palette size
fn calculate_bits_per_block(palette_size: usize) -> usize {
    if palette_size <= 1 {
        return 1; // Minimum 1 bit
    }
    let bits = (palette_size as f64).log2().ceil() as usize;
    bits.max(1)
}

/// Decode packed long array into block indices
fn decode_packed_array(data: &fastnbt::LongArray, bits_per_block: usize, count: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(count);
    let mask = (1u64 << bits_per_block) - 1;

    let mut bit_offset = 0usize;

    for _ in 0..count {
        let long_index = bit_offset / 64;
        let bit_in_long = bit_offset % 64;

        if long_index >= data.len() {
            result.push(0);
            bit_offset += bits_per_block;
            continue;
        }

        let long_val = data[long_index] as u64;

        let value = if bit_in_long + bits_per_block <= 64 {
            // Value fits in single long
            ((long_val >> bit_in_long) & mask) as usize
        } else {
            // Value spans two longs
            let bits_in_first = 64 - bit_in_long;
            let bits_in_second = bits_per_block - bits_in_first;

            let first_part = long_val >> bit_in_long;
            let second_part = if long_index + 1 < data.len() {
                let next_long = data[long_index + 1] as u64;
                (next_long & ((1u64 << bits_in_second) - 1)) << bits_in_first
            } else {
                0
            };

            ((first_part | second_part) & mask) as usize
        };

        result.push(value);
        bit_offset += bits_per_block;
    }

    result
}
