use emerald::{GameSettings, RenderSettings, World};
use specs::{ReadStorage, System};
use std::process;

use objects::camera::Camera;
use tcod::colors;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode;

pub mod objects;
pub mod utils;
pub mod world;

use crate::objects::sprite::EntitySprite;
use crate::world::tile::TILE_SIZE;
use objects::game::GameObject;
use world::chunk::CHUNK_SIZE;
use world::level::Level;
use world::position::Position;
use world::position::DIRECTIONS;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAX_FPS: i32 = 20;

pub struct Tcon {
    root: Root,
    con: Offscreen,
}

pub struct Game {
    level: Level,
    player: GameObject,
    camera: Camera,
}

pub struct Rust {
    world: World,
    level: Level,
}

impl Game for Rust {}

pub const RES_WIDTH: f32 = 640.0;
pub const RES_HEIGHT: f32 = 480.0;

fn main() {
    let mut settings = GameSettings::default();
    let render_settings = RenderSettings {
        resolution: (RES_WIDTH as u32, RES_HEIGHT as u32),
        ..Default::default()
    };
    settings.render_settings = render_settings;

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust")
        .init();

    let camera = Camera {
        pos: pos!(0, 0),
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
    };

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut tcon = Tcon { root, con };

    let mut level = Level::new(rand::random::<i32>());
    level.load_chunk(pos!(0, 0));
    let player = GameObject::new(0, 0, '@', colors::DARK_AMBER);

    let mut game = Game {
        level,
        player,
        camera,
    };

    {
        // Render the initial state
        tcon.con.set_default_background(colors::BLACK);
        // Clear previous frame
        tcon.con.clear();
        tcon.root.clear();

        tick(&mut game);
        render(&mut tcon, &game);
        tcon.root.flush();
    }

    while !tcon.root.window_closed() {
        tcon.con.set_default_background(colors::BLACK);
        // Clear previous frame
        tcon.con.clear();
        tcon.root.clear();

        handle_keys(&mut tcon, &mut game);
        tick(&mut game);
        render(&mut tcon, &game);

        tcon.root.flush();
    }
}

fn tick(game: &mut Game) {
    let player_chunk_pos = game.player.pos.chunk_coords();
    let mut to_unload = Vec::<Position>::new();

    for (chunk_pos, _) in &game.level.loaded_chunks {
        if !chunk_pos.is_adjacent(player_chunk_pos) {
            to_unload.push(chunk_pos.clone());
        }
    }

    for dir in DIRECTIONS {
        let chunk_pos = dir + player_chunk_pos;
        if !game.level.is_loaded(&chunk_pos) {
            game.level.load_chunk(chunk_pos);
        }
    }

    for pos in to_unload {
        game.level.unload_chunk(pos);
    }
}

fn render(tcon: &mut Tcon, game: &Game) {
    // Render map
    tcon.con.set_default_foreground(colors::DARK_GREY);
    for (_, loaded_chunk) in &game.level.loaded_chunks {
        loaded_chunk.render(tcon, game);
    }

    // Render player
    game.player.draw(&mut tcon.con, game);

    // Blit to root console
    blit(
        &tcon.con,
        (0, 0),
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut &tcon.root,
        (0, 0),
        1.0,
        1.0,
    );

    // Render player coords
    let x = format!("{}", game.player.pos.x);
    let y = format!("{}", game.player.pos.y);

    tcon.con.set_default_foreground(colors::WHITE);
    for (i, c) in x.char_indices() {
        tcon.root.put_char(i as i32, 0, c, BackgroundFlag::None)
    }
    for (i, c) in y.char_indices() {
        tcon.root.put_char(i as i32, 1, c, BackgroundFlag::None)
    }
}

fn handle_keys(tcon: &mut Tcon, game: &mut Game) -> bool {
    let key = tcon.root.wait_for_keypress(true);

    let player = &mut game.player;
    let level = &game.level;

    match key {
        Key {
            code: KeyCode::Up, ..
        } => player.move_by(0, -1, level),
        Key {
            code: KeyCode::Down,
            ..
        } => player.move_by(0, 1, level),
        Key {
            code: KeyCode::Left,
            ..
        } => player.move_by(-1, 0, level),
        Key {
            code: KeyCode::Right,
            ..
        } => player.move_by(1, 0, level),
        Key {
            printable: 'w',
            ctrl: true,
            ..
        } => process::exit(0),
        _ => {}
    }

    game.camera.pos = player.pos.clone() - pos!(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
    false
}
