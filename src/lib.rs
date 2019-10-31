//! Tweek UI
//!
//! Tweek aims to be a complete GUI library for Rust that follows an [immediate mode philosophy](https://en.wikipedia.org/wiki/Immediate_Mode_GUI)
//!
//! TODO: Write module docs here.
//!
//!
//!
//!
//!
//!
//!
//!

// #![doc(html_root_url = "https://docs.rs/####")]
// #![deny(
//     // bare_trait_objects,
//     missing_docs,
//     // unused_extern_crates,
//     // unused_import_braces,
//     // unused_qualifications
// )]

#![crate_name = "tweek"]
#![crate_type = "lib"]

#[macro_use]
extern crate float_cmp;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate unwrap_to;
#[macro_use]
extern crate log;

#[cfg(not(target_arch = "wasm32"))]
extern crate env_logger;

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

extern crate image as image_rs;

pub mod core;
pub mod events;
pub mod gui;
pub mod prelude;
pub mod tools;
