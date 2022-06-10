use crate::{pos, Level, Position, TilePosition, World};
use specs::{Component, Join, Read, ReadStorage, RunningTime, System, VecStorage, WorldExt, Write};
use std::collections::HashSet;

pub struct ChunkLoader {
    pub range: i32,
}

impl Component for ChunkLoader {
    type Storage = VecStorage<ChunkLoader>;
}

impl Default for ChunkLoader {
    fn default() -> Self {
        ChunkLoader { range: 1 }
    }
}

pub struct ChunkSystem;
impl<'a> System<'a> for ChunkSystem {
    type SystemData = (
        Write<'a, Level>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, ChunkLoader>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut level, position, chunk_loader) = data;

        let mut ensure_loaded = HashSet::<Position>::new();
        for (pos, loader) in (&position, &chunk_loader).join() {
            let loader_chunk_pos = TilePosition::to_chunk(*pos);

            for x in (-loader.range)..(loader.range + 1) {
                for y in (-loader.range)..(loader.range + 1) {
                    let chunk_pos = loader_chunk_pos + pos!(x, y);
                    if ensure_loaded.insert(chunk_pos) {
                        level.ensure_loaded(chunk_pos);
                    }
                }
            }
        }

        // Unload chunks
        let mut to_unload = Vec::new();
        for (pos, _) in &level.loaded_chunks {
            if !ensure_loaded.contains(pos) {
                to_unload.push(pos.clone());
            }
        }

        for pos in to_unload {
            level.unload_chunk(pos);
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<Position>();
        world.register::<ChunkLoader>();
    }
}
