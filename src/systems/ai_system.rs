use core::borrow::Borrow;
use pathfinding::prelude::astar;

use specs::{
    Component, Entity, Join, Read, ReadStorage, RunningTime, System, VecStorage, WorldExt, Write,
    WriteStorage,
};

use crate::world::direction::DIRECTIONS;
use crate::{pos, Level, Movement, Position, TilePosition, World};

pub struct Target(Entity);

impl Component for Target {
    type Storage = VecStorage<Target>;
}

pub struct AiType {
    pub t: i32,
}

impl Default for AiType {
    fn default() -> Self {
        AiType { t: 0 }
    }
}

impl Component for AiType {
    type Storage = VecStorage<AiType>;
}

pub struct AiSystem;
impl<'a> System<'a> for AiSystem {
    type SystemData = (
        Write<'a, Level>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Movement>,
        ReadStorage<'a, Target>,
        ReadStorage<'a, AiType>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut level, position, mut movement, target, ai_type) = data;

        for (pos, mov, target, ait) in (&position, &mut movement, &target, &ai_type).join() {
            let goal = pos!(0, 0);

            let successors = |pos: &Position| {
                let mut vec = Vec::new();
                let tile = level.get_loaded_tile(*pos).unwrap();
                for dir in DIRECTIONS {
                    let n_tile = level.get_loaded_tile(*pos + dir);
                    if n_tile.is_some() {
                        let n_tile = n_tile.unwrap();
                        let n_pos = *pos + dir;
                        vec.push((n_pos, n_tile.traversal_cost));
                    }
                }
                vec.into_iter()
            };

            let heuristic = |pos: &Position| {
                return pos.dist(goal);
            };

            let success = |pos: &Position| {
                return *pos == goal || pos.is_adjacent(goal);
            };

            let path = astar(pos, successors, heuristic, success);
            if path.is_some() {
                let path = path.unwrap().0;
                mov.delta = *path.get(0).unwrap() - *pos;
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<Position>();
        world.register::<Movement>();
        world.register::<Target>();
        world.register::<AiType>();
    }
}
