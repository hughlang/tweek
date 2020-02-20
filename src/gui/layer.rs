/// Layer for Quicksilver
///
use super::*;
use crate::core::*;
use crate::events::*;
use crate::tools::*;

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    fmt,
    rc::Rc,
};

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Color, Mesh, MeshTask},
    lifecycle::{run, Settings, State, Window},
};

//-- Main -----------------------------------------------------------------------

/// This is a wrapper for the quicksilver properties that are Tweenable. A GUI object that
/// has this field can be animated with Tween
pub struct Layer {
    /// Number assigned from AppState.new_id() during view_did_load() to provide object addressing scheme
    pub(crate) id: u32,
    /// Identifies the object type this layer belongs to.
    pub(crate) type_id: TypeId,
    /// Arbitrary number a user can assign for handling event notifications
    pub tag: Option<u32>,
    /// enum to define whether an object exists and renders
    pub visibility: Visibility,
    /// The rectangular position and size bounds of the object
    pub frame: Rectangle,
    /// Optional identifier that can be used to reference or locate an object in an external framework
    /// (e.g. Stretch Node)
    pub external_id: Option<Box<dyn Any>>,
    /// The path hierarchy as an array
    pub(crate) node_path: NodePath,
    /// The initial frame for the object
    pub(crate) initial: Rectangle,
    /// The starting Props for an object
    pub(self) defaults: Vec<Prop>,
    /// The current animation. Only one is allowed, which means a hover animation cannot coincide
    /// with a click animation. FIXME: Need to merge animations with time offsets or define rules for
    /// overrides
    pub(super) animation: Option<Tween>,
    /// Cached mesh from previous render pass
    pub(super) meshes: Vec<MeshTask>,
    /// Props that are modified during Tween animation
    pub(super) transition: Transition,
    /// The rotation of the object in degrees, where 0 degrees points to "3 o'clock" (TBD)
    pub rotation: f32,
    /// Corner radius for the layer.
    pub corner_radius: f32,
    /// Experimental point outside of the frame for rotation
    pub anchor_pt: Vector,
    /// Enum to describe background of object.
    pub bg_style: BackgroundStyle,
    /// Background color for the scene. Otherwise it's transparent
    pub border_style: BorderStyle,
    /// Defines font size and color
    pub font_style: FontStyle,
    /// Current MouseState of the layer
    pub(super) mouse_state: MouseState,
    /// The animation Props for the hover event
    pub hover_effect: Option<PropSet>,
    /// The animation Props for the hover event
    pub click_effect: Option<PropSet>,
    /// Callback method for handling click action
    pub(super) on_click: Option<Box<dyn FnMut(&mut AppState) + 'static>>,
    /// Temporary storage for observers to be aggregated into AppState
    pub(crate) queued_observers: Vec<String>,
    /// Storage for events that will be processed by AppState
    pub(crate) event_listeners: HashMap<String, Box<dyn FnMut(&mut AppState, NodePath) + 'static>>,
    /// Notifications
    pub(super) notifications: Rc<RefCell<Notifications>>,
    /// The currently executing animations
    pub tween_type: TweenType,
    /// Identifies the current animation state of the layer. Only used for moving Scene objects atm
    pub layer_state: LayerState,
    /// Should the layer move/resize with the parent scene?
    pub lock_frame: bool,
    /// Should the layer move/resize with the parent scene?
    pub lock_style: bool,
    /// Should extra debug output be enabled?
    pub debug: bool,
}

impl Clone for Layer {
    /// This is just a shallow copy that only clones visual attributes
    fn clone(&self) -> Self {
        Layer {
            id: 0,
            type_id: self.type_id, // the default
            tag: self.tag,
            visibility: self.visibility,
            frame: self.frame,
            external_id: None,
            node_path: self.node_path.clone(),
            initial: self.frame,
            defaults: Vec::new(),
            animation: None,
            meshes: Vec::new(),
            transition: self.transition.clone(),
            rotation: self.rotation,
            corner_radius: self.corner_radius,
            anchor_pt: self.anchor_pt,
            bg_style: self.bg_style,
            border_style: self.border_style,
            font_style: self.font_style,
            mouse_state: MouseState::None,
            hover_effect: self.hover_effect.clone(),
            click_effect: self.click_effect.clone(),
            on_click: None,
            queued_observers: Vec::new(),
            event_listeners: HashMap::new(),
            notifications: Notifications::new(),
            tween_type: TweenType::Animation,
            layer_state: LayerState::Normal,
            lock_frame: false,
            lock_style: false,
            debug: false,
        }
    }
}

