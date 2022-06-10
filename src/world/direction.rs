use std::iter::{
    Cycle, Enumerate, Filter, FilterMap, FlatMap, Flatten, Fuse, Inspect, Map, MapWhile, Peekable,
    Product, Rev, Scan, Skip, SkipWhile, StepBy, Sum, Take, TakeWhile, Zip,
};
use std::ops::Deref;

use crate::{pos, Position, Tile};

pub const DIRECTIONS: [Direction; 8] = [
    Direction::NORTH,
    Direction::NORTH_EAST,
    Direction::EAST,
    Direction::SOUTH_EAST,
    Direction::SOUTH,
    Direction::SOUTH_WEST,
    Direction::WEST,
    Direction::NORTH_WEST,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Direction {
    dir: Position,
}

impl Deref for Direction {
    type Target = Position;

    fn deref(&self) -> &Self::Target {
        &self.dir
    }
}

impl Direction {
    pub fn new(x: i32, y: i32) -> Self {
        Direction {
            dir: pos!(x.signum(), y.signum()),
        }
    }

    pub const NORTH: Direction = Direction { dir: pos!(0, -1) };
    pub const NORTH_EAST: Direction = Direction { dir: pos!(1, -1) };
    pub const EAST: Direction = Direction { dir: pos!(1, 0) };
    pub const SOUTH_EAST: Direction = Direction { dir: pos!(1, 1) };
    pub const SOUTH: Direction = Direction { dir: pos!(0, 1) };
    pub const SOUTH_WEST: Direction = Direction { dir: pos!(-1, 1) };
    pub const WEST: Direction = Direction { dir: pos!(-1, 0) };
    pub const NORTH_WEST: Direction = Direction { dir: pos!(-1, -1) };

    pub fn invert(self) -> Self {
        return Direction::new(-self.x, -self.y);
    }

    pub fn to_index(self) -> usize {
        let mut index = 0;
        for dir in DIRECTIONS.iter() {
            if *dir == self {
                return index;
            }
            index += 1;
        }
        panic!("Direction not found");
    }

    pub fn from_index(index: usize) -> Self {
        return DIRECTIONS[index].clone();
    }

    pub fn to_position(self) -> Position {
        return self.dir.clone();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DirectionalMap<T> {
    data: [T; 8],
}

impl<T: Default> Default for DirectionalMap<T> {
    fn default() -> Self {
        Self {
            data: [
                T::default(),
                T::default(),
                T::default(),
                T::default(),
                T::default(),
                T::default(),
                T::default(),
                T::default(),
            ],
        }
    }
}

impl<T: Clone> DirectionalMap<T> {
    pub fn new(default: T) -> Self {
        Self {
            data: [
                default.clone(),
                default.clone(),
                default.clone(),
                default.clone(),
                default.clone(),
                default.clone(),
                default.clone(),
                default.clone(),
            ],
        }
    }
}

impl<T> DirectionalMap<T> {
    pub fn get(&self, dir: Direction) -> &T {
        &self.data[dir.to_index()]
    }

    pub fn set(&mut self, dir: Direction, value: T) {
        self.data[dir.to_index()] = value;
    }
    /*
        pub fn map<F>(&self, f: fn(T) -> F) -> DirectionalMap<F> {
            let new_data = self.data.map(f);
            DirectionalMap { data: new_data }
        }
    */
    pub fn north(&self) -> &T {
        &self.data[0]
    }
    pub fn north_east(&self) -> &T {
        &self.data[1]
    }
    pub fn east(&self) -> &T {
        &self.data[2]
    }
    pub fn south_east(&self) -> &T {
        &self.data[3]
    }
    pub fn south(&self) -> &T {
        &self.data[4]
    }
    pub fn south_west(&self) -> &T {
        &self.data[5]
    }
    pub fn west(&self) -> &T {
        &self.data[6]
    }
    pub fn north_west(&self) -> &T {
        &self.data[7]
    }
}
