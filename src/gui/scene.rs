use super::*;
use crate::core::*;
use crate::events::*;
use crate::tools::*;

use std::{any::TypeId, collections::BTreeMap};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, MeshTask},
    input::Key,
    lifecycle::Window,
};

use log::Level;

/// Enum wrapper for actions that can be applied to a Scene and its child objects.
#[derive(Clone, Debug)]
pub enum SceneAction {
    /// Undefined
    None,
    /// An action that specifies a Tween that is applied to a specific GUI object
    Animate(PropSet, NodeID),
}

/// Container for holding a collection of views and controls and propagating events, movements, and other
/// actions up and down the hierarchy.
pub struct Scene {
    /// The base layer
    pub layer: Layer,
    /// Display name
    pub name: String,
    /// The list of Displayable objects
    pub views: BTreeMap<u32, Box<dyn Displayable>>,
    /// The list of Responder objects
    pub controls: BTreeMap<u32, Box<dyn Responder>>,
    /// A timeline to coordinate scene animations
    pub(crate) timeline: Option<Timeline>,
    /// A storage queue for views being loaded or in transition
    views_queue: Vec<Box<dyn Displayable>>,
    /// A storage queue for controls being loaded or in transition
    controls_queue: Vec<Box<dyn Responder>>,
    /// Index in controls vec of currently selected control (ie, textfield)
    active_field_id: Option<u32>,
    /// Index in controls vec of the next selected control
    next_field_id: Option<u32>,
    /// Should this scene respond to mouse/touch events?
    /// TODO: Replace with new Scene layering hierarchy
    pub is_interactive: bool,
    /// Optional background that displays full screen and does not move. It also prevents lower scenes from
    /// receiving mouse events.
    pub bg_mask: Option<MeshTask>,
    /// Cac
    screen_size: Vector,
}

impl Scene {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);
        Scene {
            layer,
            name: "untitled".to_string(),
            views: BTreeMap::new(),
            controls: BTreeMap::new(),
            timeline: None,
            views_queue: Vec::new(),
            controls_queue: Vec::new(),
            active_field_id: None,
            next_field_id: None,
            is_interactive: true,
            bg_mask: None,
            screen_size: Vector::ZERO,
        }
    }

    /// Builder method to set the id and name for this Scene
    pub fn with_id(mut self, id: u32, name: &str) -> Self {
        self.set_id(id);
        self.name = name.to_string();
        self.layer.set_path(&[]);
        self
    }

    /// Set a Timeline object which has a collection of Sprites to animate
    pub fn set_timeline(&mut self, timeline: Timeline) {
        self.timeline = Some(timeline);
    }

    pub fn append_timeline(&mut self, view: Box<dyn Displayable>, start: f64) {
        if let Some(timeline) = &mut self.timeline {
            timeline.add_sprite(view, start);
        }
    }

    /// Add a Displayable and set the position based on Scene origin
    /// If the object is actually a Responder, warn and do not add.
    pub fn add_view(&mut self, mut view: Box<dyn Displayable>) {
        let type_id = view.get_type_id();
        if GUI_RESPONDERS.contains(&type_id) {
            // TODO: Use unwrap_or to customize name
            if let Some(name) = GUI_TYPES_MAP.get(&type_id) {
                log::warn!("Wrong type for this method call: {:?}", name);
            }
        }
        view.set_origin(self.layer.frame.pos);
        self.views_queue.push(view);
    }

    /// Add a Responder and set the position based on Scene origin
    /// Returns the id value of the view, which is assigned if the previous value was 0.
    pub fn add_control(&mut self, mut view: Box<dyn Responder>) {
        view.set_origin(self.layer.frame.pos);
        self.controls_queue.push(view);
    }

    /// This is a helper method for adding a control with a command that executes when activated as an alternative to
    /// the add_control() method
    /// Status: Experimental
    /// FIXME: The Command model was yet another attempt at events handling. Probably remove
    pub fn add_command(&mut self, cmd: Command) {
        if let Ok(button) = cmd.source.downcast::<Button>() {
            if let Ok(_event) = cmd.event.downcast::<SceneEvent>() {
                // FIXME: button.set_click_event deprecated and removed
                self.add_control(button);
            }
        } else {
            log::error!("SKIP >>>>>>>>>>>>>>>> control");
            return;
        }
    }

    /// Useful function to print out the scene hierarchy. Each Displayable object provides the
    /// debug_out() function which returns a String information about itself and display frame.
    /// This is aggregated in this function and printed out. It is called in the notify() method
    /// in Scene, so it does not need to be public.
    fn print_scene(&self) {
        if log_enabled!(Level::Debug) {
            // Don't bother building the text output if log level is not enabled
            let mut rows: Vec<String> = Vec::new();
            let text_width = 60 as usize;
            let remainder = text_width - self.name.len(); // Calculate the approximate # of chars to print
            let header = format!("=== {} {}\n", self.name, "=".repeat(remainder));
            rows.push(header);
            let out = format!("{}", self.debug_out());
            rows.push(out);
            const SEP: &str = "\n| ";
            for view in self.views.values() {
                let text = view.debug_out();
                let result = text.lines().map(|x| format!("{}{}", SEP, x)).collect();
                rows.push(result);
            }
            for view in self.controls.values() {
                let text = view.debug_out();
                let result = text.lines().map(|x| format!("{}{}", SEP, x)).collect();
                rows.push(result);
            }
            log::debug!("{}", rows.join(""));
        }
    }

    fn validate_scene(&mut self) {
        if log_enabled!(Level::Debug) {
            // Don't bother building the text output if log level is not enabled
            for view in self.views.values() {
                view.validate_position(self.layer.frame.pos);
            }
            for view in self.controls.values() {
                view.validate_position(self.layer.frame.pos);
            }
        }
    }
}

