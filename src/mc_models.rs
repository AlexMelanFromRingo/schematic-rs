//! Minecraft JSON model parser
//!
//! Parses blockstates and model JSON files from Minecraft client.jar
//! to get accurate block geometry for rendering.

use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;
use zip::ZipArchive;

/// A 3D point in model space (0-16 scale)
#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    /// Convert from Minecraft's 0-16 scale to 0-1 scale
    pub fn to_unit_scale(&self) -> (f32, f32, f32) {
        (self.0 / 16.0, self.1 / 16.0, self.2 / 16.0)
    }
}

/// Face UV coordinates [u1, v1, u2, v2]
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct FaceUV(pub [f32; 4]);

impl Default for FaceUV {
    fn default() -> Self {
        FaceUV([0.0, 0.0, 16.0, 16.0])
    }
}

/// Face rotation (multiples of 90 degrees)
#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(transparent)]
pub struct FaceRotation(pub i32);

/// A single face of a model element
#[derive(Debug, Clone, Deserialize)]
pub struct ModelFace {
    /// UV coordinates [u1, v1, u2, v2], optional (defaults to element bounds)
    pub uv: Option<FaceUV>,
    /// Texture variable reference (e.g., "#top")
    pub texture: String,
    /// Face culling direction (if this face touches a solid block, hide it)
    pub cullface: Option<String>,
    /// Texture rotation (0, 90, 180, 270)
    pub rotation: Option<FaceRotation>,
    /// Tint index for biome coloring (-1 = no tint)
    #[serde(default)]
    pub tintindex: i32,
}

/// Element rotation specification
#[derive(Debug, Clone, Deserialize)]
pub struct ElementRotation {
    /// Rotation origin point
    pub origin: Vec3,
    /// Rotation axis (x, y, or z)
    pub axis: String,
    /// Rotation angle (must be -45, -22.5, 0, 22.5, or 45)
    pub angle: f32,
    /// Whether to rescale the element to fit the original bounding box
    #[serde(default)]
    pub rescale: bool,
}

/// A single cuboid element in a model
#[derive(Debug, Clone, Deserialize)]
pub struct ModelElement {
    /// Start corner (0-16 scale)
    pub from: Vec3,
    /// End corner (0-16 scale)
    pub to: Vec3,
    /// Element rotation (optional)
    pub rotation: Option<ElementRotation>,
    /// Faces of this element (keyed by direction: down, up, north, south, west, east)
    pub faces: HashMap<String, ModelFace>,
    /// Whether to use shade (default: true)
    #[serde(default = "default_shade")]
    pub shade: bool,
}

fn default_shade() -> bool {
    true
}

/// A Minecraft block model
#[derive(Debug, Clone, Deserialize, Default)]
pub struct BlockModel {
    /// Parent model to inherit from
    pub parent: Option<String>,
    /// Texture variable definitions
    #[serde(default)]
    pub textures: HashMap<String, String>,
    /// Model elements (cuboids)
    #[serde(default)]
    pub elements: Vec<ModelElement>,
    /// Ambient occlusion (default: true)
    #[serde(default = "default_ao")]
    pub ambientocclusion: bool,
}

fn default_ao() -> bool {
    true
}

/// Model reference in blockstate with transforms
#[derive(Debug, Clone, Deserialize)]
pub struct ModelRef {
    /// Model path (e.g., "minecraft:block/stone")
    pub model: String,
    /// X rotation (0, 90, 180, 270)
    #[serde(default)]
    pub x: i32,
    /// Y rotation (0, 90, 180, 270)
    #[serde(default)]
    pub y: i32,
    /// Whether to lock UVs when rotating
    #[serde(default)]
    pub uvlock: bool,
    /// Weight for random selection (default: 1)
    #[serde(default = "default_weight")]
    pub weight: i32,
}

fn default_weight() -> i32 {
    1
}

/// A condition for multipart model
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MultipartCondition {
    /// OR condition: any of these conditions
    #[serde(rename = "OR")]
    Or {
        #[serde(rename = "OR")]
        or: Vec<HashMap<String, String>>
    },
    /// AND condition: all of these conditions must match
    #[serde(rename = "AND")]
    And {
        #[serde(rename = "AND")]
        and: Vec<HashMap<String, String>>
    },
    /// Simple condition: property -> value
    Simple(HashMap<String, String>),
}

