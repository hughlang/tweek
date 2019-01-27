#![crate_name = "tween"]
#![crate_type = "lib"]

#![feature(duration_float)]

#[macro_use] extern crate unwrap_to;

extern crate nalgebra as na;
// extern crate nalgebra_glm as glm;

// pub use crate::nalgebra::*;
pub use crate::core::*;

pub mod core;

// pub use crate::easing::*;

// pub mod tween;
// pub mod prelude;
