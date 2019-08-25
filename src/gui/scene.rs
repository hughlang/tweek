use super::*;
use crate::core::*;
use crate::tools::*;
use crate::events::*;

use std::{
    any::TypeId,
    collections::HashMap,
};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color},
    input::{Key},
    lifecycle::{Window},
};

use log::Level;

/// Enum wrapper for actions that can be applied to a Scene and its child objects.
#[derive(Clone, Debug)]
pub enum SceneAction {
    /// Undefined
    None,
    /// An action that specifies a Tween that is applied to a specific GUI object
    Animate(PropSet, TypeId, u32)
}

/// Container for holding a collection of views and controls and propagating events, movements, and other
/// actions up and down the hierarchy.
pub struct Scene {
    /// The base layer
    pub layer: Layer,
    /// Display name
    pub(crate) name: String,
    /// The list of Displayable objects
    views: Vec<Box<dyn Displayable>>,
    /// The list of Responder objects
    controls: Vec<Box<dyn Responder>>,
    /// Index in controls vec of currently selected control (ie, textfield)
    active_control_idx: Option<usize>,
    /// Index in controls vec of the next selected control
    next_control_idx: Option<usize>,
    /// HashMap that stores SceneActions
    action_map: HashMap<String, SceneAction>,
    /// Should this scene respond to mouse/touch events?
    pub is_interactive: bool,
    /// Should the theme respond to global theme changes?
    pub lock_theme: bool,
    /// Overlay for lightbox-style modals
    overlay: Option<ShapeView>,
    /// A timeline to coordinate scene animations
    timeline: Option<Timeline>,
    /// Running count of views added to this scene. Used in assigning new id values
    view_count: u32,
}

