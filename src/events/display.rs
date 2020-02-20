/// Events related to UI
///
use super::{AnyEvent, PlayState};

use std::any::TypeId;

/// Enum used in the Displayable notify method to signal event info to child views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisplayEvent {
    /// Undefined state
    None,
    /// Event called to tell a child view that it is active
    Activate,
    /// Event called to tell child views they are not the active focus
    Deactivate,
    /// Event to signal that the view is ready to display
    Ready,
    /// The Scene parent is moving
    Moving,
    /// Event to tell child view that save the current position as the new position
    Moved,
}
impl AnyEvent for DisplayEvent {}

/// An enum for Theme-related events.
/// FIXME: Move to SceneEvent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeEvent {
    /// Theme changed to the number that identifies it
    Change(u32),
}
impl AnyEvent for ThemeEvent {}

/// Enum for notifications relating to Layer objects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayerEvent {
    /// Notification dispatched from Tween object with id
    Tween(u32, PlayState),
    /// Hover event with id, type, and state
    Hover(u32, TypeId, PlayState),
    /// Click event with id, type, and state
    Click(u32, TypeId, PlayState),
    /// Move event with id, type, and state
    Move(u32, TypeId, PlayState),
    /// Rotate event with id, type, and state
    Rotate(u32, TypeId, PlayState),
}
impl AnyEvent for LayerEvent {}
