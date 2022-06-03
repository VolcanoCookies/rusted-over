use specs::Component;
use std::ops::{Add, Div, Mul, Rem, Sub};

use super::chunk::CHUNK_SIZE;

#[macro_export]
macro_rules! pos {
    ($a:expr, $b:expr) => {
        Position { x: $a, y: $b }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Component for Position {
    type Storage = specs::VecStorage<Self>;
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
    pub fn chunk_coords(self) -> Self {
        Self {
            x: if self.x >= 0 {
                self.x / CHUNK_SIZE
            } else {
                (self.x + 1) / CHUNK_SIZE - 1
            },
            y: if self.y >= 0 {
                self.y / CHUNK_SIZE
            } else {
                (self.y + 1) / CHUNK_SIZE - 1
            },
        }
    }

    // This function gets the coordinates of a tile within a chunk from the global position, and handless coordinate wraparound correctly to always produce a positive position.
    pub fn tile_coords(self) -> Self {
        return (self % CHUNK_SIZE + CHUNK_SIZE) % CHUNK_SIZE;
    }

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
}

pub struct Direction;

pub const DIRECTIONS: [Position; 8] = [
    Direction::NORTH,
    Direction::NORTH_EAST,
    Direction::EAST,
    Direction::SOUTH_EAST,
    Direction::SOUTH,
    Direction::SOUTH_WEST,
    Direction::WEST,
    Direction::NORTH_WEST,
];

impl Direction {
    const NORTH: Position = pos!(0, -1);
    const NORTH_EAST: Position = pos!(1, -1);
    const EAST: Position = pos!(1, 0);
    const SOUTH_EAST: Position = pos!(1, 1);
    const SOUTH: Position = pos!(0, 1);
    const SOUTH_WEST: Position = pos!(-1, 1);
    const WEST: Position = pos!(-1, 0);
    const NORTH_WEST: Position = pos!(-1, -1);
}
