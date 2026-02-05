//! Minecraft crafting recipes for material calculations
//!
//! This module contains recipes to break down crafted items into raw materials.

use std::collections::HashMap;

/// Recipe definition: what raw materials are needed for one item
#[derive(Debug, Clone)]
pub struct Recipe {
    /// Output item name
    pub output: &'static str,
    /// How many items this recipe produces
    pub output_count: u32,
    /// Required ingredients: (item_name, count)
    pub ingredients: &'static [(&'static str, u32)],
}

/// Get all known recipes
pub fn get_recipes() -> HashMap<&'static str, Recipe> {
    let recipes: Vec<Recipe> = vec![
        // === Wood products ===
        Recipe {
            output: "minecraft:oak_planks",
            output_count: 4,
            ingredients: &[("minecraft:oak_log", 1)],
        },
        Recipe {
            output: "minecraft:spruce_planks",
            output_count: 4,
            ingredients: &[("minecraft:spruce_log", 1)],
        },
        Recipe {
            output: "minecraft:birch_planks",
            output_count: 4,
            ingredients: &[("minecraft:birch_log", 1)],
        },
        Recipe {
            output: "minecraft:jungle_planks",
            output_count: 4,
            ingredients: &[("minecraft:jungle_log", 1)],
        },
        Recipe {
            output: "minecraft:acacia_planks",
            output_count: 4,
            ingredients: &[("minecraft:acacia_log", 1)],
        },
        Recipe {
            output: "minecraft:dark_oak_planks",
            output_count: 4,
            ingredients: &[("minecraft:dark_oak_log", 1)],
        },
        Recipe {
            output: "minecraft:mangrove_planks",
            output_count: 4,
            ingredients: &[("minecraft:mangrove_log", 1)],
        },
        Recipe {
            output: "minecraft:cherry_planks",
            output_count: 4,
            ingredients: &[("minecraft:cherry_log", 1)],
        },
        Recipe {
            output: "minecraft:bamboo_planks",
            output_count: 2,
            ingredients: &[("minecraft:bamboo_block", 1)],
        },
        Recipe {
            output: "minecraft:crimson_planks",
            output_count: 4,
            ingredients: &[("minecraft:crimson_stem", 1)],
        },
        Recipe {
            output: "minecraft:warped_planks",
            output_count: 4,
            ingredients: &[("minecraft:warped_stem", 1)],
        },
        Recipe {
            output: "minecraft:stick",
            output_count: 4,
            ingredients: &[("minecraft:any_planks", 2)],
        },

        // === Wood stairs (all types) ===
        Recipe { output: "minecraft:oak_stairs", output_count: 4, ingredients: &[("minecraft:oak_planks", 6)] },
        Recipe { output: "minecraft:spruce_stairs", output_count: 4, ingredients: &[("minecraft:spruce_planks", 6)] },
        Recipe { output: "minecraft:birch_stairs", output_count: 4, ingredients: &[("minecraft:birch_planks", 6)] },
        Recipe { output: "minecraft:jungle_stairs", output_count: 4, ingredients: &[("minecraft:jungle_planks", 6)] },
        Recipe { output: "minecraft:acacia_stairs", output_count: 4, ingredients: &[("minecraft:acacia_planks", 6)] },
        Recipe { output: "minecraft:dark_oak_stairs", output_count: 4, ingredients: &[("minecraft:dark_oak_planks", 6)] },
        Recipe { output: "minecraft:mangrove_stairs", output_count: 4, ingredients: &[("minecraft:mangrove_planks", 6)] },
        Recipe { output: "minecraft:cherry_stairs", output_count: 4, ingredients: &[("minecraft:cherry_planks", 6)] },
        Recipe { output: "minecraft:bamboo_stairs", output_count: 4, ingredients: &[("minecraft:bamboo_planks", 6)] },
        Recipe { output: "minecraft:crimson_stairs", output_count: 4, ingredients: &[("minecraft:crimson_planks", 6)] },
        Recipe { output: "minecraft:warped_stairs", output_count: 4, ingredients: &[("minecraft:warped_planks", 6)] },

        // === Wood slabs ===
        Recipe { output: "minecraft:oak_slab", output_count: 6, ingredients: &[("minecraft:oak_planks", 3)] },
        Recipe { output: "minecraft:spruce_slab", output_count: 6, ingredients: &[("minecraft:spruce_planks", 3)] },
        Recipe { output: "minecraft:birch_slab", output_count: 6, ingredients: &[("minecraft:birch_planks", 3)] },
        Recipe { output: "minecraft:jungle_slab", output_count: 6, ingredients: &[("minecraft:jungle_planks", 3)] },
        Recipe { output: "minecraft:acacia_slab", output_count: 6, ingredients: &[("minecraft:acacia_planks", 3)] },
        Recipe { output: "minecraft:dark_oak_slab", output_count: 6, ingredients: &[("minecraft:dark_oak_planks", 3)] },
        Recipe { output: "minecraft:mangrove_slab", output_count: 6, ingredients: &[("minecraft:mangrove_planks", 3)] },
        Recipe { output: "minecraft:cherry_slab", output_count: 6, ingredients: &[("minecraft:cherry_planks", 3)] },
        Recipe { output: "minecraft:bamboo_slab", output_count: 6, ingredients: &[("minecraft:bamboo_planks", 3)] },
        Recipe { output: "minecraft:crimson_slab", output_count: 6, ingredients: &[("minecraft:crimson_planks", 3)] },
        Recipe { output: "minecraft:warped_slab", output_count: 6, ingredients: &[("minecraft:warped_planks", 3)] },

        // === Wood fences ===
        Recipe { output: "minecraft:oak_fence", output_count: 3, ingredients: &[("minecraft:oak_planks", 4), ("minecraft:stick", 2)] },
        Recipe { output: "minecraft:spruce_fence", output_count: 3, ingredients: &[("minecraft:spruce_planks", 4), ("minecraft:stick", 2)] },
        Recipe { output: "minecraft:birch_fence", output_count: 3, ingredients: &[("minecraft:birch_planks", 4), ("minecraft:stick", 2)] },
        Recipe { output: "minecraft:jungle_fence", output_count: 3, ingredients: &[("minecraft:jungle_planks", 4), ("minecraft:stick", 2)] },
        Recipe { output: "minecraft:acacia_fence", output_count: 3, ingredients: &[("minecraft:acacia_planks", 4), ("minecraft:stick", 2)] },
        Recipe { output: "minecraft:dark_oak_fence", output_count: 3, ingredients: &[("minecraft:dark_oak_planks", 4), ("minecraft:stick", 2)] },
        Recipe { output: "minecraft:mangrove_fence", output_count: 3, ingredients: &[("minecraft:mangrove_planks", 4), ("minecraft:stick", 2)] },
        Recipe { output: "minecraft:cherry_fence", output_count: 3, ingredients: &[("minecraft:cherry_planks", 4), ("minecraft:stick", 2)] },
        Recipe { output: "minecraft:bamboo_fence", output_count: 3, ingredients: &[("minecraft:bamboo_planks", 4), ("minecraft:stick", 2)] },
        Recipe { output: "minecraft:crimson_fence", output_count: 3, ingredients: &[("minecraft:crimson_planks", 4), ("minecraft:stick", 2)] },
        Recipe { output: "minecraft:warped_fence", output_count: 3, ingredients: &[("minecraft:warped_planks", 4), ("minecraft:stick", 2)] },

        // === Fence gates ===
        Recipe { output: "minecraft:oak_fence_gate", output_count: 1, ingredients: &[("minecraft:oak_planks", 2), ("minecraft:stick", 4)] },
        Recipe { output: "minecraft:spruce_fence_gate", output_count: 1, ingredients: &[("minecraft:spruce_planks", 2), ("minecraft:stick", 4)] },
        Recipe { output: "minecraft:birch_fence_gate", output_count: 1, ingredients: &[("minecraft:birch_planks", 2), ("minecraft:stick", 4)] },
        Recipe { output: "minecraft:jungle_fence_gate", output_count: 1, ingredients: &[("minecraft:jungle_planks", 2), ("minecraft:stick", 4)] },
        Recipe { output: "minecraft:acacia_fence_gate", output_count: 1, ingredients: &[("minecraft:acacia_planks", 2), ("minecraft:stick", 4)] },
        Recipe { output: "minecraft:dark_oak_fence_gate", output_count: 1, ingredients: &[("minecraft:dark_oak_planks", 2), ("minecraft:stick", 4)] },
        Recipe { output: "minecraft:mangrove_fence_gate", output_count: 1, ingredients: &[("minecraft:mangrove_planks", 2), ("minecraft:stick", 4)] },
        Recipe { output: "minecraft:cherry_fence_gate", output_count: 1, ingredients: &[("minecraft:cherry_planks", 2), ("minecraft:stick", 4)] },
        Recipe { output: "minecraft:bamboo_fence_gate", output_count: 1, ingredients: &[("minecraft:bamboo_planks", 2), ("minecraft:stick", 4)] },
        Recipe { output: "minecraft:crimson_fence_gate", output_count: 1, ingredients: &[("minecraft:crimson_planks", 2), ("minecraft:stick", 4)] },
        Recipe { output: "minecraft:warped_fence_gate", output_count: 1, ingredients: &[("minecraft:warped_planks", 2), ("minecraft:stick", 4)] },

        // === Doors ===
        Recipe { output: "minecraft:oak_door", output_count: 3, ingredients: &[("minecraft:oak_planks", 6)] },
        Recipe { output: "minecraft:spruce_door", output_count: 3, ingredients: &[("minecraft:spruce_planks", 6)] },
        Recipe { output: "minecraft:birch_door", output_count: 3, ingredients: &[("minecraft:birch_planks", 6)] },
        Recipe { output: "minecraft:jungle_door", output_count: 3, ingredients: &[("minecraft:jungle_planks", 6)] },
        Recipe { output: "minecraft:acacia_door", output_count: 3, ingredients: &[("minecraft:acacia_planks", 6)] },
        Recipe { output: "minecraft:dark_oak_door", output_count: 3, ingredients: &[("minecraft:dark_oak_planks", 6)] },
        Recipe { output: "minecraft:mangrove_door", output_count: 3, ingredients: &[("minecraft:mangrove_planks", 6)] },
        Recipe { output: "minecraft:cherry_door", output_count: 3, ingredients: &[("minecraft:cherry_planks", 6)] },
        Recipe { output: "minecraft:bamboo_door", output_count: 3, ingredients: &[("minecraft:bamboo_planks", 6)] },
        Recipe { output: "minecraft:crimson_door", output_count: 3, ingredients: &[("minecraft:crimson_planks", 6)] },
        Recipe { output: "minecraft:warped_door", output_count: 3, ingredients: &[("minecraft:warped_planks", 6)] },
        Recipe { output: "minecraft:iron_door", output_count: 3, ingredients: &[("minecraft:iron_ingot", 6)] },

        // === Trapdoors ===
        Recipe { output: "minecraft:oak_trapdoor", output_count: 2, ingredients: &[("minecraft:oak_planks", 6)] },
        Recipe { output: "minecraft:spruce_trapdoor", output_count: 2, ingredients: &[("minecraft:spruce_planks", 6)] },
        Recipe { output: "minecraft:birch_trapdoor", output_count: 2, ingredients: &[("minecraft:birch_planks", 6)] },
        Recipe { output: "minecraft:jungle_trapdoor", output_count: 2, ingredients: &[("minecraft:jungle_planks", 6)] },
        Recipe { output: "minecraft:acacia_trapdoor", output_count: 2, ingredients: &[("minecraft:acacia_planks", 6)] },
        Recipe { output: "minecraft:dark_oak_trapdoor", output_count: 2, ingredients: &[("minecraft:dark_oak_planks", 6)] },
        Recipe { output: "minecraft:mangrove_trapdoor", output_count: 2, ingredients: &[("minecraft:mangrove_planks", 6)] },
        Recipe { output: "minecraft:cherry_trapdoor", output_count: 2, ingredients: &[("minecraft:cherry_planks", 6)] },
        Recipe { output: "minecraft:bamboo_trapdoor", output_count: 2, ingredients: &[("minecraft:bamboo_planks", 6)] },
        Recipe { output: "minecraft:crimson_trapdoor", output_count: 2, ingredients: &[("minecraft:crimson_planks", 6)] },
        Recipe { output: "minecraft:warped_trapdoor", output_count: 2, ingredients: &[("minecraft:warped_planks", 6)] },
        Recipe { output: "minecraft:iron_trapdoor", output_count: 1, ingredients: &[("minecraft:iron_ingot", 4)] },

        // === Pressure plates ===
        Recipe { output: "minecraft:oak_pressure_plate", output_count: 1, ingredients: &[("minecraft:oak_planks", 2)] },
        Recipe { output: "minecraft:spruce_pressure_plate", output_count: 1, ingredients: &[("minecraft:spruce_planks", 2)] },
        Recipe { output: "minecraft:birch_pressure_plate", output_count: 1, ingredients: &[("minecraft:birch_planks", 2)] },
        Recipe { output: "minecraft:jungle_pressure_plate", output_count: 1, ingredients: &[("minecraft:jungle_planks", 2)] },
        Recipe { output: "minecraft:acacia_pressure_plate", output_count: 1, ingredients: &[("minecraft:acacia_planks", 2)] },
        Recipe { output: "minecraft:dark_oak_pressure_plate", output_count: 1, ingredients: &[("minecraft:dark_oak_planks", 2)] },
        Recipe { output: "minecraft:mangrove_pressure_plate", output_count: 1, ingredients: &[("minecraft:mangrove_planks", 2)] },
        Recipe { output: "minecraft:cherry_pressure_plate", output_count: 1, ingredients: &[("minecraft:cherry_planks", 2)] },
        Recipe { output: "minecraft:bamboo_pressure_plate", output_count: 1, ingredients: &[("minecraft:bamboo_planks", 2)] },
        Recipe { output: "minecraft:crimson_pressure_plate", output_count: 1, ingredients: &[("minecraft:crimson_planks", 2)] },
        Recipe { output: "minecraft:warped_pressure_plate", output_count: 1, ingredients: &[("minecraft:warped_planks", 2)] },
        Recipe { output: "minecraft:stone_pressure_plate", output_count: 1, ingredients: &[("minecraft:stone", 2)] },
        Recipe { output: "minecraft:polished_blackstone_pressure_plate", output_count: 1, ingredients: &[("minecraft:polished_blackstone", 2)] },
        Recipe { output: "minecraft:heavy_weighted_pressure_plate", output_count: 1, ingredients: &[("minecraft:iron_ingot", 2)] },
        Recipe { output: "minecraft:light_weighted_pressure_plate", output_count: 1, ingredients: &[("minecraft:gold_ingot", 2)] },

        // === Buttons ===
        Recipe { output: "minecraft:oak_button", output_count: 1, ingredients: &[("minecraft:oak_planks", 1)] },
        Recipe { output: "minecraft:spruce_button", output_count: 1, ingredients: &[("minecraft:spruce_planks", 1)] },
        Recipe { output: "minecraft:birch_button", output_count: 1, ingredients: &[("minecraft:birch_planks", 1)] },
        Recipe { output: "minecraft:jungle_button", output_count: 1, ingredients: &[("minecraft:jungle_planks", 1)] },
        Recipe { output: "minecraft:acacia_button", output_count: 1, ingredients: &[("minecraft:acacia_planks", 1)] },
        Recipe { output: "minecraft:dark_oak_button", output_count: 1, ingredients: &[("minecraft:dark_oak_planks", 1)] },
        Recipe { output: "minecraft:mangrove_button", output_count: 1, ingredients: &[("minecraft:mangrove_planks", 1)] },
        Recipe { output: "minecraft:cherry_button", output_count: 1, ingredients: &[("minecraft:cherry_planks", 1)] },
        Recipe { output: "minecraft:bamboo_button", output_count: 1, ingredients: &[("minecraft:bamboo_planks", 1)] },
        Recipe { output: "minecraft:crimson_button", output_count: 1, ingredients: &[("minecraft:crimson_planks", 1)] },
        Recipe { output: "minecraft:warped_button", output_count: 1, ingredients: &[("minecraft:warped_planks", 1)] },
        Recipe { output: "minecraft:stone_button", output_count: 1, ingredients: &[("minecraft:stone", 1)] },
        Recipe { output: "minecraft:polished_blackstone_button", output_count: 1, ingredients: &[("minecraft:polished_blackstone", 1)] },

        // === Signs ===
        Recipe { output: "minecraft:oak_sign", output_count: 3, ingredients: &[("minecraft:oak_planks", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:spruce_sign", output_count: 3, ingredients: &[("minecraft:spruce_planks", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:birch_sign", output_count: 3, ingredients: &[("minecraft:birch_planks", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:jungle_sign", output_count: 3, ingredients: &[("minecraft:jungle_planks", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:acacia_sign", output_count: 3, ingredients: &[("minecraft:acacia_planks", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:dark_oak_sign", output_count: 3, ingredients: &[("minecraft:dark_oak_planks", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:mangrove_sign", output_count: 3, ingredients: &[("minecraft:mangrove_planks", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:cherry_sign", output_count: 3, ingredients: &[("minecraft:cherry_planks", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:bamboo_sign", output_count: 3, ingredients: &[("minecraft:bamboo_planks", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:crimson_sign", output_count: 3, ingredients: &[("minecraft:crimson_planks", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:warped_sign", output_count: 3, ingredients: &[("minecraft:warped_planks", 6), ("minecraft:stick", 1)] },

        // === Stone products ===
        Recipe {
            output: "minecraft:stone_bricks",
            output_count: 4,
            ingredients: &[("minecraft:stone", 4)],
        },
        Recipe {
            output: "minecraft:stone_brick_stairs",
            output_count: 4,
            ingredients: &[("minecraft:stone_bricks", 6)],
        },
        Recipe {
            output: "minecraft:stone_brick_slab",
            output_count: 6,
            ingredients: &[("minecraft:stone_bricks", 3)],
        },
        Recipe {
            output: "minecraft:cobblestone_stairs",
            output_count: 4,
            ingredients: &[("minecraft:cobblestone", 6)],
        },
        Recipe {
            output: "minecraft:cobblestone_slab",
            output_count: 6,
            ingredients: &[("minecraft:cobblestone", 3)],
        },
        Recipe {
            output: "minecraft:cobblestone_wall",
            output_count: 6,
            ingredients: &[("minecraft:cobblestone", 6)],
        },
        Recipe {
            output: "minecraft:smooth_stone",
            output_count: 1,
            ingredients: &[("minecraft:stone", 1)], // smelting
        },
        Recipe {
            output: "minecraft:smooth_stone_slab",
            output_count: 6,
            ingredients: &[("minecraft:smooth_stone", 3)],
        },
        Recipe {
            output: "minecraft:stone",
            output_count: 1,
            ingredients: &[("minecraft:cobblestone", 1)], // smelting
        },
        Recipe {
            output: "minecraft:bricks",
            output_count: 1,
            ingredients: &[("minecraft:brick", 4)],
        },
        Recipe {
            output: "minecraft:brick",
            output_count: 1,
            ingredients: &[("minecraft:clay_ball", 1)], // smelting
        },
        Recipe {
            output: "minecraft:brick_stairs",
            output_count: 4,
            ingredients: &[("minecraft:bricks", 6)],
        },
        Recipe {
            output: "minecraft:brick_slab",
            output_count: 6,
            ingredients: &[("minecraft:bricks", 3)],
        },
        Recipe {
            output: "minecraft:brick_wall",
            output_count: 6,
            ingredients: &[("minecraft:bricks", 6)],
        },
        // Cracked stone bricks (smelting)
        Recipe {
            output: "minecraft:cracked_stone_bricks",
            output_count: 1,
            ingredients: &[("minecraft:stone_bricks", 1)],
        },
        Recipe {
            output: "minecraft:mossy_stone_bricks",
            output_count: 1,
            ingredients: &[("minecraft:stone_bricks", 1), ("minecraft:vine", 1)],
        },
        Recipe {
            output: "minecraft:mossy_cobblestone",
            output_count: 1,
            ingredients: &[("minecraft:cobblestone", 1), ("minecraft:vine", 1)],
        },
        Recipe {
            output: "minecraft:stone_brick_wall",
            output_count: 6,
            ingredients: &[("minecraft:stone_bricks", 6)],
        },

        // === Deepslate ===
        Recipe {
            output: "minecraft:polished_deepslate",
            output_count: 4,
            ingredients: &[("minecraft:cobbled_deepslate", 4)],
        },
        Recipe {
            output: "minecraft:deepslate_bricks",
            output_count: 4,
            ingredients: &[("minecraft:polished_deepslate", 4)],
        },
        Recipe {
            output: "minecraft:deepslate_tiles",
            output_count: 4,
            ingredients: &[("minecraft:deepslate_bricks", 4)],
        },
        Recipe {
            output: "minecraft:chiseled_deepslate",
            output_count: 1,
            ingredients: &[("minecraft:cobbled_deepslate", 2)], // via slabs
        },
        // Cracked variants (smelting)
        Recipe {
            output: "minecraft:cracked_deepslate_bricks",
            output_count: 1,
            ingredients: &[("minecraft:deepslate_bricks", 1)],
        },
        Recipe {
            output: "minecraft:cracked_deepslate_tiles",
            output_count: 1,
            ingredients: &[("minecraft:deepslate_tiles", 1)],
        },
        // Deepslate stairs and slabs
        Recipe {
            output: "minecraft:cobbled_deepslate_stairs",
            output_count: 4,
            ingredients: &[("minecraft:cobbled_deepslate", 6)],
        },
        Recipe {
            output: "minecraft:cobbled_deepslate_slab",
            output_count: 6,
            ingredients: &[("minecraft:cobbled_deepslate", 3)],
        },
        Recipe {
            output: "minecraft:cobbled_deepslate_wall",
            output_count: 6,
            ingredients: &[("minecraft:cobbled_deepslate", 6)],
        },
        Recipe {
            output: "minecraft:polished_deepslate_stairs",
            output_count: 4,
            ingredients: &[("minecraft:polished_deepslate", 6)],
        },
        Recipe {
            output: "minecraft:polished_deepslate_slab",
            output_count: 6,
            ingredients: &[("minecraft:polished_deepslate", 3)],
        },
        Recipe {
            output: "minecraft:polished_deepslate_wall",
            output_count: 6,
            ingredients: &[("minecraft:polished_deepslate", 6)],
        },
        Recipe {
            output: "minecraft:deepslate_brick_stairs",
            output_count: 4,
            ingredients: &[("minecraft:deepslate_bricks", 6)],
        },
        Recipe {
            output: "minecraft:deepslate_brick_slab",
            output_count: 6,
            ingredients: &[("minecraft:deepslate_bricks", 3)],
        },
        Recipe {
            output: "minecraft:deepslate_brick_wall",
            output_count: 6,
            ingredients: &[("minecraft:deepslate_bricks", 6)],
        },
        Recipe {
            output: "minecraft:deepslate_tile_stairs",
            output_count: 4,
            ingredients: &[("minecraft:deepslate_tiles", 6)],
        },
        Recipe {
            output: "minecraft:deepslate_tile_slab",
            output_count: 6,
            ingredients: &[("minecraft:deepslate_tiles", 3)],
        },
        Recipe {
            output: "minecraft:deepslate_tile_wall",
            output_count: 6,
            ingredients: &[("minecraft:deepslate_tiles", 6)],
        },

        // === Blackstone ===
        Recipe {
            output: "minecraft:polished_blackstone",
            output_count: 4,
            ingredients: &[("minecraft:blackstone", 4)],
        },
        Recipe {
            output: "minecraft:polished_blackstone_bricks",
            output_count: 4,
            ingredients: &[("minecraft:polished_blackstone", 4)],
        },
        Recipe {
            output: "minecraft:chiseled_polished_blackstone",
            output_count: 1,
            ingredients: &[("minecraft:blackstone", 2)], // via slabs
        },
        // Cracked blackstone (smelting)
        Recipe {
            output: "minecraft:cracked_polished_blackstone_bricks",
            output_count: 1,
            ingredients: &[("minecraft:polished_blackstone_bricks", 1)],
        },
        // Blackstone stairs and slabs
        Recipe {
            output: "minecraft:blackstone_stairs",
            output_count: 4,
            ingredients: &[("minecraft:blackstone", 6)],
        },
        Recipe {
            output: "minecraft:blackstone_slab",
            output_count: 6,
            ingredients: &[("minecraft:blackstone", 3)],
        },
        Recipe {
            output: "minecraft:blackstone_wall",
            output_count: 6,
            ingredients: &[("minecraft:blackstone", 6)],
        },
        Recipe {
            output: "minecraft:polished_blackstone_stairs",
            output_count: 4,
            ingredients: &[("minecraft:polished_blackstone", 6)],
        },
        Recipe {
            output: "minecraft:polished_blackstone_slab",
            output_count: 6,
            ingredients: &[("minecraft:polished_blackstone", 3)],
        },
        Recipe {
            output: "minecraft:polished_blackstone_wall",
            output_count: 6,
            ingredients: &[("minecraft:polished_blackstone", 6)],
        },
        Recipe {
            output: "minecraft:polished_blackstone_brick_stairs",
            output_count: 4,
            ingredients: &[("minecraft:polished_blackstone_bricks", 6)],
        },
        Recipe {
            output: "minecraft:polished_blackstone_brick_slab",
            output_count: 6,
            ingredients: &[("minecraft:polished_blackstone_bricks", 3)],
        },
        Recipe {
            output: "minecraft:polished_blackstone_brick_wall",
            output_count: 6,
            ingredients: &[("minecraft:polished_blackstone_bricks", 6)],
        },

        // === Nether ===
        Recipe {
            output: "minecraft:nether_bricks",
            output_count: 1,
            ingredients: &[("minecraft:nether_brick", 4)],
        },
        Recipe {
            output: "minecraft:nether_brick",
            output_count: 1,
            ingredients: &[("minecraft:netherrack", 1)], // smelting
        },
        Recipe {
            output: "minecraft:red_nether_bricks",
            output_count: 1,
            ingredients: &[("minecraft:nether_brick", 2), ("minecraft:nether_wart", 2)],
        },
        Recipe {
            output: "minecraft:cracked_nether_bricks",
            output_count: 1,
            ingredients: &[("minecraft:nether_bricks", 1)], // smelting
        },
        Recipe {
            output: "minecraft:chiseled_nether_bricks",
            output_count: 1,
            ingredients: &[("minecraft:nether_bricks", 2)], // via slabs
        },
        Recipe {
            output: "minecraft:nether_brick_stairs",
            output_count: 4,
            ingredients: &[("minecraft:nether_bricks", 6)],
        },
        Recipe {
            output: "minecraft:nether_brick_slab",
            output_count: 6,
            ingredients: &[("minecraft:nether_bricks", 3)],
        },
        Recipe {
            output: "minecraft:nether_brick_wall",
            output_count: 6,
            ingredients: &[("minecraft:nether_bricks", 6)],
        },
        Recipe {
            output: "minecraft:nether_brick_fence",
            output_count: 6,
            ingredients: &[("minecraft:nether_bricks", 4), ("minecraft:nether_brick", 2)],
        },
        Recipe {
            output: "minecraft:red_nether_brick_stairs",
            output_count: 4,
            ingredients: &[("minecraft:red_nether_bricks", 6)],
        },
        Recipe {
            output: "minecraft:red_nether_brick_slab",
            output_count: 6,
            ingredients: &[("minecraft:red_nether_bricks", 3)],
        },
        Recipe {
            output: "minecraft:red_nether_brick_wall",
            output_count: 6,
            ingredients: &[("minecraft:red_nether_bricks", 6)],
        },
        Recipe {
            output: "minecraft:quartz_block",
            output_count: 1,
            ingredients: &[("minecraft:quartz", 4)],
        },
        Recipe {
            output: "minecraft:quartz_bricks",
            output_count: 1,
            ingredients: &[("minecraft:quartz_block", 4)],
        },
        Recipe {
            output: "minecraft:smooth_quartz",
            output_count: 1,
            ingredients: &[("minecraft:quartz_block", 1)], // smelting
        },

        // === Metal blocks ===
        Recipe {
            output: "minecraft:iron_block",
            output_count: 1,
            ingredients: &[("minecraft:iron_ingot", 9)],
        },
        Recipe {
            output: "minecraft:gold_block",
            output_count: 1,
            ingredients: &[("minecraft:gold_ingot", 9)],
        },
        Recipe {
            output: "minecraft:diamond_block",
            output_count: 1,
            ingredients: &[("minecraft:diamond", 9)],
        },
        Recipe {
            output: "minecraft:emerald_block",
            output_count: 1,
            ingredients: &[("minecraft:emerald", 9)],
        },
        Recipe {
            output: "minecraft:lapis_block",
            output_count: 1,
            ingredients: &[("minecraft:lapis_lazuli", 9)],
        },
        Recipe {
            output: "minecraft:redstone_block",
            output_count: 1,
            ingredients: &[("minecraft:redstone", 9)],
        },
        Recipe {
            output: "minecraft:coal_block",
            output_count: 1,
            ingredients: &[("minecraft:coal", 9)],
        },
        Recipe {
            output: "minecraft:copper_block",
            output_count: 1,
            ingredients: &[("minecraft:copper_ingot", 9)],
        },
        Recipe {
            output: "minecraft:netherite_block",
            output_count: 1,
            ingredients: &[("minecraft:netherite_ingot", 9)],
        },
        Recipe {
            output: "minecraft:netherite_ingot",
            output_count: 1,
            ingredients: &[("minecraft:netherite_scrap", 4), ("minecraft:gold_ingot", 4)],
        },
        Recipe {
            output: "minecraft:raw_iron_block",
            output_count: 1,
            ingredients: &[("minecraft:raw_iron", 9)],
        },
        Recipe {
            output: "minecraft:raw_gold_block",
            output_count: 1,
            ingredients: &[("minecraft:raw_gold", 9)],
        },
        Recipe {
            output: "minecraft:raw_copper_block",
            output_count: 1,
            ingredients: &[("minecraft:raw_copper", 9)],
        },

        // === Glass ===
        Recipe {
            output: "minecraft:glass",
            output_count: 1,
            ingredients: &[("minecraft:sand", 1)], // smelting
        },
        Recipe {
            output: "minecraft:glass_pane",
            output_count: 16,
            ingredients: &[("minecraft:glass", 6)],
        },
        // Stained glass
        Recipe {
            output: "minecraft:white_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:white_dye", 1)],
        },
        Recipe {
            output: "minecraft:red_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:red_dye", 1)],
        },
        Recipe {
            output: "minecraft:black_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:black_dye", 1)],
        },
        Recipe {
            output: "minecraft:blue_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:blue_dye", 1)],
        },
        Recipe {
            output: "minecraft:green_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:green_dye", 1)],
        },
        Recipe {
            output: "minecraft:yellow_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:yellow_dye", 1)],
        },
        Recipe {
            output: "minecraft:orange_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:orange_dye", 1)],
        },
        Recipe {
            output: "minecraft:purple_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:purple_dye", 1)],
        },
        Recipe {
            output: "minecraft:cyan_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:cyan_dye", 1)],
        },
        Recipe {
            output: "minecraft:pink_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:pink_dye", 1)],
        },
        Recipe {
            output: "minecraft:gray_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:gray_dye", 1)],
        },
        Recipe {
            output: "minecraft:light_gray_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:light_gray_dye", 1)],
        },
        Recipe {
            output: "minecraft:light_blue_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:light_blue_dye", 1)],
        },
        Recipe {
            output: "minecraft:lime_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:lime_dye", 1)],
        },
        Recipe {
            output: "minecraft:magenta_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:magenta_dye", 1)],
        },
        Recipe {
            output: "minecraft:brown_stained_glass",
            output_count: 8,
            ingredients: &[("minecraft:glass", 8), ("minecraft:brown_dye", 1)],
        },

        // === Concrete ===
        Recipe {
            output: "minecraft:white_concrete_powder",
            output_count: 8,
            ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:white_dye", 1)],
        },
        Recipe {
            output: "minecraft:red_concrete_powder",
            output_count: 8,
            ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:red_dye", 1)],
        },
        Recipe {
            output: "minecraft:black_concrete_powder",
            output_count: 8,
            ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:black_dye", 1)],
        },
        Recipe {
            output: "minecraft:gray_concrete_powder",
            output_count: 8,
            ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:gray_dye", 1)],
        },
        // Concrete (from powder + water, 1:1)
        Recipe {
            output: "minecraft:white_concrete",
            output_count: 1,
            ingredients: &[("minecraft:white_concrete_powder", 1)],
        },
        Recipe {
            output: "minecraft:red_concrete",
            output_count: 1,
            ingredients: &[("minecraft:red_concrete_powder", 1)],
        },
        Recipe {
            output: "minecraft:black_concrete",
            output_count: 1,
            ingredients: &[("minecraft:black_concrete_powder", 1)],
        },
        Recipe {
            output: "minecraft:gray_concrete",
            output_count: 1,
            ingredients: &[("minecraft:gray_concrete_powder", 1)],
        },

        // === Wool ===
        Recipe {
            output: "minecraft:white_wool",
            output_count: 1,
            ingredients: &[("minecraft:string", 4)],
        },
        Recipe {
            output: "minecraft:red_wool",
            output_count: 1,
            ingredients: &[("minecraft:white_wool", 1), ("minecraft:red_dye", 1)],
        },
        Recipe {
            output: "minecraft:black_wool",
            output_count: 1,
            ingredients: &[("minecraft:white_wool", 1), ("minecraft:black_dye", 1)],
        },
        Recipe {
            output: "minecraft:gray_wool",
            output_count: 1,
            ingredients: &[("minecraft:white_wool", 1), ("minecraft:gray_dye", 1)],
        },

        // === Terracotta ===
        Recipe {
            output: "minecraft:terracotta",
            output_count: 1,
            ingredients: &[("minecraft:clay", 1)], // smelting clay block
        },
        Recipe {
            output: "minecraft:white_terracotta",
            output_count: 8,
            ingredients: &[("minecraft:terracotta", 8), ("minecraft:white_dye", 1)],
        },
        Recipe {
            output: "minecraft:red_terracotta",
            output_count: 8,
            ingredients: &[("minecraft:terracotta", 8), ("minecraft:red_dye", 1)],
        },
        Recipe {
            output: "minecraft:orange_terracotta",
            output_count: 8,
            ingredients: &[("minecraft:terracotta", 8), ("minecraft:orange_dye", 1)],
        },
        Recipe {
            output: "minecraft:black_terracotta",
            output_count: 8,
            ingredients: &[("minecraft:terracotta", 8), ("minecraft:black_dye", 1)],
        },

        // === Sandstone ===
        Recipe {
            output: "minecraft:sandstone",
            output_count: 1,
            ingredients: &[("minecraft:sand", 4)],
        },
        Recipe {
            output: "minecraft:smooth_sandstone",
            output_count: 1,
            ingredients: &[("minecraft:sandstone", 1)], // smelting
        },
        Recipe {
            output: "minecraft:cut_sandstone",
            output_count: 4,
            ingredients: &[("minecraft:sandstone", 4)],
        },
        Recipe {
            output: "minecraft:red_sandstone",
            output_count: 1,
            ingredients: &[("minecraft:red_sand", 4)],
        },
        Recipe {
            output: "minecraft:smooth_red_sandstone",
            output_count: 1,
            ingredients: &[("minecraft:red_sandstone", 1)], // smelting
        },

        // === Prismarine ===
        Recipe {
            output: "minecraft:prismarine",
            output_count: 1,
            ingredients: &[("minecraft:prismarine_shard", 4)],
        },
        Recipe {
            output: "minecraft:prismarine_bricks",
            output_count: 1,
            ingredients: &[("minecraft:prismarine_shard", 9)],
        },
        Recipe {
            output: "minecraft:dark_prismarine",
            output_count: 1,
            ingredients: &[("minecraft:prismarine_shard", 8), ("minecraft:black_dye", 1)],
        },
        Recipe {
            output: "minecraft:sea_lantern",
            output_count: 1,
            ingredients: &[("minecraft:prismarine_shard", 4), ("minecraft:prismarine_crystals", 5)],
        },

        // === End stone ===
        Recipe {
            output: "minecraft:end_stone_bricks",
            output_count: 4,
            ingredients: &[("minecraft:end_stone", 4)],
        },
        Recipe {
            output: "minecraft:purpur_block",
            output_count: 4,
            ingredients: &[("minecraft:popped_chorus_fruit", 4)],
        },
        Recipe {
            output: "minecraft:purpur_pillar",
            output_count: 1,
            ingredients: &[("minecraft:purpur_block", 2)], // via slabs
        },

        // === Misc ===
        Recipe {
            output: "minecraft:bookshelf",
            output_count: 1,
            ingredients: &[("minecraft:any_planks", 6), ("minecraft:book", 3)],
        },
        Recipe {
            output: "minecraft:book",
            output_count: 1,
            ingredients: &[("minecraft:paper", 3), ("minecraft:leather", 1)],
        },
        Recipe {
            output: "minecraft:paper",
            output_count: 3,
            ingredients: &[("minecraft:sugar_cane", 3)],
        },
        Recipe {
            output: "minecraft:hay_block",
            output_count: 1,
            ingredients: &[("minecraft:wheat", 9)],
        },
        Recipe {
            output: "minecraft:bone_block",
            output_count: 1,
            ingredients: &[("minecraft:bone_meal", 9)],
        },
        Recipe {
            output: "minecraft:slime_block",
            output_count: 1,
            ingredients: &[("minecraft:slime_ball", 9)],
        },
        Recipe {
            output: "minecraft:honey_block",
            output_count: 1,
            ingredients: &[("minecraft:honey_bottle", 4)],
        },
        Recipe {
            output: "minecraft:packed_ice",
            output_count: 1,
            ingredients: &[("minecraft:ice", 9)],
        },
        Recipe {
            output: "minecraft:blue_ice",
            output_count: 1,
            ingredients: &[("minecraft:packed_ice", 9)],
        },
        Recipe {
            output: "minecraft:snow_block",
            output_count: 1,
            ingredients: &[("minecraft:snowball", 4)],
        },
        Recipe {
            output: "minecraft:glowstone",
            output_count: 1,
            ingredients: &[("minecraft:glowstone_dust", 4)],
        },
        Recipe {
            output: "minecraft:tnt",
            output_count: 1,
            ingredients: &[("minecraft:gunpowder", 5), ("minecraft:sand", 4)],
        },
        Recipe {
            output: "minecraft:melon",
            output_count: 1,
            ingredients: &[("minecraft:melon_slice", 9)],
        },
        Recipe {
            output: "minecraft:dried_kelp_block",
            output_count: 1,
            ingredients: &[("minecraft:dried_kelp", 9)],
        },

        // === Mud and clay ===
        Recipe {
            output: "minecraft:packed_mud",
            output_count: 1,
            ingredients: &[("minecraft:mud", 1), ("minecraft:wheat", 1)],
        },
        Recipe {
            output: "minecraft:mud_bricks",
            output_count: 4,
            ingredients: &[("minecraft:packed_mud", 4)],
        },
        Recipe {
            output: "minecraft:clay",
            output_count: 1,
            ingredients: &[("minecraft:clay_ball", 4)],
        },

        // === Tuff ===
        Recipe {
            output: "minecraft:polished_tuff",
            output_count: 4,
            ingredients: &[("minecraft:tuff", 4)],
        },
        Recipe {
            output: "minecraft:tuff_bricks",
            output_count: 4,
            ingredients: &[("minecraft:polished_tuff", 4)],
        },

        // === Copper variants ===
        Recipe {
            output: "minecraft:cut_copper",
            output_count: 4,
            ingredients: &[("minecraft:copper_block", 4)],
        },
        Recipe {
            output: "minecraft:cut_copper_stairs",
            output_count: 4,
            ingredients: &[("minecraft:cut_copper", 6)],
        },
        Recipe {
            output: "minecraft:cut_copper_slab",
            output_count: 6,
            ingredients: &[("minecraft:cut_copper", 3)],
        },

        // === Amethyst ===
        Recipe {
            output: "minecraft:amethyst_block",
            output_count: 1,
            ingredients: &[("minecraft:amethyst_shard", 4)],
        },

        // === Calcite - natural only, no crafting ===

        // === Dripstone - natural only ===

        // === Smooth basalt ===
        Recipe {
            output: "minecraft:smooth_basalt",
            output_count: 1,
            ingredients: &[("minecraft:basalt", 1)], // smelting
        },
        Recipe {
            output: "minecraft:polished_basalt",
            output_count: 4,
            ingredients: &[("minecraft:basalt", 4)],
        },

        // === Redstone components ===
        Recipe {
            output: "minecraft:redstone_lamp",
            output_count: 1,
            ingredients: &[("minecraft:redstone", 4), ("minecraft:glowstone", 1)],
        },
        Recipe {
            output: "minecraft:observer",
            output_count: 1,
            ingredients: &[("minecraft:cobblestone", 6), ("minecraft:redstone", 2), ("minecraft:quartz", 1)],
        },
        Recipe {
            output: "minecraft:piston",
            output_count: 1,
            ingredients: &[("minecraft:any_planks", 3), ("minecraft:cobblestone", 4), ("minecraft:iron_ingot", 1), ("minecraft:redstone", 1)],
        },
        Recipe {
            output: "minecraft:sticky_piston",
            output_count: 1,
            ingredients: &[("minecraft:piston", 1), ("minecraft:slime_ball", 1)],
        },
        Recipe {
            output: "minecraft:dispenser",
            output_count: 1,
            ingredients: &[("minecraft:cobblestone", 7), ("minecraft:bow", 1), ("minecraft:redstone", 1)],
        },
        Recipe {
            output: "minecraft:dropper",
            output_count: 1,
            ingredients: &[("minecraft:cobblestone", 7), ("minecraft:redstone", 1)],
        },
        Recipe {
            output: "minecraft:hopper",
            output_count: 1,
            ingredients: &[("minecraft:iron_ingot", 5), ("minecraft:chest", 1)],
        },
        Recipe {
            output: "minecraft:comparator",
            output_count: 1,
            ingredients: &[("minecraft:redstone_torch", 3), ("minecraft:quartz", 1), ("minecraft:stone", 3)],
        },
        Recipe {
            output: "minecraft:repeater",
            output_count: 1,
            ingredients: &[("minecraft:redstone_torch", 2), ("minecraft:redstone", 1), ("minecraft:stone", 3)],
        },
        Recipe {
            output: "minecraft:redstone_torch",
            output_count: 1,
            ingredients: &[("minecraft:stick", 1), ("minecraft:redstone", 1)],
        },
        Recipe {
            output: "minecraft:lever",
            output_count: 1,
            ingredients: &[("minecraft:stick", 1), ("minecraft:cobblestone", 1)],
        },

        // === Containers ===
        Recipe {
            output: "minecraft:chest",
            output_count: 1,
            ingredients: &[("minecraft:any_planks", 8)],
        },
        Recipe {
            output: "minecraft:barrel",
            output_count: 1,
            ingredients: &[("minecraft:any_planks", 6), ("minecraft:any_slab", 2)],
        },
        Recipe {
            output: "minecraft:furnace",
            output_count: 1,
            ingredients: &[("minecraft:cobblestone", 8)],
        },
        Recipe {
            output: "minecraft:blast_furnace",
            output_count: 1,
            ingredients: &[("minecraft:iron_ingot", 5), ("minecraft:furnace", 1), ("minecraft:smooth_stone", 3)],
        },
        Recipe {
            output: "minecraft:smoker",
            output_count: 1,
            ingredients: &[("minecraft:any_log", 4), ("minecraft:furnace", 1)],
        },
        Recipe {
            output: "minecraft:crafting_table",
            output_count: 1,
            ingredients: &[("minecraft:any_planks", 4)],
        },

        // === Rails ===
        Recipe {
            output: "minecraft:rail",
            output_count: 16,
            ingredients: &[("minecraft:iron_ingot", 6), ("minecraft:stick", 1)],
        },
        Recipe {
            output: "minecraft:powered_rail",
            output_count: 6,
            ingredients: &[("minecraft:gold_ingot", 6), ("minecraft:stick", 1), ("minecraft:redstone", 1)],
        },
        Recipe {
            output: "minecraft:detector_rail",
            output_count: 6,
            ingredients: &[("minecraft:iron_ingot", 6), ("minecraft:stone_pressure_plate", 1), ("minecraft:redstone", 1)],
        },
        Recipe {
            output: "minecraft:activator_rail",
            output_count: 6,
            ingredients: &[("minecraft:iron_ingot", 6), ("minecraft:stick", 2), ("minecraft:redstone_torch", 1)],
        },

        // === Lanterns ===
        Recipe {
            output: "minecraft:lantern",
            output_count: 1,
            ingredients: &[("minecraft:iron_nugget", 8), ("minecraft:torch", 1)],
        },
        Recipe {
            output: "minecraft:soul_lantern",
            output_count: 1,
            ingredients: &[("minecraft:iron_nugget", 8), ("minecraft:soul_torch", 1)],
        },
        Recipe {
            output: "minecraft:torch",
            output_count: 4,
            ingredients: &[("minecraft:stick", 1), ("minecraft:coal", 1)],
        },
        Recipe {
            output: "minecraft:soul_torch",
            output_count: 4,
            ingredients: &[("minecraft:stick", 1), ("minecraft:coal", 1), ("minecraft:soul_sand", 1)],
        },

        // === Colored Concrete (16 colors) ===
        // Concrete is made by dropping concrete powder into water
        Recipe { output: "minecraft:white_concrete", output_count: 1, ingredients: &[("minecraft:white_concrete_powder", 1)] },
        Recipe { output: "minecraft:orange_concrete", output_count: 1, ingredients: &[("minecraft:orange_concrete_powder", 1)] },
        Recipe { output: "minecraft:magenta_concrete", output_count: 1, ingredients: &[("minecraft:magenta_concrete_powder", 1)] },
        Recipe { output: "minecraft:light_blue_concrete", output_count: 1, ingredients: &[("minecraft:light_blue_concrete_powder", 1)] },
        Recipe { output: "minecraft:yellow_concrete", output_count: 1, ingredients: &[("minecraft:yellow_concrete_powder", 1)] },
        Recipe { output: "minecraft:lime_concrete", output_count: 1, ingredients: &[("minecraft:lime_concrete_powder", 1)] },
        Recipe { output: "minecraft:pink_concrete", output_count: 1, ingredients: &[("minecraft:pink_concrete_powder", 1)] },
        Recipe { output: "minecraft:gray_concrete", output_count: 1, ingredients: &[("minecraft:gray_concrete_powder", 1)] },
        Recipe { output: "minecraft:light_gray_concrete", output_count: 1, ingredients: &[("minecraft:light_gray_concrete_powder", 1)] },
        Recipe { output: "minecraft:cyan_concrete", output_count: 1, ingredients: &[("minecraft:cyan_concrete_powder", 1)] },
        Recipe { output: "minecraft:purple_concrete", output_count: 1, ingredients: &[("minecraft:purple_concrete_powder", 1)] },
        Recipe { output: "minecraft:blue_concrete", output_count: 1, ingredients: &[("minecraft:blue_concrete_powder", 1)] },
        Recipe { output: "minecraft:brown_concrete", output_count: 1, ingredients: &[("minecraft:brown_concrete_powder", 1)] },
        Recipe { output: "minecraft:green_concrete", output_count: 1, ingredients: &[("minecraft:green_concrete_powder", 1)] },
        Recipe { output: "minecraft:red_concrete", output_count: 1, ingredients: &[("minecraft:red_concrete_powder", 1)] },
        Recipe { output: "minecraft:black_concrete", output_count: 1, ingredients: &[("minecraft:black_concrete_powder", 1)] },

        // === Concrete Powder (4 sand + 4 gravel + 1 dye = 8 powder) ===
        Recipe { output: "minecraft:white_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:white_dye", 1)] },
        Recipe { output: "minecraft:orange_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:orange_dye", 1)] },
        Recipe { output: "minecraft:magenta_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:magenta_dye", 1)] },
        Recipe { output: "minecraft:light_blue_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:light_blue_dye", 1)] },
        Recipe { output: "minecraft:yellow_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:yellow_dye", 1)] },
        Recipe { output: "minecraft:lime_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:lime_dye", 1)] },
        Recipe { output: "minecraft:pink_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:pink_dye", 1)] },
        Recipe { output: "minecraft:gray_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:gray_dye", 1)] },
        Recipe { output: "minecraft:light_gray_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:light_gray_dye", 1)] },
        Recipe { output: "minecraft:cyan_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:cyan_dye", 1)] },
        Recipe { output: "minecraft:purple_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:purple_dye", 1)] },
        Recipe { output: "minecraft:blue_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:blue_dye", 1)] },
        Recipe { output: "minecraft:brown_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:brown_dye", 1)] },
        Recipe { output: "minecraft:green_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:green_dye", 1)] },
        Recipe { output: "minecraft:red_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:red_dye", 1)] },
        Recipe { output: "minecraft:black_concrete_powder", output_count: 8, ingredients: &[("minecraft:sand", 4), ("minecraft:gravel", 4), ("minecraft:black_dye", 1)] },

        // === Colored Terracotta (8 terracotta + 1 dye = 8 colored) ===
        Recipe { output: "minecraft:white_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:white_dye", 1)] },
        Recipe { output: "minecraft:orange_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:orange_dye", 1)] },
        Recipe { output: "minecraft:magenta_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:magenta_dye", 1)] },
        Recipe { output: "minecraft:light_blue_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:light_blue_dye", 1)] },
        Recipe { output: "minecraft:yellow_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:yellow_dye", 1)] },
        Recipe { output: "minecraft:lime_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:lime_dye", 1)] },
        Recipe { output: "minecraft:pink_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:pink_dye", 1)] },
        Recipe { output: "minecraft:gray_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:gray_dye", 1)] },
        Recipe { output: "minecraft:light_gray_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:light_gray_dye", 1)] },
        Recipe { output: "minecraft:cyan_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:cyan_dye", 1)] },
        Recipe { output: "minecraft:purple_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:purple_dye", 1)] },
        Recipe { output: "minecraft:blue_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:blue_dye", 1)] },
        Recipe { output: "minecraft:brown_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:brown_dye", 1)] },
        Recipe { output: "minecraft:green_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:green_dye", 1)] },
        Recipe { output: "minecraft:red_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:red_dye", 1)] },
        Recipe { output: "minecraft:black_terracotta", output_count: 8, ingredients: &[("minecraft:terracotta", 8), ("minecraft:black_dye", 1)] },

        // Base terracotta from clay
        Recipe { output: "minecraft:terracotta", output_count: 1, ingredients: &[("minecraft:clay", 1)] }, // smelting

        // === Glazed Terracotta (smelting colored terracotta) ===
        Recipe { output: "minecraft:white_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:white_terracotta", 1)] },
        Recipe { output: "minecraft:orange_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:orange_terracotta", 1)] },
        Recipe { output: "minecraft:magenta_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:magenta_terracotta", 1)] },
        Recipe { output: "minecraft:light_blue_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:light_blue_terracotta", 1)] },
        Recipe { output: "minecraft:yellow_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:yellow_terracotta", 1)] },
        Recipe { output: "minecraft:lime_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:lime_terracotta", 1)] },
        Recipe { output: "minecraft:pink_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:pink_terracotta", 1)] },
        Recipe { output: "minecraft:gray_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:gray_terracotta", 1)] },
        Recipe { output: "minecraft:light_gray_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:light_gray_terracotta", 1)] },
        Recipe { output: "minecraft:cyan_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:cyan_terracotta", 1)] },
        Recipe { output: "minecraft:purple_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:purple_terracotta", 1)] },
        Recipe { output: "minecraft:blue_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:blue_terracotta", 1)] },
        Recipe { output: "minecraft:brown_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:brown_terracotta", 1)] },
        Recipe { output: "minecraft:green_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:green_terracotta", 1)] },
        Recipe { output: "minecraft:red_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:red_terracotta", 1)] },
        Recipe { output: "minecraft:black_glazed_terracotta", output_count: 1, ingredients: &[("minecraft:black_terracotta", 1)] },

        // === Colored Wool (1 wool + 1 dye = 1 colored wool) ===
        Recipe { output: "minecraft:orange_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:orange_dye", 1)] },
        Recipe { output: "minecraft:magenta_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:magenta_dye", 1)] },
        Recipe { output: "minecraft:light_blue_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:light_blue_dye", 1)] },
        Recipe { output: "minecraft:yellow_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:yellow_dye", 1)] },
        Recipe { output: "minecraft:lime_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:lime_dye", 1)] },
        Recipe { output: "minecraft:pink_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:pink_dye", 1)] },
        Recipe { output: "minecraft:gray_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:gray_dye", 1)] },
        Recipe { output: "minecraft:light_gray_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:light_gray_dye", 1)] },
        Recipe { output: "minecraft:cyan_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:cyan_dye", 1)] },
        Recipe { output: "minecraft:purple_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:purple_dye", 1)] },
        Recipe { output: "minecraft:blue_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:blue_dye", 1)] },
        Recipe { output: "minecraft:brown_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:brown_dye", 1)] },
        Recipe { output: "minecraft:green_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:green_dye", 1)] },
        Recipe { output: "minecraft:red_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:red_dye", 1)] },
        Recipe { output: "minecraft:black_wool", output_count: 1, ingredients: &[("minecraft:white_wool", 1), ("minecraft:black_dye", 1)] },

        // === Colored Stained Glass (8 glass + 1 dye = 8 stained) ===
        Recipe { output: "minecraft:white_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:white_dye", 1)] },
        Recipe { output: "minecraft:orange_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:orange_dye", 1)] },
        Recipe { output: "minecraft:magenta_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:magenta_dye", 1)] },
        Recipe { output: "minecraft:light_blue_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:light_blue_dye", 1)] },
        Recipe { output: "minecraft:yellow_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:yellow_dye", 1)] },
        Recipe { output: "minecraft:lime_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:lime_dye", 1)] },
        Recipe { output: "minecraft:pink_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:pink_dye", 1)] },
        Recipe { output: "minecraft:gray_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:gray_dye", 1)] },
        Recipe { output: "minecraft:light_gray_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:light_gray_dye", 1)] },
        Recipe { output: "minecraft:cyan_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:cyan_dye", 1)] },
        Recipe { output: "minecraft:purple_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:purple_dye", 1)] },
        Recipe { output: "minecraft:blue_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:blue_dye", 1)] },
        Recipe { output: "minecraft:brown_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:brown_dye", 1)] },
        Recipe { output: "minecraft:green_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:green_dye", 1)] },
        Recipe { output: "minecraft:red_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:red_dye", 1)] },
        Recipe { output: "minecraft:black_stained_glass", output_count: 8, ingredients: &[("minecraft:glass", 8), ("minecraft:black_dye", 1)] },

        // Base glass from sand
        Recipe { output: "minecraft:glass", output_count: 1, ingredients: &[("minecraft:sand", 1)] }, // smelting

        // === Stained Glass Panes (6 stained glass = 16 panes) ===
        Recipe { output: "minecraft:white_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:white_stained_glass", 6)] },
        Recipe { output: "minecraft:orange_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:orange_stained_glass", 6)] },
        Recipe { output: "minecraft:magenta_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:magenta_stained_glass", 6)] },
        Recipe { output: "minecraft:light_blue_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:light_blue_stained_glass", 6)] },
        Recipe { output: "minecraft:yellow_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:yellow_stained_glass", 6)] },
        Recipe { output: "minecraft:lime_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:lime_stained_glass", 6)] },
        Recipe { output: "minecraft:pink_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:pink_stained_glass", 6)] },
        Recipe { output: "minecraft:gray_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:gray_stained_glass", 6)] },
        Recipe { output: "minecraft:light_gray_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:light_gray_stained_glass", 6)] },
        Recipe { output: "minecraft:cyan_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:cyan_stained_glass", 6)] },
        Recipe { output: "minecraft:purple_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:purple_stained_glass", 6)] },
        Recipe { output: "minecraft:blue_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:blue_stained_glass", 6)] },
        Recipe { output: "minecraft:brown_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:brown_stained_glass", 6)] },
        Recipe { output: "minecraft:green_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:green_stained_glass", 6)] },
        Recipe { output: "minecraft:red_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:red_stained_glass", 6)] },
        Recipe { output: "minecraft:black_stained_glass_pane", output_count: 16, ingredients: &[("minecraft:black_stained_glass", 6)] },

        // Regular glass pane
        Recipe { output: "minecraft:glass_pane", output_count: 16, ingredients: &[("minecraft:glass", 6)] },

        // === Beds (3 wool + 3 planks = 1 bed) ===
        Recipe { output: "minecraft:white_bed", output_count: 1, ingredients: &[("minecraft:white_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:orange_bed", output_count: 1, ingredients: &[("minecraft:orange_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:magenta_bed", output_count: 1, ingredients: &[("minecraft:magenta_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:light_blue_bed", output_count: 1, ingredients: &[("minecraft:light_blue_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:yellow_bed", output_count: 1, ingredients: &[("minecraft:yellow_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:lime_bed", output_count: 1, ingredients: &[("minecraft:lime_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:pink_bed", output_count: 1, ingredients: &[("minecraft:pink_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:gray_bed", output_count: 1, ingredients: &[("minecraft:gray_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:light_gray_bed", output_count: 1, ingredients: &[("minecraft:light_gray_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:cyan_bed", output_count: 1, ingredients: &[("minecraft:cyan_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:purple_bed", output_count: 1, ingredients: &[("minecraft:purple_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:blue_bed", output_count: 1, ingredients: &[("minecraft:blue_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:brown_bed", output_count: 1, ingredients: &[("minecraft:brown_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:green_bed", output_count: 1, ingredients: &[("minecraft:green_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:red_bed", output_count: 1, ingredients: &[("minecraft:red_wool", 3), ("minecraft:any_planks", 3)] },
        Recipe { output: "minecraft:black_bed", output_count: 1, ingredients: &[("minecraft:black_wool", 3), ("minecraft:any_planks", 3)] },

        // === Banners (6 wool + 1 stick = 1 banner) ===
        Recipe { output: "minecraft:white_banner", output_count: 1, ingredients: &[("minecraft:white_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:orange_banner", output_count: 1, ingredients: &[("minecraft:orange_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:magenta_banner", output_count: 1, ingredients: &[("minecraft:magenta_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:light_blue_banner", output_count: 1, ingredients: &[("minecraft:light_blue_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:yellow_banner", output_count: 1, ingredients: &[("minecraft:yellow_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:lime_banner", output_count: 1, ingredients: &[("minecraft:lime_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:pink_banner", output_count: 1, ingredients: &[("minecraft:pink_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:gray_banner", output_count: 1, ingredients: &[("minecraft:gray_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:light_gray_banner", output_count: 1, ingredients: &[("minecraft:light_gray_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:cyan_banner", output_count: 1, ingredients: &[("minecraft:cyan_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:purple_banner", output_count: 1, ingredients: &[("minecraft:purple_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:blue_banner", output_count: 1, ingredients: &[("minecraft:blue_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:brown_banner", output_count: 1, ingredients: &[("minecraft:brown_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:green_banner", output_count: 1, ingredients: &[("minecraft:green_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:red_banner", output_count: 1, ingredients: &[("minecraft:red_wool", 6), ("minecraft:stick", 1)] },
        Recipe { output: "minecraft:black_banner", output_count: 1, ingredients: &[("minecraft:black_wool", 6), ("minecraft:stick", 1)] },

        // === Carpets (2 wool = 3 carpet) ===
        Recipe { output: "minecraft:white_carpet", output_count: 3, ingredients: &[("minecraft:white_wool", 2)] },
        Recipe { output: "minecraft:orange_carpet", output_count: 3, ingredients: &[("minecraft:orange_wool", 2)] },
        Recipe { output: "minecraft:magenta_carpet", output_count: 3, ingredients: &[("minecraft:magenta_wool", 2)] },
        Recipe { output: "minecraft:light_blue_carpet", output_count: 3, ingredients: &[("minecraft:light_blue_wool", 2)] },
        Recipe { output: "minecraft:yellow_carpet", output_count: 3, ingredients: &[("minecraft:yellow_wool", 2)] },
        Recipe { output: "minecraft:lime_carpet", output_count: 3, ingredients: &[("minecraft:lime_wool", 2)] },
        Recipe { output: "minecraft:pink_carpet", output_count: 3, ingredients: &[("minecraft:pink_wool", 2)] },
        Recipe { output: "minecraft:gray_carpet", output_count: 3, ingredients: &[("minecraft:gray_wool", 2)] },
        Recipe { output: "minecraft:light_gray_carpet", output_count: 3, ingredients: &[("minecraft:light_gray_wool", 2)] },
        Recipe { output: "minecraft:cyan_carpet", output_count: 3, ingredients: &[("minecraft:cyan_wool", 2)] },
        Recipe { output: "minecraft:purple_carpet", output_count: 3, ingredients: &[("minecraft:purple_wool", 2)] },
        Recipe { output: "minecraft:blue_carpet", output_count: 3, ingredients: &[("minecraft:blue_wool", 2)] },
        Recipe { output: "minecraft:brown_carpet", output_count: 3, ingredients: &[("minecraft:brown_wool", 2)] },
        Recipe { output: "minecraft:green_carpet", output_count: 3, ingredients: &[("minecraft:green_wool", 2)] },
        Recipe { output: "minecraft:red_carpet", output_count: 3, ingredients: &[("minecraft:red_wool", 2)] },
        Recipe { output: "minecraft:black_carpet", output_count: 3, ingredients: &[("minecraft:black_wool", 2)] },

        // === Candles (1 string + 1 honeycomb = 1 candle) ===
        Recipe { output: "minecraft:candle", output_count: 1, ingredients: &[("minecraft:string", 1), ("minecraft:honeycomb", 1)] },
        Recipe { output: "minecraft:white_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:white_dye", 1)] },
        Recipe { output: "minecraft:orange_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:orange_dye", 1)] },
        Recipe { output: "minecraft:magenta_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:magenta_dye", 1)] },
        Recipe { output: "minecraft:light_blue_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:light_blue_dye", 1)] },
        Recipe { output: "minecraft:yellow_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:yellow_dye", 1)] },
        Recipe { output: "minecraft:lime_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:lime_dye", 1)] },
        Recipe { output: "minecraft:pink_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:pink_dye", 1)] },
        Recipe { output: "minecraft:gray_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:gray_dye", 1)] },
        Recipe { output: "minecraft:light_gray_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:light_gray_dye", 1)] },
        Recipe { output: "minecraft:cyan_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:cyan_dye", 1)] },
        Recipe { output: "minecraft:purple_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:purple_dye", 1)] },
        Recipe { output: "minecraft:blue_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:blue_dye", 1)] },
        Recipe { output: "minecraft:brown_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:brown_dye", 1)] },
        Recipe { output: "minecraft:green_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:green_dye", 1)] },
        Recipe { output: "minecraft:red_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:red_dye", 1)] },
        Recipe { output: "minecraft:black_candle", output_count: 1, ingredients: &[("minecraft:candle", 1), ("minecraft:black_dye", 1)] },

        // === Shulker Boxes (1 chest + 2 shulker shells = 1 shulker box) ===
        Recipe { output: "minecraft:shulker_box", output_count: 1, ingredients: &[("minecraft:chest", 1), ("minecraft:shulker_shell", 2)] },
        Recipe { output: "minecraft:white_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:white_dye", 1)] },
        Recipe { output: "minecraft:orange_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:orange_dye", 1)] },
        Recipe { output: "minecraft:magenta_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:magenta_dye", 1)] },
        Recipe { output: "minecraft:light_blue_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:light_blue_dye", 1)] },
        Recipe { output: "minecraft:yellow_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:yellow_dye", 1)] },
        Recipe { output: "minecraft:lime_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:lime_dye", 1)] },
        Recipe { output: "minecraft:pink_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:pink_dye", 1)] },
        Recipe { output: "minecraft:gray_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:gray_dye", 1)] },
        Recipe { output: "minecraft:light_gray_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:light_gray_dye", 1)] },
        Recipe { output: "minecraft:cyan_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:cyan_dye", 1)] },
        Recipe { output: "minecraft:purple_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:purple_dye", 1)] },
        Recipe { output: "minecraft:blue_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:blue_dye", 1)] },
        Recipe { output: "minecraft:brown_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:brown_dye", 1)] },
        Recipe { output: "minecraft:green_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:green_dye", 1)] },
        Recipe { output: "minecraft:red_shulker_box", output_count: 1, ingredients: &[("minecraft:shulker_box", 1), ("minecraft:red_dye", 1)] },
        Recipe { output: "minecraft:black_shulker_box", output_count: 1, ingredients: &[("minecraft:black_dye", 1), ("minecraft:shulker_box", 1)] },
    ];

    recipes.into_iter().map(|r| (r.output, r)).collect()
}

