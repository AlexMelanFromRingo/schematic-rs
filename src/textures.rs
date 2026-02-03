//! Texture extraction and management for Minecraft block textures
//!
//! Extracts textures from installed Minecraft client.jar

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use image::{GenericImageView, ImageBuffer, Rgba};

/// Get the default Minecraft directory based on OS
pub fn get_minecraft_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        dirs::data_dir().map(|p| p.join(".minecraft"))
    }

    #[cfg(target_os = "macos")]
    {
        dirs::home_dir().map(|p| p.join("Library/Application Support/minecraft"))
    }

    #[cfg(target_os = "linux")]
    {
        dirs::home_dir().map(|p| p.join(".minecraft"))
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        None
    }
}

/// Get the texture cache directory
pub fn get_cache_dir() -> Option<PathBuf> {
    dirs::cache_dir().map(|p| p.join("schematic-rs").join("textures"))
}

/// Find the latest Minecraft client.jar
pub fn find_client_jar(minecraft_dir: &Path) -> Option<PathBuf> {
    let versions_dir = minecraft_dir.join("versions");
    if !versions_dir.exists() {
        return None;
    }

    let mut jars: Vec<(PathBuf, String)> = Vec::new();

    if let Ok(entries) = fs::read_dir(&versions_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let version_name = path.file_name()?.to_string_lossy().to_string();
                // Skip snapshots and pre-releases for stability
                if version_name.contains('-') || version_name.contains("w") {
                    continue;
                }
                let jar_path = path.join(format!("{}.jar", version_name));
                if jar_path.exists() {
                    jars.push((jar_path, version_name));
                }
            }
        }
    }

    // Sort by version (simple string sort works for 1.x.x format)
    jars.sort_by(|a, b| b.1.cmp(&a.1));
    jars.first().map(|(p, _)| p.clone())
}

/// Extract block textures from client.jar to cache directory
pub fn extract_textures(jar_path: &Path, cache_dir: &Path) -> std::io::Result<usize> {
    let file = File::open(jar_path)?;
    let mut archive = ZipArchive::new(file).map_err(|e| std::io::Error::other(e.to_string()))?;

    fs::create_dir_all(cache_dir)?;

    let mut count = 0;
    let prefix = "assets/minecraft/textures/block/";

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| std::io::Error::other(e.to_string()))?;
        let name = file.name().to_string();

        if name.starts_with(prefix) && name.ends_with(".png") {
            let texture_name = &name[prefix.len()..];
            let dest_path = cache_dir.join(texture_name);

            // Create parent dirs if needed
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;

            let mut dest_file = File::create(&dest_path)?;
            dest_file.write_all(&contents)?;
            count += 1;
        }
    }

    Ok(count)
}

/// Check if textures are cached
pub fn textures_cached(cache_dir: &Path) -> bool {
    cache_dir.exists() && cache_dir.join("stone.png").exists()
}

/// Texture manager for block textures
pub struct TextureManager {
    texture_dir: PathBuf,
    texture_map: HashMap<String, PathBuf>,
}

impl TextureManager {
    /// Create a new texture manager with the given texture directory
    pub fn new(texture_dir: PathBuf) -> Self {
        let mut manager = Self {
            texture_dir,
            texture_map: HashMap::new(),
        };
        manager.scan_textures();
        manager
    }

    /// Try to initialize from cache or extract from Minecraft
    pub fn from_minecraft() -> Option<Self> {
        Self::from_minecraft_with_path(None)
    }

    /// Try to initialize with optional custom Minecraft path or jar path
    pub fn from_minecraft_with_path(custom_path: Option<&Path>) -> Option<Self> {
        let cache_dir = get_cache_dir()?;

        // Determine jar path
        let jar_path = if let Some(path) = custom_path {
            if path.extension().is_some_and(|e| e == "jar") {
                // Direct jar path
                path.to_path_buf()
            } else {
                // Minecraft directory - look for client jar
                find_client_jar(path)?
            }
        } else {
            // Auto-detect
            let mc_dir = get_minecraft_dir()?;
            find_client_jar(&mc_dir)?
        };

        // Check if we need to re-extract (different jar)
        let jar_marker = cache_dir.join(".source_jar");
        let jar_path_str = jar_path.to_string_lossy().to_string();
        let need_extract = if textures_cached(&cache_dir) {
            // Check if source jar changed
            match std::fs::read_to_string(&jar_marker) {
                Ok(cached_jar) => cached_jar.trim() != jar_path_str,
                Err(_) => true,
            }
        } else {
            true
        };

        if need_extract {
            eprintln!("Extracting textures from {:?}...", jar_path);
            match extract_textures(&jar_path, &cache_dir) {
                Ok(count) => {
                    eprintln!("Extracted {} textures", count);
                    // Save source jar path
                    let _ = std::fs::write(&jar_marker, &jar_path_str);
                }
                Err(e) => {
                    eprintln!("Failed to extract textures: {}", e);
                    return None;
                }
            }
        }

        Some(Self::new(cache_dir))
    }