impl Layer {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        Layer {
            id: 0,
            type_id: TypeId::of::<Layer>(), // the default
            tag: None,
            visibility: Visibility::Visible,
            frame,
            external_id: None,
            node_path: NodePath::default(),
            initial: frame,
            defaults: Vec::new(),
            animation: None,
            meshes: Vec::new(),
            transition: Transition::new(frame, Color::WHITE, 0.0),
            rotation: 0.0,
            corner_radius: 0.0,
            anchor_pt: frame.center(),
            bg_style: BackgroundStyle::None,
            border_style: BorderStyle::None,
            font_style: FontStyle::new(14.0, Color::BLACK),
            mouse_state: MouseState::None,
            hover_effect: None,
            click_effect: None,
            on_click: None,
            queued_observers: Vec::new(),
            event_listeners: HashMap::new(),
            notifications: Notifications::new(),
            tween_type: TweenType::Animation,
            layer_state: LayerState::Normal,
            lock_frame: false,
            lock_style: false,
            debug: false,
        }
    }

    /// Setter for id
    pub fn set_id(&mut self, id: u32) {
        self.id = id
    }
    /// Getter for id
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /// Important: Enables additional logging for this object.
    /// Since it enables debug on a dependent Tween, you should call this *after*
    /// setting the Tween animation.
    pub fn enable_debug(&mut self) {
        self.debug = true;
        if let Some(tween) = &mut self.animation {
            tween.debug = true;
        }
    }

    /// A setter to set the parent path and append this layer to the path
    pub fn set_path(&mut self, path: &[NodeID]) -> NodePath {
        let mut path = path.to_vec();
        path.push(NodeID::new(self.id, self.type_id));

        self.node_path = NodePath::new(path);
        self.node_path.clone()
    }

    /// Setter for Tween animation. Only needed outside of the Tweek crate
    /// Use start_animation() for immediate animation.
    pub fn set_animation(&mut self, tween: Tween) {
        self.animation = Some(tween)
    }

    /// Temporary storage of a notification that this object is waiting for.
    /// These are collected into AppState.observers and used when the matching
    /// notification is received.
    pub fn add_observer(&mut self, name: &str) {
        self.queued_observers.push(name.to_string());
    }

    /// Method to temporarily store an event to listen for
    pub fn add_listener(&mut self, key: &str, cb: Box<dyn FnMut(&mut AppState, NodePath) + 'static>) {
        self.event_listeners.insert(key.to_string(), cb);
    }

    /// Should be called when SceneEvent::Ready notify event is sent
    pub(super) fn on_ready(&mut self) {
        self.init_props();
        self.initial = self.frame;
        self.defaults = Tween::load_props(self);
    }

    /// Called by Displayable after notify(DisplayEvent::Move)
    /// Note: do not save any animation props or reset to original state.
    /// That should be explicitly called through reset()
    pub(super) fn on_move_complete(&mut self) {
        log::debug!(
            "on_move_complete: <{}> [{}] >> {}",
            gui_print_type(&self.type_id),
            self.get_id(),
            self.debug_out()
        );
        self.meshes.clear();
        self.layer_state = LayerState::Completed;
    }

    /// Set the default animations
    pub(super) fn apply_theme(&mut self, theme: &mut Theme) -> bool {
        if self.lock_style {
            log::debug!("Style locked: <{}> [{}]", gui_print_type(&self.type_id), self.get_id());
            return false;
        }
        // General style rules
        self.font_style = FontStyle::new(theme.font_size, theme.fg_color);

        // Style rules for GUI input fields
        if GUI_INPUTS.contains(&self.type_id) {
            self.border_style = BorderStyle::SolidLine(theme.input_fg_color, theme.border_width);
            self.bg_style = BackgroundStyle::Solid(theme.input_bg_color);
            if self.hover_effect.is_none() {
                self.hover_effect = Some(theme.on_view_hover.clone());
            }
        }
        // Button style rules
        if self.type_id == TypeId::of::<Button>() {
            if self.hover_effect.is_none() {
                self.hover_effect = Some(theme.on_button_hover.clone());
            }
            if self.click_effect.is_none() {
                self.click_effect = Some(theme.on_button_click.clone());
            }
            self.bg_style = BackgroundStyle::Solid(theme.button_bg_color);
            self.font_style = FontStyle::new(theme.font_size, theme.button_fg_color);
        }
        log::trace!("apply_theme: <{}> [{}] >> {}", gui_print_type(&self.type_id), self.get_id(), self.debug_style());
        true
    }

    /// If animation is running, run updates
    pub(crate) fn tween_update(&mut self, app_state: &mut AppState) {
        if app_state.offset != Vector::ZERO {
            self.frame.pos = self.initial.pos + app_state.offset;
            self.layer_state = LayerState::Moving;
        }

        self.notifications.borrow_mut().clear();
        if let Some(tween) = &mut self.animation {
            let mut notifier = Notifier::new();
            self.notifications.borrow_mut().attach(&mut notifier);
            let current = app_state.clock.current_time();
            // Tell tween to update its state
            tween.status(&mut notifier, Box::new(current));
            if let Some(propset) = tween.request_update(&mut notifier, Box::new(current)) {
                self.update_props(&*propset.props);
            }
            // Filter for TweenEvents
            let events = self.notifications.borrow_mut().events.filter::<TweenEvent>();

            // TODO: use iterator
            for evt in events {
                match evt {
                    TweenEvent::Started => {
                        log::trace!("Event: {} {:?}", self.debug_id(), self.tween_type);
                    }
                    TweenEvent::Completed => {
                        log::trace!("Event: {} {:?}", self.debug_id(), self.tween_type);
                        log::trace!("Layer: {:?}", self);
                        self.meshes.clear();
                        // Broadcast the TweenEvent on the event_bus
                        app_state.event_bus.dispatch_event(evt, self.node_id(), self.tag);
                        // Normalize rotation to 0-360
                        self.rotation = self.transition.rotation % 360.0;
                    }
                    _ => (),
                }
            }
        }
    }

    /// A unified method for trying to re-use cached meshes to redraw the object.
    /// If the object is_animating(), translate the position if it is moving.
    /// If the object is being moved by its parent Scene, then try to translate the
    /// position based on that.
    pub(crate) fn prepare_render(&mut self, window: &mut Window) -> Vec<MeshTask> {
        if self.debug {
            self.draw_border(window);
        }
        let mut results: Vec<MeshTask> = Vec::new();
        if self.meshes.is_empty() {
            return results;
        }

        if self.is_animating() {
            let transform = self.build_transform();
            for task in &self.meshes {
                let mut task = task.clone();
                task.apply_transform(transform);
                results.push(task);
            }
        } else {
            match self.layer_state {
                LayerState::Normal | LayerState::Completed => {
                    // Layer is not moving and cached meshes are expected to be accurate.
                    for task in &self.meshes {
                        results.push(task.clone());
                    }
                }
                LayerState::Moving => {
                    // Layer is moving, so translate the cached meshes to current position
                    let offset = self.get_movement_offset();
                    if self.debug {
                        log::trace!("{:?} @{:?} offset={:?}", self.debug_id(), self.debug_out(), offset);
                    }

                    for task in &self.meshes {
                        let mut task = task.clone();
                        for (_, vertex) in task.vertices.iter_mut().enumerate() {
                            vertex.pos = Transform::translate(offset) * vertex.pos;
                        }
                        results.push(task);
                    }
                }
            }
        }
        results
    }

    /// Create a transform for an object that is currently animating its position and/or rotation
    pub(super) fn build_transform(&self) -> Transform {
        let rotation = {
            if self.is_animating() {
                self.transition.rotation
            } else {
                self.rotation
            }
        };
        let trans = Transform::translate(self.frame.top_left() + self.frame.size() / 2)
            * Transform::rotate(rotation)
            * Transform::translate(-self.frame.size / 2)
            * Transform::scale(self.frame.size);
        trans
    }

    /// Standard method called by components when mouseover occurs
    pub(super) fn handle_mouse_over(&mut self, pt: &Vector) -> bool {
        if pt.overlaps_rectangle(&self.frame) {
            // if self.debug {
            //     log::trace!("Hover over {}", self.debug_out());
            // }
            match self.mouse_state {
                MouseState::None => {
                    // change state to hover and start animations
                    self.mouse_state = MouseState::Hover;
                    if let Some(transition) = &self.hover_effect {
                        let trans = transition.clone();
                        if trans.duration > 0.0 {
                            self.animate_with_props(trans, true);
                            self.tween_type = TweenType::Hover;
                        } else {
                            self.apply_props(&trans.props);
                        }
                    }
                }
                _ => (),
            }
            return true;
        } else {
            match self.mouse_state {
                MouseState::Hover => {
                    // log::debug!("Mouse out at: {:?}", pt);
                    if let Some(_) = &self.hover_effect {
                        self.apply_props(&self.defaults.clone());
                        self.mouse_state = MouseState::None;
                        // self.animation = None;
                    }
                }
                _ => (),
            }
        }
        false
    }

    /// Handle a mouse_down or mouse_up event and execute animation
    /// Called by GUI trait methods for handling clicks.
    pub(super) fn handle_click_animation(&mut self) {
        if let Some(transition) = &self.click_effect {
            let trans = transition.clone();
            if trans.duration > 0.0 {
                self.animate_with_props(trans, true);
                self.tween_type = TweenType::Click;
            } else {
                self.apply_props(&trans.props);
            }
        }
    }

    /// Start animating with the given PropSet
    pub fn animate_with_props(&mut self, propset: PropSet, autoplay: bool) {
        self.init_props();
        let mut tween = Tween::with(self.id, self).using_props(propset.clone());
        if autoplay {
            &tween.play();
        }
        self.animation = Some(tween);
        self.tween_type = propset.event;
    }

    /// Method to call when starting an animation. This will copy the current properties into Transition
    pub fn start_animation(&mut self, mut tween: Tween) {
        self.init_props();
        &mut tween.play();
        self.animation = Some(tween);
    }

    /// Method to evaluate BackgroundStyle and BorderStyle and draw the Mesh for the
    /// background. Used by Button, Scene, etc during render
    pub(super) fn draw_background(&self, window: &mut Window) {
        let border: (Option<Color>, f32) = {
            match self.border_style {
                BorderStyle::None => (None, 0.0),
                BorderStyle::SolidLine(color, width) => (Some(color), width),
            }
        };
        let mut mesh = match self.bg_style {
            BackgroundStyle::Solid(color) => {
                if self.is_transitioning() {
                    DrawShape::rectangle(
                        &self.transition.frame,
                        Some(self.transition.color),
                        border.0,
                        border.1,
                        self.corner_radius,
                    )
                } else {
                    DrawShape::rectangle(&self.frame, Some(color), border.0, border.1, self.corner_radius)
                }
            }
            _ => Mesh::new(),
        };
        if mesh.vertices.len() > 0 {
            // for v in &mesh.vertices {
            //     eprintln!("{:?} {:?} {:?}", self.debug_id(), v.pos, v.col);
            // }
            // for g in &mesh.triangles {
            //     eprintln!("{:?} {:?}", self.debug_id(), g.indices);
            // }

            let mut task = MeshTask::new(0);
            task.vertices.append(&mut mesh.vertices);
            task.triangles.append(&mut mesh.triangles);
            window.add_task(task);
        }
    }

    /// Method to draw a border based on self.BorderStyle
    pub(super) fn draw_border(&self, window: &mut Window) {
        let mut mesh = match self.border_style {
            BorderStyle::SolidLine(color, width) => {
                // TODO: allow rounded corners?
                if self.is_transitioning() {
                    DrawShape::rectangle(&self.transition.frame, None, Some(color), width, self.corner_radius)
                } else {
                    DrawShape::rectangle(&self.frame, None, Some(color), width, self.corner_radius)
                }
            }
            _ => Mesh::new(),
        };
        if mesh.vertices.len() > 0 {
            let mut task = MeshTask::new(0);
            task.vertices.append(&mut mesh.vertices);
            task.triangles.append(&mut mesh.triangles);
            window.add_task(task);
        }
    }

    /// Method to calculate how far an object has moved. This is used by Scene objects to
    /// pass move offset data to child objects
    pub(super) fn get_movement_offset(&self) -> Vector {
        if self.initial.pos != self.frame.pos {
            return self.frame.pos - self.initial.pos;
        }
        Vector::ZERO
    }

    /// Returns a Rect within the parent rect padded by the specified values, using
    /// the coordinate system of this Layer object. That is, the origin is based on (0.0, 0.0)
    /// in this self.frame
    ///
    /// TBD: Should we follow CSS pattern or Apple's UIEdgeInsets pattern?
    /// – CSS margins: https://developer.mozilla.org/en-US/docs/Web/CSS/margin
    /// – Apple iOS insets: https://developer.apple.com/documentation/uikit/1624475-uiedgeinsetsmake
    /// It seems more logical to use left-top-right-bottom, so mapping the x, y of a Rect is more obvious.
    pub(super) fn inset_by(&self, left: f32, top: f32, right: f32, bottom: f32) -> Rectangle {
        Rectangle::new(
            (self.frame.pos.x + left, self.frame.pos.y + top),
            (self.frame.size.x - left - right, self.frame.size.y - top - bottom),
        )
    }

    /// This returns a rect relative to the current self.frame coordinates using
    /// the coordinate system the parent of the this Layer object.
    pub(super) fn offset_by(&self, left: f32, top: f32, right: f32, bottom: f32) -> Rectangle {
        Rectangle::new(
            (self.frame.pos.x + left, self.frame.pos.y + top),
            (self.frame.size.x - left - right, self.frame.size.y - top - bottom),
        )
    }

    /// Check if animation tween exists
    pub fn has_animation(&self) -> bool {
        self.animation.is_some()
    }

    /// More detailed: Does the tween animation exist and is it running?
    pub fn is_animating(&self) -> bool {
        if let Some(tween) = &self.animation {
            match tween.state {
                PlayState::Running | PlayState::Finishing => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        false
    }

    pub fn is_transitioning(&self) -> bool {
        if self.is_animating() {
            true
        } else {
            match self.mouse_state {
                MouseState::Hover => true,
                _ => false,
            }
        }
    }

    pub fn node_id(&self) -> NodeID {
        NodeID::new(self.id, self.type_id)
    }

    /// TODO: Use from Displayable base.rs or discard
    pub fn print_node(&self) -> String {
        format!("{}-{}", gui_print_type(&self.type_id), self.id)
    }

    /// TODO: Use from Displayable base.rs or discard
    pub fn debug_id(&self) -> String {
        format!("<{}> [{}]", gui_print_type(&self.type_id), self.id)
    }

    /// Standard format for printing out view information. In general, traits implementors do not need to override it. However, if an object contains
    /// nested views, it may be useful to print out those details. OptionGroup is one example.
    pub fn debug_out(&self) -> String {
        format!("{} {}", self.debug_id(), self.debug_frame())
    }

    pub fn debug_frame(&self) -> String {
        let frame = self.frame;
        format!("Pos({:.1},{:.1}) Size({:.1},{:.1})", frame.pos.x, frame.pos.y, frame.size.x, frame.size.y)
    }

    /**
     * Print all style details:
        – font style
        – background
        – border
    */
    pub fn debug_style(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        let color = self.font_style.get_color();
        let out = format!(
            "Font: #{:02X}{:02X}{:02X} size={}",
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            self.font_style.get_size()
        );
        lines.push(out);
        let color = self.bg_style.get_color();
        let out = format!(
            "BG: #{:02X}{:02X}{:02X} alpha={:1}",
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            color.a
        );
        lines.push(out);
        let border = self.border_style.get_border();
        let out = format!(
            "Border: #{:02X}{:02X}{:02X} alpha={:1} width={:1}",
            (border.0.r * 255.0) as u8,
            (border.0.g * 255.0) as u8,
            (border.0.b * 255.0) as u8,
            border.0.a,
            border.1
        );
        lines.push(out);
        let result = lines.join(" // ");
        result
    }

    /// A helper method to read the Vec<Prop> from a Tween animation and determine what the final size will be
    /// after the animation has completed
    pub fn evaluate_end_rect(&self) -> Rectangle {
        if let Some(tween) = &self.animation {
            let props = tween.get_end_props();
            let mut rect = self.frame.clone();
            for prop in props {
                match prop {
                    Prop::Position(pos) => {
                        rect.pos.x = pos.x;
                        rect.pos.y = pos.y;
                    }
                    Prop::Size(size) => {
                        rect.size.x = size.x;
                        rect.size.y = size.y;
                    }
                    _ => (),
                }
            }
            rect
        } else {
            self.frame
        }
    }

    /// Convenience method to identify if the type and id match this Layer
    pub fn is_me(&self, type_id: TypeId, id: u32) -> bool {
        if self.type_id == type_id && self.id == id {
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for Layer {
    /// Special debug output that trims the extra Vector wrappers.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LAYER: <{}>-[{}]-Pos({:.2})-Size({:.2})",
            gui_print_type(&self.type_id),
            self.id,
            self.frame.pos,
            self.frame.size
        )
    }
}

// ************************************************************************************
// Tweenable
// ************************************************************************************

impl Tweenable for Layer {
    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Alpha(_) => Prop::Alpha(FloatProp::new(self.transition.color.a)),
            Prop::Color(_) => Prop::Color(ColorRGBA::new(
                // Return rgb values are in range 0.0 to 255.0
                self.transition.color.r * 255.0 as f32,
                self.transition.color.g * 255.0 as f32,
                self.transition.color.b * 255.0 as f32,
                self.transition.color.a * 255.0 as f32,
            )),
            Prop::Tint(_) => Prop::Tint(ColorRGBA::new(
                // Return rgb values are in range 0.0 to 255.0
                self.transition.tint.r * 255.0 as f32,
                self.transition.tint.g * 255.0 as f32,
                self.transition.tint.b * 255.0 as f32,
                self.transition.tint.a * 255.0 as f32,
            )),
            Prop::Rotate(_) => Prop::Rotate(FloatProp::new(self.rotation)),
            Prop::Position(_) => Prop::Position(Point2D::new(self.frame.pos.x, self.frame.pos.y)),
            Prop::Size(_) => Prop::Size(Frame2D::new(self.frame.size.x, self.frame.size.y)),
            Prop::Border(_, _) => match self.border_style {
                BorderStyle::None => Prop::Border(None, FloatProp::new(0.0)),
                BorderStyle::SolidLine(color, width) => {
                    let rgb = (
                        color.r * 255.0 as f32,
                        color.g * 255.0 as f32,
                        color.b * 255.0 as f32,
                        color.a * 255.0 as f32,
                    );
                    Prop::Border(Some(ColorRGBA::new(rgb.0, rgb.1, rgb.2, rgb.3)), FloatProp::new(width))
                }
            },
            _ => Prop::None,
        }
    }

    fn update_prop(&mut self, prop: &Prop) {
        match prop {
            Prop::Alpha(val) => self.transition.color.a = val[0] as f32,
            Prop::Color(rgba) => {
                // Apply rgb values in range 0.0 to 1.0
                self.transition.color.r = rgba[0] / 255.0;
                self.transition.color.g = rgba[1] / 255.0;
                self.transition.color.b = rgba[2] / 255.0;
                self.transition.color.a = rgba[3] / 255.0;
            }
            Prop::Tint(rgba) => {
                // Apply rgb values in range 0.0 to 1.0
                self.transition.tint.r = rgba[0] / 255.0;
                self.transition.tint.g = rgba[1] / 255.0;
                self.transition.tint.b = rgba[2] / 255.0;
                self.transition.tint.a = rgba[3] / 255.0;
            }
            Prop::Rotate(val) => self.transition.rotation = val[0] as f32,
            Prop::Position(pos) => {
                self.frame.pos.x = pos[0] as f32;
                self.frame.pos.y = pos[1] as f32;
            }
            Prop::Size(size) => {
                // Temporarily handle buttons differently until we get apply the same frame animation rules to all objects
                if self.type_id == TypeId::of::<Button>() {
                    let origin = self.frame.center() - Vector::new(size[0] / 2.0, size[1] / 2.0);
                    self.transition.frame.pos = origin;
                    self.transition.frame.size.x = size[0];
                    self.transition.frame.size.y = size[1];
                } else {
                    self.frame.size.x = size[0] as f32;
                    self.frame.size.y = size[1] as f32;
                }
            }
            Prop::Border(rgba, width) => {
                if let Some(rgba) = rgba {
                    self.transition.border_width = width[0] as f32;
                    // Apply rgb values in range 0.0 to 1.0
                    let color =
                        Color { r: rgba[0] / 255.0, g: rgba[1] / 255.0, b: rgba[2] / 255.0, a: rgba[3] / 255.0 };
                    self.transition.border_color = color;
                }
            }
            _ => (),
        }
    }

    /// Method to copy initialise the Transition with the official values
    fn init_props(&mut self) {
        // log::trace!("init_props {} origin={:?} anchor_pt={:?}", self.debug_id(), self.frame.pos, self.anchor_pt);
        // FIXME: With ShapeView, this is wrong because the initial mesh has a fill color. However, with animation,
        // that color is not set properly
        self.transition.color = self.bg_style.get_color();
        let border = self.border_style.get_border();
        self.transition.border_color = border.0;
        self.transition.border_width = border.1;
        self.transition.rotation = self.rotation;
    }

    /// After animation is complete, save the temporary props to the corresponding layer fields
    fn save_props(&mut self) {
        self.rotation = self.transition.rotation;
        match self.bg_style {
            BackgroundStyle::Solid(_) => {
                self.bg_style = BackgroundStyle::Solid(self.transition.color);
            }
            _ => (),
        }
        match self.border_style {
            BorderStyle::SolidLine(_, _) => {
                self.border_style = BorderStyle::SolidLine(self.transition.border_color, self.transition.border_width);
            }
            _ => (),
        }
        self.initial = self.frame;
    }

    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Alpha(val) => self.transition.color.a = val[0] as f32,
            Prop::Color(rgba) => {
                if rgba[3] == 0.0 {
                    self.bg_style = BackgroundStyle::None;
                } else {
                    let color =
                        Color { r: rgba[0] / 255.0, g: rgba[1] / 255.0, b: rgba[2] / 255.0, a: rgba[3] / 255.0 };
                    self.bg_style = BackgroundStyle::Solid(color);
                }
            }
            Prop::Tint(rgba) => {
                let _color = Color { r: rgba[0] / 255.0, g: rgba[1] / 255.0, b: rgba[2] / 255.0, a: rgba[3] / 255.0 };
            }
            Prop::Rotate(val) => self.rotation = val[0] as f32,
            Prop::Position(pos) => {
                self.frame.pos.x = pos[0] as f32;
                self.frame.pos.y = pos[1] as f32;
            }
            Prop::Size(size) => {
                self.frame.size.x = size[0] as f32;
                self.frame.size.y = size[1] as f32
            }
            Prop::Border(rgba, width) => {
                if let Some(rgba) = rgba {
                    let color =
                        Color { r: rgba[0] / 255.0, g: rgba[1] / 255.0, b: rgba[2] / 255.0, a: rgba[3] / 255.0 };
                    self.border_style = BorderStyle::SolidLine(color, width[0] as f32)
                } else {
                    self.border_style = BorderStyle::None
                }
            }
            _ => (),
        }
    }
}

