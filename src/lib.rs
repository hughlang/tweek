#![crate_name = "tweek"]
#![crate_type = "lib"]
#![feature(duration_float)]

// #[macro_use]
// extern crate lazy_static;
#[macro_use]
extern crate unwrap_to;

extern crate cgmath;
extern crate glyph_brush;
extern crate image;
extern crate quicksilver;
extern crate rand;
extern crate uuid;

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

pub mod core;
pub mod prelude;
pub mod quicksilver_ui;
pub mod shared;
