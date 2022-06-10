use ggez::graphics::spritebatch::{SpriteBatch, SpriteIdx};
use ggez::graphics::{DrawParam, Image, Rect};
use ggez::{graphics, Context, GameResult};

use crate::{Point2, Tile};

#[derive(Clone, Copy, Debug)]
pub struct SpriteId {
    pub x: usize,
    pub y: usize,
}

impl SpriteId {
    pub fn new(x: usize, y: usize) -> Self {
        SpriteId { x, y }
    }

    pub const EMPTY: SpriteId = SpriteId { x: 0, y: 0 };
    pub const PLAYER: SpriteId = SpriteId { x: 2, y: 0 };
    pub const TREE_A: SpriteId = SpriteId { x: 3, y: 0 };
    pub const TREE_B: SpriteId = SpriteId { x: 4, y: 0 };

    pub const WALL_S_OPEN: SpriteId = SpriteId { x: 0, y: 1 };
    pub const WALL_NS_OPEN: SpriteId = SpriteId { x: 0, y: 2 };
    pub const WALL_N_OPEN: SpriteId = SpriteId { x: 0, y: 3 };

    pub const WALL_ES_OPEN: SpriteId = SpriteId { x: 1, y: 1 };
    pub const WALL_NES_OPEN: SpriteId = SpriteId { x: 1, y: 2 };
    pub const WALL_NE_OPEN: SpriteId = SpriteId { x: 1, y: 3 };
    pub const WALL_ESW_OPEN: SpriteId = SpriteId { x: 2, y: 1 };
    pub const WALL_NESW_OPEN: SpriteId = SpriteId { x: 2, y: 2 };
    pub const WALL_NEW_OPEN: SpriteId = SpriteId { x: 2, y: 3 };
    pub const WALL_SW_OPEN: SpriteId = SpriteId { x: 3, y: 1 };
    pub const WALL_NSW_OPEN: SpriteId = SpriteId { x: 3, y: 2 };
    pub const WALL_NW_OPEN: SpriteId = SpriteId { x: 3, y: 3 };

    pub const WALL_ES_OPEN_CORNER: SpriteId = SpriteId { x: 4, y: 1 };
    pub const WALL_NES_OPEN_CORNER: SpriteId = SpriteId { x: 4, y: 2 };
    pub const WALL_NE_OPEN_CORNER: SpriteId = SpriteId { x: 4, y: 3 };
    pub const WALL_ESW_OPEN_CORNER: SpriteId = SpriteId { x: 5, y: 1 };
    pub const WALL_NESW_OPEN_CORNER: SpriteId = SpriteId { x: 5, y: 2 };
    pub const WALL_NEW_OPEN_CORNER: SpriteId = SpriteId { x: 5, y: 3 };
    pub const WALL_SW_OPEN_CORNER: SpriteId = SpriteId { x: 6, y: 1 };
    pub const WALL_NSW_OPEN_CORNER: SpriteId = SpriteId { x: 6, y: 2 };
    pub const WALL_NW_OPEN_CORNER: SpriteId = SpriteId { x: 6, y: 3 };
}

pub struct SpriteAtlas {
    pub batch: SpriteBatch,
    cell_size: usize,
    cell_width: f32,
    cell_height: f32,
    width: usize,
    height: usize,
}

impl SpriteAtlas {
    pub fn from_batch(
        batch: SpriteBatch,
        cell_size: usize,
        cells_x: usize,
        cells_y: usize,
    ) -> Self {
        let width = cell_size * cells_x;
        let height = cell_size * cells_y;
        let cell_width = 1.0 / cells_x as f32;
        let cell_height = 1.0 / cells_y as f32;

        SpriteAtlas {
            batch,
            cell_size,
            cell_width,
            cell_height,
            width,
            height,
        }
    }

    pub fn new(ctx: &mut Context, path: &str, cell_size: usize) -> Self {
        let image = Image::new(ctx, path).unwrap();
        let width = image.width() as usize / cell_size;
        let height = image.height() as usize / cell_size;
        let batch = SpriteBatch::new(image.clone());

        let cell_width = cell_size as f32 / image.width() as f32;
        let cell_height = cell_size as f32 / image.height() as f32;

        SpriteAtlas {
            batch,
            cell_size,
            cell_width,
            cell_height,
            width,
            height,
        }
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.batch, DrawParam::default())?;
        Ok(())
    }

    pub fn add(&mut self, sprite_id: &SpriteId, x: f32, y: f32, zoom: f32) -> SpriteIdx {
        if sprite_id.x >= self.width || sprite_id.y >= self.height {
            panic!("Sprite id {} {} is out of bounds", sprite_id.x, sprite_id.y);
        }

        let sprite_x = sprite_id.x as f32 / self.width as f32;
        let sprite_y = sprite_id.y as f32 / self.height as f32;

        let src = Rect::new(sprite_x, sprite_y, self.cell_width, self.cell_height);

        let scale_factor = Tile::SIZE / self.cell_size as f32 * zoom;

        let draw_params = DrawParam::new()
            .dest(Point2 {
                x: x * zoom,
                y: y * zoom,
            })
            .src(src)
            .scale([scale_factor, scale_factor]);

        return self.batch.add(draw_params);
    }
}
