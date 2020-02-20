/// Events related to media players
///
use super::AnyEvent;

/// An enum for specifying the common player events
/// Unused
#[derive(Debug, Clone, Copy)]
pub enum PlayerEvent {
    /// Start play
    Play,
    /// Put everything back
    Reset,
    /// Replay
    Replay,
    /// Pause
    Pause,
    /// Reverse
    Reverse,
    /// Skip forward
    SkipForward(f32),
}
impl AnyEvent for PlayerEvent {}

/// An enum for Theme-related events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TweenEvent {
    Started,
    Restarting,
    Completed,
}
impl AnyEvent for TweenEvent {
    fn to_string(&self) -> String {
        format!("TweenEvent::{:?}", self)
    }
}

/// An enum for specifying the common player events
/// Unused
#[derive(Debug, Clone, Copy)]
pub enum TimelineEvent {
    /// Interim step to signal that playing should start
    Starting,
    /// Near final state that allows one last update call to deliver tween end_state props
    Finishing,
    /// The playback has completed
    Completed,
    /// Animation is restarting after being Idle. This happens for animations that repeat
    Restarting,
}
impl AnyEvent for TimelineEvent {}
