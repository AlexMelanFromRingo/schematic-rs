//! Block geometry definitions for partial/non-full blocks
//!
//! This module defines the 3D shapes of Minecraft blocks that don't fill
//! the entire 1x1x1 cube space. Used for:
//! - Determining if a block occludes adjacent faces
//! - Generating correct mesh geometry for partial blocks

use std::collections::HashMap;

/// Axis-aligned bounding box (coordinates 0.0-1.0 within block space)
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: (f32, f32, f32),
    pub max: (f32, f32, f32),
}

impl AABB {
    pub const fn new(min: (f32, f32, f32), max: (f32, f32, f32)) -> Self {
        Self { min, max }
    }

    /// Full cube from (0,0,0) to (1,1,1)
    pub const fn full() -> Self {
        Self::new((0.0, 0.0, 0.0), (1.0, 1.0, 1.0))
    }

    /// Check if this AABB fully covers a face of the unit cube
    pub fn covers_face(&self, face: Face) -> bool {
        const E: f32 = 0.001; // epsilon for float comparison

        match face {
            Face::XNeg => self.min.0 <= E &&
                          self.min.1 <= E && self.max.1 >= 1.0 - E &&
                          self.min.2 <= E && self.max.2 >= 1.0 - E,
            Face::XPos => self.max.0 >= 1.0 - E &&
                          self.min.1 <= E && self.max.1 >= 1.0 - E &&
                          self.min.2 <= E && self.max.2 >= 1.0 - E,
            Face::YNeg => self.min.1 <= E &&
                          self.min.0 <= E && self.max.0 >= 1.0 - E &&
                          self.min.2 <= E && self.max.2 >= 1.0 - E,
            Face::YPos => self.max.1 >= 1.0 - E &&
                          self.min.0 <= E && self.max.0 >= 1.0 - E &&
                          self.min.2 <= E && self.max.2 >= 1.0 - E,
            Face::ZNeg => self.min.2 <= E &&
                          self.min.0 <= E && self.max.0 >= 1.0 - E &&
                          self.min.1 <= E && self.max.1 >= 1.0 - E,
            Face::ZPos => self.max.2 >= 1.0 - E &&
                          self.min.0 <= E && self.max.0 >= 1.0 - E &&
                          self.min.1 <= E && self.max.1 >= 1.0 - E,
        }
    }
}

/// Face direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    XNeg, XPos, // -X, +X
    YNeg, YPos, // -Y (bottom), +Y (top)
    ZNeg, ZPos, // -Z, +Z
}

impl Face {
    pub const ALL: [Face; 6] = [Face::XNeg, Face::XPos, Face::YNeg, Face::YPos, Face::ZNeg, Face::ZPos];
}

/// Block geometry - one or more AABBs
#[derive(Debug, Clone)]
pub enum BlockGeometry {
    /// Full 1x1x1 cube
    Full,
    /// Single box with custom dimensions
    Single(AABB),
    /// Multiple boxes (stairs, fences, etc.)
    Multi(Vec<AABB>),
    /// No solid geometry (air, flowers, etc.)
    Empty,
}

impl BlockGeometry {
    /// Check if this geometry fully covers a specific face
    pub fn covers_face(&self, face: Face) -> bool {
        match self {
            BlockGeometry::Full => true,
            BlockGeometry::Single(aabb) => aabb.covers_face(face),
            BlockGeometry::Multi(boxes) => {
                // For multi-box geometry, we'd need to check if boxes together
                // cover the entire face. For simplicity, return false (conservative)
                // TODO: implement proper face coverage check for multi-box
                boxes.iter().any(|b| b.covers_face(face))
            }
            BlockGeometry::Empty => false,
        }
    }

    /// Check if this is a full cube
    pub fn is_full(&self) -> bool {
        matches!(self, BlockGeometry::Full)
    }

    /// Check if this block has any solid geometry
    pub fn is_solid(&self) -> bool {
        !matches!(self, BlockGeometry::Empty)
    }

