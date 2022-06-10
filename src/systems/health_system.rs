use specs::world::EntitiesRes;
use specs::{
    Component, Entities, Entity, Join, LazyUpdate, Read, ReadStorage, System, World, WorldExt,
};

pub struct Health {
    pub health: i32,
}

impl Default for Health {
    fn default() -> Self {
        Health { health: 1 }
    }
}

impl Component for Health {
    type Storage = specs::VecStorage<Self>;
}

pub struct HealthSystem;

impl<'a> System<'a> for HealthSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, Health>, Read<'a, LazyUpdate>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, health, updater) = data;

        for (entity, health) in (&entities, &health).join() {
            if health.health <= 0 {
                entities.delete(entity);
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<Health>();
    }
}
