use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::{nadk::display::Color565, physic::BoundingBox, renderer::mesh::QuadDir};

pub mod rendering {
    pub const SCREEN_WIDTH: usize = 320;
    pub const SCREEN_HEIGHT: usize = 240;

    pub const SCREEN_TILE_SUBDIVISION: usize = 4; // Minimum 2

    pub const MIN_FOV: f32 = 30.;
    pub const MAX_FOV: f32 = 110.;

    pub const FOV: f32 = 45.;

    #[cfg(feature = "epsilon")]
    pub const MAX_TRIANGLES: usize = 1500;
    #[cfg(feature = "upsilon")]
    pub const MAX_TRIANGLES: usize = 1200; // Sorry Upsilon users

    pub const MAX_RENDER_DISTANCE: usize = 2; // You shouldn't go higher

    pub const BLURING_SCREEN_SUBDIVISION: usize = 5;
    pub const BLURING_RADIUS: isize = 2;

    pub const MAX_ENTITY_RENDER_DISTANCE: f32 = 10.;

    pub const ITEM_ENTITY_SPRITE_SIZE: f32 = 0.8;
}

pub mod color_palette {
    use crate::nadk::display::Color565;

    pub const MENU_OUTLINE_COLOR: Color565 = Color565::from_rgb888(150, 150, 150);
    pub const MENU_ELEMENT_BACKGROUND_COLOR: Color565 = Color565::from_rgb888(230, 230, 230);
    pub const MENU_ELEMENT_BACKGROUND_COLOR_HOVER: Color565 = Color565::from_rgb888(190, 190, 190);
    pub const MENU_TEXT_COLOR: Color565 = Color565::from_rgb888(0, 0, 0);
    pub const MENU_BACKGROUND_COLOR: Color565 = Color565::from_rgb888(255, 255, 255);

    pub const GAMEUI_SLOT_COLOR: Color565 = Color565::from_rgb888(80, 80, 80);
    pub const GAMEUI_SLOT_DEFAULT_OUTLINE_COLOR: Color565 = Color565::from_rgb888(120, 120, 120);
}

pub mod save_manager {
    pub const SETTINGS_FILENAME: &str = "settings.ncd"; // NCD = NumCraftData

    pub const WORLD_VERSION: u16 = 0; // Update the version at each world breaking update
}

pub mod world {
    pub const CHUNK_SIZE: usize = 8; // MAX 8

    pub const MAX_ITEM_MERGING_DISTANCE: f32 = 2.;
    pub const ITEM_MAGNET_FORCE: f32 = 10.;
    pub const MAX_PLAYER_ITEM_MAGNET_DISTANCE: f32 = 2.2;

    pub const TNT_EXPLOSION_RADIUS: isize = 2;
}

pub mod player {
    use core::f32::consts::PI;

    pub const ROTATION_SPEED: f32 = PI / 3.0; // rad / sec
    pub const FLY_SPEED: f32 = 4.0;
    pub const WALK_FORCE: f32 = 20.0;
    pub const MAX_WALKING_VELOCITY: f32 = 4.;
    pub const JUMP_FORCE: f32 = 5.;
}

pub mod physic {
    use nalgebra::Vector3;

    pub const GRAVITY_FACTOR: f32 = 10.0;
    pub const MAX_FALLING_VELOCITY: f32 = 5.;
    pub const ON_FLOOR_FRICTION: f32 = 10.;

    pub const BLOCK_COLLISION_SCANNING_SIZE: Vector3<isize> = Vector3::new(2, 3, 2);
}

