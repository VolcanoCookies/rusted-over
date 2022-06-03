use emerald::{Emerald, Sprite};

pub struct EntitySprite {
    pub sprite: Sprite,
}

impl EntitySprite {
    pub fn new(mut emd: Emerald, path: &str) -> Self {
        Self {
            sprite: emd.loader().sprite(path).unwrap(),
        }
    }
}
