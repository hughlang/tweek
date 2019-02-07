#![crate_name = "tween"]
#![crate_type = "lib"]

#![feature(duration_float)]
// #![feature(type_ascription)]

#[macro_use] extern crate unwrap_to;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

extern crate crossbeam_channel;
extern crate crossbeam_utils;
extern crate cgmath;
extern crate ggez;

pub use crossbeam_channel::*;
pub use crossbeam_utils::*;

pub use crate::core::*;

pub mod core;

