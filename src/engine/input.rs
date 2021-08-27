use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use std::collections::HashSet;
use std::process::exit;

#[derive(Debug, Default)]
pub struct Inputs {
    keyboard: HashSet<Scancode>,
}

impl Inputs {
    pub fn up(&self) -> bool {
        self.keyboard.contains(&Scancode::Up)
    }

    pub fn down(&self) -> bool {
        self.keyboard.contains(&Scancode::Down)
    }

    pub fn left(&self) -> bool {
        self.keyboard.contains(&Scancode::Left)
    }

    pub fn right(&self) -> bool {
        self.keyboard.contains(&Scancode::Right)
    }

    pub fn fire(&self) -> bool {
        self.keyboard.contains(&Scancode::LCtrl)
    }

    pub fn do_key_down(&mut self, scancode: Option<Scancode>, repeat: bool) {
        if !repeat {
            if let Some(s) = scancode {
                self.keyboard.insert(s);
            }
        }
    }

    pub fn do_key_up(&mut self, scancode: Option<Scancode>, repeat: bool) {
        if !repeat {
            if let Some(s) = scancode {
                self.keyboard.remove(&s);
            }
        }
    }
}

pub fn do_input(events: &mut EventPump, inputs: &mut Inputs) {
    for event in events.poll_iter() {
        match event {
            Event::Quit { .. } => exit(0),
            Event::KeyDown {
                scancode, repeat, ..
            } => inputs.do_key_down(scancode, repeat),
            Event::KeyUp {
                scancode, repeat, ..
            } => inputs.do_key_up(scancode, repeat),
            _ => {}
        }
    }
}
