use crate::objects::sprite_atlas::SpriteId;
use crate::world::tile::TileType;

pub trait TileSpriteSet {
    fn get_sprite_id(&self, neighbours: [TileType; 8]) -> SpriteId;
}

pub struct WallTileSpriteSet;

impl TileSpriteSet for WallTileSpriteSet {
    fn get_sprite_id(&self, neighbours: [TileType; 8]) -> SpriteId {
        match neighbours {
            [TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall, TileType::Wall] => {
                SpriteId::WALL_NESW_OPEN
            }
            _ => SpriteId::WALL_N_OPEN,
        }
    }
}
