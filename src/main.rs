use std::time::Instant;
use std::{env, path};

use ggez::event::{
    Axis, Button, ErrorOrigin, EventHandler, GamepadId, KeyCode, KeyMods, MouseButton,
};
use ggez::graphics::{Color, DrawParam, FilterMode, Rect, Text};
use ggez::input::keyboard::{is_key_pressed, pressed_keys};
use ggez::mint::Point2;
use ggez::{event, graphics, timer, Context, ContextBuilder, GameError, GameResult};
use specs::{Dispatcher, DispatcherBuilder, World, WorldExt};

use objects::camera::Camera;
use world::level::Level;
use world::position::Position;

use crate::objects::entities::Entities;
use crate::objects::sprite_atlas::SpriteAtlas;
use crate::systems::ai_system::AiSystem;
use crate::systems::chunk_system::{ChunkLoader, ChunkSystem};
use crate::systems::control_system::{Control, ControlSystem, Keyboard};
use crate::systems::health_system::HealthSystem;
use crate::systems::movement_system::{Movement, MovementSystem};
use crate::systems::render_system::RenderSystem;
use crate::world::chunk::Chunk;
use crate::world::position::{ChunkPosition, TilePosition, WorldPosition};
use crate::world::tile::Tile;

pub mod gui;
pub mod objects;
pub mod systems;
pub mod utils;
pub mod world;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

pub const RES_WIDTH: f32 = 640.0;
pub const RES_HEIGHT: f32 = 480.0;

const MAX_FPS: i32 = 20;

struct Player {
    pos: Position,
}

pub struct Rusted {
    world: World,
    camera: Camera,
    atlas: SpriteAtlas,
    player: Player,
    dispatcher: Dispatcher<'static, 'static>,
}

impl Rusted {
    pub fn new(ctx: &mut Context) -> Self {
        let mut world = World::new();

        let mut level = Level::new(rand::random::<i32>());
        level.load_chunk(pos!(0, 0));

        let camera = Camera {
            pos: Position { x: 0, y: 0 },
            zoom: 1.0,
            width: RES_WIDTH as i32,
            height: RES_HEIGHT as i32,
        };

        let atlas = SpriteAtlas::new(ctx, "/atlas.png", 32);

        let atlas_resource = SpriteAtlas::new(ctx, "/atlas.png", 32);
        let camera_resource = Camera {
            pos: Position { x: 0, y: 0 },
            zoom: 1.0,
            width: RES_WIDTH as i32,
            height: RES_HEIGHT as i32,
        };

        world.insert(level);
        world.insert(Keyboard::default());
        world.insert(atlas_resource);
        world.insert(camera_resource);

        let mut dispatcher = DispatcherBuilder::new()
            .with(ControlSystem, "control", &[])
            .with(AiSystem, "ai", &[])
            .with(MovementSystem, "movement", &["control", "ai"])
            .with(ChunkSystem, "chunk", &[])
            .with(HealthSystem, "health", &[])
            .with_thread_local(RenderSystem)
            .build();

        dispatcher.setup(&mut world);

        // Spawn Player
        let e = Entities::create_player(&mut world);
        Entities::create_ai(&mut world, e);
        Entities::create_ai(&mut world, e);
        Entities::create_ai(&mut world, e);
        Entities::create_ai(&mut world, e);
        Entities::create_ai(&mut world, e);

        Rusted {
            world,
            camera,
            atlas,
            player: Player {
                pos: Position { x: 0, y: 0 },
            },
            dispatcher,
        }
    }
}

impl EventHandler for Rusted {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        const MTP: u32 = 20;