/// A multipart entry
#[derive(Debug, Clone, Deserialize)]
pub struct MultipartEntry {
    /// Condition for when to apply this model
    pub when: Option<MultipartCondition>,
    /// Model(s) to apply
    pub apply: MultipartApply,
}

/// Model application (single or list)
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum MultipartApply {
    Single(ModelRef),
    Multiple(Vec<ModelRef>),
}

/// Blockstate variants definition
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Variants {
    /// Single model for a variant
    Single(ModelRef),
    /// Multiple weighted models for random selection
    Multiple(Vec<ModelRef>),
}

/// Blockstate definition
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Blockstate {
    /// Variants-based blockstate (one model per state combination)
    Variants {
        variants: HashMap<String, Variants>,
    },
    /// Multipart blockstate (combine multiple models based on conditions)
    Multipart {
        multipart: Vec<MultipartEntry>,
    },
}

/// Resolved model with all elements and textures
#[derive(Debug, Clone, Default)]
pub struct ResolvedModel {
    /// All elements from this model and parents
    pub elements: Vec<ModelElement>,
    /// All resolved texture paths
    pub textures: HashMap<String, String>,
    /// Ambient occlusion
    pub ambient_occlusion: bool,
}

/// Minecraft model manager - loads and caches models from client.jar
pub struct ModelManager {
    /// Cached blockstates (vanilla)
    blockstates: HashMap<String, Blockstate>,
    /// Cached models (vanilla)
    models: HashMap<String, BlockModel>,
    /// Resource pack blockstates (override vanilla)
    resource_pack_blockstates: HashMap<String, Blockstate>,
    /// Resource pack models (override vanilla)
    resource_pack_models: HashMap<String, BlockModel>,
    /// Resolved models cache
    resolved_cache: HashMap<String, ResolvedModel>,
}

impl ModelManager {
    /// Create a new model manager from a Minecraft client.jar
    pub fn from_jar<P: AsRef<Path>>(jar_path: P) -> std::io::Result<Self> {
        Self::from_jar_with_resource_pack(jar_path, None::<&std::path::Path>)
    }

    /// Create a new model manager from a Minecraft client.jar with optional resource pack
    pub fn from_jar_with_resource_pack<P: AsRef<Path>, R: AsRef<Path>>(
        jar_path: P,
        resource_pack: Option<R>,
    ) -> std::io::Result<Self> {
        let jar_path = jar_path.as_ref();
        let file = std::fs::File::open(jar_path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| std::io::Error::other(format!("Failed to open jar: {}", e)))?;

        let mut blockstates = HashMap::new();
        let mut models = HashMap::new();

        // Load all blockstates
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| std::io::Error::other(e.to_string()))?;
            let name = file.name().to_string();

