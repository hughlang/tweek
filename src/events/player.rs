/// Events related to media players
///
use super::{AnyEvent};

/// An enum for specifying the common player events
/// Unused
#[derive(Debug, Clone, Copy)]
pub enum PlayerEvent {
    /// Start play
    Play,
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