        while timer::check_update_time(ctx, MTP) {
            let pressed_keys = ggez::input::keyboard::pressed_keys(ctx);
            let active_mods = ggez::input::keyboard::active_mods(ctx);

            ggez::input::keyboard::pressed_keys(ctx);

            {
                let mut world = &self.world;
                let mut keyboard = world.write_resource::<Keyboard>();
                *keyboard = Keyboard {
                    pressed_keys: pressed_keys.clone(),
                    active_mods,
                };
            }

            {
                self.dispatcher.dispatch(&mut self.world);
                self.world.maintain();
            }

            if is_key_pressed(ctx, KeyCode::W) && is_key_pressed(ctx, KeyCode::LControl) {
                ctx.continuing = false;
            }

            {
                graphics::clear(
                    ctx,
                    Color::new(10.0 / 256.0, 34.0 / 256.0, 34.0 / 256.0, 1.0),
                );

                let world = &mut self.world;
                let mut world_atlas = world.write_resource::<SpriteAtlas>();
                world_atlas.draw(ctx);

                let camera = world.read_resource::<Camera>();
                let level = world.read_resource::<Level>();

                let mb = &mut graphics::MeshBuilder::new();

                let factor = Chunk::SIZE as f32 * Tile::SIZE * camera.zoom;
                mb.line(
                    &[[0.0, 0.0], [0.0, factor], [factor, factor], [factor, 0.0]],
                    4.0,
                    Color::RED,
                );

                let mesh = mb.build(ctx).unwrap();

                for chunk in level.loaded_chunks.values() {
                    let screen_pos = ChunkPosition::to_screen(chunk.pos, &*camera);
                    graphics::draw(
                        ctx,
                        &mesh,
                        DrawParam::default().dest([
                            screen_pos.x as f32 * camera.zoom,
                            screen_pos.y as f32 * camera.zoom,
                        ]),
                    );
                }

                // Render fps
                let fps = timer::fps(ctx) as i32;
                let mut text = Text::new(format!("FPS: {}", fps));
                let player_pos = camera.pos + pos!(camera.width / 2, camera.height / 2);
                let center_screen_chunk = WorldPosition::to_chunk(player_pos);
                text.add(format!(
                    "\nChunk pos: {}  {}",
                    center_screen_chunk.x, center_screen_chunk.y
                ));
                text.add(format!("\nLoaded chunks: {}", level.loaded_chunks.len()));
                text.add(format!("\nZoom: {}", camera.zoom));
                graphics::draw(ctx, &text, DrawParam::default());

                graphics::present(ctx);
            }
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let pressed_keys = ggez::input::keyboard::pressed_keys(ctx);
            let active_mods = ggez::input::keyboard::active_mods(ctx);

            ggez::input::keyboard::pressed_keys(ctx);

            {
                let mut world = &self.world;
                let mut keyboard = world.write_resource::<Keyboard>();
                *keyboard = Keyboard {
                    pressed_keys: pressed_keys.clone(),
                    active_mods,
                };
            }

            {
                self.dispatcher.dispatch(&mut self.world);
                self.world.maintain();
            }

            if is_key_pressed(ctx, KeyCode::W) && is_key_pressed(ctx, KeyCode::LControl) {
                ctx.continuing = false;
            }

            {
                graphics::clear(
                    ctx,
                    Color::new(10.0 / 256.0, 34.0 / 256.0, 34.0 / 256.0, 1.0),
                );

                let world = &mut self.world;
                let mut world_atlas = world.write_resource::<SpriteAtlas>();
                world_atlas.draw(ctx)?;

                let camera = world.read_resource::<Camera>();
                let level = world.read_resource::<Level>();

                let mb = &mut graphics::MeshBuilder::new();

                let factor = Chunk::SIZE as f32 * Tile::SIZE * camera.zoom;
                mb.line(
                    &[[0.0, 0.0], [0.0, factor], [factor, factor], [factor, 0.0]],
                    4.0,
                    Color::RED,
                )?;

                let mesh = mb.build(ctx)?;

                for chunk in level.loaded_chunks.values() {
                    let screen_pos = ChunkPosition::to_screen(chunk.pos, &*camera);
                    graphics::draw(
                        ctx,
                        &mesh,
                        DrawParam::default().dest([
                            screen_pos.x as f32 * camera.zoom,
                            screen_pos.y as f32 * camera.zoom,
                        ]),
                    )?;
                }

                // Render fps
                let fps = timer::fps(ctx) as i32;
                let mut text = Text::new(format!("FPS: {}", fps));
                let player_pos = camera.pos + pos!(camera.width / 2, camera.height / 2);
                let center_screen_chunk = WorldPosition::to_chunk(player_pos);
                text.add(format!(
                    "\nChunk pos: {}  {}",
                    center_screen_chunk.x, center_screen_chunk.y
                ));
                text.add(format!("\nLoaded chunks: {}", level.loaded_chunks.len()));
                text.add(format!("\nZoom: {}", camera.zoom));
                graphics::draw(ctx, &text, DrawParam::default());

                graphics::present(ctx)?;
            }
        }

        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let mut camera = self.world.write_resource::<Camera>();
        println!("Resize event: {}x{}", camera.width, camera.height);
        camera.width = width as i32;
        camera.height = height as i32;

        graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height));
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, dx: f32, dy: f32) {
        let mut camera = self.world.write_resource::<Camera>();
        if dy == 1.0 {
            camera.zoom += 0.1;
        } else if dy == -1.0 {
            camera.zoom += -0.1;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        return Ok(());

        /*
        let atlas = &mut self.atlas;
        let level = &mut self.level;
        let camera = &self.camera;

        // Clear previous frame
        atlas.clear();
        graphics::clear(ctx, Color::BLACK);

        // Render level
        level.render(atlas, camera)?;

        // Render player
        self.player.render(ctx, atlas, camera)?;

        self.atlas.draw(ctx)?;

        let mb = &mut graphics::MeshBuilder::new();

        let factor = Chunk::SIZE as f32 * Tile::SIZE * camera.zoom;
        mb.line(
            &[[0.0, 0.0], [0.0, factor], [factor, factor], [factor, 0.0]],
            4.0,
            Color::RED,
        )?;

        let mesh = mb.build(ctx)?;

        for chunk in level.loaded_chunks.values() {
            let screen_pos = ChunkPosition::to_screen(chunk.pos, camera);
            graphics::draw(
                ctx,
                &mesh,
                DrawParam::default().dest([screen_pos.x as f32, screen_pos.y as f32]),
            )?;
        }

        // Render fps
        let fps = timer::fps(ctx) as i32;
        let mut text = Text::new(format!("FPS: {}", fps));
        let player_chunk_pos = TilePosition::to_chunk(self.player.pos);
        text.add(format!(
            "\nChunk pos: {}  {}",
            player_chunk_pos.x, player_chunk_pos.y
        ));
        graphics::draw(ctx, &text, DrawParam::default());

        graphics::present(ctx)?;
        Ok(())
        */
    }
}

fn main() {
    let mut context_builder = ContextBuilder::new("Rusted", "Volcano")
        .window_setup(ggez::conf::WindowSetup::default().title("Rusty"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(RES_WIDTH, RES_HEIGHT)
                .resizable(true),
        );

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        println!("Adding path {:?}", path);
        context_builder = context_builder.add_resource_path(path);
    }

    let (mut ctx, event_loop) = context_builder
        .build()
        .expect("Failed to build ggez context");

    graphics::set_default_filter(&mut ctx, FilterMode::Nearest);

    let rusted = Rusted::new(&mut ctx);

    println!("{}", graphics::renderer_info(&ctx).unwrap());
    event::run(ctx, event_loop, rusted);
}