    /// Get all AABBs for mesh generation
    pub fn get_boxes(&self) -> Vec<AABB> {
        match self {
            BlockGeometry::Full => vec![AABB::full()],
            BlockGeometry::Single(aabb) => vec![*aabb],
            BlockGeometry::Multi(boxes) => boxes.clone(),
            BlockGeometry::Empty => vec![],
        }
    }
}

// ============================================================================
// Common geometry constants
// ============================================================================

/// Bottom slab (y: 0.0 to 0.5)
pub const SLAB_BOTTOM: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 0.5, 1.0));
/// Top slab (y: 0.5 to 1.0)
pub const SLAB_TOP: AABB = AABB::new((0.0, 0.5, 0.0), (1.0, 1.0, 1.0));

/// Snow layer height (1/8 of block per layer)
pub const fn snow_layer(layers: u8) -> AABB {
    let h = layers as f32 / 8.0;
    AABB::new((0.0, 0.0, 0.0), (1.0, h, 1.0))
}

/// Carpet (very thin, 1/16 height)
pub const CARPET: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 0.0625, 1.0));

/// Pressure plate
pub const PRESSURE_PLATE: AABB = AABB::new((0.0625, 0.0, 0.0625), (0.9375, 0.0625, 0.9375));

/// Fence post (center)
pub const FENCE_POST: AABB = AABB::new((0.375, 0.0, 0.375), (0.625, 1.0, 0.625));

/// Glass pane / iron bars (center, north-south oriented)
pub const PANE_NS: AABB = AABB::new((0.4375, 0.0, 0.0), (0.5625, 1.0, 1.0));
/// Glass pane / iron bars (center, east-west oriented)
pub const PANE_EW: AABB = AABB::new((0.0, 0.0, 0.4375), (1.0, 1.0, 0.5625));

/// Ladder (attached to wall)
pub const LADDER_NORTH: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 1.0, 0.1875));
pub const LADDER_SOUTH: AABB = AABB::new((0.0, 0.0, 0.8125), (1.0, 1.0, 1.0));
pub const LADDER_WEST: AABB = AABB::new((0.0, 0.0, 0.0), (0.1875, 1.0, 1.0));
pub const LADDER_EAST: AABB = AABB::new((0.8125, 0.0, 0.0), (1.0, 1.0, 1.0));

/// Trapdoor (closed, bottom)
pub const TRAPDOOR_BOTTOM: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 0.1875, 1.0));
/// Trapdoor (closed, top)
pub const TRAPDOOR_TOP: AABB = AABB::new((0.0, 0.8125, 0.0), (1.0, 1.0, 1.0));

/// Door (lower half, facing directions)
pub const DOOR_NORTH: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 1.0, 0.1875));
pub const DOOR_SOUTH: AABB = AABB::new((0.0, 0.0, 0.8125), (1.0, 1.0, 1.0));
pub const DOOR_WEST: AABB = AABB::new((0.0, 0.0, 0.0), (0.1875, 1.0, 1.0));
pub const DOOR_EAST: AABB = AABB::new((0.8125, 0.0, 0.0), (1.0, 1.0, 1.0));

/// Bed (0.5625 height)
pub const BED: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 0.5625, 1.0));

/// Chest
pub const CHEST: AABB = AABB::new((0.0625, 0.0, 0.0625), (0.9375, 0.875, 0.9375));

/// Enchanting table
pub const ENCHANTING_TABLE: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 0.75, 1.0));

/// End portal frame
pub const END_PORTAL_FRAME: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 0.8125, 1.0));

/// Hopper
pub const HOPPER_TOP: AABB = AABB::new((0.0, 0.625, 0.0), (1.0, 1.0, 1.0));
pub const HOPPER_MIDDLE: AABB = AABB::new((0.25, 0.25, 0.25), (0.75, 0.625, 0.75));
pub const HOPPER_BOTTOM: AABB = AABB::new((0.375, 0.0, 0.375), (0.625, 0.25, 0.625));