    /// Scan the texture directory for available textures
    fn scan_textures(&mut self) {
        if let Ok(entries) = fs::read_dir(&self.texture_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "png").unwrap_or(false) {
                    if let Some(stem) = path.file_stem() {
                        let name = stem.to_string_lossy().to_string();
                        self.texture_map.insert(name, path);
                    }
                }
            }
        }
    }

    /// Get texture path for a block name
    pub fn get_texture(&self, block_name: &str) -> Option<&PathBuf> {
        let name = block_name
            .strip_prefix("minecraft:")
            .unwrap_or(block_name);

        // Direct match
        if let Some(path) = self.texture_map.get(name) {
            return Some(path);
        }

        // Try common variations
        let variations = get_texture_variations(name);
        for var in variations {
            if let Some(path) = self.texture_map.get(&var) {
                return Some(path);
            }
        }

        None
    }

    /// Get the texture directory path
    pub fn texture_dir(&self) -> &Path {
        &self.texture_dir
    }

    /// Check if textures are available
    pub fn has_textures(&self) -> bool {
        !self.texture_map.is_empty()
    }

    /// Get count of loaded textures
    pub fn texture_count(&self) -> usize {
        self.texture_map.len()
    }
}

/// Get tint color for a block (if it needs tinting)
/// Returns (r, g, b) multiplier where 1.0 = no change
pub fn get_block_tint(block_name: &str) -> Option<(f32, f32, f32)> {
    let name = block_name.strip_prefix("minecraft:").unwrap_or(block_name);

    // Leaves use foliage color (green tint)
    // Default foliage color from plains biome
    if name.contains("leaves") {
        return Some((0.47, 0.74, 0.34)); // #77BD2F - plains foliage
    }

    // Grass blocks and grass use grass color
    if name == "grass_block" || name == "grass" || name == "tall_grass" {
        return Some((0.57, 0.74, 0.35)); // #91BD59 - plains grass
    }

    // Vines
    if name.contains("vine") {
        return Some((0.47, 0.74, 0.34));
    }

    // Lily pad
    if name == "lily_pad" {
        return Some((0.47, 0.74, 0.34));
    }

    None
}

/// Apply tint to an image and save to destination
/// The tint multiplies each pixel's RGB values
pub fn apply_tint_and_save(src_path: &Path, dest_path: &Path, tint: (f32, f32, f32)) -> std::io::Result<()> {
    let img = image::open(src_path)
        .map_err(|e| std::io::Error::other(format!("Failed to open image: {}", e)))?;

    let (width, height) = img.dimensions();
    let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let [r, g, b, a] = pixel.0;
        let new_r = ((r as f32 * tint.0).min(255.0)) as u8;
        let new_g = ((g as f32 * tint.1).min(255.0)) as u8;
        let new_b = ((b as f32 * tint.2).min(255.0)) as u8;
        output.put_pixel(x, y, Rgba([new_r, new_g, new_b, a]));
    }

    output.save(dest_path)
        .map_err(|e| std::io::Error::other(format!("Failed to save image: {}", e)))?;

    Ok(())
}

/// Copy texture with optional tinting
pub fn copy_texture_with_tint(src_path: &Path, dest_path: &Path, block_name: &str) -> std::io::Result<()> {
    if let Some(tint) = get_block_tint(block_name) {
        apply_tint_and_save(src_path, dest_path, tint)
    } else {
        std::fs::copy(src_path, dest_path)?;
        Ok(())
    }
}

/// Get texture name variations for a block
fn get_texture_variations(name: &str) -> Vec<String> {
    let mut variations = Vec::new();

    // Handle _top, _side, _front suffixes
    variations.push(format!("{}_top", name));
    variations.push(format!("{}_side", name));
    variations.push(format!("{}_front", name));

    // Handle planks
    if name.ends_with("_planks") {
        variations.push(name.replace("_planks", "_planks"));
    }

    // Handle logs
    if name.ends_with("_log") {
        variations.push(format!("{}_top", name));
        variations.push(name.replace("_log", "_log_top"));
    }

    // Handle stairs/slabs - use base block texture
    if name.ends_with("_stairs") {
        let base = name.replace("_stairs", "");
        variations.push(base.clone());
        variations.push(format!("{}_top", base));
        variations.push(format!("{}s", base)); // stone_brick -> stone_bricks
    }

    if name.ends_with("_slab") {
        let base = name.replace("_slab", "");
        variations.push(base.clone());
        variations.push(format!("{}_top", base));
        variations.push(format!("{}s", base));
    }

    // Handle walls
    if name.ends_with("_wall") {
        let base = name.replace("_wall", "");
        variations.push(base);
    }

    // Handle concrete
    if name.ends_with("_concrete") {
        variations.push(name.to_string());
    }

    // Handle wool
    if name.ends_with("_wool") {
        variations.push(name.to_string());
    }

    // Handle glass
    if name.contains("glass") {
        variations.push(name.to_string());
        if name.contains("stained") {
            variations.push(name.replace("_stained", ""));
        }
    }

    // Handle terracotta
    if name.ends_with("_terracotta") {
        variations.push(name.to_string());
    }

    // Handle stone variants
    if name == "stone_bricks" {
        variations.push("stone_bricks".to_string());
    }

    // Handle deepslate
    if name.contains("deepslate") {
        variations.push(name.to_string());
        variations.push(format!("{}_top", name));
    }

    variations
}
