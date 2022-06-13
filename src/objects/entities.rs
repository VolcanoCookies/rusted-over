use rand::Rng;
use specs::{Builder, Entity, World, WorldExt};

use crate::objects::sprite_atlas::SpriteId;
use crate::systems::ai_system::{Ai, Target};
use crate::systems::health_system::Health;
use crate::{pos, ChunkLoader, Control, Movement, Position};

pub struct Entities;

impl Entities {
    pub fn create_player(world: &mut World) -> Entity {
        world
            .create_entity()
            .with(Position::default())
            .with(Movement::default())
            .with(SpriteId::PLAYER)
            .with(Control)
            .with(ChunkLoader::default())
            .with(Health::default())
            .build()
    }

    pub fn create_ai(world: &mut World, target: Entity) -> Entity {
        let x = rand::thread_rng().gen_range(-32..32);
        let y = rand::thread_rng().gen_range(-32..32);
        world
            .create_entity()
            .with(pos!(x, y))
            .with(Movement::default())
            .with(SpriteId::TREE_A)
            .with(Ai::default())
            .with(Target(target))
            .with(Health::default())
            .build()
    }
}
