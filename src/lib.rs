pub mod schematic;
pub mod schem;
pub mod litematica;
pub mod block;
pub mod block_geometry;
pub mod mc_models;
pub mod error;
pub mod recipes;
pub mod export3d;
pub mod textures;

pub use schematic::Schematic;
pub use schem::Schem;
pub use litematica::Litematica;
pub use block::{Block, BlockState};
pub use error::SchemError;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use flate2::read::GzDecoder;

/// Unified schematic representation
#[derive(Debug)]
pub struct UnifiedSchematic {
    pub format: SchematicFormat,
    pub width: u16,
    pub height: u16,
    pub length: u16,
    pub blocks: Vec<Block>,
    pub block_entities: Vec<BlockEntity>,
    pub entities: Vec<Entity>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub enum SchematicFormat {
    /// Old MCEdit format (.schematic)
    Legacy,
    /// Sponge Schematic v2 (.schem)
    SpongeV2,
    /// Sponge Schematic v3 (.schem)
    SpongeV3,
    /// Litematica format (.litematic)
    Litematica,
}

#[derive(Debug, Clone, Default)]
pub struct BlockEntity {
    pub id: String,
    pub pos: (i32, i32, i32),
    pub data: std::collections::HashMap<String, String>,
}

impl BlockEntity {
    /// Check if this is a sign
    pub fn is_sign(&self) -> bool {
        self.id.contains("sign")
    }

    /// Extract text from a sign (supports both old and new formats)
    pub fn get_sign_text(&self) -> Option<SignText> {
        if !self.is_sign() {
            return None;
        }

        let mut front_lines = Vec::new();
        let mut back_lines = Vec::new();

        // Try new format (1.20+): front_text/back_text with messages
        if let Some(front) = self.data.get("front_text") {
            front_lines = parse_sign_text_compound(front);
        }
        if let Some(back) = self.data.get("back_text") {
            back_lines = parse_sign_text_compound(back);
        }

        // Try old format: Text1, Text2, Text3, Text4
        if front_lines.is_empty() {
            for i in 1..=4 {
                let key = format!("Text{}", i);
                if let Some(text) = self.data.get(&key) {
                    let parsed = parse_json_text(text);
                    if !parsed.is_empty() {
                        front_lines.push(parsed);
                    }
                }
            }
        }

        if front_lines.is_empty() && back_lines.is_empty() {
            return None;
        }

        Some(SignText {
            front: front_lines,
            back: back_lines,
        })
    }
}

/// Parsed sign text
#[derive(Debug, Clone, Default)]
pub struct SignText {
    pub front: Vec<String>,
    pub back: Vec<String>,
}

impl SignText {
    pub fn is_empty(&self) -> bool {
        self.front.iter().all(|s| s.is_empty()) && self.back.iter().all(|s| s.is_empty())
    }

    pub fn front_text(&self) -> String {
        self.front.join("\n")
    }

    pub fn back_text(&self) -> String {
        self.back.join("\n")
    }
}

/// Parse JSON text component to plain text
fn parse_json_text(json_str: &str) -> String {
    // Handle raw quoted string
    let trimmed = json_str.trim();
    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        let inner = &trimmed[1..trimmed.len()-1];
        // Unescape basic escapes
        return inner.replace("\\\"", "\"").replace("\\n", "\n");
    }

    // Try to parse as JSON
    if trimmed.starts_with('{') {
        // Extract "text" field from JSON object
        if let Some(start) = trimmed.find("\"text\"") {
            let after_key = &trimmed[start + 6..];
            if let Some(colon) = after_key.find(':') {
                let after_colon = after_key[colon + 1..].trim_start();
                if after_colon.starts_with('"') {
                    // Find closing quote
                    let mut end = 1;
                    let chars: Vec<char> = after_colon.chars().collect();
                    while end < chars.len() {
                        if chars[end] == '"' && (end == 0 || chars[end-1] != '\\') {
                            break;
                        }
                        end += 1;
                    }
                    let text: String = chars[1..end].iter().collect();
                    return text.replace("\\\"", "\"").replace("\\n", "\n");
                }
            }
        }
    }

    // Return as-is if can't parse
    trimmed.to_string()
}

