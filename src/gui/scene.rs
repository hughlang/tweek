use super::*;
use crate::core::*;
use crate::events::*;
use crate::tools::*;

use std::{any::TypeId, collections::HashMap};

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
    Animate(PropSet, Node),
}

/// Container for holding a collection of views and controls and propagating events, movements, and other
/// actions up and down the hierarchy.
pub struct Scene {
    /// The base layer
    pub layer: Layer,
    /// Display name
    pub(crate) name: String,
    /// The list of Displayable objects
    pub views: Vec<Box<dyn Displayable>>,
    /// The list of Responder objects
    pub controls: Vec<Box<dyn Responder>>,
    /// Index in controls vec of currently selected control (ie, textfield)
    active_control_idx: Option<usize>,
    /// Index in controls vec of the next selected control
    next_control_idx: Option<usize>,
    /// Initial storage of added Commands as a mapping of the source to target
    pub event_actions: HashMap<(SceneEvent, Option<String>), SceneAction>,
    /// Should this scene respond to mouse/touch events?
    pub is_interactive: bool,
    // pub event_triggers: HashMap<(u32, TypeId, SceneEve)
    /// Records last SceneEvent. Used to determine whether modal is displayed or not.
    last_event: SceneEvent,
    /// Optional background that displays full screen and does not move. It also prevents lower scenes from
    /// receiving mouse events.
    pub bg_mask: Option<MeshTask>,
    /// A timeline to coordinate scene animations
    timeline: Option<Timeline>,
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
            views: Vec::new(),
            controls: Vec::new(),
            active_control_idx: None,
            next_control_idx: None,
            event_actions: HashMap::new(),
            is_interactive: true,
            last_event: SceneEvent::None,
            bg_mask: None,
            timeline: None,
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
        self.views.push(view);
    }

    /// Add a Responder and set the position based on Scene origin
    /// Returns the id value of the view, which is assigned if the previous value was 0.
    pub fn add_control(&mut self, mut view: Box<dyn Responder>) {
        view.set_origin(self.layer.frame.pos);
        self.controls.push(view);
    }

    /// This is a helper method for adding a control with a command that executes when activated as an alternative to
    /// the add_control() method
    /// Status: Experimental
    ///
    pub fn add_command(&mut self, cmd: Command) {
        if let Ok(mut button) = cmd.source.downcast::<Button>() {
            if let Ok(event) = cmd.event.downcast::<SceneEvent>() {
                button.set_click_event(event);
                self.add_control(button);

                // Get the route path for the new object and use that as part of the event_actions key
                // let path = print_full_path(node);
                // log::debug!("add_control path={:?}", path);
                let target = Node::new(cmd.target_id, cmd.target_type);
                // self.event_actions.insert((event, Some(path)), SceneAction::Animate(cmd.transition, target));
            }
        } else {
            log::error!("SKIP >>>>>>>>>>>>>>>> control");
            return;
        }
    }

    /// Handle the given Event.
    /// TBD: Remove?
    /// Status: Experimental
    // pub fn handle_event(&mut self, event: &SceneEvent, _source: &Option<String>) {
    //     self.last_event = event.clone();
    //     match event {
    //         SceneEvent::Show(target) => {
    //             if target.id == self.get_id() && target.type_id == self.get_type_id() {
    //                 let frame = Rectangle::new((0.0, 0.0), (self.screen_size.x, self.screen_size.y));
    //                 // TODO: set from theme?
    //                 let mut fill_color = Color::from_hex("#000000");
    //                 fill_color.a = 0.7;
    //                 let mut mesh = DrawShape::rectangle(&frame, Some(fill_color), None, 0.0, 0.0);
    //                 let mut mesh_task = MeshTask::new(0);
    //                 mesh_task.append(&mut mesh);
    //                 self.bg_mask = Some(mesh_task);
    //             }
    //         }
    //         SceneEvent::Hide(_) => {
    //             self.bg_mask = None;
    //         }
    //         _ => (),
    //     }
    // }

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
            for view in &self.views {
                let text = view.debug_out();
                let result = text.lines().map(|x| format!("{}{}", SEP, x)).collect();
                rows.push(result);
            }
            for view in &self.controls {
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
            for view in &self.views {
                view.validate_position(self.layer.frame.pos);
            }
            for view in &self.controls {
                view.validate_position(self.layer.frame.pos);
            }
        }
    }
}

impl Displayable for Scene {
    fn get_id(&self) -> u32 {
        self.layer.get_id()
    }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

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

        for view in &mut self.controls {
            view.align_view(origin);
        }
        for view in &mut self.views {
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
        for view in &mut self.controls {
            view.set_theme(theme);
        }
        for view in &mut self.views {
            view.set_theme(theme);
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.set_theme(theme);
        }
    }