/// Raw materials that cannot be broken down further
pub fn is_raw_material(name: &str) -> bool {
    matches!(name,
        // Ores and raw forms
        "minecraft:coal" |
        "minecraft:raw_iron" |
        "minecraft:raw_gold" |
        "minecraft:raw_copper" |
        "minecraft:diamond" |
        "minecraft:emerald" |
        "minecraft:lapis_lazuli" |
        "minecraft:redstone" |
        "minecraft:quartz" |
        "minecraft:netherite_scrap" |
        "minecraft:amethyst_shard" |
        "minecraft:prismarine_shard" |
        "minecraft:prismarine_crystals" |
        "minecraft:glowstone_dust" |
        "minecraft:nether_wart" |

        // Ingots (consider as raw for simplicity)
        "minecraft:iron_ingot" |
        "minecraft:gold_ingot" |
        "minecraft:copper_ingot" |
        "minecraft:netherite_ingot" |
        "minecraft:iron_nugget" |
        "minecraft:gold_nugget" |

        // Natural blocks
        "minecraft:cobblestone" |
        "minecraft:stone" |
        "minecraft:deepslate" |
        "minecraft:cobbled_deepslate" |
        "minecraft:blackstone" |
        "minecraft:basalt" |
        "minecraft:netherrack" |
        "minecraft:soul_sand" |
        "minecraft:soul_soil" |
        "minecraft:end_stone" |
        "minecraft:obsidian" |
        "minecraft:crying_obsidian" |
        "minecraft:calcite" |
        "minecraft:tuff" |
        "minecraft:dripstone_block" |
        "minecraft:pointed_dripstone" |
        "minecraft:moss_block" |
        "minecraft:sculk" |
        "minecraft:mud" |

        // Dirt/grass
        "minecraft:dirt" |
        "minecraft:grass_block" |
        "minecraft:podzol" |
        "minecraft:mycelium" |
        "minecraft:coarse_dirt" |
        "minecraft:rooted_dirt" |

        // Sand/gravel
        "minecraft:sand" |
        "minecraft:red_sand" |
        "minecraft:gravel" |
        "minecraft:clay_ball" |

        // Logs
        "minecraft:oak_log" |
        "minecraft:spruce_log" |
        "minecraft:birch_log" |
        "minecraft:jungle_log" |
        "minecraft:acacia_log" |
        "minecraft:dark_oak_log" |
        "minecraft:mangrove_log" |
        "minecraft:cherry_log" |
        "minecraft:bamboo_block" |
        "minecraft:crimson_stem" |
        "minecraft:warped_stem" |
        "minecraft:any_log" |
        "minecraft:any_planks" |
        "minecraft:any_slab" |

        // Ice/snow
        "minecraft:ice" |
        "minecraft:snowball" |

        // Organic
        "minecraft:string" |
        "minecraft:leather" |
        "minecraft:sugar_cane" |
        "minecraft:wheat" |
        "minecraft:bone_meal" |
        "minecraft:slime_ball" |
        "minecraft:honey_bottle" |
        "minecraft:melon_slice" |
        "minecraft:gunpowder" |
        "minecraft:dried_kelp" |
        "minecraft:popped_chorus_fruit" |

        // Dyes
        "minecraft:white_dye" |
        "minecraft:red_dye" |
        "minecraft:orange_dye" |
        "minecraft:yellow_dye" |
        "minecraft:lime_dye" |
        "minecraft:green_dye" |
        "minecraft:cyan_dye" |
        "minecraft:light_blue_dye" |
        "minecraft:blue_dye" |
        "minecraft:purple_dye" |
        "minecraft:magenta_dye" |
        "minecraft:pink_dye" |
        "minecraft:brown_dye" |
        "minecraft:black_dye" |
        "minecraft:gray_dye" |
        "minecraft:light_gray_dye" |

        // Special
        "minecraft:bow" |
        "minecraft:stick" |
        "minecraft:book" |

        // Wool (white is base, obtained from sheep)
        "minecraft:white_wool" |

        // Clay (mined from clay blocks)
        "minecraft:clay" |

        // Honeycomb (from bee nests)
        "minecraft:honeycomb" |

        // Shulker shell (from shulkers)
        "minecraft:shulker_shell"
    )
}

