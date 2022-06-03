use crate::Position;

pub struct Camera {
    pub(crate) pos: Position,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

impl Camera {
    pub fn in_view(&self, pos: &Position) -> bool {
        return pos.x >= self.pos.x
            && pos.y >= self.pos.y
            && pos.x < self.pos.x + self.width
            && pos.y < self.pos.y + self.height;
    }
}
