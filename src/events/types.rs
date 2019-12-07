/// Type enums that Events depend on but are not AnyEvent enums
///

/// The PlayState represents the running state and lifecycle of a Tween animation
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum PlayState {
    /// Not scheduled to start yet. An external call will change it to Pending/Running
    Waiting,
    /// the initial state before play begins
    Pending,
    /// Interim step to signal that playing should start
    Starting,
    /// when play starts
    Running,
    /// after play has completed and waiting for next instruction
    Idle,
    /// not in use
    Cancelled,
    /// Near final state that allows one last update call to deliver tween end_state props
    Finishing,
    /// The playback has completed
    Completed,
    /// Animation is restarting after being Idle. This happens for animations that repeat
    Restarting,
}

/// Enum to define what kind of animation is executing or has completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TweenType {
    /// No tween yet. Use as default until defined within PropSet
    None,
    /// Use for any kind of raw animation not triggered by a user
    Animation,
    /// A Scene or other object is moving
    Move,
    /// A hover animation
    Hover,
    /// A click animation
    Click,
    /// A rotation animation
    Rotation,
}
