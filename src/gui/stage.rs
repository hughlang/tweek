/// Stage acts as the coordinator when there are multiple tweens being animated with one or more timelines.
///
use super::*;
use crate::core::*;
use crate::events::*;

use std::{any::TypeId, collections::HashMap};

use quicksilver::{
    geom::{Rectangle, Vector},
    input::Key,
    lifecycle::Window,
};

//-- Main -----------------------------------------------------------------------

/// Stage serves as a parent for one or more Scenes
pub struct Stage {
    layer: Layer,
    /// Title for display
    pub title: String,
    /// All of the Scenes
    pub scenes: Vec<Scene>,
    /// Storage of added Commands as a mapping of the source to target
    event_actions: HashMap<(SceneEvent, Option<String>), SceneAction>,
    /// Rudimentary storage of an node_id and the Route it probably matches. FIXME later
    route_map: HashMap<String, String>,
}

impl Stage {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);
        Stage {
            layer,
            title: String::default(),
            scenes: Vec::new(),
            event_actions: HashMap::new(),
            route_map: HashMap::new(),
        }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn load_routes(&mut self) {
        for scene in &mut self.scenes {
            log::trace!("=== Routes in Scene: {} =====================", scene.name);
            for route in scene.get_routes() {
                log::trace!("Route: {}", route);
                let parts = route.split("/");
                if let Some(part) = parts.last() {
                    // TODO: Warn if key already exists
                    self.route_map.insert(part.to_string(), route.to_string());
                }
            }
        }
    }
}

// ************************************************************************************
// ************************************************************************************

impl Displayable for Stage {
    fn get_id(&self) -> u32 {
        self.layer.get_id()
    }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Stage>()
    }

    fn get_layer(&self) -> &Layer {
        &self.layer
    }

    fn get_layer_mut(&mut self) -> &mut Layer {
        &mut self.layer
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    // Note: A Stage never moves and is always represents the full screen/window.
    // Only reposition the child scenes
    fn align_view(&mut self, origin: Vector) {
        for scene in &mut self.scenes {
            scene.align_view(origin);
        }
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

    fn handle_event(&mut self, event: &EventBox) {
        // match evt {
        //     SceneEvent::Show(_) => {
        //         self.nav_scene.is_interactive = false;
        //     }
        //     SceneEvent::Hide(_) => {
        //         self.nav_scene.is_interactive = true;
        //     }
        //     _ => (),
        // }
        if let Ok(evt) = event.downcast_ref::<SceneEvent>() {
            // log::debug!("SceneEvent={:?}", evt);
            // log::debug!("Source={:?}", event.event_info());
            let source = event.event_info();
            if let Some(action) = self.event_actions.get(&(evt.clone(), source.clone())) {
                log::debug!("Found action={:?}", action);
                match action {
                    SceneAction::Animate(propset, node) => {
                        if let Some(route) = self.route_map.get(&node.id_string()) {
                            for scene in &mut self.scenes {
                                if let Some(layer) = scene.get_layer_for_route(&route) {
                                    log::debug!("Found layer for route={:?}", route);
                                    layer.animate_with_props(propset.clone(), true);
                                    scene.handle_event(event);
                                    // scene.handle_event(event, source);
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        } else {
            for scene in &mut self.scenes {
                scene.handle_event(event);
            }
        }
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Ready => {
                self.load_routes();
                for scene in &mut self.scenes {
                    scene.notify(event);
                    // For Ready event, capture all commands from scenes
                    for (k, v) in scene.event_actions.drain() {
                        self.event_actions.insert(k, v);
                    }
                }
            }
            _ => {
                for scene in &mut self.scenes {
                    scene.notify(event);
                }
            }
        }
    }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
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

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        for scene in &mut self.scenes {
            let hover = scene.handle_mouse_at(pt, window);
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