#[allow(unreachable_patterns)]
impl EntityType {
    pub fn get_bbox(&self) -> Option<BoundingBox> {
        match self {
            EntityType::Player => Some(BoundingBox {
                offset: Vector3::new(-0.4, -0.5, -0.4),
                size: Vector3::new(0.8, 1.8, 0.8),
            }),
            EntityType::Item => Some(BoundingBox {
                offset: Vector3::new(-0.2, -0.2, -0.2),
                size: Vector3::new(0.4, 0.4, 0.4),
            }),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Player = 0,
    Item = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    Sand = 4,
    Cobblestone = 5,
    Border = 6,
    Log = 7,
    Leaves = 8,
    Planks = 9,
    // Decorative colored blocks (creative mode only for now)
    Red = 10,
    Orange = 11,
    Yellow = 12,
    Lime = 13,
    Cyan = 14,
    Blue = 15,
    Purple = 16,
    Magenta = 17,
    Pink = 18,
    White = 19,
    Gray = 20,
    Black = 21,
    Tnt = 22,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum ItemType {
    Air = 0,

    StoneBlock = 1,
    GrassBlock = 2,
    DirtBlock = 3,
    SandBlock = 4,
    CobblestoneBlock = 5,
    BorderBlock = 6,
    LogBlock = 7,
    LeavesBlock = 8,
    PlanksBlock = 9,
    // Decorative colored blocks (creative mode only for now)
    RedBlock = 10,
    OrangeBlock = 11,
    YellowBlock = 12,
    LimeBlock = 13,
    CyanBlock = 14,
    BlueBlock = 15,
    PurpleBlock = 16,
    MagentaBlock = 17,
    PinkBlock = 18,
    WhiteBlock = 19,
    GrayBlock = 20,
    BlackBlock = 21,
    TntBlock = 22,
    // Not a block: used to ignite Tnt when used on it, never placed itself
    FlintAndSteel = 23,
}

impl ItemType {
    pub fn get_texture_id(&self) -> u8 {
        match *self {
            ItemType::Air => 0,

            ItemType::StoneBlock => 1,
            ItemType::GrassBlock => 2,
            ItemType::DirtBlock => 3, // 4 is the other texture of the grass block
            ItemType::SandBlock => 5,
            ItemType::CobblestoneBlock => 6,
            ItemType::BorderBlock => 7,
            ItemType::LogBlock => 8,
            ItemType::LeavesBlock => 9,
            ItemType::PlanksBlock => 10,

            ItemType::RedBlock => 11,
            ItemType::OrangeBlock => 12,
            ItemType::YellowBlock => 13,
            ItemType::LimeBlock => 14,
            ItemType::CyanBlock => 15,
            ItemType::BlueBlock => 16,
            ItemType::PurpleBlock => 17,
            ItemType::MagentaBlock => 18,
            ItemType::PinkBlock => 19,
            ItemType::WhiteBlock => 20,
            ItemType::GrayBlock => 21,
            ItemType::BlackBlock => 22,
            ItemType::TntBlock => 23,
            ItemType::FlintAndSteel => 24,
        }
    }

    pub const fn get_from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(ItemType::Air),

            1 => Some(ItemType::StoneBlock),
            2 => Some(ItemType::GrassBlock),
            3 => Some(ItemType::DirtBlock),
            4 => Some(ItemType::SandBlock),
            5 => Some(ItemType::CobblestoneBlock),
            6 => Some(ItemType::BorderBlock),
            7 => Some(ItemType::LogBlock),
            8 => Some(ItemType::LeavesBlock),
            9 => Some(ItemType::PlanksBlock),

            10 => Some(ItemType::RedBlock),
            11 => Some(ItemType::OrangeBlock),
            12 => Some(ItemType::YellowBlock),
            13 => Some(ItemType::LimeBlock),
            14 => Some(ItemType::CyanBlock),
            15 => Some(ItemType::BlueBlock),
            16 => Some(ItemType::PurpleBlock),
            17 => Some(ItemType::MagentaBlock),
            18 => Some(ItemType::PinkBlock),
            19 => Some(ItemType::WhiteBlock),
            20 => Some(ItemType::GrayBlock),
            21 => Some(ItemType::BlackBlock),
            22 => Some(ItemType::TntBlock),
            23 => Some(ItemType::FlintAndSteel),
            _ => None,
        }
    }

    pub fn get_max_stack_amount(&self) -> u8 {
        match *self {
            ItemType::Air => 0,
            ItemType::StoneBlock => 64,
            ItemType::GrassBlock => 64,
            ItemType::DirtBlock => 64,
            ItemType::SandBlock => 64,
            ItemType::CobblestoneBlock => 64,
            ItemType::BorderBlock => 64,
            ItemType::LogBlock => 64,
            ItemType::LeavesBlock => 64,
            ItemType::PlanksBlock => 64,

            ItemType::RedBlock => 64,
            ItemType::OrangeBlock => 64,
            ItemType::YellowBlock => 64,
            ItemType::LimeBlock => 64,
            ItemType::CyanBlock => 64,
            ItemType::BlueBlock => 64,
            ItemType::PurpleBlock => 64,
            ItemType::MagentaBlock => 64,
            ItemType::PinkBlock => 64,
            ItemType::WhiteBlock => 64,
            ItemType::GrayBlock => 64,
            ItemType::BlackBlock => 64,
            ItemType::TntBlock => 64,
            // A tool, not a stack of blocks: doesn't stack.
            ItemType::FlintAndSteel => 1,
        }
    }

    pub fn get_matching_block_type(&self) -> Option<BlockType> {
        match self {
            ItemType::Air => None,
            ItemType::StoneBlock => Some(BlockType::Stone),
            ItemType::GrassBlock => Some(BlockType::Grass),
            ItemType::DirtBlock => Some(BlockType::Dirt),
            ItemType::SandBlock => Some(BlockType::Sand),
            ItemType::CobblestoneBlock => Some(BlockType::Cobblestone),
            ItemType::BorderBlock => Some(BlockType::Border),
            ItemType::LogBlock => Some(BlockType::Log),
            ItemType::LeavesBlock => Some(BlockType::Leaves),
            ItemType::PlanksBlock => Some(BlockType::Planks),

            ItemType::RedBlock => Some(BlockType::Red),
            ItemType::OrangeBlock => Some(BlockType::Orange),
            ItemType::YellowBlock => Some(BlockType::Yellow),
            ItemType::LimeBlock => Some(BlockType::Lime),
            ItemType::CyanBlock => Some(BlockType::Cyan),
            ItemType::BlueBlock => Some(BlockType::Blue),
            ItemType::PurpleBlock => Some(BlockType::Purple),
            ItemType::MagentaBlock => Some(BlockType::Magenta),
            ItemType::PinkBlock => Some(BlockType::Pink),
            ItemType::WhiteBlock => Some(BlockType::White),
            ItemType::GrayBlock => Some(BlockType::Gray),
            ItemType::BlackBlock => Some(BlockType::Black),
            ItemType::TntBlock => Some(BlockType::Tnt),
            // Not placeable on its own - using it does something only when aimed at Tnt.
            ItemType::FlintAndSteel => None,
        }
    }
}

impl BlockType {
    pub fn is_air(&self) -> bool {
        *self == BlockType::Air
    }