/// Get stonecutter recipes (1:1 ratios for stairs/slabs)
/// Stonecutter is more efficient than crafting table
pub fn get_stonecutter_recipes() -> HashMap<&'static str, Recipe> {
    let recipes: Vec<Recipe> = vec![
        // Stone stairs and slabs (1:1 with stonecutter)
        Recipe { output: "minecraft:stone_stairs", output_count: 1, ingredients: &[("minecraft:stone", 1)] },
        Recipe { output: "minecraft:stone_slab", output_count: 2, ingredients: &[("minecraft:stone", 1)] },
        Recipe { output: "minecraft:cobblestone_stairs", output_count: 1, ingredients: &[("minecraft:cobblestone", 1)] },
        Recipe { output: "minecraft:cobblestone_slab", output_count: 2, ingredients: &[("minecraft:cobblestone", 1)] },
        Recipe { output: "minecraft:cobblestone_wall", output_count: 1, ingredients: &[("minecraft:cobblestone", 1)] },
        Recipe { output: "minecraft:mossy_cobblestone_stairs", output_count: 1, ingredients: &[("minecraft:mossy_cobblestone", 1)] },
        Recipe { output: "minecraft:mossy_cobblestone_slab", output_count: 2, ingredients: &[("minecraft:mossy_cobblestone", 1)] },
        Recipe { output: "minecraft:mossy_cobblestone_wall", output_count: 1, ingredients: &[("minecraft:mossy_cobblestone", 1)] },
        Recipe { output: "minecraft:stone_brick_stairs", output_count: 1, ingredients: &[("minecraft:stone_bricks", 1)] },
        Recipe { output: "minecraft:stone_brick_slab", output_count: 2, ingredients: &[("minecraft:stone_bricks", 1)] },
        Recipe { output: "minecraft:stone_brick_wall", output_count: 1, ingredients: &[("minecraft:stone_bricks", 1)] },
        Recipe { output: "minecraft:mossy_stone_brick_stairs", output_count: 1, ingredients: &[("minecraft:mossy_stone_bricks", 1)] },
        Recipe { output: "minecraft:mossy_stone_brick_slab", output_count: 2, ingredients: &[("minecraft:mossy_stone_bricks", 1)] },
        Recipe { output: "minecraft:mossy_stone_brick_wall", output_count: 1, ingredients: &[("minecraft:mossy_stone_bricks", 1)] },
        Recipe { output: "minecraft:smooth_stone_slab", output_count: 2, ingredients: &[("minecraft:smooth_stone", 1)] },

        // Granite
        Recipe { output: "minecraft:granite_stairs", output_count: 1, ingredients: &[("minecraft:granite", 1)] },
        Recipe { output: "minecraft:granite_slab", output_count: 2, ingredients: &[("minecraft:granite", 1)] },
        Recipe { output: "minecraft:granite_wall", output_count: 1, ingredients: &[("minecraft:granite", 1)] },
        Recipe { output: "minecraft:polished_granite_stairs", output_count: 1, ingredients: &[("minecraft:polished_granite", 1)] },
        Recipe { output: "minecraft:polished_granite_slab", output_count: 2, ingredients: &[("minecraft:polished_granite", 1)] },

        // Diorite
        Recipe { output: "minecraft:diorite_stairs", output_count: 1, ingredients: &[("minecraft:diorite", 1)] },
        Recipe { output: "minecraft:diorite_slab", output_count: 2, ingredients: &[("minecraft:diorite", 1)] },
        Recipe { output: "minecraft:diorite_wall", output_count: 1, ingredients: &[("minecraft:diorite", 1)] },
        Recipe { output: "minecraft:polished_diorite_stairs", output_count: 1, ingredients: &[("minecraft:polished_diorite", 1)] },
        Recipe { output: "minecraft:polished_diorite_slab", output_count: 2, ingredients: &[("minecraft:polished_diorite", 1)] },

        // Andesite
        Recipe { output: "minecraft:andesite_stairs", output_count: 1, ingredients: &[("minecraft:andesite", 1)] },
        Recipe { output: "minecraft:andesite_slab", output_count: 2, ingredients: &[("minecraft:andesite", 1)] },
        Recipe { output: "minecraft:andesite_wall", output_count: 1, ingredients: &[("minecraft:andesite", 1)] },
        Recipe { output: "minecraft:polished_andesite_stairs", output_count: 1, ingredients: &[("minecraft:polished_andesite", 1)] },
        Recipe { output: "minecraft:polished_andesite_slab", output_count: 2, ingredients: &[("minecraft:polished_andesite", 1)] },

        // Deepslate
        Recipe { output: "minecraft:cobbled_deepslate_stairs", output_count: 1, ingredients: &[("minecraft:cobbled_deepslate", 1)] },
        Recipe { output: "minecraft:cobbled_deepslate_slab", output_count: 2, ingredients: &[("minecraft:cobbled_deepslate", 1)] },
        Recipe { output: "minecraft:cobbled_deepslate_wall", output_count: 1, ingredients: &[("minecraft:cobbled_deepslate", 1)] },
        Recipe { output: "minecraft:polished_deepslate_stairs", output_count: 1, ingredients: &[("minecraft:polished_deepslate", 1)] },
        Recipe { output: "minecraft:polished_deepslate_slab", output_count: 2, ingredients: &[("minecraft:polished_deepslate", 1)] },
        Recipe { output: "minecraft:polished_deepslate_wall", output_count: 1, ingredients: &[("minecraft:polished_deepslate", 1)] },
        Recipe { output: "minecraft:deepslate_brick_stairs", output_count: 1, ingredients: &[("minecraft:deepslate_bricks", 1)] },
        Recipe { output: "minecraft:deepslate_brick_slab", output_count: 2, ingredients: &[("minecraft:deepslate_bricks", 1)] },
        Recipe { output: "minecraft:deepslate_brick_wall", output_count: 1, ingredients: &[("minecraft:deepslate_bricks", 1)] },
        Recipe { output: "minecraft:deepslate_tile_stairs", output_count: 1, ingredients: &[("minecraft:deepslate_tiles", 1)] },
        Recipe { output: "minecraft:deepslate_tile_slab", output_count: 2, ingredients: &[("minecraft:deepslate_tiles", 1)] },
        Recipe { output: "minecraft:deepslate_tile_wall", output_count: 1, ingredients: &[("minecraft:deepslate_tiles", 1)] },

        // Blackstone
        Recipe { output: "minecraft:blackstone_stairs", output_count: 1, ingredients: &[("minecraft:blackstone", 1)] },
        Recipe { output: "minecraft:blackstone_slab", output_count: 2, ingredients: &[("minecraft:blackstone", 1)] },
        Recipe { output: "minecraft:blackstone_wall", output_count: 1, ingredients: &[("minecraft:blackstone", 1)] },
        Recipe { output: "minecraft:polished_blackstone_stairs", output_count: 1, ingredients: &[("minecraft:polished_blackstone", 1)] },
        Recipe { output: "minecraft:polished_blackstone_slab", output_count: 2, ingredients: &[("minecraft:polished_blackstone", 1)] },
        Recipe { output: "minecraft:polished_blackstone_wall", output_count: 1, ingredients: &[("minecraft:polished_blackstone", 1)] },
        Recipe { output: "minecraft:polished_blackstone_brick_stairs", output_count: 1, ingredients: &[("minecraft:polished_blackstone_bricks", 1)] },
        Recipe { output: "minecraft:polished_blackstone_brick_slab", output_count: 2, ingredients: &[("minecraft:polished_blackstone_bricks", 1)] },
        Recipe { output: "minecraft:polished_blackstone_brick_wall", output_count: 1, ingredients: &[("minecraft:polished_blackstone_bricks", 1)] },

        // Nether bricks
        Recipe { output: "minecraft:nether_brick_stairs", output_count: 1, ingredients: &[("minecraft:nether_bricks", 1)] },
        Recipe { output: "minecraft:nether_brick_slab", output_count: 2, ingredients: &[("minecraft:nether_bricks", 1)] },
        Recipe { output: "minecraft:nether_brick_wall", output_count: 1, ingredients: &[("minecraft:nether_bricks", 1)] },
        Recipe { output: "minecraft:red_nether_brick_stairs", output_count: 1, ingredients: &[("minecraft:red_nether_bricks", 1)] },
        Recipe { output: "minecraft:red_nether_brick_slab", output_count: 2, ingredients: &[("minecraft:red_nether_bricks", 1)] },
        Recipe { output: "minecraft:red_nether_brick_wall", output_count: 1, ingredients: &[("minecraft:red_nether_bricks", 1)] },

        // Quartz
        Recipe { output: "minecraft:quartz_stairs", output_count: 1, ingredients: &[("minecraft:quartz_block", 1)] },
        Recipe { output: "minecraft:quartz_slab", output_count: 2, ingredients: &[("minecraft:quartz_block", 1)] },
        Recipe { output: "minecraft:smooth_quartz_stairs", output_count: 1, ingredients: &[("minecraft:smooth_quartz", 1)] },
        Recipe { output: "minecraft:smooth_quartz_slab", output_count: 2, ingredients: &[("minecraft:smooth_quartz", 1)] },

        // Bricks
        Recipe { output: "minecraft:brick_stairs", output_count: 1, ingredients: &[("minecraft:bricks", 1)] },
        Recipe { output: "minecraft:brick_slab", output_count: 2, ingredients: &[("minecraft:bricks", 1)] },
        Recipe { output: "minecraft:brick_wall", output_count: 1, ingredients: &[("minecraft:bricks", 1)] },
        Recipe { output: "minecraft:mud_brick_stairs", output_count: 1, ingredients: &[("minecraft:mud_bricks", 1)] },
        Recipe { output: "minecraft:mud_brick_slab", output_count: 2, ingredients: &[("minecraft:mud_bricks", 1)] },
        Recipe { output: "minecraft:mud_brick_wall", output_count: 1, ingredients: &[("minecraft:mud_bricks", 1)] },

        // Sandstone
        Recipe { output: "minecraft:sandstone_stairs", output_count: 1, ingredients: &[("minecraft:sandstone", 1)] },
        Recipe { output: "minecraft:sandstone_slab", output_count: 2, ingredients: &[("minecraft:sandstone", 1)] },
        Recipe { output: "minecraft:sandstone_wall", output_count: 1, ingredients: &[("minecraft:sandstone", 1)] },
        Recipe { output: "minecraft:smooth_sandstone_stairs", output_count: 1, ingredients: &[("minecraft:smooth_sandstone", 1)] },
        Recipe { output: "minecraft:smooth_sandstone_slab", output_count: 2, ingredients: &[("minecraft:smooth_sandstone", 1)] },
        Recipe { output: "minecraft:red_sandstone_stairs", output_count: 1, ingredients: &[("minecraft:red_sandstone", 1)] },
        Recipe { output: "minecraft:red_sandstone_slab", output_count: 2, ingredients: &[("minecraft:red_sandstone", 1)] },
        Recipe { output: "minecraft:red_sandstone_wall", output_count: 1, ingredients: &[("minecraft:red_sandstone", 1)] },
        Recipe { output: "minecraft:smooth_red_sandstone_stairs", output_count: 1, ingredients: &[("minecraft:smooth_red_sandstone", 1)] },
        Recipe { output: "minecraft:smooth_red_sandstone_slab", output_count: 2, ingredients: &[("minecraft:smooth_red_sandstone", 1)] },

        // Prismarine
        Recipe { output: "minecraft:prismarine_stairs", output_count: 1, ingredients: &[("minecraft:prismarine", 1)] },
        Recipe { output: "minecraft:prismarine_slab", output_count: 2, ingredients: &[("minecraft:prismarine", 1)] },
        Recipe { output: "minecraft:prismarine_wall", output_count: 1, ingredients: &[("minecraft:prismarine", 1)] },
        Recipe { output: "minecraft:prismarine_brick_stairs", output_count: 1, ingredients: &[("minecraft:prismarine_bricks", 1)] },
        Recipe { output: "minecraft:prismarine_brick_slab", output_count: 2, ingredients: &[("minecraft:prismarine_bricks", 1)] },
        Recipe { output: "minecraft:dark_prismarine_stairs", output_count: 1, ingredients: &[("minecraft:dark_prismarine", 1)] },
        Recipe { output: "minecraft:dark_prismarine_slab", output_count: 2, ingredients: &[("minecraft:dark_prismarine", 1)] },

        // End stone
        Recipe { output: "minecraft:end_stone_brick_stairs", output_count: 1, ingredients: &[("minecraft:end_stone_bricks", 1)] },
        Recipe { output: "minecraft:end_stone_brick_slab", output_count: 2, ingredients: &[("minecraft:end_stone_bricks", 1)] },
        Recipe { output: "minecraft:end_stone_brick_wall", output_count: 1, ingredients: &[("minecraft:end_stone_bricks", 1)] },

        // Purpur
        Recipe { output: "minecraft:purpur_stairs", output_count: 1, ingredients: &[("minecraft:purpur_block", 1)] },
        Recipe { output: "minecraft:purpur_slab", output_count: 2, ingredients: &[("minecraft:purpur_block", 1)] },

        // Copper (cut copper)
        Recipe { output: "minecraft:cut_copper_stairs", output_count: 1, ingredients: &[("minecraft:cut_copper", 1)] },
        Recipe { output: "minecraft:cut_copper_slab", output_count: 2, ingredients: &[("minecraft:cut_copper", 1)] },
        Recipe { output: "minecraft:exposed_cut_copper_stairs", output_count: 1, ingredients: &[("minecraft:exposed_cut_copper", 1)] },
        Recipe { output: "minecraft:exposed_cut_copper_slab", output_count: 2, ingredients: &[("minecraft:exposed_cut_copper", 1)] },
        Recipe { output: "minecraft:weathered_cut_copper_stairs", output_count: 1, ingredients: &[("minecraft:weathered_cut_copper", 1)] },
        Recipe { output: "minecraft:weathered_cut_copper_slab", output_count: 2, ingredients: &[("minecraft:weathered_cut_copper", 1)] },
        Recipe { output: "minecraft:oxidized_cut_copper_stairs", output_count: 1, ingredients: &[("minecraft:oxidized_cut_copper", 1)] },
        Recipe { output: "minecraft:oxidized_cut_copper_slab", output_count: 2, ingredients: &[("minecraft:oxidized_cut_copper", 1)] },
        Recipe { output: "minecraft:waxed_cut_copper_stairs", output_count: 1, ingredients: &[("minecraft:waxed_cut_copper", 1)] },
        Recipe { output: "minecraft:waxed_cut_copper_slab", output_count: 2, ingredients: &[("minecraft:waxed_cut_copper", 1)] },
        Recipe { output: "minecraft:waxed_exposed_cut_copper_stairs", output_count: 1, ingredients: &[("minecraft:waxed_exposed_cut_copper", 1)] },
        Recipe { output: "minecraft:waxed_exposed_cut_copper_slab", output_count: 2, ingredients: &[("minecraft:waxed_exposed_cut_copper", 1)] },
        Recipe { output: "minecraft:waxed_weathered_cut_copper_stairs", output_count: 1, ingredients: &[("minecraft:waxed_weathered_cut_copper", 1)] },
        Recipe { output: "minecraft:waxed_weathered_cut_copper_slab", output_count: 2, ingredients: &[("minecraft:waxed_weathered_cut_copper", 1)] },
        Recipe { output: "minecraft:waxed_oxidized_cut_copper_stairs", output_count: 1, ingredients: &[("minecraft:waxed_oxidized_cut_copper", 1)] },
        Recipe { output: "minecraft:waxed_oxidized_cut_copper_slab", output_count: 2, ingredients: &[("minecraft:waxed_oxidized_cut_copper", 1)] },

        // Tuff
        Recipe { output: "minecraft:tuff_stairs", output_count: 1, ingredients: &[("minecraft:tuff", 1)] },
        Recipe { output: "minecraft:tuff_slab", output_count: 2, ingredients: &[("minecraft:tuff", 1)] },
        Recipe { output: "minecraft:tuff_wall", output_count: 1, ingredients: &[("minecraft:tuff", 1)] },
        Recipe { output: "minecraft:polished_tuff_stairs", output_count: 1, ingredients: &[("minecraft:polished_tuff", 1)] },
        Recipe { output: "minecraft:polished_tuff_slab", output_count: 2, ingredients: &[("minecraft:polished_tuff", 1)] },
        Recipe { output: "minecraft:polished_tuff_wall", output_count: 1, ingredients: &[("minecraft:polished_tuff", 1)] },
        Recipe { output: "minecraft:tuff_brick_stairs", output_count: 1, ingredients: &[("minecraft:tuff_bricks", 1)] },
        Recipe { output: "minecraft:tuff_brick_slab", output_count: 2, ingredients: &[("minecraft:tuff_bricks", 1)] },
        Recipe { output: "minecraft:tuff_brick_wall", output_count: 1, ingredients: &[("minecraft:tuff_bricks", 1)] },
    ];

    recipes.into_iter().map(|r| (r.output, r)).collect()
}

