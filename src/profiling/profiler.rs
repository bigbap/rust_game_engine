use crate::prelude::qp_core::Timer;

pub struct Profiler {
    timer: Timer
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            timer: Timer::new()
        }
    }

    pub fn begin(&mut self) {
        self.timer.delta();
    }

    pub fn end(&mut self) -> f32 {
        self.timer.delta() * 1000.0
    }
}