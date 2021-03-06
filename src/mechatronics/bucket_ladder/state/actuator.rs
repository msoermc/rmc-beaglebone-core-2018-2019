use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct GlobalActuatorLimitState {
    upper: Arc<AtomicBool>,
    lower: Arc<AtomicBool>,
}

impl GlobalActuatorLimitState {
    pub fn new() -> Self {
        GlobalActuatorLimitState {
            upper: Arc::new(AtomicBool::new(false)),
            lower: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn get_current_state(&self) -> ActuatorLimitStateInstance {
        ActuatorLimitStateInstance::new(
            self.upper.load(Ordering::Relaxed),
            self.lower.load(Ordering::Relaxed),
        )
    }

    pub fn set_upper(&self, upper: bool) {
        self.upper.store(upper, Ordering::Relaxed);
    }

    pub fn set_lower(&self, lower: bool) {
        self.lower.store(lower, Ordering::Relaxed);
    }

    pub fn get_upper(&self) -> Arc<AtomicBool> {
        self.upper.clone()
    }

    pub fn get_lower(&self) -> Arc<AtomicBool> {
        self.lower.clone()
    }
}

#[derive(Serialize)]
pub struct ActuatorLimitStateInstance {
    upper: bool,
    lower: bool,
}

impl ActuatorLimitStateInstance {
    fn new(upper: bool, lower: bool) -> Self {
        ActuatorLimitStateInstance {
            upper,
            lower,
        }
    }

    pub fn get_upper(&self) -> bool {
        self.upper
    }

    pub fn get_lower(&self) -> bool {
        self.lower
    }
}