/// Calculate raw materials needed for a block count
pub fn calculate_materials(blocks: &HashMap<String, usize>) -> HashMap<String, f64> {
    calculate_materials_with_options(blocks, false)
}

/// Calculate raw materials with options
/// - `use_stonecutter`: If true, uses stonecutter recipes (1:1 ratios) for stairs/slabs/walls
pub fn calculate_materials_with_options(blocks: &HashMap<String, usize>, use_stonecutter: bool) -> HashMap<String, f64> {
    let mut recipes = get_recipes();

    // Override with stonecutter recipes if enabled
    if use_stonecutter {
        for (name, recipe) in get_stonecutter_recipes() {
            recipes.insert(name, recipe);
        }
    }

    let mut materials: HashMap<String, f64> = HashMap::new();
    let mut to_process: Vec<(String, f64)> = blocks.iter()
        .filter(|(name, _)| !name.contains("air"))
        .map(|(name, count)| (name.clone(), *count as f64))
        .collect();

    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 100;

    while !to_process.is_empty() && iterations < MAX_ITERATIONS {
        iterations += 1;
        let mut next_round: Vec<(String, f64)> = Vec::new();

        for (item, count) in to_process {
            if is_raw_material(&item) {
                *materials.entry(item).or_insert(0.0) += count;
            } else if let Some(recipe) = recipes.get(item.as_str()) {
                let batches = count / recipe.output_count as f64;
                for (ingredient, ing_count) in recipe.ingredients.iter() {
                    next_round.push((ingredient.to_string(), batches * *ing_count as f64));
                }
            } else {
                // Unknown recipe - treat as raw material
                *materials.entry(item).or_insert(0.0) += count;
            }
        }

        to_process = next_round;
    }

    materials
}
