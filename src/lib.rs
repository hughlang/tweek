#![crate_name = "tween"]
#![crate_type = "lib"]

#![feature(duration_float)]

#[macro_use] extern crate unwrap_to;

extern crate cgmath;

pub use crate::core::*;

pub mod core;

// pub use crate::easing::*;

// pub mod tween;
// pub mod prelude;
