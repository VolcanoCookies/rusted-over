use emerald::nalgebra::Transform2;
use emerald::tilemap::Tilemap;
use emerald::{Emerald, Game, GameSettings, RenderSettings, Sprite, Transform, Vector2, World};
use ggez::ContextBuilder;

use objects::camera::Camera;

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

pub struct Rust {
    world: World,
    level: Level,
}

impl Rust {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut level = Level::new(rand::random::<i32>());
        level.load_chunk(pos!(0, 0));

        Rust { world, level }
    }
}

impl Game for Rust {
    fn initialize(&mut self, mut emd: Emerald) {
        emd.set_asset_folder_root(String::from("./assets/"));
        let texture_key = emd.loader().texture("atlas.png").unwrap();
        let mut tilemap = Tilemap::new(texture_key, Vector2::new(16, 16), 4, 4);

        let sprite = emd.loader().sprite("atlas.png").unwrap();
        self.world.spawn((sprite, Transform::default()));
    }

    fn update(&mut self, mut emd: Emerald<'_, '_, '_>) {
        for (_, (_, transform)) in self.world.query::<(&Sprite, &mut Transform)>().iter() {
            transform.translation.x += 1.0;
        }
    }

    fn draw(&mut self, mut emd: Emerald) {
        emd.graphics().begin().unwrap();
        emd.graphics().draw_world(&mut self.world).unwrap();
        emd.graphics().render().unwrap();
    }
}

pub const RES_WIDTH: f32 = 640.0;
pub const RES_HEIGHT: f32 = 480.0;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Rusted", "Volcano")
        .build()
        .expect("aieee");

    let rust = Rust {

    }

    let mut settings = GameSettings::default();
    let render_settings = RenderSettings {
        resolution: (RES_WIDTH as u32, RES_HEIGHT as u32),
        ..Default::default()
    };
    settings.render_settings = render_settings;
    emerald::start(Box::new(Rust::new()), settings);
}

/*
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
*/