impl Scene {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);
        Scene {
            layer,
            name: "Scene".to_string(),
            views: Vec::new(),
            controls: Vec::new(),
            active_control_idx: None,
            next_control_idx: None,
            action_map: HashMap::new(),
            is_interactive: true,
            lock_theme: false,
            overlay: None,
            timeline: None,
            view_count: 0,
        }
    }

    pub fn with_id(mut self, id: u32, name: &str) -> Self {
        self.set_id(id);
        self.name = name.to_string();
        self
    }

    pub fn set_timeline(&mut self, timeline: Timeline) {
        self.timeline = Some(timeline);
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
            return;
        }

        self.view_count += 1;
        if view.get_id() == 0 {
            view.set_id(self.get_id() + self.view_count);
        }
        let offset = view.get_frame().pos - self.layer.frame.pos;
        view.get_layer_mut().anchor_pt = offset;

        self.views.push(view);
    }

    /// Add a Responder and set the position based on Scene origin
    pub fn add_control(&mut self, mut view: Box<dyn Responder>) {
        self.view_count += 1;
        if view.get_id() == 0 {
            view.set_id(self.get_id() + self.view_count);
        }
        let offset = view.get_frame().pos - self.layer.frame.pos;
        view.get_layer_mut().anchor_pt = offset;

        self.controls.push(view);
    }

    /// This is a helper method for adding a control with a command that executes when activated as an alternative to
    /// the add_control() method
    /// Status: Experimental
    ///
    pub fn add_command(&mut self, cmd: Command) {
        if let Ok(mut button) = cmd.control.downcast::<Button>() {
            if let Some(cb) = cmd.action {
                button.set_onclick(cb);
                self.add_control(button);
            }
        } else {
            eprintln!("SKIP >>>>>>>>>>>>>>>> control");
        }
        match cmd.transition.event {
            TweenType::Move => {
                if let Ok(mut event) = cmd.result.downcast::<SceneEvent>() {
                    self.add_event_action(event, SceneAction::Animate(cmd.transition, cmd.target_type, cmd.target_id));
                    log::debug!("Add Command with target={:?} // count={}", event, self.action_map.len());
                }
            }
            _ => ()
        }
    }

    pub fn find_action(&self, event: SceneEvent) -> Option<SceneAction> {
        let key = format!("{:?}", event);
        if let Some(action) = self.action_map.get(&key) {
            return Some(action.clone());
        }
        None
    }

    /// Add SceneAction to the action_map
    /// Status: Experimental
    fn add_event_action(&mut self, event: SceneEvent, action: SceneAction) {
        let key = format!("{:?}", event);
        eprintln!(">>>>>>>>> key={:?} action={:?}", key, action);
        self.action_map.insert(key, action);
    }

    /// Handle the given Event
    /// Status: Experimental
    pub fn handle_event(&mut self, event: &SceneEvent) -> bool {
        println!(">>> handle_event {:?}", event);
        let key = format!("{:?}", event);
        if let Some(action) = self.action_map.get(&key) {
            self.handle_action(&action.clone());
        }
        false
    }

    pub fn handle_action(&mut self, action: &SceneAction) -> bool {
        match action {
            SceneAction::Animate(propset, type_id, id) => {
                if id != &self.get_id() {
                    log::error!("REJECTING [{}] type={:?} -- {:?}", id, gui_print_type(type_id), propset);
                    return false;
                }
                log::debug!("[{}] type={:?} -- {:?}", id, gui_print_type(type_id), propset);
                let type_id = type_id.clone();
                if type_id == TypeId::of::<Scene>() {
                    self.layer.animate_with_props(propset.clone());
                } else {
                    // Note: these have never been tested, since the only SceneAction implementation is
                    // moving a modal Scene.
                    let found: bool = false;
                    for view in &mut self.views {
                        if type_id == view.get_type_id() && id == &view.get_id() {
                            view.get_layer_mut().animate_with_props(propset.clone());
                            return true;
                        }
                    }
                    if !found {
                        for view in &mut self.controls {
                            if type_id == view.get_type_id() && id == &view.get_id() {
                                view.get_layer_mut().animate_with_props(propset.clone());
                                return true;
                            }
                        }
                    }
                }
            }
            _ => ()
        }
        false
    }

    /// Display an darkened overlay view for a lightbox modal effect
    pub fn show_overlay(&mut self) {
        let frame = self.layer.frame.clone();
        let fill_color = Color::from_hex("#CCCCCC");
        let mut rectangle = DrawShape::rectangle(&frame, Some(fill_color), None, 0.0, 0.0);
        let mut shape = ShapeView::new(frame, ShapeDef::Rectangle).with_mesh(&mut rectangle);
        shape.layer.apply_props(&[alpha(0.3)]);
        self.overlay = Some(shape);
        // self.add_view(Box::new(shape));
    }

    /// Hide the overlay by removing it.
    pub fn hide_overlay(&mut self) {
        self.overlay = None;
    }

    /// Useful function to print out the scene hierarchy. Each Displayable object provides the
    /// debug_out() function which returns a String information about itself and display frame.
    /// This is aggregated in this function and printed out. It is called in the notify() method
    /// in Scene, so it does not need to be public.
    fn print_scene(&self) {
        if log_enabled!(Level::Debug) { // Don't bother building the text output if log level is not enabled
            let mut rows: Vec<String> = Vec::new();
            let text_width = 60 as usize;
            let remainder = text_width - self.name.len(); // Calculate the approximate # of chars to print
            let header = format!("\n=== {} {}", self.name, "=".repeat(remainder));
            rows.push(header);
            let out = format!("\n{}", self.debug_out());
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
        if log_enabled!(Level::Debug) { // Don't bother building the text output if log level is not enabled
            for view in &mut self.views {
                let offset = view.get_frame().pos - self.layer.frame.pos;
                let anchor = view.get_layer_mut().anchor_pt;
                if anchor != offset {
                    let element = view.debug_out();
                    log::error!("Wrong position: Expected={:?} actual={:?}", self.layer.frame.pos + anchor, self.layer.frame.pos + offset);
                    log::error!("Element={:?}", view.debug_out());
                }
            }
            for view in &mut self.controls {
                let offset = view.get_frame().pos - self.layer.frame.pos;
                let anchor = view.get_layer_mut().anchor_pt;
                if anchor != offset {
                    let element = view.debug_out();
                    log::error!("Wrong position: Expected={:?} actual={:?}", self.layer.frame.pos + anchor, self.layer.frame.pos + offset);
                    log::error!("Element={:?}", view.debug_out());
                }
            }
        }
    }
}

