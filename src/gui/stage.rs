/// Stage acts as the coordinator when there are multiple tweens being animated with one or more timelines.
///
use super::*;
use crate::core::*;
use crate::events::*;

use std::{
    any::TypeId,
    collections::{BTreeMap, HashMap},
};

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
    /// All of the Scenes in map where the key is the Layer id
    pub(crate) scenes: BTreeMap<u32, Scene>,
    /// Holds event listeners
    pub context: StageContext,
}

impl Stage {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);
        Stage { layer, title: String::default(), scenes: BTreeMap::new(), context: StageContext::new() }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        let id = (self.scenes.len() + 1) as u32 * ID_RANGE_SIZE;
        self.scenes.insert(id, scene);
    }

    /// This method must be called when the application is ready to run.
    /// It loads the view hierarchy into AppState and loads event listeners
    /// It overwrites the default/empty StageContext when finished.
    pub fn stage_ready(&mut self, app_state: &mut AppState) {
        let mut context = StageContext::new();
        self.view_will_load(&mut context, app_state);
        self.notify(&DisplayEvent::Ready);
        self.context = context;
    }

    /// Given a NodePath path, find the corresponding UI Displayable object
    pub fn find_view_by_path(&mut self, path: NodePath) -> Option<&mut Layer> {
        let mut i: usize = 0;
        if let Some(node) = path.nodes.get(i) {
            if let Some(scene) = self.scenes.get_mut(&node.id) {
                // Top level is always a Scene.
                i += 1;
                if let Some(node) = path.nodes.get(i) {
                    if node.type_id == TypeId::of::<Timeline>() {
                        // This node is a Timeline. Get it and find the next node
                        if let Some(timeline) = &mut scene.timeline {
                            i += 1;
                            if let Some(node) = path.nodes.get(i) {
                                if let Some(sprite) = timeline.sprites.get_mut(&node.id) {
                                    return Some(sprite.view.get_layer_mut());
                                }
                            }
                        }
                    } else {
                        // The node is either a view or control
                        if let Some(view) = scene.views.get_mut(&node.id) {
                            return Some(view.get_layer_mut());
                        } else if let Some(view) = scene.controls.get_mut(&node.id) {
                            return Some(view.get_layer_mut());
                        }
                    }
                }
            }
        }
        log::warn!("Could not find view at path: {}", path.as_string());
        None
    }
}

// ************************************************************************************
// Displayable
// ************************************************************************************

impl Displayable for Stage {
    fn view_will_load(&mut self, ctx: &mut StageContext, app_state: &mut AppState) {
        // Iterate through scenes and initalize id values and register observers
        for (id, scene) in &mut self.scenes {
            // This forces the counter in app_state to use the scene.id as its base
            app_state.set_next_id(*id);
            // And this forces an increment on app_state.next_id and redundantly sets the scene.id
            scene.set_id(app_state.new_id());
            // Stage doesn't exist as a parent path, so set empty slice as path for scene
            scene.get_layer_mut().set_path(&[]);
            // Pass AppState into scene to let it build the tree
            scene.view_will_load(ctx, app_state);
            // If the scene is subscriber to notifications, add them here.
            for key in &scene.get_layer().queued_observers {
                app_state.register_observer(key.clone(), scene.get_layer().node_path.clone())
            }
            // Add event listeners from node to AppState
            let subscriber = scene.get_layer().node_path.clone();
            for (key, cb) in scene.get_layer_mut().event_listeners.drain() {
                ctx.add_event_listener(key, cb, subscriber.clone());
            }
        }
        app_state.print_tree();
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
        for scene in &mut self.scenes.values_mut() {
            scene.align_view(origin);
        }
    }

    fn move_to(&mut self, pos: (f32, f32)) {
        self.layer.frame.pos.x = pos.0;
        self.layer.frame.pos.y = pos.1;
    }

    fn set_theme(&mut self, theme: &mut Theme) {
        for scene in &mut self.scenes.values_mut() {
            scene.set_theme(theme);
        }
    }