impl Displayable for Scene {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Scene>()
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

    fn align_view(&mut self, origin: Vector) {
        log::debug!("New origin={:?}", origin);
        let anchor_pt = self.get_layer().anchor_pt;
        self.get_layer_mut().frame.pos = anchor_pt + origin;

        for view in &mut self.controls.values_mut() {
            view.align_view(origin);
        }
        for view in &mut self.views.values_mut() {
            view.align_view(origin);
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.align_view(origin);
        }
    }

    fn move_to(&mut self, pos: (f32, f32)) {
        self.layer.frame.pos.x = pos.0;
        self.layer.frame.pos.y = pos.1;
        // TODO: Move child objects
    }

    fn set_theme(&mut self, theme: &mut Theme) {
        let ok = self.layer.apply_theme(theme);
        if !ok {
            return;
        }
        for view in &mut self.controls.values_mut() {
            view.set_theme(theme);
        }
        for view in &mut self.views.values_mut() {
            view.set_theme(theme);
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.set_theme(theme);
        }
    }

    fn handle_event(&mut self, event: &EventBox, app_state: &mut AppState) {
        if let Some(timeline) = &mut self.timeline {
            timeline.handle_event(event, app_state);
        }
        for view in &mut self.controls.values_mut() {
            view.handle_event(event, app_state);
        }
        for view in &mut self.views.values_mut() {
            view.handle_event(event, app_state);
        }

        if let Ok(evt) = event.downcast_ref::<SceneEvent>() {
            log::debug!("{} SceneEvent={:?}", self.layer.debug_id(), evt);
            // log::debug!("Source={:?}", event.event_source());

            match evt {
                SceneEvent::Show(target) => {
                    if target.id == self.get_id() && target.type_id == self.get_type_id() {
                        let frame = Rectangle::new((0.0, 0.0), (self.screen_size.x, self.screen_size.y));
                        // TODO: set from theme?
                        let mut fill_color = Color::from_hex("#000000");
                        fill_color.a = 0.7;
                        let mut mesh = DrawShape::rectangle(&frame, Some(fill_color), None, 0.0, 0.0);
                        let mut mesh_task = MeshTask::new(0);
                        mesh_task.append(&mut mesh);
                        self.bg_mask = Some(mesh_task);
                    }
                }
                SceneEvent::Hide(_) => {
                    self.bg_mask = None;
                }
                _ => (),
            }
        }
        if let Ok(evt) = event.downcast_ref::<PlayerEvent>() {
            log::debug!("{} PlayerEvent={:?}", self.debug_id(), evt);
            match evt {
                PlayerEvent::Reset => {
                    self.reset();
                    self.print_scene();
                    self.validate_scene();
                }
                _ => (),
            }
        }
    }

