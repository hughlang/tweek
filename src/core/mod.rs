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

// TODO: Move these to Tools
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(target_arch = "wasm32")]
use stdweb::console;
#[cfg(target_arch = "wasm32")]
use stdweb::web::Date;

#[cfg(not(target_arch = "wasm32"))]
pub fn current_time() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_millis() as f64 / 1000.0
}
#[cfg(not(target_arch = "wasm32"))]
pub fn elapsed_time(since: f64) -> f64 {
    let elapsed = current_time() - since;
    elapsed
}
#[cfg(not(target_arch = "wasm32"))]
pub fn debug_log(_text: &str) {
    // debug::log!(text);
}

#[cfg(target_arch = "wasm32")]
pub fn current_time() -> f64 {
    Date::now() / 1000.0
}
#[cfg(target_arch = "wasm32")]
pub fn elapsed_time(since: f64) -> f64 {
    current_time() - since
}
#[cfg(target_arch = "wasm32")]
pub fn debug_log(text: &str) {
    console!(log, text);
}