    fn handle_event(&mut self, event: &EventBox, app_state: &mut AppState) {
        // Try to locate the event using the event string and u32 sender.id
        // The sender is provided in EventBus::dispatch_event
        let sender = event.sender();
        if let Ok(evt) = event.downcast_ref::<TweenEvent>() {
            let event_key = evt.to_string();
            log::debug!("handle_event: event_key={:?} sender={:?}", event_key, sender.id_string());
            if let Some((node_path, cb)) = self.context.event_listeners.get_mut(&(event_key, sender.id)) {
                // Execute the callback providing the AppState.
                // TODO: Consider providing NodePath of subscriber so that posted notification can include
                // the sender so that the controller can determine correct action.
                (*cb)(app_state, node_path.clone());
            }
        }
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Ready => {
                for scene in &mut self.scenes.values_mut() {
                    scene.notify(event);
                }
            }
            _ => {
                for scene in &mut self.scenes.values_mut() {
                    scene.notify(event);
                }
            }
        }
    }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        for scene in &mut self.scenes.values_mut() {
            scene.update(window, state);
        }
        // TODO: Implement observer actions
        state.send_notifications.clear();
    }

    /// The top-level objects in the scene should all use the scene's coordinate system and
    /// therefore, this render() method should only call render() for all child Displayable objects.
    /// That's the current plan. It may change.
    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        for scene in &mut self.scenes.values_mut() {
            scene.render(theme, window);
        }
    }

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        for scene in &mut self.scenes.values_mut() {
            let hover = scene.handle_mouse_at(pt, window);
            if hover {
                return true;
            }
        }
        false
    }
}

// ************************************************************************************
// Responder
// ************************************************************************************

impl Responder for Stage {
    fn set_field_value(&mut self, value: &FieldValue, type_id: TypeId, layer_id: u32) -> bool {
        for scene in &mut self.scenes.values_mut() {
            let success = scene.set_field_value(value, type_id, layer_id);
            if success {
                return true;
            }
        }
        false
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        for scene in &mut self.scenes.values_mut() {
            scene.handle_mouse_down(pt, state);
        }
        false
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        for scene in &mut self.scenes.values_mut() {
            scene.handle_mouse_up(pt, state);
        }
        false
    }

    fn handle_mouse_scroll(&mut self, pt: &Vector, state: &mut AppState) {
        for scene in &mut self.scenes.values_mut() {
            scene.handle_mouse_scroll(pt, state);
        }
    }

    fn handle_key_press(&mut self, c: char, window: &mut Window) {
        for scene in &mut self.scenes.values_mut() {
            scene.handle_key_press(c, window);
        }
    }

    fn handle_key_command(&mut self, key: &Key, window: &mut Window) -> bool {
        for scene in &mut self.scenes.values_mut() {
            scene.handle_key_command(key, window);
        }
        false
    }
}

// ************************************************************************************
// StageContext
// ************************************************************************************

/// A holder of data for the current stage
pub struct StageContext {
    /// A HashMap of listeners where key is [String name] and [id of subscriber]
    /// and value is [NodePath of subscriber] and [Callback provided by Layer::add_listener]
    pub(crate) event_listeners: HashMap<(String, u32), (NodePath, Box<dyn FnMut(&mut AppState, NodePath) + 'static>)>,
}

impl StageContext {
    pub fn new() -> Self {
        StageContext { event_listeners: HashMap::new() }
    }

    /// This is called during view_did_load() for all object Layers that have a temporary
    /// array of NodeEvents. It is intended to create a mapping between an event sender
    /// and an event receiver. This means the AppState needs to provide that to a controller
    /// so that an event can target the receiver.
    /// Hence, it may be a mistake to use assign_tag to store a mapping of the sender, when
    /// it really needs to identify the receiver and impose an action on it.
    pub(crate) fn add_event_listener(
        &mut self,
        key: String,
        cb: Box<dyn FnMut(&mut AppState, NodePath) + 'static>,
        subscriber: NodePath,
    ) {
        let id: u32 = subscriber.last_node().id;
        log::debug!("add_event_listener for key={:?} id={:?}", key, subscriber.as_string());
        self.event_listeners.insert((key, id), (subscriber, cb));
    }
}