            if name.starts_with("assets/minecraft/blockstates/") && name.ends_with(".json") {
                let block_name = name
                    .strip_prefix("assets/minecraft/blockstates/")
                    .unwrap()
                    .strip_suffix(".json")
                    .unwrap();

                let mut content = String::new();
                file.read_to_string(&mut content)?;

                match serde_json::from_str::<Blockstate>(&content) {
                    Ok(bs) => {
                        blockstates.insert(block_name.to_string(), bs);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse blockstate {}: {}", block_name, e);
                    }
                }
            }
        }

        // Load all block models
        let file = std::fs::File::open(jar_path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| std::io::Error::other(e.to_string()))?;
            let name = file.name().to_string();

            if name.starts_with("assets/minecraft/models/block/") && name.ends_with(".json") {
                let model_name = name
                    .strip_prefix("assets/minecraft/models/")
                    .unwrap()
                    .strip_suffix(".json")
                    .unwrap();

                let mut content = String::new();
                file.read_to_string(&mut content)?;

                match serde_json::from_str::<BlockModel>(&content) {
                    Ok(model) => {
                        models.insert(model_name.to_string(), model);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse model {}: {}", model_name, e);
                    }
                }
            }
        }

        eprintln!("Loaded {} blockstates and {} models", blockstates.len(), models.len());

        let mut manager = Self {
            blockstates,
            models,
            resource_pack_blockstates: HashMap::new(),
            resource_pack_models: HashMap::new(),
            resolved_cache: HashMap::new(),
        };

        // Load resource pack if provided
        if let Some(pack_path) = resource_pack {
            match manager.load_resource_pack(pack_path.as_ref()) {
                Ok((bs_count, model_count)) => {
                    if bs_count > 0 || model_count > 0 {
                        eprintln!("Loaded {} blockstates and {} models from resource pack", bs_count, model_count);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load resource pack: {}", e);
                }
            }
        }

        Ok(manager)
    }

    /// Load blockstates and models from a resource pack (ZIP file)
    pub fn load_resource_pack(&mut self, pack_path: &Path) -> std::io::Result<(usize, usize)> {
        let file = std::fs::File::open(pack_path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| std::io::Error::other(format!("Failed to open resource pack: {}", e)))?;

        let mut bs_count = 0;
        let mut model_count = 0;

        // Load blockstates
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| std::io::Error::other(e.to_string()))?;
            let name = file.name().to_string();

            if name.starts_with("assets/minecraft/blockstates/") && name.ends_with(".json") {
                let block_name = name
                    .strip_prefix("assets/minecraft/blockstates/")
                    .unwrap()
                    .strip_suffix(".json")
                    .unwrap();

                let mut content = String::new();
                file.read_to_string(&mut content)?;

                match serde_json::from_str::<Blockstate>(&content) {
                    Ok(bs) => {
                        self.resource_pack_blockstates.insert(block_name.to_string(), bs);
                        bs_count += 1;
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse resource pack blockstate {}: {}", block_name, e);
                    }
                }
            }
        }

        // Reload archive for models
        let file = std::fs::File::open(pack_path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        // Load models
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| std::io::Error::other(e.to_string()))?;
            let name = file.name().to_string();

            if name.starts_with("assets/minecraft/models/block/") && name.ends_with(".json") {
                let model_name = name
                    .strip_prefix("assets/minecraft/models/")
                    .unwrap()
                    .strip_suffix(".json")
                    .unwrap();

                let mut content = String::new();
                file.read_to_string(&mut content)?;

                match serde_json::from_str::<BlockModel>(&content) {
                    Ok(model) => {
                        self.resource_pack_models.insert(model_name.to_string(), model);
                        model_count += 1;
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse resource pack model {}: {}", model_name, e);
                    }
                }
            }
        }

        // Clear resolved cache since models may have changed
        self.resolved_cache.clear();

        Ok((bs_count, model_count))
    }

    /// Get model references for a block with given properties
    /// Checks resource pack first, then falls back to vanilla
    pub fn get_models_for_block(&self, block_name: &str, properties: &HashMap<String, String>) -> Vec<(ModelRef, String)> {
        let name = block_name.strip_prefix("minecraft:").unwrap_or(block_name);

        // Check resource pack first, then vanilla
        let blockstate = self.resource_pack_blockstates.get(name)
            .or_else(|| self.blockstates.get(name));

        let Some(blockstate) = blockstate else {
            return Vec::new();
        };

        match blockstate {
            Blockstate::Variants { variants } => {
                // Find matching variant by checking if all variant properties match
                // This handles cases where schematic has extra properties (waterlogged, etc.)
                let variant = if properties.is_empty() {
                    variants.get("")
                        .or_else(|| variants.get("normal"))
                        .or_else(|| variants.values().next())
                } else {
                    // Try to find a variant where all its properties match ours
                    let mut best_match: Option<(&String, &Variants)> = None;
                    let mut best_score = 0;

                    for (variant_key, variant_val) in variants.iter() {
                        if variant_key.is_empty() {
                            continue;
                        }

                        // Parse variant key into property map
                        let variant_props: HashMap<&str, &str> = variant_key
                            .split(',')
                            .filter_map(|part| {
                                let mut split = part.splitn(2, '=');
                                match (split.next(), split.next()) {
                                    (Some(k), Some(v)) => Some((k, v)),
                                    _ => None,
                                }
                            })
                            .collect();

                        // Check if all variant properties match our block's properties
                        let all_match = variant_props.iter().all(|(k, v)| {
                            properties.get(*k).map(|pv| pv.as_str() == *v).unwrap_or(false)
                        });

                        if all_match {
                            let score = variant_props.len();
                            if score > best_score {
                                best_score = score;
                                best_match = Some((variant_key, variant_val));
                            }
                        }
                    }

                    best_match.map(|(_, v)| v)
                        .or_else(|| variants.get(""))
                        .or_else(|| variants.values().next())
                };

                match variant {
                    Some(Variants::Single(model_ref)) => {
                        vec![(model_ref.clone(), name.to_string())]
                    }
                    Some(Variants::Multiple(refs)) => {
                        // Just use the first one (or could be random weighted)
                        if let Some(r) = refs.first() {
                            vec![(r.clone(), name.to_string())]
                        } else {
                            Vec::new()
                        }
                    }
                    None => Vec::new(),
                }
            }
            Blockstate::Multipart { multipart } => {
                let mut result = Vec::new();

                for entry in multipart {
                    let matches = match &entry.when {
                        None => true,
                        Some(MultipartCondition::Simple(conditions)) => {
                            conditions.iter().all(|(key, expected)| {
                                // Handle multiple values separated by |
                                let values: Vec<&str> = expected.split('|').collect();
                                properties.get(key)
                                    .map(|v| values.contains(&v.as_str()))
                                    .unwrap_or(false)
                            })
                        }
                        Some(MultipartCondition::Or { or }) => {
                            or.iter().any(|conditions| {
                                conditions.iter().all(|(key, expected)| {
                                    let values: Vec<&str> = expected.split('|').collect();
                                    properties.get(key)
                                        .map(|v| values.contains(&v.as_str()))
                                        .unwrap_or(false)
                                })
                            })
                        }
                        Some(MultipartCondition::And { and }) => {
                            and.iter().all(|conditions| {
                                conditions.iter().all(|(key, expected)| {
                                    let values: Vec<&str> = expected.split('|').collect();
                                    properties.get(key)
                                        .map(|v| values.contains(&v.as_str()))
                                        .unwrap_or(false)
                                })
                            })
                        }
                    };

                    if matches {
                        match &entry.apply {
                            MultipartApply::Single(model_ref) => {
                                result.push((model_ref.clone(), name.to_string()));
                            }
                            MultipartApply::Multiple(refs) => {
                                if let Some(r) = refs.first() {
                                    result.push((r.clone(), name.to_string()));
                                }
                            }
                        }
                    }
                }

                result
            }
        }
    }

    /// Resolve a model by name, following parent chain
    /// Checks resource pack first, then falls back to vanilla
    pub fn resolve_model(&mut self, model_path: &str) -> Option<ResolvedModel> {
        // Check cache first
        if let Some(cached) = self.resolved_cache.get(model_path) {
            return Some(cached.clone());
        }

        // Normalize path
        let normalized = model_path
            .strip_prefix("minecraft:")
            .unwrap_or(model_path);

        // Check resource pack first, then vanilla
        let model = self.resource_pack_models.get(normalized)
            .or_else(|| self.models.get(normalized))?
            .clone();

        // Resolve parent chain
        let mut resolved = ResolvedModel::default();
        resolved.ambient_occlusion = model.ambientocclusion;

        // Start with parent's elements and textures
        if let Some(parent) = &model.parent {
            let parent_path = parent.strip_prefix("minecraft:").unwrap_or(parent);
            // Skip builtin parents
            if !parent_path.starts_with("builtin/") {
                if let Some(parent_resolved) = self.resolve_model(parent_path) {
                    resolved = parent_resolved;
                }
            }
        }

        // Override with this model's textures
        for (key, value) in &model.textures {
            resolved.textures.insert(key.clone(), value.clone());
        }

        // Resolve texture references (e.g., #particle -> #side -> actual texture)
        let mut final_textures = HashMap::new();
        for (key, value) in &resolved.textures {
            let resolved_value = self.resolve_texture_ref(value, &resolved.textures);
            final_textures.insert(key.clone(), resolved_value);
        }
        resolved.textures = final_textures;

        // Override elements if this model has any
        if !model.elements.is_empty() {
            resolved.elements = model.elements.clone();
        }

        // Cache and return
        self.resolved_cache.insert(model_path.to_string(), resolved.clone());
        Some(resolved)
    }

    /// Resolve a texture reference like "#side" to actual texture path
    fn resolve_texture_ref(&self, texture: &str, textures: &HashMap<String, String>) -> String {
        if !texture.starts_with('#') {
            return texture.to_string();
        }

        let key = &texture[1..];
        if let Some(value) = textures.get(key) {
            if value.starts_with('#') {
                // Recursive reference
                self.resolve_texture_ref(value, textures)
            } else {
                value.clone()
            }
        } else {
            texture.to_string()
        }
    }

    /// Get resolved texture path for a face
    pub fn resolve_face_texture(&self, face: &ModelFace, textures: &HashMap<String, String>) -> String {
        self.resolve_texture_ref(&face.texture, textures)
    }

    /// Get the number of loaded blockstates
    pub fn blockstate_count(&self) -> usize {
        self.blockstates.len()
    }

    /// Get the number of loaded models
    pub fn model_count(&self) -> usize {
        self.models.len()
    }
}

