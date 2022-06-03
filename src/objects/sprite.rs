use emerald::{Emerald, Sprite};
use specs::Component;

pub struct EntitySprite {
    pub sprite: Sprite,
}

impl Component for EntitySprite {
    type Storage = specs::VecStorage<Self>;
}

impl EntitySprite {
    pub fn new(emd: &mut Emerald, path: &str) -> Self {
        Self {
            sprite: emd.loader().sprite(path).unwrap(),
        }
    }
}
