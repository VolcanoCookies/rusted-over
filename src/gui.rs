use ggez::Context;

pub struct Gui;

impl Gui {
    pub fn new(ctx: &Context) -> Self {
        Self
    }

    pub fn render(&mut self, ctx: &mut Context) {
        //let w = ctx.conf.window_mode.width;
        //let h = ctx.conf.window_mode.height;
    }

    pub fn update_mouse_pos(&mut self, x: i32, y: i32) {}

    pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {}
}