    pub fn get_texture_id(&self, dir: QuadDir) -> u8 {
        match *self {
            BlockType::Air => 0,
            BlockType::Stone => 1,
            BlockType::Grass => {
                if dir == QuadDir::Top {
                    2
                } else {
                    3
                }
            }
            BlockType::Dirt => 3,
            BlockType::Sand => 5,
            BlockType::Cobblestone => 6,
            BlockType::Border => 7,
            BlockType::Log => 8,
            BlockType::Leaves => 9,
            BlockType::Planks => 10,

            BlockType::Red => 11,
            BlockType::Orange => 12,
            BlockType::Yellow => 13,
            BlockType::Lime => 14,
            BlockType::Cyan => 15,
            BlockType::Blue => 16,
            BlockType::Purple => 17,
            BlockType::Magenta => 18,
            BlockType::Pink => 19,
            BlockType::White => 20,
            BlockType::Gray => 21,
            BlockType::Black => 22,
            BlockType::Tnt => 23,
        }
    }

    pub const fn get_from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(BlockType::Air),
            1 => Some(BlockType::Stone),
            2 => Some(BlockType::Grass),
            3 => Some(BlockType::Dirt),
            4 => Some(BlockType::Sand),
            5 => Some(BlockType::Cobblestone),
            6 => Some(BlockType::Border),
            7 => Some(BlockType::Log),
            8 => Some(BlockType::Leaves),
            9 => Some(BlockType::Planks),