/// Lectern
pub const LECTERN_BASE: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 0.125, 1.0));
pub const LECTERN_POST: AABB = AABB::new((0.25, 0.125, 0.25), (0.75, 0.875, 0.75));
pub const LECTERN_TOP: AABB = AABB::new((0.0, 0.875, 0.0), (1.0, 1.0, 1.0));

/// Cauldron (hollow - outer shell)
pub const CAULDRON: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 1.0, 1.0)); // Simplified

/// Anvil (simplified)
pub const ANVIL: AABB = AABB::new((0.125, 0.0, 0.0), (0.875, 1.0, 1.0));

/// Bell
pub const BELL: AABB = AABB::new((0.25, 0.25, 0.25), (0.75, 1.0, 0.75));

/// Brewing stand
pub const BREWING_STAND: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 0.875, 1.0));

/// Flower pot
pub const FLOWER_POT: AABB = AABB::new((0.3125, 0.0, 0.3125), (0.6875, 0.375, 0.6875));

/// Lantern (hanging)
pub const LANTERN_HANGING: AABB = AABB::new((0.3125, 0.0625, 0.3125), (0.6875, 0.5, 0.6875));
/// Lantern (standing)
pub const LANTERN_STANDING: AABB = AABB::new((0.3125, 0.0, 0.3125), (0.6875, 0.4375, 0.6875));

/// Candle (single)
pub const CANDLE: AABB = AABB::new((0.4375, 0.0, 0.4375), (0.5625, 0.375, 0.5625));

/// Torch
pub const TORCH_STANDING: AABB = AABB::new((0.4375, 0.0, 0.4375), (0.5625, 0.625, 0.5625));

/// Wall torch (attached to wall)
pub fn wall_torch(facing: &str) -> AABB {
    match facing {
        "north" => AABB::new((0.4375, 0.1875, 0.5625), (0.5625, 0.8125, 1.0)),
        "south" => AABB::new((0.4375, 0.1875, 0.0), (0.5625, 0.8125, 0.4375)),
        "west" => AABB::new((0.5625, 0.1875, 0.4375), (1.0, 0.8125, 0.5625)),
        "east" => AABB::new((0.0, 0.1875, 0.4375), (0.4375, 0.8125, 0.5625)),
        _ => TORCH_STANDING,
    }
}

/// Button
pub fn button(face: &str, facing: &str) -> AABB {
    match face {
        "floor" => AABB::new((0.3125, 0.0, 0.375), (0.6875, 0.125, 0.625)),
        "ceiling" => AABB::new((0.3125, 0.875, 0.375), (0.6875, 1.0, 0.625)),
        "wall" => match facing {
            "north" => AABB::new((0.3125, 0.375, 0.875), (0.6875, 0.625, 1.0)),
            "south" => AABB::new((0.3125, 0.375, 0.0), (0.6875, 0.625, 0.125)),
            "west" => AABB::new((0.875, 0.375, 0.3125), (1.0, 0.625, 0.6875)),
            "east" => AABB::new((0.0, 0.375, 0.3125), (0.125, 0.625, 0.6875)),
            _ => AABB::new((0.3125, 0.375, 0.875), (0.6875, 0.625, 1.0)),
        },
        _ => AABB::new((0.3125, 0.0, 0.375), (0.6875, 0.125, 0.625)),
    }
}

/// Lever
pub fn lever(face: &str, facing: &str) -> AABB {
    match face {
        "floor" => AABB::new((0.3125, 0.0, 0.25), (0.6875, 0.625, 0.75)),
        "ceiling" => AABB::new((0.3125, 0.375, 0.25), (0.6875, 1.0, 0.75)),
        "wall" => match facing {
            "north" => AABB::new((0.3125, 0.25, 0.625), (0.6875, 0.75, 1.0)),
            "south" => AABB::new((0.3125, 0.25, 0.0), (0.6875, 0.75, 0.375)),
            "west" => AABB::new((0.625, 0.25, 0.3125), (1.0, 0.75, 0.6875)),
            "east" => AABB::new((0.0, 0.25, 0.3125), (0.375, 0.75, 0.6875)),
            _ => AABB::new((0.3125, 0.25, 0.625), (0.6875, 0.75, 1.0)),
        },
        _ => AABB::new((0.3125, 0.0, 0.25), (0.6875, 0.625, 0.75)),
    }
}

