use hashbrown::HashMap;

use crate::pos;

use super::{
    chunk::{LoadedChunk, UnloadedChunk, CHUNK_SIZE},
    position::Position,
    tile::Tile,
};

#[derive(Debug)]
pub struct Level {
    pub unloaded_chunks: HashMap<Position, UnloadedChunk>,
    pub loaded_chunks: HashMap<Position, LoadedChunk>,
    pub seed: i32,
}

impl Level {
    pub fn new(seed: i32) -> Self {
        let mut unloaded_chunks = HashMap::new();

        // Create origin chunk
        unloaded_chunks.insert(pos!(0, 0), UnloadedChunk::generate(pos!(0, 0), seed));

        Level {
            unloaded_chunks,
            loaded_chunks: HashMap::new(),
            seed,
        }
    }

    pub fn is_loaded(&self, pos: &Position) -> bool {
        return self.loaded_chunks.contains_key(pos);
    }

    pub fn unload_chunk(&mut self, chunk_pos: Position) -> Option<&UnloadedChunk> {
        let chunk = self.loaded_chunks.remove(&chunk_pos)?;
        self.unloaded_chunks.insert(chunk_pos, chunk.to_unloaded());
        return self.unloaded_chunks.get(&chunk_pos);
    }

    pub fn load_chunk(&mut self, chunk_pos: Position) -> &LoadedChunk {
        // Does not check if a chunk is already loaded, would lead to the chunk being generated again and overriding the already loaded chunk
        let chunk_option = self.unloaded_chunks.remove(&chunk_pos);
        let unloaded_chunk = chunk_option.unwrap_or(UnloadedChunk::generate(chunk_pos, self.seed));
        self.loaded_chunks
            .insert(chunk_pos, unloaded_chunk.to_loaded());
        return self.loaded_chunks.get(&chunk_pos).unwrap();
    }

    pub fn load_chunk_at(&mut self, pos: Position) -> &LoadedChunk {
        let chunk_pos = pos / CHUNK_SIZE;
        return self.load_chunk(chunk_pos);
    }

    pub fn get_loaded_tile(&self, p: Position) -> Option<&Tile> {
        let loaded_chunk = self.loaded_chunks.get(&p.chunk_coords())?;
        let tile_pos = p.tile_coords();
        let tile = &loaded_chunk.tiles[tile_pos.x as usize][tile_pos.y as usize];
        Some(tile)
    }
}
