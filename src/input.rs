use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

pub struct Input {
    pub up: i8,
    pub down: i8,
    pub left: i8,
    pub right: i8,
}

impl Input {

    pub fn new() -> Input {
        Input {
            up: 0,
            down: 0,
            left: 0,
            right: 0,
        }
    }

    pub fn callback(&mut self, event: &sdl2::event::Event) {
        match event {
            Event::KeyDown {keycode: Some(key), ..} => {
                match key {
                    Keycode::D | Keycode::Right => {
                        self.right = 1;
                    },
                    Keycode::A | Keycode::Left => {
                        self.left = 1;
                    },
                    Keycode::W | Keycode::Up => {
                        self.up = 1;
                    },
                    Keycode::S | Keycode::Down => {
                        self.down = 1;
                    },
                    _ => {},
                }
            },
            Event::KeyUp {keycode: Some(key), ..} => {
                match key {
                    Keycode::D | Keycode::Right => {
                        self.right = 0;
                    },
                    Keycode::A | Keycode::Left => {
                        self.left = 0;
                    },
                    Keycode::W | Keycode::Up => {
                        self.up = 0;
                    },
                    Keycode::S | Keycode::Down => {
                        self.down = 0;
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}

