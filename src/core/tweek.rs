/// Tweek acts as the coordinator when there are multiple tweens being animated with one or more timelines.
///
use crate::events::*;
use crate::gui::*;

use std::{
    any::TypeId
};

use quicksilver::{
    geom::{Rectangle, Vector},
    input::{Key},
    lifecycle::{Window},
};

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
}

/// Mutable state object passed through Responder methods for capturing and handling
/// user events from keyboard and mouse
pub struct AppState {
    /// Ratio value to alter speed of playback, where 1.0 is natural time
    pub time_scale: f32,
    /// Elapsed time
    pub elapsed_time: f64,
    /// Total time
    pub total_time: f64,
    /// Offset x-y when Scene is animating/moving
    pub offset: (f32, f32),
    /// The event queue
    pub event_bus: EventBus,
    /// Stores the index value of the row that was clicked on.
    pub row_target: Option<usize>,
}

impl AppState {
    /// Constructor
    pub fn new() -> Self {
        AppState {
            time_scale: 1.0,
            elapsed_time: 0.0,
            total_time: 0.0,
            offset: (0.0, 0.0),
            event_bus: EventBus::default(),
            row_target: None,
        }
    }

    /// Hacky way of forcing top-level controller to zero
    pub fn zero_offset(&mut self) {
        self.offset = (0.0, 0.0);
    }
}

//-- Main -----------------------------------------------------------------------

/// Tweek acts as a coordinator when multiple tween animations are added to a Timeline
/// for animation.
pub struct Tweek {
    layer: Layer,
    pub scenes: Vec<Scene>,
}

impl Tweek {
    /// Constructor
    ///
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);
        Tweek { layer, scenes: Vec::new() }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    /// This function provides a passthrough for Quicksilver State lifecycle
    pub fn handle_event(&mut self, event: &SceneEvent) -> bool {
        for scene in &mut self.scenes {
            let handled = scene.handle_event(event);
            if handled {
                return true;
            }
        }
        false
    }
}

// ************************************************************************************
// ************************************************************************************

impl Displayable for Tweek {

    fn get_id(&self) -> u32 { self.layer.get_id() }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Tweek>()
    }

    fn get_layer_mut(&mut self) -> &mut Layer {
        &mut self.layer
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn move_to(&mut self, pos: (f32, f32)) {
        self.layer.frame.pos.x = pos.0;
        self.layer.frame.pos.y = pos.1;
    }

    fn set_theme(&mut self, theme: &mut Theme) {
        for scene in &mut self.scenes {
            scene.set_theme(theme);
        }
    }

    fn notify(&mut self, event: &DisplayEvent) {
        for scene in &mut self.scenes {
            scene.notify(event);
        }
    }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        self.layer.tween_update();
        for scene in &mut self.scenes {
            scene.update(window, state);
        }
    }

    /// The top-level objects in the scene should all use the scene's coordinate system and
    /// therefore, this render() method should only call render() for all child Displayable objects.
    /// That's the current plan. It may change.
    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        for scene in &mut self.scenes {
            scene.render(theme, window);
        }
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        for scene in &mut self.scenes {
            let hover = scene.handle_mouse_at(pt);
            if hover {
                return true;
            }
        }
        false
    }
}
// ************************************************************************************
// ************************************************************************************

impl Responder for Tweek {
    fn set_field_value(&mut self, value: &FieldValue, type_id: TypeId, layer_id: u32) -> bool {
        for scene in &mut self.scenes {
            let success = scene.set_field_value(value, type_id, layer_id);
            if success {
                return true;
            }
        }
        false
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        for scene in &mut self.scenes {
            scene.handle_mouse_down(pt, state);
        }
        false
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        for scene in &mut self.scenes {
            scene.handle_mouse_up(pt, state);
        }
        false
    }

    fn handle_mouse_scroll(&mut self, pt: &Vector, state: &mut AppState) {
        for scene in &mut self.scenes {
            scene.handle_mouse_scroll(pt, state);
        }
    }

    fn handle_key_press(&mut self, c: char, window: &mut Window) {
        for scene in &mut self.scenes {
            scene.handle_key_press(c, window);
        }
    }

    fn handle_key_command(&mut self, key: &Key, window: &mut Window) -> bool {
        for scene in &mut self.scenes {
            scene.handle_key_command(key, window);
        }
        false
    }
}
