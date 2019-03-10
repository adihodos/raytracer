use std::cell::RefCell;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct BasicTimer {
    start: RefCell<Instant>,
    end: RefCell<Instant>,
}

impl BasicTimer {
    pub fn new() -> BasicTimer {
        BasicTimer {
            start: RefCell::new(Instant::now()),
            end: RefCell::new(Instant::now()),
        }
    }

    pub fn start(&self) {
        self.start.replace(Instant::now());
    }

    pub fn end(&self) {
        self.end.replace(Instant::now());
    }

    fn elapsed(&self) -> Duration {
        let s = *self.start.borrow();
        let e = *self.end.borrow();

        e - s
    }

    pub fn elapsed_millis(&self) -> f32 {
        self.elapsed().as_millis() as f32
    }

    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed().as_secs() as f32
    }
}
