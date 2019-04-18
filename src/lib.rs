#![crate_name = "tweek"]
#![crate_type = "lib"]
#![feature(duration_float)]

#[macro_use]
extern crate unwrap_to;
extern crate cgmath;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_mut)]
extern crate ggez;
extern crate glyph_brush;
extern crate image;
extern crate rand;
extern crate uuid;

// pub use crate::core::*;
// pub use crate::ggez::*;

pub mod core;
pub mod ggez_support;
pub mod prelude;
