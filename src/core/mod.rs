//! Tweek Core
//!
//! This module contains the animation core code that enables all GUI components to support
//! animation through the Layer property.
//!
//! This module should not have dependencies on sibling crates and should be agnostic to the
//! graphics backend. For example, the gui crate is completely dependent on Quicksilver as the
//! game/graphics engine, but this module should not have such dependencies.

pub use self::animator::*;
pub use self::colors::*;
pub use self::ease::*;
pub use self::property::*;
pub use self::timeline::*;
pub use self::tweek::*;
pub use self::tween::*;

mod animator;
mod colors;
mod ease;
mod property;
mod timeline;
mod tweek;
mod tween;

/// Helper function to convert hex color string to rgb tuple, each in range 0.0 to 255.0
/// Copied from Quicksilver Color::from_hex function. Only allows format "#FFFFFF" and
/// not the shortened 3-character form.
pub fn rgb_from_hex(hex: &str) -> (f32, f32, f32, f32) {
    let trimmed_hex = hex.trim_start_matches('#');
    match trimmed_hex.len() {
        6 => {
            let red = u8::from_str_radix(&trimmed_hex[0..=1], 16).unwrap();
            let green = u8::from_str_radix(&trimmed_hex[2..=3], 16).unwrap();
            let blue = u8::from_str_radix(&trimmed_hex[4..=5], 16).unwrap();
            (red as f32, green as f32, blue as f32, 255.0)
        }
        8 => {
            let red = u8::from_str_radix(&trimmed_hex[0..=1], 16).unwrap();
            let green = u8::from_str_radix(&trimmed_hex[2..=3], 16).unwrap();
            let blue = u8::from_str_radix(&trimmed_hex[4..=5], 16).unwrap();
            let alpha = u8::from_str_radix(&trimmed_hex[6..=7], 16).unwrap();
            (red as f32, green as f32, blue as f32, alpha as f32)
        }
        _ => panic!("Malformed hex string"),
    }
}

/// Helper method to convert color in u32 format (like 0xFFFFFF) to rgb u8 values
/// Currently unused
pub fn hex_to_rgb(c: u32) -> (u8, u8, u8) {
    let rp = ((c & 0x00FF_0000u32) >> 16) as u8;
    let gp = ((c & 0x0000_FF00u32) >> 8) as u8;
    let bp = (c & 0x0000_00FFu32) as u8;
    (rp, gp, bp)
}

// TODO: Move these to Tools
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(target_arch = "wasm32")]
use stdweb::console;
#[cfg(target_arch = "wasm32")]
use stdweb::web::Date;

/// Provides the current unix epoch time in seconds for non-wasm target
/// Since this project supports wasm target environments, it is not possible to use
/// Rust std Instant and Duration, and thus we need to use more primitive means.
#[cfg(not(target_arch = "wasm32"))]
pub fn current_time() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_millis() as f64 / 1000.0
}
/// Provides the elapsed time give the epoch time provided by current_time() for non-wasm
#[cfg(not(target_arch = "wasm32"))]
pub fn elapsed_time(since: f64) -> f64 {
    let elapsed = current_time() - since;
    elapsed
}
/// This is a placeholder since normal console logging is handled by the env_logger.
/// Below, the matching wasm debug_log. TBD: can we remove this?
#[cfg(not(target_arch = "wasm32"))]
pub fn debug_log(_text: &str) {
    // debug::log!(text);
}

/// Provides the current unix epoch time in seconds for wasm target
#[cfg(target_arch = "wasm32")]
pub fn current_time() -> f64 {
    Date::now() / 1000.0
}
/// Provides the elapsed time give the epoch time provided by current_time() for wasm
#[cfg(target_arch = "wasm32")]
pub fn elapsed_time(since: f64) -> f64 {
    current_time() - since
}
/// Provides console.log for logging in web browser in wasm target
#[cfg(target_arch = "wasm32")]
pub fn debug_log(text: &str) {
    console!(log, text);
}