            10 => Some(BlockType::Red),
            11 => Some(BlockType::Orange),
            12 => Some(BlockType::Yellow),
            13 => Some(BlockType::Lime),
            14 => Some(BlockType::Cyan),
            15 => Some(BlockType::Blue),
            16 => Some(BlockType::Purple),
            17 => Some(BlockType::Magenta),
            18 => Some(BlockType::Pink),
            19 => Some(BlockType::White),
            20 => Some(BlockType::Gray),
            21 => Some(BlockType::Black),
            22 => Some(BlockType::Tnt),
            _ => None,
        }
    }

    pub const fn get_hardness(&self) -> f32 {
        match self {
            BlockType::Air => 0.,
            BlockType::Stone => 2.,
            BlockType::Grass => 1.2,
            BlockType::Dirt => 1.,
            BlockType::Sand => 1.,
            BlockType::Cobblestone => 2.2,
            BlockType::Border => -1.,
            BlockType::Log => 1.5,
            BlockType::Leaves => 0.3,
            BlockType::Planks => 1.2,

            // Decorative colored blocks: same softness as planks
            BlockType::Red => 0.8,
            BlockType::Orange => 0.8,
            BlockType::Yellow => 0.8,
            BlockType::Lime => 0.8,
            BlockType::Cyan => 0.8,
            BlockType::Blue => 0.8,
            BlockType::Purple => 0.8,
            BlockType::Magenta => 0.8,
            BlockType::Pink => 0.8,
            BlockType::White => 0.8,
            BlockType::Gray => 0.8,
            BlockType::Black => 0.8,
            BlockType::Tnt => 0.5,
        }
    }

    pub const fn get_dropped_item_type(&self) -> ItemType {
        match self {
            BlockType::Air => ItemType::Air,
            BlockType::Stone => ItemType::CobblestoneBlock,
            BlockType::Grass => ItemType::DirtBlock,
            BlockType::Dirt => ItemType::DirtBlock,
            BlockType::Sand => ItemType::SandBlock,
            BlockType::Cobblestone => ItemType::CobblestoneBlock,
            BlockType::Border => ItemType::BorderBlock,
            BlockType::Log => ItemType::LogBlock,
            BlockType::Leaves => ItemType::LeavesBlock,
            BlockType::Planks => ItemType::PlanksBlock,

            BlockType::Red => ItemType::RedBlock,
            BlockType::Orange => ItemType::OrangeBlock,
            BlockType::Yellow => ItemType::YellowBlock,
            BlockType::Lime => ItemType::LimeBlock,
            BlockType::Cyan => ItemType::CyanBlock,
            BlockType::Blue => ItemType::BlueBlock,
            BlockType::Purple => ItemType::PurpleBlock,
            BlockType::Magenta => ItemType::MagentaBlock,
            BlockType::Pink => ItemType::PinkBlock,
            BlockType::White => ItemType::WhiteBlock,
            BlockType::Gray => ItemType::GrayBlock,
            BlockType::Black => ItemType::BlackBlock,
            BlockType::Tnt => ItemType::TntBlock,
        }
    }
}

pub fn get_quad_color_from_texture_id(id: u8) -> Color565 {
    match id {
        1 => Color565::from_rgb888(160, 160, 160),
        2 => Color565::from_rgb888(21, 147, 0),
        3 => Color565::from_rgb888(120, 77, 49),
        5 => Color565::from_rgb888(208, 199, 6),
        6 => Color565::from_rgb888(178, 178, 178),
        7 => Color565::from_rgb888(19, 19, 19),
        8 => Color565::from_rgb888(79, 53, 30),
        9 => Color565::from_rgb888(36, 75, 37),
        10 => Color565::from_rgb888(152, 124, 61),

        11 => Color565::from_rgb888(200, 40, 40),   // Red
        12 => Color565::from_rgb888(230, 130, 30),  // Orange
        13 => Color565::from_rgb888(230, 210, 40),  // Yellow
        14 => Color565::from_rgb888(90, 200, 60),   // Lime
        15 => Color565::from_rgb888(40, 190, 190),  // Cyan
        16 => Color565::from_rgb888(40, 90, 220),   // Blue
        17 => Color565::from_rgb888(140, 60, 200),  // Purple
        18 => Color565::from_rgb888(210, 60, 180),  // Magenta
        19 => Color565::from_rgb888(240, 160, 190), // Pink
        20 => Color565::from_rgb888(235, 235, 235), // White
        21 => Color565::from_rgb888(100, 100, 100), // Gray
        22 => Color565::from_rgb888(25, 25, 25),    // Black

        23 => Color565::from_rgb888(198, 82, 28), // Tnt

        _ => Color565::from_rgb888(0, 0, 0),
        // 255 is reserved for block outline
    }
}
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::{nadk::display::Color565, physic::BoundingBox, renderer::mesh::QuadDir};

