/// Base UI stuff
///
use crate::core::*;

use std::any::TypeId;

use super::theme::Theme;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color, Font},
    input::{ButtonState, Key, MouseButton, MouseCursor},
    lifecycle::{Event, Window},
};

pub enum DisplayEvent {
    Activate,
    Deactivate,
    Ready,
    Resize(Vector),
}

pub enum FieldValue {
    None,
    Text(String),
    Selections(Vec<usize>),
    Checkbox(bool),
    // Options(Vec<bool>),
}

/// This trait lives in quicksilver_ui because it is heavily tied into quicksilver.
/// It defines the necessary methods for operating within a quicksilver run loop to provide info, prepare objects for
/// display, and render them.
pub trait TKDisplayable {
    fn get_type_id(&self) -> TypeId;

    fn get_frame(&self) -> Rectangle;

    /// This trait method should set the specified theme to the TweenLayer and
    /// to apply the colors and fonts for each Displayable object.
    /// This is the default action. Uncomment this line or apply the them to a child Displayable property
    /// self.layer.theme = theme.clone();
    fn set_theme(&mut self, _theme: &Theme);

    fn set_origin(&mut self, _origin: &Vector) {}

    fn notify(&mut self, _event: &DisplayEvent) {}

    /// Purpose: apply default props
    fn reset(&mut self) {}

    /// This method is essential if you are animating display characteristics and
    /// expect the object to return to its original state.
    fn load_defaults(&mut self) {}

    fn layout(&mut self, _origin: Vector, _theme: &Theme) {}

    fn update(&mut self) -> TKResult {
        Ok(())
    }

    fn render(&mut self, _theme: &mut Theme, _window: &mut Window) -> TKResult {
        Ok(())
    }

    fn get_perimeter_frame(&self) -> Option<Rectangle> {
        None
    }

    fn set_hover_animation(&mut self, _props: &[Prop], _seconds: f64) {}

    /// The mouse was moved; it provides both absolute x and y coordinates in the window,
    /// and relative x and y coordinates compared to its last position.
    fn handle_mouse_at(&mut self, _pt: &Vector) -> bool {
        false
    }
}

impl TKDisplayable {}

/// This trait is implemented by Button and other controls to conveniently handle mouse
/// events in a game/animation runloop. The mutable TKState parameter allows the developer
/// to arbitrarily add u32 values to specify that a specific action should be handled in
/// another part of the code.
pub trait TKResponder: TKDisplayable {
    fn get_field_value(&self) -> FieldValue {
        FieldValue::None
    }

    fn has_focus(&self) -> bool {
        false
    }

    /// A mouse button was pressed
    fn handle_mouse_down(&mut self, _pt: &Vector, _state: &mut TKState) -> bool {
        false
    }

    /// A mouse button was released
    fn handle_mouse_up(&mut self, _pt: &Vector, _state: &mut TKState) -> bool {
        false
    }

    /// The mousewheel was scrolled, vertically (y, positive away from and negative toward the user)
    /// or horizontally (x, positive to the right and negative to the left).
    fn handle_mouse_scroll(&mut self, _pt: &Vector, _state: &mut TKState) {}

    /// A keyboard button was pressed.
    fn handle_key_press(&mut self, _c: char, _window: &mut Window) {}

    // TODO: Handle all kinds of command keys: backspace, enter, etc.
    // A true response means the parent Scene or other entity should evaluate the response.
    fn handle_key_command(&mut self, _key: &Key, _window: &mut Window) -> bool {
        false
    }
}
