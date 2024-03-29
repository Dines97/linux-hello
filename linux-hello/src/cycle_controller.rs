use std::{
    fmt, thread,
    time::{Duration, Instant},
};

pub struct CycleController {
    instant: Instant,
}

impl CycleController {
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.instant = Instant::now();
    }

    pub fn throttle(&self, cps: f64) {
        if cps <= 0_f64 {
            panic!("cps must be positive not zero value")
        }

        let spc: f64 = 1_f64 / cps;
        let sleep_duration =
            Duration::from_secs_f64(f64::max(0_f64, spc - self.instant.elapsed().as_secs_f64()));
        thread::sleep(sleep_duration)
    }
}

impl fmt::Display for CycleController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cps: f64 = 1_f64 / f64::max(self.instant.elapsed().as_secs_f64(), f64::MIN_POSITIVE);

        write!(f, "CPS: {}", cps)
    }
}
