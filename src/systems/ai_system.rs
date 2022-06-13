use pathfinding::prelude::astar;
use std::borrow::Borrow;
use std::mem::MaybeUninit;
use std::ops::Range;

use specs::{
    Component, Entity, Join, Read, ReadStorage, System, VecStorage, WorldExt, Write, WriteStorage,
};

use crate::utils::matrix::matrix;
use crate::world::chunk::LoadedChunk;
use crate::world::direction::DIRECTIONS;
use crate::{pos, Level, Movement, Position, Tile, TilePosition, World};

pub struct Target(pub Entity);

impl Component for Target {
    type Storage = VecStorage<Target>;
}

pub struct Pathing {
    pub path: Vec<Position>,
    pub goal_tile_pos: Position,
    pub cost: i32,
    pub cur: usize,
}

impl Pathing {
    pub fn new(path: Vec<Position>, goal_tile_pos: Position, cost: i32) -> Self {
        Pathing {
            path,
            goal_tile_pos,
            cost,
            cur: 0,
        }
    }

    pub fn next(&mut self) -> Option<Position> {
        if self.cur >= self.path.len() {
            return None;
        }
        let pos = self.path[self.cur];
        self.cur += 1;
        Some(pos)
    }
}

pub struct Ai {
    pub t: i32,
    pub pathing: Option<Pathing>,
}

impl Default for Ai {
    fn default() -> Self {
        Ai {
            t: 0,
            pathing: None,
        }
    }
}

impl Component for Ai {
    type Storage = VecStorage<Ai>;
}

impl Ai {
    #[inline(always)]
    pub fn has_path(&self) -> bool {
        self.pathing.is_some()
    }

    pub fn find_path(
        &mut self,
        current_tile_pos: Position,
        goal_tile_pos: Position,
        level: &Level,
    ) -> bool {
        let current_chunk_pos = TilePosition::to_chunk(current_tile_pos);
        let mut chunk_cache = ChunkCache::<7>::new(current_chunk_pos, &level);

        let successors = |pos: &Position| {
            let neighbours = chunk_cache.get_tile_neighbours(*pos);
            neighbours
                .into_iter()
                .map(|(tile, pos)| (pos, tile.traversal_cost))
                .collect::<Vec<(Position, i32)>>()
        };

        let heuristic = |pos: &Position| {
            return pos.dist(goal_tile_pos);
        };

        let success = |pos: &Position| {
            return *pos == goal_tile_pos || pos.is_adjacent(goal_tile_pos);
        };

        let path = astar(&current_tile_pos, successors, heuristic, success);

        return if path.is_some() {
            let path = path.unwrap();
            self.pathing = Some(Pathing::new(path.0, goal_tile_pos, path.1));
            true
        } else {
            false
        };
    }

    #[inline(always)]
    pub fn next_pos(&mut self) -> Option<Position> {
        return if self.pathing.is_none() {
            None
        } else {
            let pathing = self.pathing.as_mut().unwrap();
            let next = pathing.next();

            if next.is_some() {
                next
            } else {
                self.pathing = None;
                None
            }
        };
    }
}

pub struct AiSystem;

impl AiSystem {
    pub const NAV_RANGE: usize = 3;
}

impl<'a> System<'a> for AiSystem {
    type SystemData = (
        Read<'a, Level>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Movement>,
        ReadStorage<'a, Target>,
        WriteStorage<'a, Ai>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (level, position, mut movement, target, mut ai_type) = data;

        const nav_rad: usize = AiSystem::NAV_RANGE * 2 + 1;

        for (pos, mov, target, ait) in (&position, &mut movement, &target, &mut ai_type).join() {
            mov.delta = pos!(0, 0);
            let goal = position.get(target.0).unwrap();

            if !ait.has_path() || !ait.pathing.as_ref().unwrap().goal_tile_pos.eq(goal) {
                ait.find_path(pos.clone(), *goal, &level);
            }

            let next_pos = ait.next_pos();
            if next_pos.is_some() && pos.dist(*goal) >= 3 {
                mov.delta = next_pos.unwrap() - pos.clone();
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<Position>();
        world.register::<Movement>();
        world.register::<Target>();
        world.register::<Ai>();
    }
}

struct ChunkCache<'a, const R: usize> {
    pub offset_chunk_pos: Position,
    pub chunk_range: Range<i32>,
    pub cache: [[(bool, bool, MaybeUninit<&'a LoadedChunk>); R]; R],
    pub level: &'a Level,
}

impl<'a, const R: usize> ChunkCache<'a, R> {
    pub fn new(center_chunk_pos: Position, level: &'a Level) -> Self {
        let offset_chunk_pos =
            center_chunk_pos - pos!(((R - R % 2) / 2) as i32, ((R - R % 2) / 2) as i32);
        let chunk_range = 0..R as i32;
        Self {
            offset_chunk_pos,
            chunk_range,
            cache: matrix::<(bool, bool, MaybeUninit<&LoadedChunk>), R, R>(|| {
                (false, false, MaybeUninit::<&LoadedChunk>::uninit())
            }),
            level,
        }
    }

    fn get_chunk(&mut self, offset_chunk_pos: Position) -> Option<&LoadedChunk> {
        if !self.in_range(offset_chunk_pos) {
            return None;
        }
        let (cached, _, _) = self.cache[offset_chunk_pos.x as usize][offset_chunk_pos.y as usize];
        if !cached {
            self.cache[offset_chunk_pos.x as usize][offset_chunk_pos.y as usize].0 = true;
            let chunk_option = self
                .level
                .get_loaded_chunk(offset_chunk_pos + self.offset_chunk_pos);
            if chunk_option.is_some() {
                self.cache[offset_chunk_pos.x as usize][offset_chunk_pos.y as usize].1 = true;
                self.cache[offset_chunk_pos.x as usize][offset_chunk_pos.y as usize].2 =
                    MaybeUninit::new(chunk_option.unwrap());
            }
        }

        let (cached, exists, chunk) =
            self.cache[offset_chunk_pos.x as usize][offset_chunk_pos.y as usize];
        return if cached && exists {
            unsafe { Some(chunk.assume_init()) }
        } else {
            None
        };
    }

    #[inline(always)]
    fn in_range(&self, offset_chunk_pos: Position) -> bool {
        return self.chunk_range.contains(&offset_chunk_pos.x)
            && self.chunk_range.contains(&offset_chunk_pos.y);
    }

    pub fn get_tile_neighbours(&mut self, tile_pos: Position) -> Vec<(Tile, Position)> {
        let mut vec = Vec::new();

        let off = self.offset_chunk_pos.clone();

        for dir in DIRECTIONS {
            let n_tile_pos = tile_pos + dir;
            let offset_chunk_pos = TilePosition::to_chunk(n_tile_pos) - off;

            let chunk = self.get_chunk(offset_chunk_pos);
            if chunk.is_some() {
                let chunk = chunk.unwrap();
                let tile = chunk
                    .get_tile_at(TilePosition::to_tile_in_chunk(n_tile_pos))
                    .clone();
                vec.push((tile, n_tile_pos));
            }
        }
        vec
    }
}