/// Apply rotation to a point around origin
pub fn rotate_point(point: (f32, f32, f32), x_rot: i32, y_rot: i32) -> (f32, f32, f32) {
    let (mut x, mut y, mut z) = point;

    // Center point for rotation (0.5, 0.5, 0.5 in unit scale)
    let cx = 0.5;
    let cy = 0.5;
    let cz = 0.5;

    // Translate to origin
    x -= cx;
    y -= cy;
    z -= cz;

    // Apply X rotation
    match x_rot {
        90 => {
            let (new_y, new_z) = (-z, y);
            y = new_y;
            z = new_z;
        }
        180 => {
            y = -y;
            z = -z;
        }
        270 => {
            let (new_y, new_z) = (z, -y);
            y = new_y;
            z = new_z;
        }
        _ => {}
    }

    // Apply Y rotation
    match y_rot {
        90 => {
            let (new_x, new_z) = (-z, x);
            x = new_x;
            z = new_z;
        }
        180 => {
            x = -x;
            z = -z;
        }
        270 => {
            let (new_x, new_z) = (z, -x);
            x = new_x;
            z = new_z;
        }
        _ => {}
    }

    // Translate back
    (x + cx, y + cy, z + cz)
}

/// Face direction enum for rotation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceDirection {
    Down, Up, North, South, West, East,
}