    fn notify(&mut self, event: &DisplayEvent) {
        log::debug!("{} notify event={:?}", self.debug_id(), event);
        match event {
            DisplayEvent::Ready => {
                self.layer.on_ready();
                self.print_scene();
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
                self.print_scene();
                self.validate_scene();
            }
            _ => {}
        }

        for view in &mut self.controls.values_mut() {
            view.notify(event);
        }
        for view in &mut self.views.values_mut() {
            view.notify(event);
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.notify(event);
        }
    }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        self.screen_size = Vector::new(state.window_size.0, state.window_size.1); // Add this to DisplayEvent instead
        self.layer.tween_update(state);

        // Awkwardly, check if another control will become active and first try to
        // deactivate the previous control. Then activate the next one
        // FIXME: Need better model for handling active/next field events.
        if let Some(next_field_id) = self.next_field_id {
            if let Some(last_field_id) = self.active_field_id {
                if last_field_id != next_field_id {
                    if let Some(view) = &mut self.controls.get_mut(&last_field_id) {
                        view.notify(&DisplayEvent::Deactivate);
                    }
                }
            }
            if let Some(view) = &mut self.controls.get_mut(&next_field_id) {
                view.notify(&DisplayEvent::Activate);
                self.active_field_id = Some(next_field_id);
                self.next_field_id = None;
            }
        }

        if self.layer.is_animating() {
            state.offset = self.layer.get_movement_offset();
            if state.offset != Vector::ZERO {
                self.layer.layer_state = LayerState::Moving;
            }
        // log::trace!("{} scene offset={:?}", self.layer.debug_id(), state.offset);
        } else {
            state.offset = Vector::ZERO;
        }

        for view in &mut self.controls.values_mut() {
            view.update(window, state);
        }
        for view in &mut self.views.values_mut() {
            view.update(window, state);
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.update(window, state);
        }

