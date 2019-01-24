use std::sync::Arc;
use std::sync::RwLock;

use super::*;

pub struct TestMotor {
    inverted: Arc<RwLock<bool>>,
    speed: Arc<RwLock<f32>>,
}

impl MotorController for TestMotor {
    fn set_speed(&mut self, new_speed: f32) {
        let new_speed = if *self.inverted.read().unwrap() {
            -new_speed
        } else {
            new_speed
        };
        *self.speed.write().unwrap() = new_speed;
    }

    fn stop(&mut self) {
        self.set_speed(0.0)
    }

    fn invert(&mut self) {
        let inverted = *self.inverted.read().unwrap();
        *self.inverted.write().unwrap() = !inverted;
        self.stop()
    }

    fn get_motor_state(&self) -> MotorState {
        unimplemented!()
    }
}

impl TestMotor {
    pub fn new(inverted: Arc<RwLock<bool>>, speed: Arc<RwLock<f32>>) -> TestMotor {
        TestMotor {
            inverted,
            speed,
        }
    }
}