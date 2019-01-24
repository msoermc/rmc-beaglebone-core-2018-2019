use super::*;

const FLOAT_ERROR: f32 = 0.05;

pub struct PrintMotor {
    name: String,
    inverted: bool,
    last: f32,
    is_stopped: bool,
}

impl MotorController for PrintMotor {
    fn set_speed(&mut self, new_speed: f32) -> Result<(), MotorFailure> {
        if self.last - new_speed < FLOAT_ERROR && new_speed - self.last > FLOAT_ERROR {
            info!("{}: -> {}", self.name, new_speed);
            self.last = new_speed;
        }

        self.is_stopped = false;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), MotorFailure> {
        if !self.is_stopped {
            info!("{}: STOP", self.name);
            self.is_stopped = true;
        }
        Ok(())
    }

    fn invert(&mut self) -> Result<(), MotorFailure> {
        info!("{}: INVERT", self.name);
        self.inverted = !self.inverted;
        Ok(())
    }

    fn is_inverted(&self) -> Result<bool, MotorFailure> {
        Ok(self.inverted)
    }
}

impl PrintMotor {
    pub fn new(name: &str) -> PrintMotor {
        PrintMotor {
            name: name.to_string(),
            inverted: false,
            last: -10.0,
            is_stopped: false
        }
    }
}