/// Rail (flat)
pub const RAIL_FLAT: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 0.125, 1.0));

/// Redstone repeater/comparator
pub const REPEATER: AABB = AABB::new((0.0, 0.0, 0.0), (1.0, 0.125, 1.0));

/// Chain
pub const CHAIN_Y: AABB = AABB::new((0.40625, 0.0, 0.40625), (0.59375, 1.0, 0.59375));

/// End rod
pub const END_ROD_Y: AABB = AABB::new((0.375, 0.0, 0.375), (0.625, 1.0, 0.625));

/// Lightning rod
pub const LIGHTNING_ROD: AABB = AABB::new((0.375, 0.0, 0.375), (0.625, 1.0, 0.625));

/// Wall (center post)
pub const WALL_POST: AABB = AABB::new((0.25, 0.0, 0.25), (0.75, 1.0, 0.75));
/// Wall segment (north)
pub const WALL_NORTH: AABB = AABB::new((0.3125, 0.0, 0.0), (0.6875, 0.875, 0.5));
/// Wall segment (south)
pub const WALL_SOUTH: AABB = AABB::new((0.3125, 0.0, 0.5), (0.6875, 0.875, 1.0));
/// Wall segment (west)
pub const WALL_WEST: AABB = AABB::new((0.0, 0.0, 0.3125), (0.5, 0.875, 0.6875));
/// Wall segment (east)
pub const WALL_EAST: AABB = AABB::new((0.5, 0.0, 0.3125), (1.0, 0.875, 0.6875));

// ============================================================================
// Stair geometry helpers
// ============================================================================

/// Get stair geometry based on facing and half
pub fn stair_geometry(facing: &str, half: &str, shape: &str) -> BlockGeometry {
    let bottom = half == "bottom";

    // Base slab
    let base = if bottom { SLAB_BOTTOM } else { SLAB_TOP };

    // Step position depends on facing and shape
    let step = match (facing, shape, bottom) {
        // Straight stairs
        ("north", "straight", true) => AABB::new((0.0, 0.5, 0.0), (1.0, 1.0, 0.5)),
        ("north", "straight", false) => AABB::new((0.0, 0.0, 0.0), (1.0, 0.5, 0.5)),
        ("south", "straight", true) => AABB::new((0.0, 0.5, 0.5), (1.0, 1.0, 1.0)),
        ("south", "straight", false) => AABB::new((0.0, 0.0, 0.5), (1.0, 0.5, 1.0)),
        ("west", "straight", true) => AABB::new((0.0, 0.5, 0.0), (0.5, 1.0, 1.0)),
        ("west", "straight", false) => AABB::new((0.0, 0.0, 0.0), (0.5, 0.5, 1.0)),
        ("east", "straight", true) => AABB::new((0.5, 0.5, 0.0), (1.0, 1.0, 1.0)),
        ("east", "straight", false) => AABB::new((0.5, 0.0, 0.0), (1.0, 0.5, 1.0)),

        // Inner/outer corners - simplified to straight for now
        // TODO: implement proper corner geometry
        _ => match (facing, bottom) {
            ("north", true) => AABB::new((0.0, 0.5, 0.0), (1.0, 1.0, 0.5)),
            ("north", false) => AABB::new((0.0, 0.0, 0.0), (1.0, 0.5, 0.5)),
            ("south", true) => AABB::new((0.0, 0.5, 0.5), (1.0, 1.0, 1.0)),
            ("south", false) => AABB::new((0.0, 0.0, 0.5), (1.0, 0.5, 1.0)),
            ("west", true) => AABB::new((0.0, 0.5, 0.0), (0.5, 1.0, 1.0)),
            ("west", false) => AABB::new((0.0, 0.0, 0.0), (0.5, 0.5, 1.0)),
            ("east", true) => AABB::new((0.5, 0.5, 0.0), (1.0, 1.0, 1.0)),
            ("east", false) => AABB::new((0.5, 0.0, 0.0), (1.0, 0.5, 1.0)),
            _ => AABB::new((0.0, 0.5, 0.0), (1.0, 1.0, 0.5)),
        }
    };

    BlockGeometry::Multi(vec![base, step])
}