impl FaceDirection {
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "down" => Some(Self::Down),
            "up" => Some(Self::Up),
            "north" => Some(Self::North),
            "south" => Some(Self::South),
            "west" => Some(Self::West),
            "east" => Some(Self::East),
            _ => None,
        }
    }

    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Down => "down",
            Self::Up => "up",
            Self::North => "north",
            Self::South => "south",
            Self::West => "west",
            Self::East => "east",
        }
    }

    /// Rotate around X axis
    pub fn rotate_x(self, angle: i32) -> Self {
        match angle {
            90 => match self {
                Self::Up => Self::North,
                Self::North => Self::Down,
                Self::Down => Self::South,
                Self::South => Self::Up,
                other => other,
            },
            180 => match self {
                Self::Up => Self::Down,
                Self::Down => Self::Up,
                Self::North => Self::South,
                Self::South => Self::North,
                other => other,
            },
            270 => match self {
                Self::Up => Self::South,
                Self::South => Self::Down,
                Self::Down => Self::North,
                Self::North => Self::Up,
                other => other,
            },
            _ => self,
        }
    }

    /// Rotate around Y axis
    pub fn rotate_y(self, angle: i32) -> Self {
        match angle {
            90 => match self {
                Self::North => Self::East,
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
                other => other,
            },
            180 => match self {
                Self::North => Self::South,
                Self::South => Self::North,
                Self::East => Self::West,
                Self::West => Self::East,
                other => other,
            },
            270 => match self {
                Self::North => Self::West,
                Self::West => Self::South,
                Self::South => Self::East,
                Self::East => Self::North,
                other => other,
            },
            _ => self,
        }
    }
}

/// Rotate a face direction based on x/y rotation
pub fn rotate_face_direction(face: &str, x_rot: i32, y_rot: i32) -> &'static str {
    let Some(dir) = FaceDirection::from_str(face) else {
        return "north"; // Default fallback
    };

    dir.rotate_x(x_rot).rotate_y(y_rot).as_str()
}

/// A generated quad ready for OBJ export
#[derive(Debug, Clone)]
pub struct GeneratedQuad {
    /// Four vertices in counter-clockwise order
    pub vertices: [(f32, f32, f32); 4],
    /// UV coordinates for each vertex
    pub uv_coords: [(f32, f32); 4],
    /// Texture path (e.g., "block/stone")
    pub texture: String,
    /// Face direction for culling
    pub face_dir: FaceDirection,
    /// Tint index (-1 = no tint)
    pub tint_index: i32,
}

