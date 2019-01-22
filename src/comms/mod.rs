use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::State;

use crate::control::RobotView;
use rocket_contrib::json::Json;
use rocket::config::Value;

/// A `SendableMessage` is an object that can be encoded as a message and sent off to another device.
pub trait SendableMessage: Send {
    fn encode(&self) -> String;
}

/// The `ServerSender` is a view into a `RobotCommunicator` that other threads/objects
/// can use to request that messages be sent.
#[derive(Clone, Debug)]
pub struct ServerSender {
    channel: Sender<Box<SendableMessage>>,
}

impl ServerSender {
    /// Sends a message to the remote receiver and returns `Err(LogData)` if the channel hangs up.
    pub fn send_message(&self, message: Box<SendableMessage>) {
        self.channel.send(message).expect("Failed to send message!");
    }

    /// Constructs a new `ServerSender`
    fn new(channel: Sender<Box<SendableMessage>>) -> Self {
        Self {
            channel
        }
    }
}

struct ServerState {
    receiver: Mutex<Receiver<Box<SendableMessage>>>,
    robot_controller: Mutex<RobotView>,
}

struct Drive {}

/// Launches the server
pub fn launch(robot_controller: RobotView) -> ServerSender {
    let (send, recv) = channel();

    let comms_view = ServerSender::new(send);

    let state = ServerState {
        receiver: Mutex::new(recv),
        robot_controller: Mutex::new(robot_controller),
    };
    rocket::ignite()
        .manage(state)
        .mount("/",
               routes![handle_drive,
                              handle_enable_drive,
                              handle_disable_drive,
                              handle_kill,
                              handle_revive,
                              handle_brake,
                              index,
                              files])
        .launch();

    comms_view
}

#[post("/drive/<left>/<right>")]
fn handle_drive(left: f32, right: f32, state: State<ServerState>) -> Status {
    info!("Received drive message: [{}, {}]", left, right);
    match state.robot_controller.lock() {
        Ok(controller) => if controller.drive(left, right).is_err() {
            Status::BadRequest
        } else {
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/enable/drive_train")]
fn handle_enable_drive(state: State<ServerState>) -> Status {
    info!("Received enable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.enable_drive_train();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/disable/drive_train")]
fn handle_disable_drive(state: State<ServerState>) -> Status {
    info!("Received disable drive message");
    match state.robot_controller.lock() {
        Ok(controller) => {
            controller.disable_drive_train();
            Status::Ok
        }
        Err(_) => Status::InternalServerError
    }
}

#[post("/kill")]
fn handle_kill(state: State<ServerState>) -> Status {
    info!("Received kill message");
    if state.robot_controller.lock().unwrap().kill().is_err() {
        Status::InternalServerError
    } else {
        Status::Ok
    }
}

#[post("/brake")]
fn handle_brake(state: State<ServerState>) -> Status {
    info!("Received brake message");
    state.robot_controller.lock().unwrap().brake();
    Status::Ok
}

#[post("/revive")]
fn handle_revive(state: State<ServerState>) -> Status {
    info!("Received revive message");
    if state.robot_controller.lock().unwrap().revive().is_err() {
        Status::InternalServerError
    } else {
        Status::Ok
    }
}

#[get("/")]
fn index() -> Option<NamedFile> {
    info!("Received static request for index!");
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    info!("Received static request: {:?}", file);
    NamedFile::open(Path::new("static/").join(file)).ok()
}