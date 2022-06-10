use specs::{Builder, World, WorldExt};

use crate::objects::sprite_atlas::SpriteId;
use crate::systems::ai_system::AiType;
use crate::systems::health_system::Health;
use crate::{pos, ChunkLoader, Control, Movement, Position};

pub struct Entities;

impl Entities {
    pub fn create_player(world: &mut World) {
        world
            .create_entity()
            .with(Position::default())
            .with(Movement::default())
            .with(SpriteId::PLAYER)
            .with(Control)
            .with(ChunkLoader::default())
            .with(Health::default())
            .build();
    }

    pub fn create_ai(world: &mut World) {
        world
            .create_entity()
            .with(pos!(10, 10))
            .with(Movement::default())
            .with(SpriteId::TREE_A)
            .with(AiType::default())
            .with(Health::default())
            .build();
    }
}
