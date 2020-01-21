/// Cursor – an animated cursor used in text editors
///
use super::*;
use crate::core::*;
use crate::events::*;
use crate::tools::DrawShape;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Color, Image, Mesh},
    lifecycle::{run, Settings, State, Window},
};

use std::any::TypeId;

/// A simple cursor with optional flashing animation.
/// This is used in the TextArea and TextField controls.
pub struct Cursor {
    /// The base layer
    pub layer: Layer,
}

impl Cursor {
    /// Create a new cursor at the specified points and line_width. A Rect is created so that any motion tweening
    /// can be applied.
    pub fn new(pt1: Vector, pt2: Vector, _line_width: f32) -> Self {
        let rect = Rectangle::new((pt1.x, pt1.y), ((pt2.x - pt1.x).abs(), (pt2.y - pt1.y).abs()));
        let layer = Layer::new(rect);
        Cursor { layer }
    }

    /// Builder method to get the default animation. You can also customize the animation Props
    /// and call start_animation directly.
    /// TBD: Maybe move this into the Displayable trait.
    pub fn default_animation(mut self) -> Self {
        self.start_animation(&[color("#000000EE")], 0.25); // Hex color for black with very low alpha
        self
    }

    /// Start the animation with the specified Props and flashing duration
    /// TODO: allow repeat delay to be customized
    pub fn start_animation(&mut self, props: &[Prop], seconds: f64) {
        let mut tween =
            Tween::with(0, &self.layer).to(&props.to_vec()).duration(seconds).repeat(u32::max_value(), 0.1).yoyo();
        &tween.play();
        self.layer.animation = Some(tween);
        self.layer.tween_type = TweenType::Animation;
    }

    /// Method used by TextField to render the cursor
    /// TODO: Evaluate different from render_at_point
    pub fn render_line(&self, pt1: &Vector, pt2: &Vector, _theme: &Theme) -> Mesh {
        let color = self.layer.transition.color;
        let pts: [&Vector; 2] = [pt1, pt2];
        let line = DrawShape::line(&pts, color, 2.0);
        line
    }
}

impl Displayable for Cursor {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Cursor>()
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

    fn move_to(&mut self, pos: (f32, f32)) {
        self.layer.frame.pos.x = pos.0;
        self.layer.frame.pos.y = pos.1;
    }

    fn set_theme(&mut self, theme: &mut Theme) {
        let ok = self.layer.apply_theme(theme);
        if !ok {
            return;
        }
        self.layer.border_style = BorderStyle::SolidLine(theme.cursor_color, 2.0);
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Ready => {
                self.layer.on_ready();
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
            }
            _ => {}
        }
    }

    fn update(&mut self, _window: &mut Window, state: &mut AppState) {
        self.layer.tween_update(state);
    }
}

// impl fmt::Debug for Cursor {
//     /// Special debug output that trims the extra Vector wrappers.
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "LAYER: <{}>-[{}]-Pos({:.2})-Size({:.2}) Color",
//             gui_print_type(&self.type_id),
//             self.id,
//             self.frame.pos,
//             self.frame.size
//         )
//     }
// }
