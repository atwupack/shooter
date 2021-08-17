use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use sdl2::event::Event;
use std::process::exit;

#[derive(Debug, Default)]
pub struct Inputs {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Inputs {

    pub fn up(&self) -> bool {
        self.up
    }

    pub fn down(&self) -> bool {
        self.down
    }

    pub fn left(&self) -> bool {
        self.left
    }

    pub fn right(&self) -> bool {
        self.right
    }

    pub fn do_key_down(&mut self, scancode: Option<Scancode>, repeat: bool) {
        if !repeat {
            match scancode {
                Some(Scancode::Up) => {self.up = true},
                Some(Scancode::Down) => {self.down = true},
                Some(Scancode::Left) => {self.left = true},
                Some(Scancode::Right) => {self.right = true},
                _ => {}
            }
        }
    }

    pub fn do_key_up(&mut self, scancode: Option<Scancode>, repeat: bool) {
        if !repeat {
            match scancode {
                Some(Scancode::Up) => {self.up = false},
                Some(Scancode::Down) => {self.down = false},
                Some(Scancode::Left) => {self.left = false},
                Some(Scancode::Right) => {self.right = false},
                _ => {}
            }
        }
    }
}

pub fn do_input(events: &mut EventPump, inputs: &mut Inputs) {
    for event in events.poll_iter() {
        match event {
            Event::Quit {..} => exit(0),
            Event::KeyDown { scancode, repeat, .. } => {
                inputs.do_key_down(scancode, repeat)
            },
            Event::KeyUp { scancode, repeat, .. } => {
                inputs.do_key_up(scancode, repeat)
            },
            _ => {}
        }
    }
}

