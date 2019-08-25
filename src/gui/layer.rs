/// Layer for Quicksilver
///
use super::*;
use crate::core::*;
use crate::events::*;
use crate::tools::*;

use std::{
    any::TypeId,
    fmt,
    cell::RefCell,
    rc::Rc,
};

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Color, Mesh, MeshTask, FontStyle},
    lifecycle::{run, Settings, State, Window},
};

//-- Main -----------------------------------------------------------------------

/// This is a wrapper for the quicksilver properties that are Tweenable. A GUI object that
/// has this field can be animated with Tween
pub struct Layer {
    /// Arbitrary id value. TBD
    id: u32,
    /// Identifies the object type this layer belongs to.
    pub(super) type_id: TypeId,
    /// The rectangular position and size bounds of the object
    pub frame: Rectangle, // TODO: make private
    /// The initial frame for the object
    pub(crate) initial: Rectangle,
    /// The starting Props for an object
    pub(self) defaults: Vec<Prop>,
    /// The current animation. Only one is allowed, which means a hover animation cannot coincide
    /// with a click animation. FIXME: Need to merge animations with time offsets or define rules for
    /// overrides
    pub(super) animation: Option<Tween>,
    /// Props that are modified during Tween animation
    pub(super) transition: Transition,
    /// The rotation of the object in degrees, where 0 degrees points to "3 o'clock" (TBD)
    pub rotation: f32,
    /// Corner radius for the layer.
    pub corner_radius: f32,
    /// Experimental point outside of the frame for rotation
    pub anchor_pt: Vector,
    /// Enum to describe background of object. TODO: deprecate bg_color
    pub bg_style: BackgroundStyle,
    /// Background color for the scene. Otherwise it's transparent
    pub border_style: BorderStyle,
    /// Defines font size and color
    pub font_style: FontStyle,
    /// Current MouseState of the layer
    pub(super) mouse_state: MouseState,
    /// The animation Props for the hover event
    pub(super) hover_effect: Option<PropSet>,
    /// The animation Props for the hover event
    pub(super) click_effect: Option<PropSet>,
    /// Callback method for handling click action
    pub(super) on_click: Option<Box<dyn FnMut(&mut AppState) + 'static>>,
    /// Notifications
    pub(super) notifications: Rc<RefCell<Notifications>>,
    /// The currently executing animations
    pub(self) tween_type: TweenType,
    /// Should the layer move/resize with the parent scene?
    pub lock_frame: bool,
    /// Should the layer move/resize with the parent scene?
    pub lock_style: bool,
    /// Should extra debug output be enabled?
    pub debug: bool,
}

