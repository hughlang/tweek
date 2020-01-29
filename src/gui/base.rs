/// Base UI stuff
///
use crate::core::*;
use crate::events::*;

use std::any::{Any, TypeId};

use super::{gui_print_type, layer::Layer, stage::StageContext, theme::Theme};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    input::Key,
    lifecycle::Window,
};

/// Enum used as return type for Responder get_field_value() method to wrap the value
/// of the field
///
#[derive(Clone, Debug)]
pub enum FieldValue {
    /// Return type if the field has no value, such as no selection in listbox. Different from empty string
    None,
    /// String value of a field
    Text(String),
    /// One or more selection indexes from a listbox or similar array
    Selections(Vec<usize>),
    /// Checkbox value
    Checkbox(bool),
}

/// This trait lives in quicksilver_ui because it is heavily tied into quicksilver.
/// It defines the necessary methods for operating within a quicksilver run loop to provide info, prepare objects for
/// display, and render them.
pub trait Displayable: Any {
    /// Get the layer id
    fn get_id(&self) -> u32 {
        self.get_layer().id
    }

    /// Set the layer.id to identify it. This is called from the view_did_load() function
    /// using u32 values assigned from the AppState object.
    /// It also assigns the same value to Tween.tween_id which helps with console debugging
    /// See also: https://github.com/rayet-inc/tweek/issues/33#issuecomment-578890730
    fn set_id(&mut self, id: u32) {
        self.get_layer_mut().id = id;
        self.get_layer_mut().type_id = self.get_type_id();
        if let Some(tween) = &mut self.get_layer_mut().animation {
            tween.tween_id = id;
        }
    }

    /// Method that provides unique identifier for each displayable object type. Useful for custom handling of
    /// UI behaviors
    fn get_type_id(&self) -> TypeId;

    /// Get reference to the Layer
    fn get_layer(&self) -> &Layer;

    /// Get a mutable reference to the Layer
    fn get_layer_mut(&mut self) -> &mut Layer;

    /// Get the Layer frame value for the object
    fn get_frame(&self) -> Rectangle;

    /// Convenience method for creating a Rectangle relative to the current Displayable frame
    fn sub_frame(&self, pos: (f32, f32), size: (f32, f32)) -> Rectangle {
        // TODO: Check that child frame fits within Scene?
        let frame = Rectangle::new((self.get_frame().pos.x + pos.0, self.get_frame().pos.y + pos.1), (size.0, size.1));
        frame
    }

    /// Calculate the content size within the object to help with layout and other UI tasks.
    /// Override if the content size might be different than the frame size
    fn get_content_size(&self) -> Vector {
        self.get_frame().size
    }

    /// Use this to make sure that the anchor_pt is set when adding objects to a Scene.
    /// Override if child views need to be updated as well.
    fn set_origin(&mut self, origin: Vector) {
        let offset = self.get_frame().pos - origin;
        self.get_layer_mut().anchor_pt = offset;
        // log::debug!("set_origin for {} – anchor_pt={:?}", self.debug_id(), offset);
    }

    /// A helper method that moves a Displayable component layer by the specified offset. For Scene objects, all child
    /// objects are moved as well. This is a convenience that lets you build a Scene with a 0,0 origin and then
    /// reposition when the Scene has been fully populated
    fn align_view(&mut self, origin: Vector) {
        let anchor_pt = self.get_layer().anchor_pt;
        self.get_layer_mut().frame.pos = anchor_pt + origin;
        log::trace!("align_view {} pos={:?} anchor_pt={:?}", self.debug_id(), self.get_layer().frame.pos, anchor_pt);
    }

    fn validate_position(&self, origin: Vector) {
        let offset = self.get_layer().frame.pos - origin;
        let anchor = self.get_layer().anchor_pt;
        if anchor != offset {
            log::error!(
                "Wrong position: Expected={:?} actual={:?}",
                self.get_layer().frame.pos + anchor,
                self.get_layer().frame.pos + offset
            );
            log::error!("Node={:?}", self.debug_out());
        }
    }

    /// Reposition the object by setting its origin point
    fn move_to(&mut self, pos: (f32, f32));

    /// Set the foreground color of an object. This will vary depending on the object.
    /// For objects containing text, it is the font color and/or any vector image content
    fn set_tint(&mut self, _color: Color) {}

    /// This trait method should set the specified theme to the Layer and
    /// to apply the colors and fonts for each Displayable object.
    /// This is the default action. Uncomment this line or apply the them to a child Displayable property
    /// self.layer.theme = theme.clone();
    fn set_theme(&mut self, _theme: &mut Theme);

