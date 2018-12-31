use crate::comms::reading::Command;
use crate::comms::driver_station::DriverStationInterface;
use crate::drive_train::DriveTrainCommand;
use crate::comms::reading::CommandReader;
use crate::logging::log_data::LogData;
use crate::comms::get_wrong_arg_count_log;

pub struct BrakeCommand {}

pub struct BrakeCommandParser {}

impl<I> Command<I> for BrakeCommand where I: DriverStationInterface {
    fn accept(&self, interface: &I) {
        let command = DriveTrainCommand::Stop;
        interface.send_drive_train_command(command);
    }
}

impl BrakeCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl BrakeCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl<I> CommandReader<I> for BrakeCommandParser where I: DriverStationInterface {
    fn read(&self, args: &[&str]) -> Result<Box<Command<I>>, LogData> {
        if args.len() > 1 {
            Err(get_wrong_arg_count_log(args, 1, args.len() as u64))
        } else {
            Ok(Box::new(BrakeCommand::new()))
        }
    }
}