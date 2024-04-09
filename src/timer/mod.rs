#[cfg(test)]
mod tests;

use std::time::{Instant, Duration};

pub struct Timer {
    _accumulator: Duration,
    _started: bool,
    _instant: Instant
}
 
impl Timer {
    pub fn new() -> Timer {
        Timer {_accumulator: Duration::new(0, 0),
            _started: false,
            _instant: Instant::now()
        }
    }

    pub fn start(& mut self) {
        if !self._started {
            self._started = true;
            self._instant = Instant::now();
        }
    }

    pub fn stop(&mut self) {
        if self._started {
            self._accumulator += self._instant.elapsed();
            self._started = false;
        }
    }
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self._accumulator = Duration::new(0, 0);
        self._started = false;
    }

    pub fn get_time(&self) -> &Duration {
        &self._accumulator
    }

    pub fn get_time_instant(&self) -> Duration {
        self._instant.elapsed()
    }
}