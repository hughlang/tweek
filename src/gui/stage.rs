/// Stage acts as the coordinator when there are multiple tweens being animated with one or more timelines.
///
use super::*;
use crate::core::*;
use crate::events::*;

use std::{
    any::TypeId,
    collections::BTreeMap,
};

use quicksilver::{
    geom::{Rectangle, Vector},
    input::{Key},
    lifecycle::{Window},
};

//-- Support -----------------------------------------------------------------------

pub struct Route {
    pub id: u32,
    pub type_id: TypeId,
    pub route_map: BTreeMap<u32, Route>
}

impl Route {
    pub fn new(id: u32, type_id: TypeId) -> Self {
        Route {
            id, type_id, route_map: BTreeMap::new(),
        }
    }
}

//-- Main -----------------------------------------------------------------------

/// Stage acts as a coordinator when multiple tween animations are added to a Timeline
/// for animation.
pub struct Stage {
    layer: Layer,
    // routes:
    pub title: String,
    pub scenes: Vec<Scene>,

}

impl Stage {
    /// Constructor
    ///
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);
        Stage { layer, title: String::default(), scenes: Vec::new() }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    /// This function provides a passthrough for Quicksilver State lifecycle
    pub fn handle_event(&mut self, event: &SceneEvent) -> bool {
        let action: SceneAction = {
            let mut action = SceneAction::None;
            for scene in &mut self.scenes {
                if let Some(result) = scene.find_action(event.clone()) {
                    action = result;
                }
            }
            action
        };

        match action {
            SceneAction::Animate(_, _, id) => {
                if let Some(scene) = self.scenes.iter_mut().find(|s| s.get_id() == id) {
                    let handled = scene.handle_action(&action.clone());
                }
            }
            _ => ()
        }
        // if let Some(action) = action {
        //
        // }
        false
    }

    pub fn print_routes(&mut self) {
        for scene in &mut self.scenes {
            log::debug!("=== Routes in Scene: {} =====================", scene.name);
            for route in scene.get_routes() {
                log::debug!("{}", route);
            }
        }
    }

    fn load_routes(&mut self) {
        for scene in &mut self.scenes {
            for routes in scene.get_routes() {

            }
        }
    }
}

// ************************************************************************************
// ************************************************************************************

impl Displayable for Stage {

    fn get_id(&self) -> u32 { self.layer.get_id() }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Stage>()
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

        match event {
            DisplayEvent::Ready => {
                self.print_routes();
            }
            _ => {}
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

impl Responder for Stage {
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