pub mod rendering {
    pub const SCREEN_WIDTH: usize = 320;
    pub const SCREEN_HEIGHT: usize = 240;

    pub const SCREEN_TILE_SUBDIVISION: usize = 4; // Minimum 2

    pub const MIN_FOV: f32 = 30.;
    pub const MAX_FOV: f32 = 110.;

    pub const FOV: f32 = 45.;

    #[cfg(feature = "epsilon")]
    pub const MAX_TRIANGLES: usize = 1500;
    #[cfg(feature = "upsilon")]
    pub const MAX_TRIANGLES: usize = 1200; // Sorry Upsilon users

    pub const MAX_RENDER_DISTANCE: usize = 2; // You shouldn't go higher

    pub const BLURING_SCREEN_SUBDIVISION: usize = 5;
    pub const BLURING_RADIUS: isize = 2;

    pub const MAX_ENTITY_RENDER_DISTANCE: f32 = 10.;

    pub const ITEM_ENTITY_SPRITE_SIZE: f32 = 0.8;
}

pub mod color_palette {
    use crate::nadk::display::Color565;

    pub const MENU_OUTLINE_COLOR: Color565 = Color565::from_rgb888(150, 150, 150);
    pub const MENU_ELEMENT_BACKGROUND_COLOR: Color565 = Color565::from_rgb888(230, 230, 230);
    pub const MENU_ELEMENT_BACKGROUND_COLOR_HOVER: Color565 = Color565::from_rgb888(190, 190, 190);
    pub const MENU_TEXT_COLOR: Color565 = Color565::from_rgb888(0, 0, 0);
    pub const MENU_BACKGROUND_COLOR: Color565 = Color565::from_rgb888(255, 255, 255);

    pub const GAMEUI_SLOT_COLOR: Color565 = Color565::from_rgb888(80, 80, 80);
    pub const GAMEUI_SLOT_DEFAULT_OUTLINE_COLOR: Color565 = Color565::from_rgb888(120, 120, 120);
}

pub mod save_manager {
    pub const SETTINGS_FILENAME: &str = "settings.ncd"; // NCD = NumCraftData

    pub const WORLD_VERSION: u16 = 0; // Update the version at each world breaking update
}

pub mod world {
    pub const CHUNK_SIZE: usize = 8; // MAX 8

    pub const MAX_ITEM_MERGING_DISTANCE: f32 = 2.;
    pub const ITEM_MAGNET_FORCE: f32 = 10.;
    pub const MAX_PLAYER_ITEM_MAGNET_DISTANCE: f32 = 2.2;
}

pub mod player {
    use core::f32::consts::PI;

    pub const ROTATION_SPEED: f32 = PI / 3.0; // rad / sec
    pub const FLY_SPEED: f32 = 4.0;
    pub const WALK_FORCE: f32 = 20.0;
    pub const MAX_WALKING_VELOCITY: f32 = 4.;
    pub const JUMP_FORCE: f32 = 5.;
}

pub mod physic {
    use nalgebra::Vector3;

    pub const GRAVITY_FACTOR: f32 = 10.0;
    pub const MAX_FALLING_VELOCITY: f32 = 5.;
    pub const ON_FLOOR_FRICTION: f32 = 10.;

    pub const BLOCK_COLLISION_SCANNING_SIZE: Vector3<isize> = Vector3::new(2, 3, 2);
}

