use crate::objects::sprite_atlas::SpriteId;
use crate::world::direction::{Direction, DirectionalMap};

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub traversal_cost: i32,
    pub sprite_id: SpriteId,
}

pub enum TileType {
    Floor,
    Wall,
    Empty,
}

impl Tile {
    pub const SIZE: f32 = 32.0;

    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            traversal_cost: 1,
            sprite_id: SpriteId::EMPTY,
        }
    }

    pub fn grass() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            traversal_cost: 1,
            sprite_id: SpriteId::EMPTY,
        }
    }

    pub fn rock() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            traversal_cost: 1000000,
            sprite_id: SpriteId::WALL_NESW_OPEN,
        }
    }

    pub fn wall(neighbours: [bool; 8]) -> Self {
        let sprite_id = match neighbours {
            [true, true, true, true, true, true, true, true] => SpriteId::WALL_NESW_OPEN,
            [true, true, true, true, true, true, true, false] => SpriteId::WALL_NES_OPEN,
            _ => SpriteId::WALL_NESW_OPEN,
        };

        Tile {
            blocked: true,
            block_sight: true,
            traversal_cost: 1000000,
            sprite_id,
        }
    }
}