impl Displayable for Scene {

    fn get_id(&self) -> u32 { self.layer.get_id() }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Scene>()
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
        self.layer.apply_theme(theme);
        // match self.layer.bg_style {
        //     BackgroundStyle::Solid(_, corner) => {
        //         self.layer.bg_style = BackgroundStyle::Solid(theme.bg_color, corner)
        //     }
        //     _ => ()
        // }
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

    fn notify(&mut self, event: &DisplayEvent) {

        for view in &mut self.controls {
            view.notify(event);
        }
        for view in &mut self.views {
            view.notify(event);
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.notify(event);
        }
        eprintln!("===== notify! {:?} ======", event);
        match event {
            DisplayEvent::Ready => {
                self.layer.on_ready();
                self.print_scene();
                self.validate_scene();
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
            }
            _ => {}
        }
    }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        self.layer.tween_update();

        let events = self.layer.notifications.borrow_mut().events.filter::<LayerEvent>();
        for evt in events {
            match evt {
                LayerEvent::Move(id, type_id, state) => {
                    match state {
                        PlayState::Completed => {
                            let out = self.debug_out();
                            eprintln!("{:?} –– {:?}", evt, out);
                            self.notify(&DisplayEvent::Moved);
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        }
        let offset = self.layer.get_movement_offset();
        state.offset = (offset.x, offset.y);

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
            view.notify(&DisplayEvent::Activate);
            self.active_control_idx = Some(next_idx);
            self.next_control_idx = None;
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
    }

    /// The top-level objects in the scene should all use the scene's coordinate system and
    /// therefore, this render() method should only call render() for all child Displayable objects.
    /// That's the current plan. It may change.
    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        // self.layer.draw_background(window);

        let mut mask_areas: Vec<Rectangle> = Vec::new();

        for view in &mut self.views {
            view.render(theme, window);
        }
        for view in &mut self.controls {
            if let Some(perimeter) = view.get_perimeter_frame() {
                let mut blocks = UITools::get_perimeter_blocks(&view.get_frame(), &perimeter);
                mask_areas.append(&mut blocks);
            }
            view.render(theme, window);
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.render(theme, window);
        }

        // if mask_areas.len() > 0 {
        //     let mesh_task = UITools::draw_rectangles(mask_areas, self.layer.transition.color);
        //     window.add_task(mesh_task);
        // }
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        // TODO: Verify if hover is handled ok
        for view in &mut self.controls {
            let hover = view.handle_mouse_at(pt);
            if hover {
                return true;
            }
        }
        for view in &mut self.views {
            let hover = view.handle_mouse_at(pt);
            if hover {
                return true;
            }
        }
        if let Some(timeline) = &mut self.timeline {
            timeline.handle_mouse_at(pt);
        }
        false
    }

    fn get_routes(&mut self) -> Vec<String> {
        let mut routes: Vec<String> = Vec::new();
        let base = format!("/{}-{}", gui_print_type(&self.get_type_id()), self.get_id());
        routes.push(base.clone());
        for view in &mut self.views {
            for path in view.get_routes() {
                let route = format!("{}{}", &base, path);
                routes.push(route);
            }
        }
        for view in &mut self.controls {
            for path in view.get_routes() {
                let route = format!("{}{}", &base, path);
                routes.push(route);
            }
        }
        if let Some(timeline) = &mut self.timeline {
            for path in timeline.get_routes() {
                let route = format!("{}{}", &base, path);
                routes.push(route);
            }
        }
        routes
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