#[allow(unreachable_patterns)]
impl EntityType {
    pub fn get_bbox(&self) -> Option<BoundingBox> {
        match self {
            EntityType::Player => Some(BoundingBox {
                offset: Vector3::new(-0.4, -0.5, -0.4),
                size: Vector3::new(0.8, 1.8, 0.8),
            }),
            EntityType::Item => Some(BoundingBox {
                offset: Vector3::new(-0.2, -0.2, -0.2),
                size: Vector3::new(0.4, 0.4, 0.4),
            }),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Player = 0,
    Item = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    Sand = 4,
    Cobblestone = 5,
    Border = 6,
    Log = 7,
    Leaves = 8,
    Planks = 9,
    // Decorative colored blocks (creative mode only for now)
    Red = 10,
    Orange = 11,
    Yellow = 12,
    Lime = 13,
    Cyan = 14,
    Blue = 15,
    Purple = 16,
    Magenta = 17,
    Pink = 18,
    White = 19,
    Gray = 20,
    Black = 21,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum ItemType {
    Air = 0,

    StoneBlock = 1,
    GrassBlock = 2,
    DirtBlock = 3,
    SandBlock = 4,
    CobblestoneBlock = 5,
    BorderBlock = 6,
    LogBlock = 7,
    LeavesBlock = 8,
    PlanksBlock = 9,
    // Decorative colored blocks (creative mode only for now)
    RedBlock = 10,
    OrangeBlock = 11,
    YellowBlock = 12,
    LimeBlock = 13,
    CyanBlock = 14,
    BlueBlock = 15,
    PurpleBlock = 16,
    MagentaBlock = 17,
    PinkBlock = 18,
    WhiteBlock = 19,
    GrayBlock = 20,
    BlackBlock = 21,
}

impl ItemType {
    pub fn get_texture_id(&self) -> u8 {
        match *self {
            ItemType::Air => 0,

            ItemType::StoneBlock => 1,
            ItemType::GrassBlock => 2,
            ItemType::DirtBlock => 3, // 4 is the other texture of the grass block
            ItemType::SandBlock => 5,
            ItemType::CobblestoneBlock => 6,
            ItemType::BorderBlock => 7,
            ItemType::LogBlock => 8,
            ItemType::LeavesBlock => 9,
            ItemType::PlanksBlock => 10,

            ItemType::RedBlock => 11,
            ItemType::OrangeBlock => 12,
            ItemType::YellowBlock => 13,
            ItemType::LimeBlock => 14,
            ItemType::CyanBlock => 15,
            ItemType::BlueBlock => 16,
            ItemType::PurpleBlock => 17,
            ItemType::MagentaBlock => 18,
            ItemType::PinkBlock => 19,
            ItemType::WhiteBlock => 20,
            ItemType::GrayBlock => 21,
            ItemType::BlackBlock => 22,
        }
    }

    pub const fn get_from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(ItemType::Air),

            1 => Some(ItemType::StoneBlock),
            2 => Some(ItemType::GrassBlock),
            3 => Some(ItemType::DirtBlock),
            4 => Some(ItemType::SandBlock),
            5 => Some(ItemType::CobblestoneBlock),
            6 => Some(ItemType::BorderBlock),
            7 => Some(ItemType::LogBlock),
            8 => Some(ItemType::LeavesBlock),
            9 => Some(ItemType::PlanksBlock),

            10 => Some(ItemType::RedBlock),
            11 => Some(ItemType::OrangeBlock),
            12 => Some(ItemType::YellowBlock),
            13 => Some(ItemType::LimeBlock),
            14 => Some(ItemType::CyanBlock),
            15 => Some(ItemType::BlueBlock),
            16 => Some(ItemType::PurpleBlock),
            17 => Some(ItemType::MagentaBlock),
            18 => Some(ItemType::PinkBlock),
            19 => Some(ItemType::WhiteBlock),
            20 => Some(ItemType::GrayBlock),
            21 => Some(ItemType::BlackBlock),
            _ => None,
        }
    }

    pub fn get_max_stack_amount(&self) -> u8 {
        match *self {
            ItemType::Air => 0,
            ItemType::StoneBlock => 64,
            ItemType::GrassBlock => 64,
            ItemType::DirtBlock => 64,
            ItemType::SandBlock => 64,
            ItemType::CobblestoneBlock => 64,
            ItemType::BorderBlock => 64,
            ItemType::LogBlock => 64,
            ItemType::LeavesBlock => 64,
            ItemType::PlanksBlock => 64,

            ItemType::RedBlock => 64,
            ItemType::OrangeBlock => 64,
            ItemType::YellowBlock => 64,
            ItemType::LimeBlock => 64,
            ItemType::CyanBlock => 64,
            ItemType::BlueBlock => 64,
            ItemType::PurpleBlock => 64,
            ItemType::MagentaBlock => 64,
            ItemType::PinkBlock => 64,
            ItemType::WhiteBlock => 64,
            ItemType::GrayBlock => 64,
            ItemType::BlackBlock => 64,
        }
    }

    pub fn get_matching_block_type(&self) -> Option<BlockType> {
        match self {
            ItemType::Air => None,
            ItemType::StoneBlock => Some(BlockType::Stone),
            ItemType::GrassBlock => Some(BlockType::Grass),
            ItemType::DirtBlock => Some(BlockType::Dirt),
            ItemType::SandBlock => Some(BlockType::Sand),
            ItemType::CobblestoneBlock => Some(BlockType::Cobblestone),
            ItemType::BorderBlock => Some(BlockType::Border),
            ItemType::LogBlock => Some(BlockType::Log),
            ItemType::LeavesBlock => Some(BlockType::Leaves),
            ItemType::PlanksBlock => Some(BlockType::Planks),

            ItemType::RedBlock => Some(BlockType::Red),
            ItemType::OrangeBlock => Some(BlockType::Orange),
            ItemType::YellowBlock => Some(BlockType::Yellow),
            ItemType::LimeBlock => Some(BlockType::Lime),
            ItemType::CyanBlock => Some(BlockType::Cyan),
            ItemType::BlueBlock => Some(BlockType::Blue),
            ItemType::PurpleBlock => Some(BlockType::Purple),
            ItemType::MagentaBlock => Some(BlockType::Magenta),
            ItemType::PinkBlock => Some(BlockType::Pink),
            ItemType::WhiteBlock => Some(BlockType::White),
            ItemType::GrayBlock => Some(BlockType::Gray),
            ItemType::BlackBlock => Some(BlockType::Black),
        }
    }
}

impl BlockType {
    pub fn is_air(&self) -> bool {
        *self == BlockType::Air
    }

