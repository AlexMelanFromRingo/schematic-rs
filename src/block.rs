use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Minecraft block
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// Block name (e.g., "minecraft:stone")
    pub name: String,
    /// Block state properties
    pub state: BlockState,
}

/// Block state properties (facing, powered, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BlockState {
    #[serde(flatten)]
    pub properties: HashMap<String, String>,
}

impl Block {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            state: BlockState::default(),
        }
    }

    pub fn with_state(name: impl Into<String>, state: BlockState) -> Self {
        Self {
            name: name.into(),
            state,
        }
    }

    pub fn air() -> Self {
        Self::new("minecraft:air")
    }

    pub fn is_air(&self) -> bool {
        matches!(
            self.name.as_str(),
            "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air" | "air"
        )
    }

    /// Get a property value
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.state.properties.get(key)
    }

    /// Get facing direction if present
    pub fn facing(&self) -> Option<&String> {
        self.get_property("facing")
    }

    /// Check if block is powered (for redstone)
    pub fn is_powered(&self) -> Option<bool> {
        self.get_property("powered").map(|v| v == "true")
    }

    /// Get display name (without minecraft: prefix)
    pub fn display_name(&self) -> &str {
        self.name
            .strip_prefix("minecraft:")
            .unwrap_or(&self.name)
    }

    /// Format block with state for display
    pub fn full_name(&self) -> String {
        if self.state.properties.is_empty() {
            self.name.clone()
        } else {
            let props: Vec<String> = self.state.properties
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            format!("{}[{}]", self.name, props.join(","))
        }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

/// Legacy block ID mapping (for .schematic format)
/// Maps numeric IDs to block names
pub fn legacy_id_to_name(id: u8, data: u8) -> String {
    // Common blocks - this is a subset, full mapping would be huge
    match id {
        0 => "minecraft:air".to_string(),
        1 => match data {
            0 => "minecraft:stone",
            1 => "minecraft:granite",
            2 => "minecraft:polished_granite",
            3 => "minecraft:diorite",
            4 => "minecraft:polished_diorite",
            5 => "minecraft:andesite",
            6 => "minecraft:polished_andesite",
            _ => "minecraft:stone",
        }.to_string(),
        2 => "minecraft:grass_block".to_string(),
        3 => match data {
            0 => "minecraft:dirt",
            1 => "minecraft:coarse_dirt",
            2 => "minecraft:podzol",
            _ => "minecraft:dirt",
        }.to_string(),
        4 => "minecraft:cobblestone".to_string(),
        5 => match data {
            0 => "minecraft:oak_planks",
            1 => "minecraft:spruce_planks",
            2 => "minecraft:birch_planks",
            3 => "minecraft:jungle_planks",
            4 => "minecraft:acacia_planks",
            5 => "minecraft:dark_oak_planks",
            _ => "minecraft:oak_planks",
        }.to_string(),
        7 => "minecraft:bedrock".to_string(),
        8 | 9 => "minecraft:water".to_string(),
        10 | 11 => "minecraft:lava".to_string(),
        12 => match data {
            0 => "minecraft:sand",
            1 => "minecraft:red_sand",
            _ => "minecraft:sand",
        }.to_string(),
        13 => "minecraft:gravel".to_string(),
        14 => "minecraft:gold_ore".to_string(),
        15 => "minecraft:iron_ore".to_string(),
        16 => "minecraft:coal_ore".to_string(),
        17 => match data & 0x3 {
            0 => "minecraft:oak_log",
            1 => "minecraft:spruce_log",
            2 => "minecraft:birch_log",
            3 => "minecraft:jungle_log",
            _ => "minecraft:oak_log",
        }.to_string(),
        18 => match data & 0x3 {
            0 => "minecraft:oak_leaves",
            1 => "minecraft:spruce_leaves",
            2 => "minecraft:birch_leaves",
            3 => "minecraft:jungle_leaves",
            _ => "minecraft:oak_leaves",
        }.to_string(),
        20 => "minecraft:glass".to_string(),
        21 => "minecraft:lapis_ore".to_string(),
        22 => "minecraft:lapis_block".to_string(),
        23 => "minecraft:dispenser".to_string(),
        24 => "minecraft:sandstone".to_string(),
        25 => "minecraft:note_block".to_string(),
        29 => "minecraft:sticky_piston".to_string(),
        33 => "minecraft:piston".to_string(),
        35 => match data {
            0 => "minecraft:white_wool",
            1 => "minecraft:orange_wool",
            2 => "minecraft:magenta_wool",
            3 => "minecraft:light_blue_wool",
            4 => "minecraft:yellow_wool",
            5 => "minecraft:lime_wool",
            6 => "minecraft:pink_wool",
            7 => "minecraft:gray_wool",
            8 => "minecraft:light_gray_wool",
            9 => "minecraft:cyan_wool",
            10 => "minecraft:purple_wool",
            11 => "minecraft:blue_wool",
            12 => "minecraft:brown_wool",
            13 => "minecraft:green_wool",
            14 => "minecraft:red_wool",
            15 => "minecraft:black_wool",
            _ => "minecraft:white_wool",
        }.to_string(),
        41 => "minecraft:gold_block".to_string(),
        42 => "minecraft:iron_block".to_string(),
        45 => "minecraft:bricks".to_string(),
        46 => "minecraft:tnt".to_string(),
        47 => "minecraft:bookshelf".to_string(),
        48 => "minecraft:mossy_cobblestone".to_string(),
        49 => "minecraft:obsidian".to_string(),
        50 => "minecraft:torch".to_string(),
        52 => "minecraft:spawner".to_string(),
        53 => "minecraft:oak_stairs".to_string(),
        54 => "minecraft:chest".to_string(),
        55 => "minecraft:redstone_wire".to_string(),
        56 => "minecraft:diamond_ore".to_string(),
        57 => "minecraft:diamond_block".to_string(),
        58 => "minecraft:crafting_table".to_string(),
        61 | 62 => "minecraft:furnace".to_string(),
        63 => "minecraft:oak_sign".to_string(),
        64 => "minecraft:oak_door".to_string(),
        65 => "minecraft:ladder".to_string(),
        66 => "minecraft:rail".to_string(),
        67 => "minecraft:cobblestone_stairs".to_string(),
        69 => "minecraft:lever".to_string(),
        70 => "minecraft:stone_pressure_plate".to_string(),
        72 => "minecraft:oak_pressure_plate".to_string(),
        73 | 74 => "minecraft:redstone_ore".to_string(),
        75 | 76 => "minecraft:redstone_torch".to_string(),
        77 => "minecraft:stone_button".to_string(),
        79 => "minecraft:ice".to_string(),
        80 => "minecraft:snow_block".to_string(),
        81 => "minecraft:cactus".to_string(),
        82 => "minecraft:clay".to_string(),
        84 => "minecraft:jukebox".to_string(),
        85 => "minecraft:oak_fence".to_string(),
        86 => "minecraft:pumpkin".to_string(),
        87 => "minecraft:netherrack".to_string(),
        88 => "minecraft:soul_sand".to_string(),
        89 => "minecraft:glowstone".to_string(),
        90 => "minecraft:nether_portal".to_string(),
        91 => "minecraft:jack_o_lantern".to_string(),
        93 | 94 => "minecraft:repeater".to_string(),
        95 => match data {
            0 => "minecraft:white_stained_glass",
            1 => "minecraft:orange_stained_glass",
            2 => "minecraft:magenta_stained_glass",
            3 => "minecraft:light_blue_stained_glass",
            4 => "minecraft:yellow_stained_glass",
            5 => "minecraft:lime_stained_glass",
            6 => "minecraft:pink_stained_glass",
            7 => "minecraft:gray_stained_glass",
            8 => "minecraft:light_gray_stained_glass",
            9 => "minecraft:cyan_stained_glass",
            10 => "minecraft:purple_stained_glass",
            11 => "minecraft:blue_stained_glass",
            12 => "minecraft:brown_stained_glass",
            13 => "minecraft:green_stained_glass",
            14 => "minecraft:red_stained_glass",
            15 => "minecraft:black_stained_glass",
            _ => "minecraft:white_stained_glass",
        }.to_string(),
        98 => match data {
            0 => "minecraft:stone_bricks",
            1 => "minecraft:mossy_stone_bricks",
            2 => "minecraft:cracked_stone_bricks",
            3 => "minecraft:chiseled_stone_bricks",
            _ => "minecraft:stone_bricks",
        }.to_string(),
        109 => "minecraft:stone_brick_stairs".to_string(),
        110 => "minecraft:mycelium".to_string(),
        112 => "minecraft:nether_bricks".to_string(),
        121 => "minecraft:end_stone".to_string(),
        123 | 124 => "minecraft:redstone_lamp".to_string(),
        125 => match data {
            0 => "minecraft:oak_slab",
            1 => "minecraft:spruce_slab",
            2 => "minecraft:birch_slab",
            3 => "minecraft:jungle_slab",
            4 => "minecraft:acacia_slab",
            5 => "minecraft:dark_oak_slab",
            _ => "minecraft:oak_slab",
        }.to_string(),
        126 => "minecraft:oak_slab".to_string(), // double slab
        129 => "minecraft:emerald_ore".to_string(),
        130 => "minecraft:ender_chest".to_string(),
        131 => "minecraft:tripwire_hook".to_string(),
        133 => "minecraft:emerald_block".to_string(),
        134 => "minecraft:spruce_stairs".to_string(),
        135 => "minecraft:birch_stairs".to_string(),
        136 => "minecraft:jungle_stairs".to_string(),
        137 => "minecraft:command_block".to_string(),
        138 => "minecraft:beacon".to_string(),
        139 => "minecraft:cobblestone_wall".to_string(),
        143 => "minecraft:oak_button".to_string(),
        145 => "minecraft:anvil".to_string(),
        146 => "minecraft:trapped_chest".to_string(),
        147 => "minecraft:light_weighted_pressure_plate".to_string(),
        148 => "minecraft:heavy_weighted_pressure_plate".to_string(),
        149 | 150 => "minecraft:comparator".to_string(),
        151 | 178 => "minecraft:daylight_detector".to_string(),
        152 => "minecraft:redstone_block".to_string(),
        153 => "minecraft:nether_quartz_ore".to_string(),
        154 => "minecraft:hopper".to_string(),
        155 => "minecraft:quartz_block".to_string(),
        156 => "minecraft:quartz_stairs".to_string(),
        157 => "minecraft:activator_rail".to_string(),
        158 => "minecraft:dropper".to_string(),
        159 => match data {
            0 => "minecraft:white_terracotta",
            1 => "minecraft:orange_terracotta",
            2 => "minecraft:magenta_terracotta",
            3 => "minecraft:light_blue_terracotta",
            4 => "minecraft:yellow_terracotta",
            5 => "minecraft:lime_terracotta",
            6 => "minecraft:pink_terracotta",
            7 => "minecraft:gray_terracotta",
            8 => "minecraft:light_gray_terracotta",
            9 => "minecraft:cyan_terracotta",
            10 => "minecraft:purple_terracotta",
            11 => "minecraft:blue_terracotta",
            12 => "minecraft:brown_terracotta",
            13 => "minecraft:green_terracotta",
            14 => "minecraft:red_terracotta",
            15 => "minecraft:black_terracotta",
            _ => "minecraft:white_terracotta",
        }.to_string(),
        160 => "minecraft:white_stained_glass_pane".to_string(), // simplified
        165 => "minecraft:slime_block".to_string(),
        166 => "minecraft:barrier".to_string(),
        169 => "minecraft:sea_lantern".to_string(),
        170 => "minecraft:hay_block".to_string(),
        172 => "minecraft:terracotta".to_string(),
        173 => "minecraft:coal_block".to_string(),
        174 => "minecraft:packed_ice".to_string(),
        179 => "minecraft:red_sandstone".to_string(),
        180 => "minecraft:red_sandstone_stairs".to_string(),
        183 => "minecraft:spruce_fence_gate".to_string(),
        184 => "minecraft:birch_fence_gate".to_string(),
        185 => "minecraft:jungle_fence_gate".to_string(),
        186 => "minecraft:dark_oak_fence_gate".to_string(),
        187 => "minecraft:acacia_fence_gate".to_string(),
        188 => "minecraft:spruce_fence".to_string(),
        189 => "minecraft:birch_fence".to_string(),
        190 => "minecraft:jungle_fence".to_string(),
        191 => "minecraft:dark_oak_fence".to_string(),
        192 => "minecraft:acacia_fence".to_string(),
        198 => "minecraft:end_rod".to_string(),
        199 => "minecraft:chorus_plant".to_string(),
        200 => "minecraft:chorus_flower".to_string(),
        201 => "minecraft:purpur_block".to_string(),
        202 => "minecraft:purpur_pillar".to_string(),
        203 => "minecraft:purpur_stairs".to_string(),
        206 => "minecraft:end_stone_bricks".to_string(),
        210 => "minecraft:repeating_command_block".to_string(),
        211 => "minecraft:chain_command_block".to_string(),
        213 => "minecraft:magma_block".to_string(),
        214 => "minecraft:nether_wart_block".to_string(),
        215 => "minecraft:red_nether_bricks".to_string(),
        216 => "minecraft:bone_block".to_string(),
        218 => "minecraft:observer".to_string(),
        219..=234 => {
            let colors = [
                "white", "orange", "magenta", "light_blue",
                "yellow", "lime", "pink", "gray",
                "light_gray", "cyan", "purple", "blue",
                "brown", "green", "red", "black",
            ];
            let color_idx = (id - 219) as usize;
            format!("minecraft:{}_shulker_box", colors.get(color_idx).unwrap_or(&"white"))
        }
        235..=250 => {
            let colors = [
                "white", "orange", "magenta", "light_blue",
                "yellow", "lime", "pink", "gray",
                "light_gray", "cyan", "purple", "blue",
                "brown", "green", "red", "black",
            ];
            let color_idx = (id - 235) as usize;
            format!("minecraft:{}_glazed_terracotta", colors.get(color_idx).unwrap_or(&"white"))
        }
        251 => match data {
            0 => "minecraft:white_concrete",
            1 => "minecraft:orange_concrete",
            2 => "minecraft:magenta_concrete",
            3 => "minecraft:light_blue_concrete",
            4 => "minecraft:yellow_concrete",
            5 => "minecraft:lime_concrete",
            6 => "minecraft:pink_concrete",
            7 => "minecraft:gray_concrete",
            8 => "minecraft:light_gray_concrete",
            9 => "minecraft:cyan_concrete",
            10 => "minecraft:purple_concrete",
            11 => "minecraft:blue_concrete",
            12 => "minecraft:brown_concrete",
            13 => "minecraft:green_concrete",
            14 => "minecraft:red_concrete",
            15 => "minecraft:black_concrete",
            _ => "minecraft:white_concrete",
        }.to_string(),
        252 => match data {
            0 => "minecraft:white_concrete_powder",
            1 => "minecraft:orange_concrete_powder",
            2 => "minecraft:magenta_concrete_powder",
            3 => "minecraft:light_blue_concrete_powder",
            4 => "minecraft:yellow_concrete_powder",
            5 => "minecraft:lime_concrete_powder",
            6 => "minecraft:pink_concrete_powder",
            7 => "minecraft:gray_concrete_powder",
            8 => "minecraft:light_gray_concrete_powder",
            9 => "minecraft:cyan_concrete_powder",
            10 => "minecraft:purple_concrete_powder",
            11 => "minecraft:blue_concrete_powder",
            12 => "minecraft:brown_concrete_powder",
            13 => "minecraft:green_concrete_powder",
            14 => "minecraft:red_concrete_powder",
            15 => "minecraft:black_concrete_powder",
            _ => "minecraft:white_concrete_powder",
        }.to_string(),
        _ => format!("minecraft:unknown_block_{}", id),
    }
}

/// Convert legacy data value to block state properties
pub fn legacy_data_to_state(id: u8, data: u8) -> BlockState {
    let mut props = HashMap::new();

    match id {
        // Logs - axis from upper bits
        17 | 162 => {
            let axis = match (data >> 2) & 0x3 {
                0 => "y",
                1 => "x",
                2 => "z",
                _ => "y",
            };
            props.insert("axis".to_string(), axis.to_string());
        }
        // Stairs - facing and half
        53 | 67 | 108 | 109 | 114 | 128 | 134 | 135 | 136 | 156 | 163 | 164 | 180 | 203 => {
            let facing = match data & 0x3 {
                0 => "east",
                1 => "west",
                2 => "south",
                3 => "north",
                _ => "east",
            };
            props.insert("facing".to_string(), facing.to_string());
            props.insert("half".to_string(), if data & 0x4 != 0 { "top" } else { "bottom" }.to_string());
        }
        // Torches - facing
        50 | 75 | 76 => {
            let facing = match data {
                1 => "east",
                2 => "west",
                3 => "south",
                4 => "north",
                5 => "up",
                _ => "up",
            };
            if data != 5 {
                props.insert("facing".to_string(), facing.to_string());
            }
        }
        // Levers
        69 => {
            let face = match data & 0x7 {
                0 | 7 => "ceiling",
                1..=4 => "wall",
                5 | 6 => "floor",
                _ => "wall",
            };
            props.insert("face".to_string(), face.to_string());
            props.insert("powered".to_string(), if data & 0x8 != 0 { "true" } else { "false" }.to_string());
        }
        // Buttons
        77 | 143 => {
            let face = match data & 0x7 {
                0 => "ceiling",
                1..=4 => "wall",
                5 => "floor",
                _ => "wall",
            };
            props.insert("face".to_string(), face.to_string());
            props.insert("powered".to_string(), if data & 0x8 != 0 { "true" } else { "false" }.to_string());
        }
        // Repeaters
        93 | 94 => {
            let facing = match data & 0x3 {
                0 => "south",
                1 => "west",
                2 => "north",
                3 => "east",
                _ => "south",
            };
            let delay = ((data >> 2) & 0x3) + 1;
            props.insert("facing".to_string(), facing.to_string());
            props.insert("delay".to_string(), delay.to_string());
            props.insert("powered".to_string(), if id == 94 { "true" } else { "false" }.to_string());
        }
        // Comparators
        149 | 150 => {
            let facing = match data & 0x3 {
                0 => "south",
                1 => "west",
                2 => "north",
                3 => "east",
                _ => "south",
            };
            props.insert("facing".to_string(), facing.to_string());
            props.insert("mode".to_string(), if data & 0x4 != 0 { "subtract" } else { "compare" }.to_string());
            props.insert("powered".to_string(), if data & 0x8 != 0 { "true" } else { "false" }.to_string());
        }
        // Pistons
        29 | 33 => {
            let facing = match data & 0x7 {
                0 => "down",
                1 => "up",
                2 => "north",
                3 => "south",
                4 => "west",
                5 => "east",
                _ => "up",
            };
            props.insert("facing".to_string(), facing.to_string());
            props.insert("extended".to_string(), if data & 0x8 != 0 { "true" } else { "false" }.to_string());
        }
        // Dispensers/droppers/observers
        23 | 158 | 218 => {
            let facing = match data & 0x7 {
                0 => "down",
                1 => "up",
                2 => "north",
                3 => "south",
                4 => "west",
                5 => "east",
                _ => "north",
            };
            props.insert("facing".to_string(), facing.to_string());
            if id == 23 || id == 158 {
                props.insert("triggered".to_string(), if data & 0x8 != 0 { "true" } else { "false" }.to_string());
            }
        }
        // Hoppers
        154 => {
            let facing = match data & 0x7 {
                0 => "down",
                2 => "north",
                3 => "south",
                4 => "west",
                5 => "east",
                _ => "down",
            };
            props.insert("facing".to_string(), facing.to_string());
            props.insert("enabled".to_string(), if data & 0x8 == 0 { "true" } else { "false" }.to_string());
        }
        // Redstone wire
        55 => {
            props.insert("power".to_string(), (data & 0xF).to_string());
        }
        // Rails
        66 => {
            let shape = match data {
                0 => "north_south",
                1 => "east_west",
                2 => "ascending_east",
                3 => "ascending_west",
                4 => "ascending_north",
                5 => "ascending_south",
                6 => "south_east",
                7 => "south_west",
                8 => "north_west",
                9 => "north_east",
                _ => "north_south",
            };
            props.insert("shape".to_string(), shape.to_string());
        }
        _ => {}
    }

    BlockState { properties: props }
}