impl Playable for Layer {
    fn play(&mut self) {
        let desc = self.debug_id();
        if let Some(tween) = &mut self.animation {
            match tween.state {
                PlayState::Waiting => {
                    log::debug!("Play animation for: {:?}", desc);
                    tween.play();
                }
                _ => (),
            }
        }
    }
    fn reset(&mut self) {
        self.frame = self.initial.clone();
        self.meshes.clear();
        log::debug!("RESET {:?} frame={:?}", self.debug_id(), self.frame);
        if let Some(tween) = &mut self.animation {
            self.meshes.clear();
            tween.reset();
        }
    }
    fn tick(&mut self) {
        if let Some(tween) = &mut self.animation {
            tween.tick();
        }
    }
    fn set_state(&mut self, state: PlayState) {
        if let Some(tween) = &mut self.animation {
            tween.state = state;
        }
    }
}

//-- Support -----------------------------------------------------------------------

/// This enum describes 3 simple states, mainly for handling moving objects
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LayerState {
    /// The idle state when the object is not being animated or moved
    Normal,
    /// Actively moving. This is defined by the AppState.offset value
    Moving,
    /// Movement has completed but still needs to be verified. When verified, it will go back
    /// to the Normal state
    Completed,
}

/// Enum to define the mouse state for each object. Primarily used for Hover states
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseState {
    /// Mouse is not interacting with the object
    None,
    /// Mouse is dragging the object
    Drag,
    /// Mouse has selected the field and it has the focus. Used by TextArea and TextField
    Focus,
    /// Mouse is hovering the field
    Hover,
    /// Indicates that a row or object is selected
    Select,
}

