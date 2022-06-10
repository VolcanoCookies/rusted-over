use ggez::event::KeyCode;
use ggez::graphics::Color;
use ggez::input::keyboard::is_key_pressed;
use ggez::{input, Context};

use crate::{
    pos,
    world::{level::Level, position::Position},
    Camera, SpriteAtlas, TilePosition,
};

trait Entity {
    fn get_pos(&self) -> Position;
    fn set_pos(&mut self, pos: Position);

    fn render(&self, atlas: &mut SpriteAtlas, camera: &Camera);
    fn update(&mut self, ctx: &Context, dt: f32);
}

pub struct Player {
    pos: Position,
}

#[derive(Debug, Copy, Clone)]
pub struct GameObject {
    pub pos: Position,
    char: char,
    color: Color,
}

impl GameObject {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        GameObject {
            pos: pos!(x, y),
            char,
            color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, level: &Level) {
        let d_loc = self.pos + pos!(dx, dy);
        let blocked = level
            .get_loaded_tile(d_loc)
            .and_then(|tile| Some(tile.blocked))
            .unwrap_or(true);

        if !blocked {
            self.pos = d_loc;
        }
    }
}