/// Apply element rotation around an origin point
fn apply_element_rotation(
    point: (f32, f32, f32),
    rotation: &ElementRotation,
) -> (f32, f32, f32) {
    let (mut x, mut y, mut z) = point;
    let (ox, oy, oz) = rotation.origin.to_unit_scale();

    // Translate to origin
    x -= ox;
    y -= oy;
    z -= oz;

    // Convert angle to radians
    let angle_rad = rotation.angle.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    // Apply rotation based on axis
    let (nx, ny, nz) = match rotation.axis.as_str() {
        "x" => (x, y * cos_a - z * sin_a, y * sin_a + z * cos_a),
        "y" => (x * cos_a + z * sin_a, y, -x * sin_a + z * cos_a),
        "z" => (x * cos_a - y * sin_a, x * sin_a + y * cos_a, z),
        _ => (x, y, z),
    };

    // Rescale if needed (to maintain bounding box)
    let (fx, fy, fz) = if rotation.rescale {
        let scale = 1.0 / cos_a.abs().max(0.001);
        (nx * scale, ny * scale, nz * scale)
    } else {
        (nx, ny, nz)
    };

    // Translate back
    (fx + ox, fy + oy, fz + oz)
}

/// Generate quads from a resolved model with rotation applied
pub fn generate_model_quads(
    model: &ResolvedModel,
    x_rot: i32,
    y_rot: i32,
    world_x: f32,
    world_y: f32,
    world_z: f32,
) -> Vec<GeneratedQuad> {
    let mut quads = Vec::new();

    for element in &model.elements {
        // Get element bounds in unit scale (0-1)
        let (x0, y0, z0) = element.from.to_unit_scale();
        let (x1, y1, z1) = element.to.to_unit_scale();

        // Generate quad for each face
        for (face_name, face) in &element.faces {
            let Some(face_dir) = FaceDirection::from_str(face_name) else {
                continue;
            };

            // Get texture
            let texture = if face.texture.starts_with('#') {
                model.textures.get(&face.texture[1..])
                    .cloned()
                    .unwrap_or_else(|| face.texture.clone())
            } else {
                face.texture.clone()
            };

            // Get UV coordinates (default to full face if not specified)
            let uv = face.uv.as_ref().map(|u| u.0).unwrap_or_else(|| {
                // Default UVs based on face direction
                match face_dir {
                    FaceDirection::Down | FaceDirection::Up => [x0 * 16.0, z0 * 16.0, x1 * 16.0, z1 * 16.0],
                    FaceDirection::North | FaceDirection::South => [x0 * 16.0, y0 * 16.0, x1 * 16.0, y1 * 16.0],
                    FaceDirection::West | FaceDirection::East => [z0 * 16.0, y0 * 16.0, z1 * 16.0, y1 * 16.0],
                }
            });

            // Generate vertices for this face (in element local space, 0-1)
            let local_verts = match face_dir {
                FaceDirection::Down => [
                    (x0, y0, z1), (x1, y0, z1), (x1, y0, z0), (x0, y0, z0),
                ],
                FaceDirection::Up => [
                    (x0, y1, z0), (x1, y1, z0), (x1, y1, z1), (x0, y1, z1),
                ],
                FaceDirection::North => [
                    (x1, y0, z0), (x0, y0, z0), (x0, y1, z0), (x1, y1, z0),
                ],
                FaceDirection::South => [
                    (x0, y0, z1), (x1, y0, z1), (x1, y1, z1), (x0, y1, z1),
                ],
                FaceDirection::West => [
                    (x0, y0, z0), (x0, y0, z1), (x0, y1, z1), (x0, y1, z0),
                ],
                FaceDirection::East => [
                    (x1, y0, z1), (x1, y0, z0), (x1, y1, z0), (x1, y1, z1),
                ],
            };

            // Apply element rotation if present (e.g., 45° for cross models)
            let element_rotated = if let Some(ref rot) = element.rotation {
                [
                    apply_element_rotation(local_verts[0], rot),
                    apply_element_rotation(local_verts[1], rot),
                    apply_element_rotation(local_verts[2], rot),
                    apply_element_rotation(local_verts[3], rot),
                ]
            } else {
                local_verts
            };

            // Apply model rotation (x_rot, y_rot from blockstate)
            let mut rotated_verts: [(f32, f32, f32); 4] = [
                rotate_point(element_rotated[0], x_rot, y_rot),
                rotate_point(element_rotated[1], x_rot, y_rot),
                rotate_point(element_rotated[2], x_rot, y_rot),
                rotate_point(element_rotated[3], x_rot, y_rot),
            ];

            // 180-degree rotations flip the winding order (improper rotation)
            // If only one of x_rot or y_rot is 180, we need to reverse vertex order
            let x_flip = x_rot == 180;
            let y_flip = y_rot == 180;
            if x_flip != y_flip {
                // Reverse winding order by swapping vertices 1 and 3
                rotated_verts.swap(1, 3);
            }

            // Transform to world space
            let world_verts = [
                (rotated_verts[0].0 + world_x, rotated_verts[0].1 + world_y, rotated_verts[0].2 + world_z),
                (rotated_verts[1].0 + world_x, rotated_verts[1].1 + world_y, rotated_verts[1].2 + world_z),
                (rotated_verts[2].0 + world_x, rotated_verts[2].1 + world_y, rotated_verts[2].2 + world_z),
                (rotated_verts[3].0 + world_x, rotated_verts[3].1 + world_y, rotated_verts[3].2 + world_z),
            ];

            // Rotate face direction to match model rotation
            let rotated_face_dir = face_dir.rotate_x(x_rot).rotate_y(y_rot);

            // UV coordinates (normalized to 0-1 range from 0-16)
            let uv_coords = [
                (uv[0] / 16.0, uv[1] / 16.0),
                (uv[2] / 16.0, uv[1] / 16.0),
                (uv[2] / 16.0, uv[3] / 16.0),
                (uv[0] / 16.0, uv[3] / 16.0),
            ];

            quads.push(GeneratedQuad {
                vertices: world_verts,
                uv_coords,
                texture,
                face_dir: rotated_face_dir,
                tint_index: face.tintindex,
            });
        }
    }

    quads
}

