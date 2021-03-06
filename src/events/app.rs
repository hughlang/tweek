/// Events related to navigation requests and actions
///
use super::AnyEvent;
use crate::core::NodeID;

use std::any::TypeId;

/// An enum for specifying the common navigation events a GUI might need
/// More types to consider:
/// * Tab selected
/// * Next/Previous
#[derive(Debug, Clone, Copy)]
pub enum NavEvent {
    /// Go back in navigation controller
    Back,
    /// Go next in a sequence, provided by the current controller
    Next,
    /// Close to the current view
    Close,
    /// Go to the Home screen
    Home,
    /// Navigate to first view controller in nav
    Root,
    /// Display modal
    Modal(u32),
    /// Open detail view for selected index
    Selected(u32),
}
impl AnyEvent for NavEvent {}

/// A generic event enum for mouse events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseEvent {
    /// Either mousedown or mouseup
    Select(TypeId, u32),
    /// Mouse action to unselect an item
    Deselect(TypeId, u32),
    /// Mouse over
    Hover(TypeId, u32),
    /// Mouse move while mousedown
    Drag(TypeId, u32),
}

impl AnyEvent for MouseEvent {}

/// An enum for specifying arbitrary actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SceneEvent {
    /// Nothing is happening
    None,
    /// Show something
    Show(NodeID),
    /// Hide something
    Hide(NodeID),
    /// Useful for carousel and other slide animation
    Transition,
    /// Theme changed to the number that identifies it
    ChangeTheme(u32),
}
impl AnyEvent for SceneEvent {}
