use crate::world::chunk::Chunk;
use crate::world::position::{ChunkPosition, TilePosition};
use crate::world::tile::Tile;
use crate::{pos, Position, WorldPosition};

pub struct Camera {
    pub pos: Position,
    pub zoom: f32,
    pub width: i32,
    pub height: i32,
}

impl Camera {
    pub fn in_view(&self, world_pos: Position) -> bool {
        let screen_pos = WorldPosition::to_screen(world_pos, self);
        return screen_pos.x >= 0
            && screen_pos.y >= 0
            && screen_pos.x < self.width
            && screen_pos.y < self.height;
    }

    pub fn rect_in_view(&self, world_pos: Position, width: f32, height: f32) -> bool {
        let screen_pos = WorldPosition::to_screen(world_pos, self);
        let screen_pos_end = screen_pos + pos!(width as i32, height as i32);
        return (0 < screen_pos_end.x)
            && (screen_pos.x < self.width)
            && (0 < screen_pos_end.y)
            && (screen_pos.y < self.height);
    }

    pub fn tile_in_view(&self, tile_pos: Position) -> bool {
        return Self::rect_in_view(
            self,
            TilePosition::to_world(tile_pos),
            Tile::SIZE,
            Tile::SIZE,
        );
    }

    pub fn chunk_in_view(&self, chunk_pos: Position) -> bool {
        return Self::rect_in_view(
            self,
            ChunkPosition::to_world(chunk_pos),
            Tile::SIZE * Chunk::SIZE as f32,
            Tile::SIZE * Chunk::SIZE as f32,
        );
    }
}