    pub fn get_texture_id(&self, dir: QuadDir) -> u8 {
        match *self {
            BlockType::Air => 0,
            BlockType::Stone => 1,
            BlockType::Grass => {
                if dir == QuadDir::Top {
                    2
                } else {
                    3
                }
            }
            BlockType::Dirt => 3,
            BlockType::Sand => 5,
            BlockType::Cobblestone => 6,
            BlockType::Border => 7,
            BlockType::Log => 8,
            BlockType::Leaves => 9,
            BlockType::Planks => 10,

            BlockType::Red => 11,
            BlockType::Orange => 12,
            BlockType::Yellow => 13,
            BlockType::Lime => 14,
            BlockType::Cyan => 15,
            BlockType::Blue => 16,
            BlockType::Purple => 17,
            BlockType::Magenta => 18,
            BlockType::Pink => 19,
            BlockType::White => 20,
            BlockType::Gray => 21,
            BlockType::Black => 22,
        }
    }

    pub const fn get_from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(BlockType::Air),
            1 => Some(BlockType::Stone),
            2 => Some(BlockType::Grass),
            3 => Some(BlockType::Dirt),
            4 => Some(BlockType::Sand),
            5 => Some(BlockType::Cobblestone),
            6 => Some(BlockType::Border),
            7 => Some(BlockType::Log),
            8 => Some(BlockType::Leaves),
            9 => Some(BlockType::Planks),

