use std::thread;
use std::time::{Duration, Instant};

/// Keeps a consistent framerate.

pub struct Clock {
    creation_time: Instant,
    past: [Duration; 2],
    lifetime: Duration,
    cycles: u64,
}

impl Clock {
    pub fn new() -> Self {
        let creation_time = Instant::now();
        let lifetime = Duration::new(0, 0);
        let past = [Duration::default(), Duration::default()];
        Self {
            creation_time,
            past,
            lifetime,
            cycles: 0,
        }
    }

    pub fn update(&mut self) {
        self.cycles += 1;
        let now = Instant::now();
        let time_delta = now.duration_since(self.creation_time + self.lifetime);
        self.lifetime = now.duration_since(self.creation_time);
        self.past[1] = self.past[0];
        self.past[0] = time_delta;
        if self.cycles % 15 == 0 {
            let fps = 1.0 / time_delta.as_secs_f32();
            println!(
                "playtime: {:.2}, fps: {fps:.2}",
                self.lifetime.as_secs_f32()
            );
        }
    }

    // Should slowdown updates to 30fps, but, it looks like, actually slows down to 60fps
    pub fn sleep(&mut self) {
        self.update();
        let average_delta = (self.past[0] + self.past[1]).as_secs_f32() / 2.0;
        // approximate frame time for 30fps;
        let frame_time = Duration::new(0, 30_000_000).as_secs_f32();
        let frame_slowdown = if average_delta < frame_time {
            Duration::from_secs_f32(frame_time - average_delta)
        } else {
            Duration::new(0, 0)
        };
        thread::sleep(frame_slowdown);
    }
}