// ============================================================================
// Main geometry lookup function
// ============================================================================

/// Get the geometry for a block based on its name and properties
pub fn get_block_geometry(name: &str, properties: &HashMap<String, String>) -> BlockGeometry {
    let name = name.strip_prefix("minecraft:").unwrap_or(name);

    // Air and related
    if matches!(name, "air" | "cave_air" | "void_air") {
        return BlockGeometry::Empty;
    }

    // Slabs
    if name.contains("slab") {
        let slab_type = properties.get("type").map(|s| s.as_str()).unwrap_or("bottom");
        return match slab_type {
            "top" => BlockGeometry::Single(SLAB_TOP),
            "double" => BlockGeometry::Full,
            _ => BlockGeometry::Single(SLAB_BOTTOM),
        };
    }

    // Stairs
    if name.contains("stairs") {
        let facing = properties.get("facing").map(|s| s.as_str()).unwrap_or("north");
        let half = properties.get("half").map(|s| s.as_str()).unwrap_or("bottom");
        let shape = properties.get("shape").map(|s| s.as_str()).unwrap_or("straight");
        return stair_geometry(facing, half, shape);
    }

    // Doors
    if name.contains("door") && !name.contains("trapdoor") {
        let facing = properties.get("facing").map(|s| s.as_str()).unwrap_or("north");
        let hinge = properties.get("hinge").map(|s| s.as_str()).unwrap_or("left");
        let open = properties.get("open").map(|s| s.as_str()).unwrap_or("false") == "true";

        // Calculate actual facing based on hinge and open state
        let actual_facing = if open {
            match (facing, hinge) {
                ("north", "left") => "west",
                ("north", "right") => "east",
                ("south", "left") => "east",
                ("south", "right") => "west",
                ("west", "left") => "south",
                ("west", "right") => "north",
                ("east", "left") => "north",
                ("east", "right") => "south",
                _ => facing,
            }
        } else {
            facing
        };

        return BlockGeometry::Single(match actual_facing {
            "north" => DOOR_NORTH,
            "south" => DOOR_SOUTH,
            "west" => DOOR_WEST,
            "east" => DOOR_EAST,
            _ => DOOR_NORTH,
        });
    }

    // Trapdoors
    if name.contains("trapdoor") {
        let half = properties.get("half").map(|s| s.as_str()).unwrap_or("bottom");
        let open = properties.get("open").map(|s| s.as_str()).unwrap_or("false") == "true";

        if open {
            let facing = properties.get("facing").map(|s| s.as_str()).unwrap_or("north");
            return BlockGeometry::Single(match facing {
                "north" => LADDER_SOUTH, // When open, acts like a ladder
                "south" => LADDER_NORTH,
                "west" => LADDER_EAST,
                "east" => LADDER_WEST,
                _ => LADDER_NORTH,
            });
        }

        return BlockGeometry::Single(if half == "top" { TRAPDOOR_TOP } else { TRAPDOOR_BOTTOM });
    }

    // Fence gates
    if name.contains("fence_gate") {
        // Simplified - gates are thin when closed
        let facing = properties.get("facing").map(|s| s.as_str()).unwrap_or("north");
        let open = properties.get("open").map(|s| s.as_str()).unwrap_or("false") == "true";

        if open {
            // When open, gate is on the sides - very simplified
            return BlockGeometry::Empty;
        }

        return BlockGeometry::Single(match facing {
            "north" | "south" => PANE_EW,
            _ => PANE_NS,
        });
    }

    // Fences
    if name.contains("fence") {
        // Just the post for now - connections would need neighbor checking
        return BlockGeometry::Single(FENCE_POST);
    }

    // Walls
    if name.contains("wall") && !name.contains("sign") {
        return BlockGeometry::Single(WALL_POST);
    }

    // Glass panes, iron bars
    if name.contains("pane") || name == "iron_bars" {
        // Simplified - just center post, connections need neighbor info
        return BlockGeometry::Single(AABB::new((0.4375, 0.0, 0.4375), (0.5625, 1.0, 0.5625)));
    }

    // Carpets
    if name.contains("carpet") {
        return BlockGeometry::Single(CARPET);
    }

    // Snow layers
    if name == "snow" {
        let layers: u8 = properties.get("layers")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        return BlockGeometry::Single(snow_layer(layers));
    }

    // Pressure plates
    if name.contains("pressure_plate") {
        return BlockGeometry::Single(PRESSURE_PLATE);
    }

    // Buttons
    if name.contains("button") {
        let face = properties.get("face").map(|s| s.as_str()).unwrap_or("wall");
        let facing = properties.get("facing").map(|s| s.as_str()).unwrap_or("north");
        return BlockGeometry::Single(button(face, facing));
    }

    // Levers
    if name == "lever" {
        let face = properties.get("face").map(|s| s.as_str()).unwrap_or("wall");
        let facing = properties.get("facing").map(|s| s.as_str()).unwrap_or("north");
        return BlockGeometry::Single(lever(face, facing));
    }

    // Torches
    if name.contains("torch") {
        if name.contains("wall") {
            let facing = properties.get("facing").map(|s| s.as_str()).unwrap_or("north");
            return BlockGeometry::Single(wall_torch(facing));
        }
        return BlockGeometry::Single(TORCH_STANDING);
    }

    // Lanterns
    if name.contains("lantern") {
        let hanging = properties.get("hanging").map(|s| s.as_str()).unwrap_or("false") == "true";
        return BlockGeometry::Single(if hanging { LANTERN_HANGING } else { LANTERN_STANDING });
    }

    // Candles
    if name.contains("candle") {
        return BlockGeometry::Single(CANDLE);
    }

    // Ladders
    if name == "ladder" {
        let facing = properties.get("facing").map(|s| s.as_str()).unwrap_or("north");
        return BlockGeometry::Single(match facing {
            "north" => LADDER_NORTH,
            "south" => LADDER_SOUTH,
            "west" => LADDER_WEST,
            "east" => LADDER_EAST,
            _ => LADDER_NORTH,
        });
    }

    // Rails
    if name.contains("rail") {
        return BlockGeometry::Single(RAIL_FLAT);
    }

    // Repeaters, comparators
    if name.contains("repeater") || name.contains("comparator") {
        return BlockGeometry::Single(REPEATER);
    }

    // Beds
    if name.contains("bed") {
        return BlockGeometry::Single(BED);
    }

    // Chests
    if name.contains("chest") {
        return BlockGeometry::Single(CHEST);
    }

    // Enchanting table
    if name == "enchanting_table" {
        return BlockGeometry::Single(ENCHANTING_TABLE);
    }

    // End portal frame
    if name == "end_portal_frame" {
        return BlockGeometry::Single(END_PORTAL_FRAME);
    }

    // Hopper
    if name == "hopper" {
        return BlockGeometry::Multi(vec![HOPPER_TOP, HOPPER_MIDDLE, HOPPER_BOTTOM]);
    }

    // Lectern
    if name == "lectern" {
        return BlockGeometry::Multi(vec![LECTERN_BASE, LECTERN_POST, LECTERN_TOP]);
    }

    // Brewing stand
    if name == "brewing_stand" {
        return BlockGeometry::Single(BREWING_STAND);
    }

    // Cauldron
    if name.contains("cauldron") {
        return BlockGeometry::Single(CAULDRON);
    }

    // Anvil
    if name.contains("anvil") {
        return BlockGeometry::Single(ANVIL);
    }

    // Bell
    if name == "bell" {
        return BlockGeometry::Single(BELL);
    }

    // Flower pot
    if name.contains("potted") || name == "flower_pot" {
        return BlockGeometry::Single(FLOWER_POT);
    }

    // Chain
    if name == "chain" {
        return BlockGeometry::Single(CHAIN_Y);
    }

    // End rod, lightning rod
    if name == "end_rod" {
        return BlockGeometry::Single(END_ROD_Y);
    }
    if name == "lightning_rod" {
        return BlockGeometry::Single(LIGHTNING_ROD);
    }

    // Signs (very thin)
    if name.contains("sign") {
        return BlockGeometry::Empty; // Signs don't occlude
    }

    // Banners
    if name.contains("banner") {
        return BlockGeometry::Empty;
    }

    // Heads, skulls
    if name.contains("head") || name.contains("skull") {
        return BlockGeometry::Single(AABB::new((0.25, 0.0, 0.25), (0.75, 0.5, 0.75)));
    }

    // Plants, flowers, crops - no occlusion
    if name.contains("flower") || name.contains("tulip") || name.contains("orchid")
        || name.contains("allium") || name.contains("bluet") || name.contains("dandelion")
        || name.contains("poppy") || name.contains("rose") || name.contains("lily")
        || name.contains("sapling") || name.contains("fern")
        || (name.contains("grass") && !name.contains("block"))
        || name.contains("crop") || name.contains("wheat") || name.contains("carrot")
        || name.contains("potato") || name.contains("beetroot") || name.contains("melon_stem")
        || name.contains("pumpkin_stem") || name.contains("vine") || name.contains("kelp")
        || name.contains("seagrass") || (name.contains("coral") && !name.contains("block"))
        || name.contains("bush") || (name.contains("bamboo") && !name.contains("block"))
        || name.contains("sugar_cane") || name.contains("dead_bush")
        || name.contains("mushroom") && !name.contains("block") {
        return BlockGeometry::Empty;
    }

    // Redstone wire
    if name == "redstone_wire" {
        return BlockGeometry::Empty;
    }

    // Tripwire
    if name.contains("tripwire") && !name.contains("hook") {
        return BlockGeometry::Empty;
    }

    // Default: full cube
    BlockGeometry::Full
}