/// Enum in Layer to describe background
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BackgroundStyle {
    /// No background
    None,
    /// Params: background color, corner radius. Use 0.0 for no corner
    Solid(Color),
}

impl BackgroundStyle {
    /// Enum method to get the output color for the style
    pub fn get_color(&self) -> Color {
        match self {
            BackgroundStyle::None => {
                let mut color = Color::WHITE;
                color.a = 0.0;
                color
            }
            BackgroundStyle::Solid(color) => *color,
        }
    }
}

/// Define border for UI object
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BorderStyle {
    /// No border
    None,
    /// Has border with specified width
    SolidLine(Color, f32),
}

impl BorderStyle {
    pub fn get_border(&self) -> (Color, f32) {
        match self {
            BorderStyle::None => {
                let mut color = Color::WHITE;
                color.a = 0.0;
                (color, 0.0)
            }
            BorderStyle::SolidLine(color, width) => (*color, *width),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
// #[non_exhaustive]
pub enum Visibility {
    /// Does not exist yet
    None,
    /// Exists but does not render
    Hidden,
    /// Semi-visible/transparent
    Partial(f32),
    /// Exists and visible
    Visible,
}

/// A wrapper for Tweenable values that is modified through update_props()
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transition {
    /// The position and size Rectangle
    pub frame: Rectangle,
    /// The current color
    pub color: Color,
    /// The foreground color. Only relevant to some nested GUI objects like Text
    pub tint: Color,
    /// The current border width
    pub border_width: f32,
    /// The current border color
    pub border_color: Color,
    /// The current rotation
    pub rotation: f32,
    /// The x-y scale factor if size is changing
    pub scale: Vector,
}

impl Transition {
    /// Constructor
    pub fn new(frame: Rectangle, color: Color, rotation: f32) -> Self {
        let border_width = 0.0;
        let border_color = Color::BLACK;
        let scale = Vector::ONE;
        Transition { frame, color, tint: color, border_width, border_color, rotation, scale }
    }
}