    /// Top-down notification of events to child objects
    fn notify(&mut self, _event: &DisplayEvent);

    /// Top-down event handling to child objects
    fn handle_event(&mut self, _event: &EventBox, _app_state: &mut AppState) {}

    /// Method to update the UI state of an object in every pass of the run loop.
    /// This is used in motion animation and internal Tween animation so that each
    /// render() call will have correct values
    fn update(&mut self, _window: &mut Window, _state: &mut AppState);

    /// Method to draw the object by making calls to the graphics backend.
    fn render(&mut self, _theme: &mut Theme, _window: &mut Window) {}

    /// The mouse was moved; it provides both absolute x and y coordinates in the window,
    /// and relative x and y coordinates compared to its last position.
    fn handle_mouse_at(&mut self, _pt: &Vector, _window: &mut Window) -> bool {
        false
    }

    /// Allow a GUI component to define the perimeter area that it needs outside of its
    /// base frame. This is currently used for masking UI overflow in ListBox and other
    /// components
    fn get_perimeter_frame(&self) -> Option<Rectangle> {
        None
    }

    /// Define the Props to animate and the duration when a mouse hover event occurs
    fn set_hover_animation(&mut self, _props: PropSet) {}

    /// Define the Props to animate and the duration when a mouse hover event occurs
    fn set_click_animation(&mut self, _props: PropSet) {}

    /// Get duration of layer animation (if present)
    fn get_tween_duration(&mut self) -> f64 {
        if let Some(tween) = &self.get_layer_mut().animation {
            return tween.total_time();
        }
        0.0
    }

    /// Standard format for printing out view information. In general, traits implementors do not need to override it. However, if an object contains
    /// nested views, it may be useful to print out those details. OptionGroup is one example.
    fn debug_out(&self) -> String {
        format!("{} {}", self.debug_id(), self.debug_frame())
    }

    /// A simple string format to display a type and id in this format: <Scene> [200]
    fn debug_id(&self) -> String {
        format!("<{}> [{}]", gui_print_type(&self.get_type_id()), self.get_id())
    }

    fn debug_frame(&self) -> String {
        let frame = self.get_frame();
        format!("Pos({:.1},{:.1}) Size({:.1},{:.1})", frame.pos.x, frame.pos.y, frame.size.x, frame.size.y)
    }

    /// Simplistic way of providing an identifier for the Type and Id of the object, which gets combined with parent
    /// and child route paths
    fn node_key(&self) -> String {
        format!("{}-{}", gui_print_type(&self.get_type_id()), self.get_id())
    }

    fn view_will_load(&mut self, _ctx: &mut StageContext, app_state: &mut AppState) {
        let path = self.get_layer().node_path.nodes.clone();
        let node_path = NodePath::new(path);
        app_state.append_node(node_path);
    }
}

/// This trait is implemented by Button and other controls to conveniently handle mouse
/// events in a game/animation runloop.
pub trait Responder: Displayable {
    /// Get the user input value of the control
    fn get_field_value(&self) -> FieldValue {
        FieldValue::None
    }

    /// Method to try and set the value of a field using an enum FieldValue wrapper.
    /// This allows one to tell a Scene object to find the object that has a matching
    /// TypeId and numeric tag to set the value of a control.
    fn set_field_value(&mut self, _value: &FieldValue, _type_id: TypeId, _layer_id: u32) -> bool {
        false
    }

    /// A mouse button was pressed
    fn handle_mouse_down(&mut self, _pt: &Vector, _state: &mut AppState) -> bool {
        false
    }

    /// A mouse button was released
    fn handle_mouse_up(&mut self, _pt: &Vector, _state: &mut AppState) -> bool {
        false
    }

    /// The mousewheel was scrolled, vertically (y, positive away from and negative toward the user)
    /// or horizontally (x, positive to the right and negative to the left).
    fn handle_mouse_scroll(&mut self, _pt: &Vector, _state: &mut AppState) {}

    /// A keyboard button was pressed.
    fn handle_key_press(&mut self, _c: char, _window: &mut Window) {}

    /// Handle keyboard input
    /// TODO: Handle all kinds of command keys: backspace, enter, etc.
    /// A true response means the parent Scene or other entity should evaluate the response.
    fn handle_key_command(&mut self, _key: &Key, _window: &mut Window) -> bool {
        false
    }
}
