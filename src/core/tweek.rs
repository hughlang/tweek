/// Tweek acts as the coordinator when there are multiple tweens being animated with one or more timelines.
///
use crate::events::*;

use quicksilver::geom::Vector;

//-- Base -----------------------------------------------------------------------

/// The Playable trait provides support for basic animation updating and control
pub trait Playable {
    /// Must implement play method to start the Playable
    fn play(&mut self);
    /// Method called in the run loop to inform playables to check and update their internal state
    fn tick(&mut self) {}
    /// Handle request to stop the current play
    fn stop(&mut self) {}
    /// Pause the current playback
    fn pause(&mut self) {}
    /// Reset the playback to initial state
    fn reset(&mut self) {}
    /// A means of forcibly setting the PlayerState
    fn set_state(&mut self, _state: PlayState) {}
}

/// Mutable state object passed through Responder methods for capturing and handling
/// user events from keyboard and mouse
pub struct AppState {
    /// The size of the window
    pub window_size: (f32, f32),
    /// Ratio value to alter speed of playback, where 1.0 is natural time
    pub time_scale: f32,
    /// Elapsed time
    pub elapsed_time: f64,
    /// Total time
    pub total_time: f64,
    /// Offset x-y when Scene is animating/moving
    pub offset: Vector,
    /// The event queue
    pub event_bus: EventBus,
    /// Stores the index value of the row that was clicked on.
    pub row_target: Option<usize>,
}

impl AppState {
    /// Constructor
    pub fn new() -> Self {
        AppState {
            window_size: (0.0, 0.0),
            time_scale: 1.0,
            elapsed_time: 0.0,
            total_time: 0.0,
            offset: Vector::ZERO,
            event_bus: EventBus::default(),
            row_target: None,
        }
    }

    /// Hacky way of forcing top-level controller to zero
    pub fn zero_offset(&mut self) {
        self.offset = Vector::ZERO;
    }
}
