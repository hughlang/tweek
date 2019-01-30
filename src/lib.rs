#![crate_name = "tween"]
#![crate_type = "lib"]

#![feature(duration_float)]

#[macro_use] extern crate unwrap_to;

extern crate crossbeam_channel;
extern crate crossbeam_utils;
extern crate cgmath;
// extern crate tokio;

// pub use tokio::io;
// pub use tokio::prelude::*;
pub use crossbeam_channel::*;
pub use crossbeam_utils::*;

pub use crate::core::*;

pub mod core;

