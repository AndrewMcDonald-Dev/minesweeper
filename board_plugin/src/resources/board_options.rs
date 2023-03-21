use bevy::prelude::{Resource, Vec3};
use serde::{Deserialize, Serialize};

// Tile size options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    // Fixed tile size
    Fixed(f32),
    // Window adaptative tile size
    Adaptive { min: f32, max: f32 },
}

// board position customization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardPosition {
    // Centered board
    Centered { offset: Vec3 },

    // Custom position
    Custom(Vec3),
}

// Board generation options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardOptions {
    pub map_size: (u16, u16),
    pub bomb_count: u16,
    pub position: BoardPosition,
    pub tile_size: TileSize,
    pub tile_padding: f32,
    pub safe_start: bool,
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            bomb_count: 30,
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0.0,
            safe_start: false,
        }
    }
}
impl Resource for BoardOptions {}
