#![crate_name = "tween"]
#![crate_type = "lib"]

#![feature(duration_float)]

#[macro_use] extern crate unwrap_to;

extern crate cgmath;
extern crate ggez;

pub use crate::core::*;
pub mod core;