            10 => Some(BlockType::Red),
            11 => Some(BlockType::Orange),
            12 => Some(BlockType::Yellow),
            13 => Some(BlockType::Lime),
            14 => Some(BlockType::Cyan),
            15 => Some(BlockType::Blue),
            16 => Some(BlockType::Purple),
            17 => Some(BlockType::Magenta),
            18 => Some(BlockType::Pink),
            19 => Some(BlockType::White),
            20 => Some(BlockType::Gray),
            21 => Some(BlockType::Black),
            _ => None,
        }
    }

    pub const fn get_hardness(&self) -> f32 {
        match self {
            BlockType::Air => 0.,
            BlockType::Stone => 2.,
            BlockType::Grass => 1.2,
            BlockType::Dirt => 1.,
            BlockType::Sand => 1.,
            BlockType::Cobblestone => 2.2,
            BlockType::Border => -1.,
            BlockType::Log => 1.5,
            BlockType::Leaves => 0.3,
            BlockType::Planks => 1.2,

            // Decorative colored blocks: same softness as planks
            BlockType::Red => 0.8,
            BlockType::Orange => 0.8,
            BlockType::Yellow => 0.8,
            BlockType::Lime => 0.8,
            BlockType::Cyan => 0.8,
            BlockType::Blue => 0.8,
            BlockType::Purple => 0.8,
            BlockType::Magenta => 0.8,
            BlockType::Pink => 0.8,
            BlockType::White => 0.8,
            BlockType::Gray => 0.8,
            BlockType::Black => 0.8,
        }
    }

    pub const fn get_dropped_item_type(&self) -> ItemType {
        match self {
            BlockType::Air => ItemType::Air,
            BlockType::Stone => ItemType::CobblestoneBlock,
            BlockType::Grass => ItemType::DirtBlock,
            BlockType::Dirt => ItemType::DirtBlock,
            BlockType::Sand => ItemType::SandBlock,
            BlockType::Cobblestone => ItemType::CobblestoneBlock,
            BlockType::Border => ItemType::BorderBlock,
            BlockType::Log => ItemType::LogBlock,
            BlockType::Leaves => ItemType::LeavesBlock,
            BlockType::Planks => ItemType::PlanksBlock,

            BlockType::Red => ItemType::RedBlock,
            BlockType::Orange => ItemType::OrangeBlock,
            BlockType::Yellow => ItemType::YellowBlock,
            BlockType::Lime => ItemType::LimeBlock,
            BlockType::Cyan => ItemType::CyanBlock,
            BlockType::Blue => ItemType::BlueBlock,
            BlockType::Purple => ItemType::PurpleBlock,
            BlockType::Magenta => ItemType::MagentaBlock,
            BlockType::Pink => ItemType::PinkBlock,
            BlockType::White => ItemType::WhiteBlock,
            BlockType::Gray => ItemType::GrayBlock,
            BlockType::Black => ItemType::BlackBlock,
        }
    }
}

pub fn get_quad_color_from_texture_id(id: u8) -> Color565 {
    match id {
        1 => Color565::from_rgb888(160, 160, 160),
        2 => Color565::from_rgb888(21, 147, 0),
        3 => Color565::from_rgb888(120, 77, 49),
        5 => Color565::from_rgb888(208, 199, 6),
        6 => Color565::from_rgb888(178, 178, 178),
        7 => Color565::from_rgb888(19, 19, 19),
        8 => Color565::from_rgb888(79, 53, 30),
        9 => Color565::from_rgb888(36, 75, 37),
        10 => Color565::from_rgb888(152, 124, 61),

        11 => Color565::from_rgb888(200, 40, 40),   // Red
        12 => Color565::from_rgb888(230, 130, 30),  // Orange
        13 => Color565::from_rgb888(230, 210, 40),  // Yellow
        14 => Color565::from_rgb888(90, 200, 60),   // Lime
        15 => Color565::from_rgb888(40, 190, 190),  // Cyan
        16 => Color565::from_rgb888(40, 90, 220),   // Blue
        17 => Color565::from_rgb888(140, 60, 200),  // Purple
        18 => Color565::from_rgb888(210, 60, 180),  // Magenta
        19 => Color565::from_rgb888(240, 160, 190), // Pink
        20 => Color565::from_rgb888(235, 235, 235), // White
        21 => Color565::from_rgb888(100, 100, 100), // Gray
        22 => Color565::from_rgb888(25, 25, 25),    // Black

        _ => Color565::from_rgb888(0, 0, 0),
        // 255 is reserved for block outline
    }
}
