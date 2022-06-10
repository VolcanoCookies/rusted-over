use ggez::graphics::MeshBuilder;
use specs::{Component, Join, Read, ReadStorage, RunningTime, System, WorldExt, WriteExpect};

use crate::objects::sprite_atlas::SpriteId;
use crate::{pos, Camera, Control, Level, Position, SpriteAtlas, Tile, TilePosition, World};

impl Component for SpriteId {
    type Storage = specs::VecStorage<Self>;
}

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Read<'a, Level>,
        WriteExpect<'a, SpriteAtlas>,
        WriteExpect<'a, Camera>,
        ReadStorage<'a, SpriteId>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Control>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (level, mut atlas, mut camera, sprite_id, position, control) = data;

        atlas.clear();

        // Move camera to player position
        for (pos, _) in (&position, &control).join() {
            camera.pos = TilePosition::to_world(*pos)
                - pos!(
                    (camera.width as f32 / camera.zoom / 2.0) as i32,
                    (camera.height as f32 / camera.zoom / 2.0) as i32
                );
        }

        // Render level
        level.render(&mut *atlas, &*camera);

        // Render entities
        for (sprite_id, pos) in (&sprite_id, &position).join() {
            let screen_pos = TilePosition::to_screen(*pos, &*camera);
            atlas.add(
                sprite_id,
                screen_pos.x as f32,
                screen_pos.y as f32,
                camera.zoom,
            );
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<SpriteId>();
    }
}