/// Check if a model fully covers a face (for face culling)
pub fn model_covers_face(model: &ResolvedModel, face: FaceDirection, x_rot: i32, y_rot: i32) -> bool {
    // Get the face direction in model space (reverse rotation)
    let model_face = face
        .rotate_y((-y_rot).rem_euclid(360))
        .rotate_x((-x_rot).rem_euclid(360));

    // Check if any element fully covers this face
    for element in &model.elements {
        let (x0, y0, z0) = element.from.to_unit_scale();
        let (x1, y1, z1) = element.to.to_unit_scale();

        let covers = match model_face {
            FaceDirection::Down => y0 <= 0.001 && x0 <= 0.001 && z0 <= 0.001 && x1 >= 0.999 && z1 >= 0.999,
            FaceDirection::Up => y1 >= 0.999 && x0 <= 0.001 && z0 <= 0.001 && x1 >= 0.999 && z1 >= 0.999,
            FaceDirection::North => z0 <= 0.001 && x0 <= 0.001 && y0 <= 0.001 && x1 >= 0.999 && y1 >= 0.999,
            FaceDirection::South => z1 >= 0.999 && x0 <= 0.001 && y0 <= 0.001 && x1 >= 0.999 && y1 >= 0.999,
            FaceDirection::West => x0 <= 0.001 && y0 <= 0.001 && z0 <= 0.001 && y1 >= 0.999 && z1 >= 0.999,
            FaceDirection::East => x1 >= 0.999 && y0 <= 0.001 && z0 <= 0.001 && y1 >= 0.999 && z1 >= 0.999,
        };

        if covers && element.faces.contains_key(model_face.as_str()) {
            return true;
        }
    }

    false
}

/// Check if a model is a full cube (can use greedy meshing)
pub fn is_full_cube_model(model: &ResolvedModel) -> bool {
    if model.elements.len() != 1 {
        return false;
    }

    let elem = &model.elements[0];
    let (x0, y0, z0) = elem.from.to_unit_scale();
    let (x1, y1, z1) = elem.to.to_unit_scale();

    // Check if it's a full 0-1 cube
    x0 <= 0.001 && y0 <= 0.001 && z0 <= 0.001 &&
    x1 >= 0.999 && y1 >= 0.999 && z1 >= 0.999
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_point() {
        // Test Y rotation 90 degrees
        let p = rotate_point((1.0, 0.5, 0.5), 0, 90);
        assert!((p.0 - 0.5).abs() < 0.001);
        assert!((p.2 - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_rotate_face() {
        assert_eq!(rotate_face_direction("north", 0, 90), "east");
        assert_eq!(rotate_face_direction("up", 90, 0), "north");
    }
}
