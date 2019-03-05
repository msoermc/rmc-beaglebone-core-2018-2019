use std::sync::Arc;
use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;

use rocket::http::Status;

use crate::robot_map::*;

use super::*;

#[cfg(test)]
mod initialization;

#[cfg(test)]
mod mechatronics;

#[cfg(test)]
mod switching;

#[cfg(test)]
mod benchmarking;