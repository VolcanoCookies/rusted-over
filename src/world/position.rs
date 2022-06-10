use crate::world::chunk::Chunk;
use crate::world::direction::Direction;
use crate::world::tile::Tile;
use crate::Camera;
use gfx_core::SubmissionResult;
use imgui::sys::__BindgenBitfieldUnit;
use specs::{Component, VecStorage};
use std::cmp::min;
use std::collections::Bound;
use std::io::SeekFrom;
use std::ops::{Add, Deref, Div, Mul, Range, RangeBounds, Rem, Sub};

#[macro_export]
macro_rules! pos {
    ($a:expr, $b:expr) => {
        Position { x: $a, y: $b }
    };
}

pub struct TilePosition;
impl TilePosition {
    pub fn to_chunk(tile_pos: Position) -> Position {
        Position {
            x: if tile_pos.x >= 0 {
                tile_pos.x / Chunk::SIZE
            } else {
                (tile_pos.x + 1) / Chunk::SIZE - 1
            },
            y: if tile_pos.y >= 0 {
                tile_pos.y / Chunk::SIZE
            } else {
                (tile_pos.y + 1) / Chunk::SIZE - 1
            },
        }
    }

    pub fn to_world(tile_pos: Position) -> Position {
        return tile_pos * Tile::SIZE;
    }

    pub fn to_screen(tile_pos: Position, camera: &Camera) -> Position {
        return WorldPosition::to_screen(TilePosition::to_world(tile_pos), camera);
    }

    pub fn to_tile_in_chunk(tile_pos: Position) -> Position {
        Position {
            x: if tile_pos.x >= 0 {
                tile_pos.x % Chunk::SIZE
            } else {
                Chunk::SIZE + ((tile_pos.x + 1) % Chunk::SIZE) - 1
            },
            y: if tile_pos.y >= 0 {
                tile_pos.y % Chunk::SIZE
            } else {
                Chunk::SIZE + ((tile_pos.y + 1) % Chunk::SIZE) - 1
            },
        }
    }
}

pub struct ChunkPosition;
impl ChunkPosition {
    pub fn to_tile(chunk_pos: Position) -> Position {
        let x = if chunk_pos.x >= 0 {
            chunk_pos.x * Chunk::SIZE
        } else {
            (chunk_pos.x + 1) * Chunk::SIZE - Chunk::SIZE
        };
        let y = if chunk_pos.y >= 0 {
            chunk_pos.y * Chunk::SIZE
        } else {
            (chunk_pos.y + 1) * Chunk::SIZE - Chunk::SIZE
        };

        Position { x, y }
    }

    pub fn to_world(chunk_pos: Position) -> Position {
        return chunk_pos * Chunk::SIZE * Tile::SIZE;
    }

    pub fn to_screen(chunk_pos: Position, camera: &Camera) -> Position {
        return WorldPosition::to_screen(ChunkPosition::to_world(chunk_pos), camera);
    }
}

pub struct WorldPosition;
impl WorldPosition {
    pub fn to_tile(world_pos: Position) -> Position {
        Position {
            x: if world_pos.x >= 0 {
                world_pos.x / Tile::SIZE as i32
            } else {
                (world_pos.x + 1) / Tile::SIZE as i32 - 1
            },
            y: if world_pos.y >= 0 {
                world_pos.y / Tile::SIZE as i32
            } else {
                (world_pos.y + 1) / Tile::SIZE as i32 - 1
            },
        }
    }

    pub fn to_chunk(world_pos: Position) -> Position {
        let tile_pos = Self::to_tile(world_pos);
        Position {
            x: if tile_pos.x >= 0 {
                tile_pos.x / Chunk::SIZE
            } else {
                (tile_pos.x + 1) / Chunk::SIZE - 1
            },
            y: if tile_pos.y >= 0 {
                tile_pos.y / Chunk::SIZE
            } else {
                (tile_pos.y + 1) / Chunk::SIZE - 1
            },
        }
    }

    pub fn to_screen(world_pos: Position, camera: &Camera) -> Position {
        return (world_pos - camera.pos) * camera.zoom;
    }
}

pub struct ScreenPosition;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<i32> for Position {
    type Output = Self;

    fn add(self, rhs: i32) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Position {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<f32> for Position {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs as i32,
            y: self.y * rhs as i32,
        }
    }
}

impl Mul for Position {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div<i32> for Position {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl Div<f32> for Position {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs as i32,
            y: self.y / rhs as i32,
        }
    }
}

impl Rem for Position {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl Rem<i32> for Position {
    type Output = Self;

    fn rem(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl Position {
    pub fn is_adjacent(self, other: Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn sgn(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    pub fn dist(self, other: Self) -> i32 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        if dx < dy {
            dy
        } else {
            dx
        }
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