        let events = self.layer.notifications.borrow_mut().events.filter::<LayerEvent>();
        for evt in events {
            match evt {
                LayerEvent::Move(_id, _type_id, evt_state) => match evt_state {
                    PlayState::Completed => {
                        state.offset = Vector::ZERO;
                        // This was added for ads-sandbox carousel view. However, this will broadcast the event to all
                        // Scenes in a Stage or Controller hierarchy.
                        // TODO: Decide how to target this event at a specific Scene in a view hierarchy
                        state.event_bus.register_event(DisplayEvent::Moved);
                        log::debug!(
                            "self.layer.frame.pos={:?} anchor_pt={:?}",
                            self.layer.frame.pos,
                            self.layer.anchor_pt
                        );
                        self.align_view(self.layer.frame.pos);
                        self.notify(&DisplayEvent::Moved);
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    /// The top-level objects in the scene should all use the scene's coordinate system and
    /// therefore, this render() method should only call render() for child Displayable objects.
    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        if let Some(mask) = &self.bg_mask {
            window.add_task(mask.clone());
        }

        self.layer.draw_background(window);
        self.layer.draw_border(window);

        for view in &mut self.views.values_mut().filter(|x| x.get_layer().visibility == Visibility::Visible) {
            view.render(theme, window);
        }
        for view in &mut self.controls.values_mut() {
            view.render(theme, window);
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.render(theme, window);
        }
    }

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        // TODO: Verify if hover is handled ok
        for view in &mut self.controls.values_mut() {
            let hover = view.handle_mouse_at(pt, window);
            if hover {
                return true;
            }
        }
        for view in &mut self.views.values_mut() {
            let hover = view.handle_mouse_at(pt, window);
            if hover {
                return true;
            }
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.handle_mouse_at(pt, window);
        }
        false
    }

    /// This should find all of the views/controls that have been queued for creation
    /// and move them into the corresponding Maps, while assigning unique id values
    /// for each.
    fn view_will_load(&mut self, ctx: &mut StageContext, app_state: &mut AppState) {
        let parent_nodes = self.get_layer().node_path.nodes.clone();
        let parent_path = NodePath::new(parent_nodes.clone());
        app_state.append_node(parent_path.clone());

        for mut view in self.views_queue.drain(..) {
            let id = app_state.new_id();
            view.set_id(id);
            view.get_layer_mut().set_path(&parent_nodes);
            view.view_will_load(ctx, app_state);

            let subscriber = view.get_layer().node_path.clone();
            if let Some(tag) = view.get_layer().tag {
                app_state.assign_tag(tag, subscriber.clone());
            }
            // If the scene is subscriber to notifications, add them here.
            for key in &view.get_layer().queued_observers {
                app_state.register_observer(key.clone(), subscriber.clone())
            }
            // Add event listeners from node to AppState
            for (key, cb) in view.get_layer_mut().event_listeners.drain() {
                ctx.add_event_listener(key, cb, subscriber.clone());
            }

            self.views.insert(id, view);
        }
        for mut view in self.controls_queue.drain(..) {
            let id = app_state.new_id();
            view.set_id(id);
            view.get_layer_mut().set_path(&parent_nodes);
            if let Some(tag) = view.get_layer().tag {
                app_state.assign_tag(tag, view.get_layer().node_path.clone());
            }
            // If the scene is subscriber to notifications, add them here.
            for key in &view.get_layer().queued_observers {
                app_state.register_observer(key.clone(), view.get_layer().node_path.clone())
            }
            // Add event listeners from node to AppState
            let subscriber = view.get_layer().node_path.clone();
            for (key, cb) in view.get_layer_mut().event_listeners.drain() {
                ctx.add_event_listener(key, cb, subscriber.clone());
            }

            self.controls.insert(id, view);
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.set_id(app_state.new_id());
            timeline.get_layer_mut().set_path(&parent_nodes);
            timeline.view_will_load(ctx, app_state);
        }
    }
}

impl Responder for Scene {
    fn set_field_value(&mut self, value: &FieldValue, type_id: TypeId, layer_id: u32) -> bool {
        for view in &mut self.controls.values_mut() {
            let success = view.set_field_value(value, type_id, layer_id);
            if success {
                return true;
            }
        }
        false
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        for (_, view) in &mut self.controls {
            let focus = view.handle_mouse_down(pt, state);
            if focus {
                self.next_field_id = Some(view.get_id());
                return true;
            }
        }
        false
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        for (_, view) in &mut self.controls {
            let focus = view.handle_mouse_up(pt, state);
            if focus {
                return true;
            }
        }
        false
    }

    fn handle_mouse_scroll(&mut self, pt: &Vector, state: &mut AppState) {
        for view in &mut self.controls.values_mut() {
            view.handle_mouse_scroll(pt, state);
        }
    }

    fn handle_key_press(&mut self, c: char, window: &mut Window) {
        if let Some(active_field_id) = self.active_field_id {
            if let Some(view) = &mut self.controls.get_mut(&active_field_id) {
                log::debug!("handled char={:?}", c);
                view.handle_key_press(c, window);
            }
        }
    }

    fn handle_key_command(&mut self, key: &Key, window: &mut Window) -> bool {
        if let Some(active_field_id) = self.active_field_id {
            if let Some(view) = &mut self.controls.get_mut(&active_field_id) {
                let handled = view.handle_key_command(key, window);
                if handled {
                    log::debug!("handled key={:?}", key);
                    match key {
                        Key::Tab => {
                            let mut iter = self.controls.keys().cycle();
                            if let Some(_) = iter.find(|x| **x == active_field_id) {
                                self.next_field_id = Some(*iter.next().unwrap());
                            }
                            log::debug!("active={:?} next={:?}", active_field_id, self.next_field_id);
                            return true;
                        }
                        Key::Return => {}
                        _ => (),
                    }
                    return true;
                }
            }
        } else {
            // TODO: Check other listeners
        }
        false
    }
}

impl Playable for Scene {
    fn play(&mut self) {}

    fn reset(&mut self) {
        for view in &mut self.views.values_mut() {
            view.get_layer_mut().reset();
        }
        for view in &mut self.controls.values_mut() {
            view.get_layer_mut().reset();
        }
    }
}