/// Check if a block is partial (doesn't fully cover all faces)
/// This is a convenience function that uses get_block_geometry
pub fn is_partial_block(name: &str, properties: &HashMap<String, String>) -> bool {
    !matches!(get_block_geometry(name, properties), BlockGeometry::Full)
}

/// Check if a block covers a specific face (used for face culling)
pub fn block_covers_face(name: &str, properties: &HashMap<String, String>, face: Face) -> bool {
    get_block_geometry(name, properties).covers_face(face)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_cube() {
        let props = HashMap::new();
        assert!(matches!(get_block_geometry("stone", &props), BlockGeometry::Full));
        assert!(matches!(get_block_geometry("minecraft:dirt", &props), BlockGeometry::Full));
    }

    #[test]
    fn test_slab() {
        let mut props = HashMap::new();
        props.insert("type".to_string(), "bottom".to_string());

        let geom = get_block_geometry("stone_slab", &props);
        assert!(matches!(geom, BlockGeometry::Single(_)));
        assert!(geom.covers_face(Face::YNeg));
        assert!(!geom.covers_face(Face::YPos));
    }

    #[test]
    fn test_air() {
        let props = HashMap::new();
        assert!(matches!(get_block_geometry("air", &props), BlockGeometry::Empty));
        assert!(matches!(get_block_geometry("minecraft:cave_air", &props), BlockGeometry::Empty));
    }

    #[test]
    fn test_stairs() {
        let mut props = HashMap::new();
        props.insert("facing".to_string(), "north".to_string());
        props.insert("half".to_string(), "bottom".to_string());
        props.insert("shape".to_string(), "straight".to_string());

        let geom = get_block_geometry("oak_stairs", &props);
        assert!(matches!(geom, BlockGeometry::Multi(_)));
    }
}
