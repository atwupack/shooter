use std::cmp::{max, min};
use std::thread;
use std::time::{Duration, Instant};

pub struct FrameRateTimer {
    whole_ms_per_frame: i32,
    remainder_per_frame: f32,
    last_frame_tick: u128,
    remainder: f32,
    start: Instant,
}

impl FrameRateTimer {
    pub fn new(target_rate: u32) -> Self {
        let ms_per_frame = 1000.0 / target_rate as f32;
        FrameRateTimer {
            whole_ms_per_frame: ms_per_frame as i32,
            remainder_per_frame: ms_per_frame.fract(),
            last_frame_tick: 0,
            remainder: 0.0,
            start: Instant::now(),
        }
    }

    pub fn cap_frame_rate(&mut self) {
        let mut wait = self.whole_ms_per_frame + (self.remainder as i32);
        self.remainder = self.remainder.fract();
        let frame_time = self.start.elapsed().as_millis() - self.last_frame_tick;
        wait -= frame_time as i32;
        if wait < 1 {
            wait = 1;
        }
        thread::sleep(Duration::from_millis(wait as u64));
        self.remainder += self.remainder_per_frame;
        self.last_frame_tick = self.start.elapsed().as_millis();
    }
}

pub(crate) fn collision(
    x1: i32,
    y1: i32,
    w1: i32,
    h1: i32,
    x2: i32,
    y2: i32,
    w2: i32,
    h2: i32,
) -> bool {
    (max(x1, y1) < min(x1 + w1, x2 + w2)) && (max(y1, y2) < min(y1 + h1, y2 + h2))
}
