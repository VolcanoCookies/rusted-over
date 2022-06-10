use crate::systems::movement_system::Movement;
use crate::{Level, Position, World};
use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard::is_key_pressed;
use ggez::winit::event::VirtualKeyCode;
use ggez::Context;
use specs::{
    AccessorCow, Component, Join, NullStorage, Read, ReadExpect, ReadStorage, RunningTime, System,
    VecStorage, WorldExt, WriteStorage,
};
use std::collections::HashSet;

pub struct Keyboard {
    pub pressed_keys: HashSet<VirtualKeyCode>,
    pub active_mods: KeyMods,
}

impl Keyboard {
    pub fn is_pressed(&self, code: VirtualKeyCode) -> bool {
        return self.pressed_keys.contains(&code);
    }

    pub fn any_pressed(&self, codes: &[VirtualKeyCode]) -> bool {
        for code in codes {
            if self.is_pressed(*code) {
                return true;
            }
        }
        return false;
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard {
            pressed_keys: HashSet::new(),
            active_mods: KeyMods::empty(),
        }
    }
}

pub struct Control;

impl Default for Control {
    fn default() -> Self {
        Control
    }
}

impl Component for Control {
    type Storage = NullStorage<Self>;
}

pub struct ControlSystem;
impl<'a> System<'a> for ControlSystem {
    type SystemData = (
        Read<'a, Keyboard>,
        WriteStorage<'a, Movement>,
        ReadStorage<'a, Control>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (keyboard, mut movements, control) = data;

        let dx = if keyboard.any_pressed(&[
            KeyCode::Numpad9,
            KeyCode::Numpad6,
            KeyCode::Numpad3,
            KeyCode::Right,
        ]) {
            1
        } else if keyboard.any_pressed(&[
            KeyCode::Numpad7,
            KeyCode::Numpad4,
            KeyCode::Numpad1,
            KeyCode::Left,
        ]) {
            -1
        } else {
            0
        };

        let dy = if keyboard.any_pressed(&[
            KeyCode::Numpad7,
            KeyCode::Numpad8,
            KeyCode::Numpad9,
            KeyCode::Up,
        ]) {
            -1
        } else if keyboard.any_pressed(&[
            KeyCode::Numpad1,
            KeyCode::Numpad2,
            KeyCode::Numpad3,
            KeyCode::Down,
        ]) {
            1
        } else {
            0
        };

        for (mov, _) in (&mut movements, &control).join() {
            mov.delta.x = dx;
            mov.delta.y = dy;
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<Movement>();
        world.register::<Control>();
    }
}