impl Layer {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        Layer {
            id: 0,
            type_id: TypeId::of::<Layer>(), // the default
            frame,
            initial: frame,
            defaults: Vec::new(),
            animation: None,
            transition: Transition::new(frame, Color::WHITE, 0.0),
            rotation: 0.0,
            corner_radius: 0.0,
            anchor_pt: Vector::ZERO,
            bg_style: BackgroundStyle::None,
            border_style: BorderStyle::None,
            font_style: FontStyle::new(14.0, Color::BLACK),
            mouse_state: MouseState::None,
            hover_effect: None,
            click_effect: None,
            on_click: None,
            notifications: Notifications::new(),
            tween_type: TweenType::None,
            lock_frame: false,
            lock_style: false,
            debug: true,
        }
    }

    /// Setter for id
    pub fn set_id(&mut self, id: u32) { self.id = id }
    /// Getter for id
    pub fn get_id(&self) -> u32 { self.id }

    /// Setter for Tween animation. Only needed outside of the Tweek crate
    pub fn set_animation(&mut self, tween: Tween) { self.animation = Some(tween) }

    /// Should be called when SceneEvent::Ready notify event is sent
    pub(super) fn on_ready(&mut self) {
        self.init_props();
        self.initial = self.frame;
        self.defaults = Tween::load_props(self);
    }

    /// Called by Displayable after notify(DisplayEvent::Move)
    pub(super) fn on_move_complete(&mut self) {
        self.init_props();
        self.initial = self.frame;
        self.defaults = Tween::load_props(self);
    }

    /// Set the default animations
    pub(super) fn apply_theme(&mut self, theme: &mut Theme) {
        if GUI_INPUTS.contains(&self.type_id) {
            self.border_style = BorderStyle::SolidLine(theme.input_fg_color, theme.border_width);
            self.bg_style = BackgroundStyle::Solid(theme.input_bg_color);
            if self.hover_effect.is_none() {
                self.hover_effect = Some(theme.on_view_hover.clone());
            }
        }
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
    }

    /// If animation is running, run updates
    pub(crate) fn tween_update(&mut self) {
        self.notifications.borrow_mut().clear();
        if let Some(tween) = &mut self.animation {
            let mut notifier = Notifier::new();
            self.notifications.borrow_mut().attach(&mut notifier);

            // Tell tween to update its state
            tween.status(&mut notifier);
            let result = tween.request_update(&mut notifier);
            if let Some(propset) = result {
                self.update_props(&*propset.props);
            }
            // Filter for TweenEvents
            let events = self.notifications.borrow_mut().events.filter::<TweenEvent>();

            // TODO: use iterator
            for evt in events {
                match evt {
                    TweenEvent::Status(id, state) => {
                        match state {
                            PlayState::Starting => {
                                log::debug!("<{}> [{}] {:?} {:?}", gui_print_type(&self.type_id), id, state, self.tween_type);
                            }
                            PlayState::Completed => {
                                log::debug!("<{}> [{}] {:?} {:?}", gui_print_type(&self.type_id), id, state, self.tween_type);
                                match self.tween_type {
                                    TweenType::Move => {
                                        self.defaults = Tween::load_props(self);
                                        self.initial = self.frame;
                                        notifier.notify(LayerEvent::Move(id, self.type_id, state))
                                    }
                                    TweenType::Hover => {
                                        self.defaults = Tween::load_props(self);
                                        self.initial = self.frame;
                                        notifier.notify(LayerEvent::Hover(id, self.type_id, state))
                                    }
                                    TweenType::Click => {
                                        self.defaults = Tween::load_props(self);
                                        self.initial = self.frame;
                                        notifier.notify(LayerEvent::Click(id, self.type_id, state))
                                    }
                                    _ => ()
                                }
                            }
                            _ => ()
                        }
                    }
                    // TweenEvent::EndState(id, props) => {
                    //     log::debug!("<{}> [{}] Target={:?}", gui_print_type(&self.type_id), id, props);
                    // }
                }
            }
        }
    }

    /// Standard method called by components when mouseover occurs
    pub(super) fn handle_mouse_over(&mut self, pt: &Vector) -> bool {
        if pt.overlaps_rectangle(&self.frame) {
            match self.mouse_state {
                MouseState::None => {
                    // change state to hover and start animations
                    self.mouse_state = MouseState::Hover;
                    if let Some(transition) = &self.hover_effect {
                        let trans = transition.clone();
                        if trans.duration > 0.0 {
                            self.animate_with_props(trans);
                            self.tween_type = TweenType::Click;
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
                self.animate_with_props(trans);
                self.tween_type = TweenType::Click;
            } else {
                self.apply_props(&trans.props);
            }
        }
    }

    /// Start animating with the given PropSet
    pub fn animate_with_props(&mut self, propset: PropSet) {
        self.init_props();
        let mut tween = Tween::with(self.id, self).using_props(propset.clone());
        &tween.play();
        self.animation = Some(tween);
        self.tween_type = propset.event;
    }

    /// Method to call when starting an animation. This will copy the current properties into Transition
    pub fn start_animation(&mut self, mut tween: Tween) {
        // FIXME: Log every detail. Record the event type
        self.init_props();
        &mut tween.play();
        self.animation = Some(tween);
    }

    /// Reset the layer by clearing mesh_tasks and other state
    pub fn reset(&mut self) {
        // self.mesh_tasks.clear();
    }

    /// Method to evaluate BackgroundStyle and BorderStyle and draw the Mesh for the
    /// background. Used by Button, Scene, etc during render
    pub(super) fn draw_background(&self, window: &mut Window) {
        let border: (Option<Color>, f32) = {
            match self.border_style {
                BorderStyle::None => {
                    (None, 0.0)
                }
                BorderStyle::SolidLine(color, width) => {
                    (Some(color), width)
                }
            }
        };
        let mut mesh = match self.bg_style {
            BackgroundStyle::Solid(color) => {
                if self.is_animating() {
                    DrawShape::rectangle(&self.frame, Some(self.transition.color), border.0, border.1, self.corner_radius)
                } else {
                    DrawShape::rectangle(&self.frame, Some(color), border.0, border.1, self.corner_radius)
                }
            }
            _ => { Mesh::new() }
        };
        if mesh.vertices.len() > 0 {
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
                DrawShape::rectangle(&self.frame, None, Some(color), width, self.corner_radius)
            }
            _ => { Mesh::new() }
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
    /// It seems more logical to use top-left-right-bottom, so mapping the x, y of a Rect is more obvious.
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

    /// Helper method to get the Lines that outline an object frame
    pub(super) fn get_border_lines(&self, width: f32) -> Vec<Line> {
        let lines = UITools::make_border_lines(&self.frame, width);
        lines
    }

    pub fn has_animation(&self) -> bool {
        self.animation.is_some()
    }

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
            Prop::Border(_, _) => {
                match self.border_style {
                    BorderStyle::None => Prop::Border(None, FloatProp::new(0.0)),
                    BorderStyle::SolidLine(color, width) => {
                        let rgb = (color.r * 255.0 as f32, color.g * 255.0 as f32, color.b * 255.0 as f32, color.a * 255.0 as f32);
                        Prop::Border(Some(ColorRGBA::new(rgb.0, rgb.1, rgb.2, rgb.3)), FloatProp::new(width))
                    }
                }
            }
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
                self.frame.size.x = size[0] as f32;
                self.frame.size.y = size[1] as f32
            }
            Prop::Border(rgba, width) => {
                if let Some(rgba) = rgba {
                    self.transition.border_width = width[0] as f32;
                    // Apply rgb values in range 0.0 to 1.0
                    let color = Color { r: rgba[0] / 255.0, g: rgba[1] / 255.0, b: rgba[2] / 255.0, a: rgba[3] / 255.0 };
                    self.transition.border_color = color;
                }
            }
            _ => (),
        }
    }

    /// Method to copy initialise the Transition with the official values
    fn init_props(&mut self) {
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
            _ => ()
        }
        match self.border_style {
            BorderStyle::SolidLine(_, _) => {
                self.border_style = BorderStyle::SolidLine(self.transition.border_color, self.transition.border_width);
            }
            _ => ()
        }
        self.initial = self.frame;
    }

    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Alpha(val) => {
                self.transition.color.a = val[0] as f32
            }
            Prop::Color(rgba) => {
                if rgba[3] == 0.0 {
                    self.bg_style = BackgroundStyle::None;
                } else {
                    let color = Color { r: rgba[0] / 255.0, g: rgba[1] / 255.0, b: rgba[2] / 255.0, a: rgba[3] / 255.0 };
                    self.bg_style = BackgroundStyle::Solid(color);
                }
            }
            Prop::Tint(rgba) => {
                let color = Color { r: rgba[0] / 255.0, g: rgba[1] / 255.0, b: rgba[2] / 255.0, a: rgba[3] / 255.0 };
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
                    let color = Color { r: rgba[0] / 255.0, g: rgba[1] / 255.0, b: rgba[2] / 255.0, a: rgba[3] / 255.0 };
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
        if let Some(tween) = &mut self.animation {
            tween.play();
        }
    }
    fn tick(&mut self) {
        if let Some(tween) = &mut self.animation {
            tween.tick();
        }
    }
}

