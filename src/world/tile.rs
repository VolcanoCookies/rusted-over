use emerald::Sprite;

pub const TILE_SIZE: f32 = 16.0;

#[derive(Clone, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub sprite: Sprite,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            sprite: Sprite::default(),
        }
    }
}
