use std::collections::LinkedList;

use noise::{NoiseFn, Perlin, Seedable};
use tcod::{colors, BackgroundFlag, Console};

use crate::{objects::game::GameObject, pos, Game, Tcon};

use super::{position::Position, tile::Tile};

pub const CHUNK_SIZE: i32 = 32;

type Tiles = [[Tile; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];

#[derive(Debug)]
pub struct UnloadedChunk {
    pub tiles: Tiles,
    pub pos: Position,
    pub objects: Vec<GameObject>,
}

impl UnloadedChunk {
    pub fn generate(chunk_pos: Position, seed: i32) -> Self {
        let mut tiles = [[Tile::empty(); CHUNK_SIZE as usize]; CHUNK_SIZE as usize];

        let mut perlin = Perlin::new();
        perlin = Seedable::set_seed(perlin, seed as u32);

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let val = perlin.get([
                    (x + chunk_pos.x * CHUNK_SIZE) as f64 / 15.0,
                    (y + chunk_pos.y * CHUNK_SIZE) as f64 / 15.0,
                ]);

                if val > 0.5 {
                    tiles[x as usize][y as usize] = Tile::wall();
                }
            }
        }

        UnloadedChunk {
            tiles,
            pos: chunk_pos,
            objects: Vec::with_capacity(0),
        }
    }

    pub fn to_loaded(self) -> LoadedChunk {
        LoadedChunk {
            tiles: self.tiles,
            pos: self.pos,
            objects: LinkedList::from_iter(self.objects.into_iter()),
        }
    }
}

#[derive(Debug)]
pub struct LoadedChunk {
    pub tiles: Tiles,
    pub pos: Position,
    pub objects: LinkedList<GameObject>,
}

impl LoadedChunk {
    pub fn render(&self, tcon: &mut Tcon, game: &Game) {
        // Render tiles
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let tile = self.tiles[x as usize][y as usize];
                let tile_pos = (self.pos * CHUNK_SIZE) + pos!(x, y);

                if game.camera.in_view(&tile_pos) {
                    let offset_pos = tile_pos - game.camera.pos;

                    if x == 0 || y == 0 || x == CHUNK_SIZE - 1 || y == CHUNK_SIZE - 1 {
                        if tile.blocked {
                            tcon.con.put_char(
                                offset_pos.x,
                                offset_pos.y,
                                tile.char,
                                BackgroundFlag::Set,
                            );
                            tcon.con.set_char_background(
                                offset_pos.x as i32,
                                offset_pos.y as i32,
                                colors::GREEN,
                                BackgroundFlag::Set,
                            );
                        } else {
                            tcon.con.put_char(
                                offset_pos.x,
                                offset_pos.y,
                                tile.char,
                                BackgroundFlag::Set,
                            );
                            tcon.con.set_char_background(
                                offset_pos.x as i32,
                                offset_pos.y as i32,
                                if game.player.pos.chunk_coords() == self.pos {
                                    colors::ORANGE
                                } else {
                                    colors::RED
                                },
                                BackgroundFlag::Set,
                            );
                        }
                    } else {
                        if tile.blocked {
                            tcon.con.put_char(
                                offset_pos.x,
                                offset_pos.y,
                                tile.char,
                                BackgroundFlag::Set,
                            );
                            tcon.con.set_char_background(
                                offset_pos.x as i32,
                                offset_pos.y as i32,
                                colors::BLUE,
                                BackgroundFlag::Set,
                            );
                        } else {
                            tcon.con.put_char(
                                offset_pos.x,
                                offset_pos.y,
                                tile.char,
                                BackgroundFlag::None,
                            );
                        }
                    }
                }
            }
        }

        // Render objects
        for object in &self.objects {
            object.draw(&mut tcon.con, game);
        }
    }

    pub fn void() -> Self {
        LoadedChunk {
            tiles: [[Tile::empty(); CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
            pos: pos!(0, 0),
            objects: LinkedList::new(),
        }
    }

    pub fn to_unloaded(self) -> UnloadedChunk {
        UnloadedChunk {
            tiles: self.tiles,
            pos: self.pos,
            objects: Vec::from_iter(self.objects.into_iter()),
        }
    }
}