impl fmt::Debug for Layer {
    /// Special debug output that trims the extra Vector wrappers.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>-{}-Pos({:.2})-Size({:.2})", self.id, gui_print_type(&self.type_id), self.frame.pos, self.frame.size)
    }
}

//-- Support -----------------------------------------------------------------------

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
            BackgroundStyle::Solid(color) => {
                *color
            }
        }
    }
}

/// Define border for UI object
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BorderStyle {
    /// No border
    None,
    /// Has border with specified width
    SolidLine(Color, f32)
}

impl BorderStyle {
    pub fn get_border(&self) -> (Color, f32) {
        match self {
            BorderStyle::None => {
                let mut color = Color::WHITE;
                color.a = 0.0;
                (color, 0.0)
            }
            BorderStyle::SolidLine(color, width) => {
                (*color, *width)
            }
        }
    }
}

/// A wrapper for Tweenable values that is modified through update_props()
pub(super) struct Transition {
    /// The position and size Rectangle
    // pub frame: Rectangle,
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
}

impl Transition {
    /// Constructor
    pub fn new(_frame: Rectangle, color: Color, rotation: f32) -> Self {
        let border_width = 0.0;
        let border_color = Color::BLACK;
        Transition { color, tint: color, border_width, border_color, rotation }
    }
}