    fn handle_event(&mut self, event: &EventBox) {
        if let Some(timeline) = &mut self.timeline {
            timeline.handle_event(event);
        }
        for view in &mut self.controls {
            view.handle_event(event);
        }
        for view in &mut self.views {
            view.handle_event(event);
        }

        if let Ok(evt) = event.downcast_ref::<SceneEvent>() {
            log::debug!("{} SceneEvent={:?}", self.layer.debug_id(), evt);
            log::debug!("Source={:?}", event.event_info());

            self.last_event = evt.clone();
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

        for view in &mut self.controls {
            view.notify(event);
        }
        for view in &mut self.views {
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
        if let Some(next_idx) = self.next_control_idx {
            if let Some(last_idx) = self.active_control_idx {
                if last_idx != next_idx {
                    let view = &mut self.controls[last_idx];
                    view.notify(&DisplayEvent::Deactivate);
                }
            }
            let view = &mut self.controls[next_idx];
            // FIXME: Redundant if first activation of field
            view.notify(&DisplayEvent::Activate);
            self.active_control_idx = Some(next_idx);
            self.next_control_idx = None;
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

        for view in &mut self.controls {
            view.update(window, state);
        }
        for view in &mut self.views {
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
        // match self.layer.layer_state {
        //     LayerState::Completed => {
        //         self.print_scene();
        //         self.validate_scene();
        //     }
        //     _ => ()
        // }
        if let Some(mask) = &self.bg_mask {
            window.add_task(mask.clone());
        }

        self.layer.draw_background(window);
        self.layer.draw_border(window);

        for view in &mut self.views {
            view.render(theme, window);
        }
        for view in &mut self.controls {
            view.render(theme, window);
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.render(theme, window);
        }
    }

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        // TODO: Verify if hover is handled ok
        for view in &mut self.controls {
            let hover = view.handle_mouse_at(pt, window);
            if hover {
                return true;
            }
        }
        for view in &mut self.views {
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

    fn get_routes(&mut self) -> Vec<String> {
        let mut routes: Vec<String> = Vec::new();
        let base = format!("{}-{}", gui_print_type(&self.get_type_id()), self.get_id());
        let route = format!("/{}", &base);
        routes.push(route);
        for view in &mut self.views {
            for path in view.get_routes() {
                let route = format!("/{}/{}", &base, path);
                routes.push(route);
            }
        }
        for view in &mut self.controls {
            for path in view.get_routes() {
                let route = format!("/{}/{}", &base, path);
                routes.push(route);
            }
        }
        if let Some(timeline) = &mut self.timeline {
            for path in timeline.get_routes() {
                let route = format!("/{}/{}", &base, path);
                routes.push(route);
            }
        }
        routes
    }

    fn get_layer_for_route(&mut self, route: &str) -> Option<&mut Layer> {
        let parts: Vec<&str> = route.split("/").filter(|x| x.len() > 0).collect();

        // If the last segment of the route is this Scene, then return it.
        if let Some(part) = parts.last() {
            if *part == self.node_key() {
                return Some(self.get_layer_mut());
            }
        }
        // Try find the route in subviews
        // let part = parts.swap_remove(0);
        let subpath = parts.join("/");
        for view in &mut self.views {
            if view.get_layer_for_route(&subpath).is_some() {
                return view.get_layer_for_route(&subpath);
            }
        }
        for view in &mut self.controls {
            if view.get_layer_for_route(&subpath).is_some() {
                return view.get_layer_for_route(&subpath);
            }
        }
        // TBD: Also check timeline sprites?
        None
    }
}

impl Responder for Scene {
    fn set_field_value(&mut self, value: &FieldValue, type_id: TypeId, layer_id: u32) -> bool {
        for view in &mut self.controls {
            let success = view.set_field_value(value, type_id, layer_id);
            if success {
                return true;
            }
        }
        false
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        for (i, view) in &mut self.controls.iter_mut().enumerate() {
            let focus = view.handle_mouse_down(pt, state);
            if focus {
                self.next_control_idx = Some(i);
                return true;
            }
        }
        false
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        for (_, view) in &mut self.controls.iter_mut().enumerate() {
            let focus = view.handle_mouse_up(pt, state);
            if focus {
                return true;
            }
        }
        false
    }

    fn handle_mouse_scroll(&mut self, pt: &Vector, state: &mut AppState) {
        for view in &mut self.controls {
            view.handle_mouse_scroll(pt, state);
        }
    }

    fn handle_key_press(&mut self, c: char, window: &mut Window) {
        if let Some(active_idx) = self.active_control_idx {
            let view = &mut self.controls[active_idx];
            view.handle_key_press(c, window);
        }
    }

    fn handle_key_command(&mut self, key: &Key, window: &mut Window) -> bool {
        if let Some(active_idx) = self.active_control_idx {
            let controls_count = self.controls.len();
            let view = &mut self.controls[active_idx];
            let handled = view.handle_key_command(key, window);
            if handled {
                match key {
                    Key::Tab => {
                        let next_idx;
                        if active_idx + 1 == controls_count {
                            next_idx = 0;
                        } else {
                            next_idx = active_idx + 1;
                        }
                        if next_idx != active_idx {
                            // log::debug!("next_idx={:?} WAS={:?}", next_idx, active_idx);
                            self.next_control_idx = Some(next_idx);
                        }
                        return true;
                    }
                    Key::Return => {}
                    _ => (),
                }
                return true;
            }
        } else {
            // TODO: Check other listeners
        }
        false
    }
}

impl ViewLifecycle for Scene {
    fn view_will_load(&mut self, theme: &mut Theme, app_state: &mut AppState) {
        self.set_id(app_state.new_id());
        for view in &mut self.views {
            view.set_id(app_state.new_id());
        }
        for view in &mut self.controls {
            view.set_id(app_state.new_id());
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.view_will_load(theme, app_state);
        }
    }
}

impl Playable for Scene {
    fn play(&mut self) {}

    fn reset(&mut self) {
        for view in &mut self.views {
            view.get_layer_mut().reset();
        }
        for view in &mut self.controls {
            view.get_layer_mut().reset();
        }
    }
}
