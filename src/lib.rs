#![crate_name = "tweek"]
#![crate_type = "lib"]
#![feature(duration_float)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate unwrap_to;

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

pub mod core;
pub mod prelude;
pub mod gui;
pub mod shared;
