use ggez::GameResult;
use std::sync::Arc;

use noise::{NoiseFn, Perlin, Seedable};
use specs::world::EntitiesRes;
use specs::Entity;

use crate::world::direction::{DirectionalMap, DIRECTIONS};
use crate::world::position::TilePosition;
use crate::{pos, Camera, ChunkPosition, Level, SpriteAtlas, Tile, WorldPosition};

use super::position::Position;

pub const CHUNK_SIZE: i32 = 32;

pub struct Chunk;

impl Chunk {
    pub const SIZE: i32 = 32;
}

type Tiles = [[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];

#[derive(Debug)]
pub struct UnloadedChunk {
    pub tiles: Tiles,
    pub pos: Position,
    pub entities: Vec<Entity>,
}

impl UnloadedChunk {
    pub fn generate(chunk_pos: Position, seed: i32) -> Self {
        let mut tiles = [[Tile::empty(); Chunk::SIZE as usize]; Chunk::SIZE as usize];

        let mut perlin = Perlin::new();
        perlin = Seedable::set_seed(perlin, seed as u32);

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let val = perlin.get([
                    (x + chunk_pos.x * CHUNK_SIZE) as f64 / 15.0,
                    (y + chunk_pos.y * CHUNK_SIZE) as f64 / 15.0,
                ]);

                if val > 0.5 {
                    tiles[x as usize][y as usize] = Tile::rock();
                }
            }
        }

        UnloadedChunk {
            tiles,
            pos: chunk_pos,
            entities: Vec::with_capacity(0),
        }
    }

    pub fn load(self /*, entities_res: EntitiesRes*/) -> LoadedChunk {
        /*
        for entity in self.entities {
            entities_res
                .build_entity()
                //.with()
                .build();
        }
        */

        LoadedChunk {
            tiles: self.tiles,
            neighbours: Default::default(),
            pos: self.pos,
        }
    }
}

#[derive(Debug)]
pub struct LoadedChunk {
    pub tiles: [[Tile; Chunk::SIZE as usize]; Chunk::SIZE as usize],
    pub neighbours: DirectionalMap<Option<Arc<LoadedChunk>>>,
    pub pos: Position,
}

impl LoadedChunk {
    pub fn void() -> Self {
        LoadedChunk {
            tiles: [[Tile::empty(); Chunk::SIZE as usize]; Chunk::SIZE as usize],
            neighbours: Default::default(),
            pos: pos!(0, 0),
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &Tile {
        if x < 0 || y < 0 || x >= CHUNK_SIZE || y >= CHUNK_SIZE {
            panic!(
                "Tried to get tile at {:?} from chunk at {:?}",
                pos!(x, y),
                self.pos
            );
        }

        return &self.tiles[x as usize][y as usize];
    }

    pub fn get_tile_at(&self, tile_pos: Position) -> &Tile {
        return self.get_tile(tile_pos.x, tile_pos.y);
    }

    pub fn render(&self, atlas: &mut SpriteAtlas, camera: &Camera) -> GameResult {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let tile_pos = pos!(x, y) + ChunkPosition::to_tile(self.pos);
                if !camera.tile_in_view(tile_pos) {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                let tile = &self.tiles[x][y];
                let tile_screen_pos = TilePosition::to_screen(tile_pos, camera);
                atlas.add(
                    &tile.sprite_id,
                    tile_screen_pos.x as f32,
                    tile_screen_pos.y as f32,
                    camera.zoom,
                );
            }
        }
        Ok(())
    }

    /*
        pub fn populate_neighbours(&mut self, neighbours: DirectionalMap<Option<Arc<LoadedChunk>>>) {
            self.neighbours = neighbours;

            let tile_range = (0 as usize)..(Chunk::SIZE as usize);
            for x in tile_range.clone() {
                for y in tile_range.clone() {
                    for dir in DIRECTIONS {
                        let nx = x + dir.x as usize;
                        let ny = y + dir.y as usize;

                        if tile_range.contains(&x) && tile_range.contains(&y) {
                            let n_tile = &mut self.tiles[nx][ny];
                            let n_tile = Arc::clone(n_tile);
                            let tile = &self.tiles[x][y];
                            {}
                            tile.neighbours.set(dir, Some(n_tile));
                        } else {
                            let n_chunk = self.neighbours.get(dir);
                            if n_chunk.is_some() {
                                let n_chunk = n_chunk.unwrap();
                                let nx = if dir.x == 1 {
                                    0usize
                                } else if dir.x == -1 {
                                    (Chunk::SIZE - 1) as usize
                                } else {
                                    nx
                                };
                                let ny = if dir.y == 1 {
                                    0usize
                                } else if dir.y == -1 {
                                    (Chunk::SIZE - 1) as usize
                                } else {
                                    ny
                                };

                                let n_tile = &mut self.tiles[nx][ny];
                                let n_tile = Arc::clone(n_tile);
                                let tile = &mut self.tiles[x][y];
                                tile.neighbours.set(dir, Some(n_tile));
                                let tile = Arc::clone(tile);
                                let n_tile = &mut n_chunk.tiles[nx][ny];
                                n_tile.neighbours.set(dir.invert(), Some(tile));
                            } else {
                                let tile = &mut self.tiles[x][y];
                                tile.neighbours.set(dir, None);
                            }
                        }
                    }
                }
            }
        }

        pub fn unpopulate_neighbours(&self) {
            let tile_range = (0 as usize)..(Chunk::SIZE as usize);
            let edge = (Chunk::SIZE - 1) as usize;
            for x in 0..2 {
                for y in 0..2 {
                    let tile = &self.tiles[x * edge][y * edge];

                    for dir in DIRECTIONS {
                        let nx = x + dir.x as usize;
                        let ny = y + dir.y as usize;

                        if !tile_range.contains(&x) || !tile_range.contains(&y) {
                            let n_chunk = self.neighbours.get(dir);
                            if n_chunk.is_some() {
                                let n_chunk = n_chunk.unwrap();
                                let nx = if dir.x == 1 {
                                    0usize
                                } else if dir.x == -1 {
                                    (Chunk::SIZE - 1) as usize
                                } else {
                                    nx
                                };
                                let ny = if dir.y == 1 {
                                    0usize
                                } else if dir.y == -1 {
                                    (Chunk::SIZE - 1) as usize
                                } else {
                                    ny
                                };

                                let n_tile = &mut n_chunk.tiles[nx][ny];
                                n_tile.neighbours.set(dir.invert(), None);
                            }
                        }
                    }
                }
            }

            for dir in DIRECTIONS {
                let n_chunk = self.neighbours.get(dir);
                if n_chunk.is_some() {
                    let n_chunk = n_chunk.unwrap();
                    n_chunk.neighbours.set(dir.invert(), None);
                }
            }
        }
    */
    pub fn unload(self, entities: Vec<Entity>) -> UnloadedChunk {
        UnloadedChunk {
            tiles: self.tiles,
            pos: self.pos,
            entities,
        }
    }
}
