use crate::{Level, Position, World};
use specs::{
    AccessorCow, Component, Join, Read, ReadStorage, RunningTime, System, VecStorage, WorldExt,
    WriteStorage,
};

pub struct Movement {
    pub delta: Position,
}

impl Component for Movement {
    type Storage = VecStorage<Movement>;
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            delta: Position::default(),
        }
    }
}

pub struct MovementSystem;
impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Read<'a, Level>,
        ReadStorage<'a, Movement>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (level, movements, mut positions) = data;
        for (mov, pos) in (&movements, &mut positions).join() {
            let tile_optional = level.get_loaded_tile(*pos + mov.delta);
            let blocked = tile_optional.map(|t| t.blocked).unwrap_or(true);
            if !blocked {
                pos.x += mov.delta.x;
                pos.y += mov.delta.y;
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<Movement>();
        world.register::<Position>();
    }
}
