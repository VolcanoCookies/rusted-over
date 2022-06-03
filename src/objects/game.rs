use tcod::{BackgroundFlag, Color, Console};

use crate::{
    pos,
    world::{level::Level, position::Position},
    Game,
};

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
        dbg!(d_loc);
        dbg!(d_loc.tile_coords());
        let blocked = level
            .get_loaded_tile(d_loc)
            .and_then(|tile| Some(tile.blocked))
            .unwrap_or(true);

        if !blocked {
            self.pos = d_loc;
        }
    }

    pub fn draw(&self, con: &mut dyn Console, game: &Game) {
        if game.camera.in_view(&self.pos) {
            let offset_pos = self.pos - game.camera.pos;
            con.set_default_foreground(self.color);
            con.put_char(offset_pos.x, offset_pos.y, self.char, BackgroundFlag::None);
        }
    }
}
