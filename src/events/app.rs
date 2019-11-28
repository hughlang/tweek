/// Events related to navigation requests and actions
///
use super::AnyEvent;
use crate::gui::Node;

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
    Modal(usize),
    /// Open detail view for selected index
    Selected(usize),
}
impl AnyEvent for NavEvent {}

/// An enum for specifying arbitrary actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SceneEvent {
    /// Nothing is happening
    None,
    /// Show something
    Show(Node),
    /// Hide something
    Hide(Node),
    /// Useful for carousel and other slide animation
    Transition,
    /// Theme changed to the number that identifies it
    ChangeTheme(u32),
}
impl AnyEvent for SceneEvent {}