/// Parse sign text compound (1.20+ format)
fn parse_sign_text_compound(data: &str) -> Vec<String> {
    let mut lines = Vec::new();

    // Look for messages array entries
    // Format: messages=["{...}", "{...}", ...]
    if let Some(start) = data.find("messages=") {
        let after = &data[start + 9..];
        // Find all JSON strings in the array
        let mut in_string = false;
        let mut current = String::new();
        let mut escape_next = false;

        for ch in after.chars() {
            if escape_next {
                current.push(ch);
                escape_next = false;
                continue;
            }

            match ch {
                '\\' => {
                    current.push(ch);
                    escape_next = true;
                }
                '"' => {
                    if in_string {
                        // End of string
                        lines.push(parse_json_text(&format!("\"{}\"", current)));
                        current.clear();
                        in_string = false;
                    } else {
                        // Start of string
                        in_string = true;
                    }
                }
                ']' if !in_string => break,
                _ => {
                    if in_string {
                        current.push(ch);
                    }
                }
            }
        }
    }

    lines
}

#[derive(Debug, Clone, Default)]
pub struct Entity {
    pub id: String,
    pub pos: (f64, f64, f64),
    pub data: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct Metadata {
    pub name: Option<String>,
    pub author: Option<String>,
    pub date: Option<i64>,
    pub required_mods: Vec<String>,
    pub extra: std::collections::HashMap<String, String>,
}

impl UnifiedSchematic {
    /// Load schematic from file, auto-detecting format
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, SchemError> {
        let path = path.as_ref();

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read first bytes to check if gzipped
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let data = if buf.starts_with(&[0x1f, 0x8b]) {
            // GZIP compressed
            let mut decoder = GzDecoder::new(&buf[..]);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)?;
            decompressed
        } else {
            buf
        };

        // Try to detect format from content, not just extension
        // Order matters: try more specific formats first

        // 1. Try Litematica (has "Regions" and "Metadata" fields)
        if let Ok(lit) = fastnbt::from_bytes::<Litematica>(&data) {
            return Ok(lit.into());
        }

        // 2. Try Sponge v3 wrapped format (root "Schematic" compound)
        if let Ok(wrapped) = fastnbt::from_bytes::<schem::SchemWrapper>(&data) {
            return Ok(wrapped.schematic.into());
        }

        // 3. Try Sponge v2/v3 direct format
        if let Ok(schem) = fastnbt::from_bytes::<Schem>(&data) {
            return Ok(schem.into());
        }

        // 4. Try legacy .schematic format
        if let Ok(schematic) = fastnbt::from_bytes::<Schematic>(&data) {
            return Ok(schematic.into());
        }

        Err(SchemError::UnknownFormat)
    }

    /// Get block at position
    pub fn get_block(&self, x: u16, y: u16, z: u16) -> Option<&Block> {
        if x >= self.width || y >= self.height || z >= self.length {
            return None;
        }
        let index = (y as usize * self.length as usize + z as usize) * self.width as usize + x as usize;
        self.blocks.get(index)
    }

    /// Count blocks by type
    pub fn block_counts(&self) -> std::collections::HashMap<String, usize> {
        let mut counts = std::collections::HashMap::new();
        for block in &self.blocks {
            *counts.entry(block.name.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// Get all unique block types
    pub fn unique_blocks(&self) -> Vec<&Block> {
        let mut seen = std::collections::HashSet::new();
        let mut unique = Vec::new();
        for block in &self.blocks {
            let key = format!("{}{:?}", block.name, block.state);
            if seen.insert(key) {
                unique.push(block);
            }
        }
        unique
    }

    /// Get dimensions as string
    pub fn dimensions_str(&self) -> String {
        format!("{}x{}x{}", self.width, self.height, self.length)
    }

    /// Total volume
    pub fn volume(&self) -> usize {
        self.width as usize * self.height as usize * self.length as usize
    }

    /// Non-air block count
    pub fn solid_blocks(&self) -> usize {
        self.blocks.iter()
            .filter(|b| !b.is_air())
            .count()
    }

    /// Get all signs with their text
    pub fn get_signs(&self) -> Vec<(&BlockEntity, SignText)> {
        self.block_entities.iter()
            .filter_map(|be| {
                be.get_sign_text().map(|text| (be, text))
            })
            .collect()
    }
}
