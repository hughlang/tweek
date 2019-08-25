/// Cursor – an animated cursor used in text editors
///
use super::*;
use crate::core::*;
use crate::events::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Color, Image},
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
    pub fn new(pt1: Vector, pt2: Vector, line_width: f32) -> Self {
        let rect = Rectangle::new((pt1.x, pt1.y), ((pt2.x - pt1.x).abs(), (pt2.y - pt1.y).abs()));
        let mut layer = Layer::new(rect);
        Cursor { layer }
    }

    /// Builder method to get the default animation. You can also customize the animation Props
    /// and call start_animation directly.
    /// TBD: Maybe move this into the Displayable trait.
    pub fn default_animation(mut self) -> Self {
        self.start_animation(&[alpha(0.0)], 0.5);
        self
    }

    /// Start the animation with the specified Props and flashing duration
    /// TODO: allow repeat delay to be customized
    pub fn start_animation(&mut self, props: &[Prop], seconds: f64) {
        let mut tween = Tween::with(0, &self.layer)
            .to(&props.to_vec())
            .duration(seconds).repeat(-1, 0.1).yoyo()
            // .debug()
            ;
        &tween.play();
        self.layer.animation = Some(tween);
    }

    /// Render the cursor given an origin point where y is the baseline for the current line.
    /// The font_size is approximately the line height, but the actual characters are shorter
    /// than that, so the height and position of the cursor should consider that.
    /// The ideal cursor should have a top position near the top of the line extending above
    /// the tallest character and extend below the baseline.
    pub fn render_at_point(&self, pt: &Vector, theme: &Theme, window: &mut Window) {
        let cursor_height = theme.font_size;
        let y2 = pt.y + cursor_height * 0.2;
        let y1 = y2 - cursor_height;
        let line = Line::new((pt.x, y1), (pt.x, y2)).with_thickness(2.0);
        window.draw_ex(&line.with_thickness(line.t), Col(Color::BLACK), Transform::IDENTITY, 9);
    }

    /// Method used by TextField to render the cursor
    /// TODO: Evaluate different from render_at_point
    pub fn render_line(&self, pt1: &Vector, pt2: &Vector, _theme: &Theme, window: &mut Window) {
        let line = Line::new(*pt1, *pt2).with_thickness(2.0);
        match self.layer.border_style {
            BorderStyle::SolidLine(color, width) => {
                window.draw_ex(&line.with_thickness(width), Col(color), Transform::IDENTITY, 9);
            }
            _ => ()
        };
    }
}

impl Displayable for Cursor {

    fn get_id(&self) -> u32 { self.layer.get_id() }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Cursor>()
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
        if self.layer.lock_style { return }
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
        self.layer.frame.pos = self.layer.initial.pos + Vector::new(state.offset.0, state.offset.1);
        self.layer.tween_update();
    }


}
