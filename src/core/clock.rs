/// A custom clock. See https://github.com/rayet-inc/tweek/issues/5
/// This Clock is used in AppState
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(target_arch = "wasm32")]
use stdweb::web::Date;

/// A service that helps with efficient calls to the system clock to minimize number of requests.
/// The system epoch time is cached so that all requests in a single run loop call provide the same timestamp.
/// For wasm, it caches system timestamps and estimates time progression based on current FPS.
pub struct Clock {
    real_ts: f64,
}

impl Clock {
    pub fn new() -> Self {
        Clock { real_ts: 0.0 }
    }
    /// Non-wasm: Update the internal timestamp using SystemTime
    #[cfg(not(target_arch = "wasm32"))]
    pub fn refresh_time(&mut self) {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        self.real_ts = since_the_epoch.as_millis() as f64 / 1000.0;
    }
    /// Wasm: Updates the internal timestamp in seconds
    #[cfg(target_arch = "wasm32")]
    pub fn refresh_time(&mut self) {
        self.real_ts = Date::now() / 1000.0;
    }

    /// Provides the current unix epoch time in seconds
    /// Since this project supports wasm target environments, it is not possible to use
    /// Rust std Instant and Duration, and thus we need to use more primitive means.
    pub fn current_time(&mut self) -> f64 {
        self.real_ts
    }

    /// Provides the elapsed time give the epoch time provided by current_time()
    pub fn elapsed_time(&self, since: f64) -> f64 {
        let elapsed = self.real_ts - since;
        elapsed
    }
}
