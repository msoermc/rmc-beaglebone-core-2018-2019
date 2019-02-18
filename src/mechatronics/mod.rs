use std::sync::Arc;
use std::sync::mpsc::Sender;

use crate::mechatronics::commands::RobotCommand;
use crate::status::robot_state::GlobalRobotState;

/// The controller module contains the `RobotController` struct.
/// The `RobotController` struct owns instances of the `DriveTrain` and the `MaterialHandler`.
pub mod controller;

pub mod commands;

/// The drive_train module contains the `DriveTrain` struct.
/// That structure is used to manage the physical drive train and perform operations on it.
pub mod drive_train;

pub mod dumper;

pub mod bucket_ladder;

pub struct MechatronicsMessengerFactory {
    messenger: MechatronicsMessageSender,
}

impl MechatronicsMessengerFactory {
    pub fn new(messenger: MechatronicsMessageSender) -> Self {
        MechatronicsMessengerFactory {
            messenger
        }
    }

    pub fn generate(&self) -> MechatronicsMessageSender {
        self.messenger.clone()
    }
}

#[derive(Clone)]
pub struct MechatronicsMessageSender {
    channel: Sender<Box<RobotCommand>>,
}

impl MechatronicsMessageSender {
    pub fn new(channel: Sender<Box<RobotCommand>>) -> Self {
        Self {
            channel,
        }
    }

    #[inline]
    pub fn send_command(&self, command: Box<RobotCommand>) {
        self.channel.send(command).unwrap();
    }
}