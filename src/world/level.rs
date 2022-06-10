use hashbrown::HashMap;
use std::sync::Arc;

use ggez::GameResult;

use crate::world::direction::{DirectionalMap, DIRECTIONS};
use crate::world::position::TilePosition;
use crate::{Camera, Rusted, SpriteAtlas};

use super::{
    chunk::{LoadedChunk, UnloadedChunk},
    position::Position,
    tile::Tile,
};

#[derive(Debug)]
pub struct Level {
    pub unloaded_chunks: HashMap<Position, UnloadedChunk>,
    pub loaded_chunks: HashMap<Position, LoadedChunk>,
    pub seed: i32,
}

impl Default for Level {
    fn default() -> Self {
        Level {
            unloaded_chunks: HashMap::new(),
            loaded_chunks: HashMap::new(),
            seed: 0,
        }
    }
}

impl Level {
    pub fn new(seed: i32) -> Self {
        Level {
            unloaded_chunks: HashMap::new(),
            loaded_chunks: HashMap::new(),
            seed,
        }
    }

    pub fn is_loaded(&self, pos: &Position) -> bool {
        return self.loaded_chunks.contains_key(pos);
    }

    pub fn unload_chunk(&mut self, chunk_pos: Position) -> Option<&UnloadedChunk> {
        let chunk = self.loaded_chunks.remove(&chunk_pos)?;
        println!("Unloaded chunk {:?}", chunk_pos);

        //chunk.unpopulate_neighbours();

        self.unloaded_chunks
            .insert(chunk_pos, chunk.unload(Vec::with_capacity(0)));
        return self.unloaded_chunks.get(&chunk_pos);
    }

    pub fn ensure_loaded(&mut self, chunk_pos: Position) -> &LoadedChunk {
        if !self.is_loaded(&chunk_pos) {
            println!("Loading chunk {:?}", chunk_pos);
            return self.load_chunk(chunk_pos);
        }

        return self.loaded_chunks.get(&chunk_pos).unwrap();
    }

    pub fn load_chunk(&mut self, chunk_pos: Position) -> &LoadedChunk {
        // Does not check if a chunk is already loaded, would lead to the chunk being generated again and overriding the already loaded chunk
        let chunk_option = self.unloaded_chunks.remove(&chunk_pos);
        let unloaded_chunk = chunk_option.unwrap_or(UnloadedChunk::generate(chunk_pos, self.seed));

        let neighbours = self.get_chunk_neighbours(chunk_pos);
        let mut loaded_chunk = unloaded_chunk.load();
        //loaded_chunk.populate_neighbours(neighbours);
        self.loaded_chunks.insert(chunk_pos, loaded_chunk);
        return self.loaded_chunks.get(&chunk_pos).unwrap();
    }

    pub fn get_loaded_tile(&self, tile_pos: Position) -> Option<&Tile> {
        let loaded_chunk = self.loaded_chunks.get(&TilePosition::to_chunk(tile_pos))?;
        let tile_chunk_pos = TilePosition::to_tile_in_chunk(tile_pos);
        let tile = loaded_chunk.get_tile(tile_chunk_pos.x, tile_chunk_pos.y);
        Some(tile)
    }

    fn get_chunk_neighbours(
        &self,
        chunk_pos: Position,
    ) -> DirectionalMap<Option<Arc<LoadedChunk>>> {
        let mut dir_map = DirectionalMap::new(None);
        for dir in DIRECTIONS {
            let n_chunk = self.loaded_chunks.get(&(chunk_pos + dir));
            dir_map.set(dir, n_chunk.map(|c| Arc::new(*c)));
        }
        dir_map
    }

    pub fn render(&self, atlas: &mut SpriteAtlas, camera: &Camera) -> GameResult {
        for (chunk_pos, chunk) in &self.loaded_chunks {
            if camera.chunk_in_view(*chunk_pos) {
                chunk.render(atlas, camera)?;
            }
        }
        Ok(())
    }
}
