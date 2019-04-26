/// Cursor – an animated cursor used in text editors
///
extern crate quicksilver;

use super::*;
use crate::core::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Color, Image},
    lifecycle::{run, Settings, State, Window},
};

use std::any::TypeId;

use std::collections::HashMap;

pub struct Cursor {
    pub layer: TweenLayer,
    defaults: HashMap<u32, Prop>,
}

impl Cursor {
    /// Create a new cursor at the specified points and line_width. A Rect is created so that any motion tweening
    /// can be applied.
    pub fn new(pt1: Vector, pt2: Vector, line_width: f32) -> Self {
        let rect = Rectangle::new((pt1.x, pt1.y), ((pt2.x - pt1.x).abs(), (pt2.y - pt1.y).abs()));
        let mut layer = TweenLayer::new(rect);
        layer.stroke = line_width;

        Cursor { layer: layer, defaults: HashMap::new() }
    }

    /// Simple way of getting a default animation. You can skip this and call start_animation
    /// on your own with your custom values.
    /// TBD: Maybe move this into the Displayable trait.
    pub fn default_animation(mut self) -> Self {
        self.start_animation(&[alpha(0.0)], 0.5);
        self
    }

    pub fn start_animation(&mut self, props: &[Prop], seconds: f64) {
        for prop in props {
            let start_prop = self.layer.get_prop(&prop);
            match start_prop {
                Prop::None => {}
                _ => {
                    &self.defaults.insert(start_prop.prop_id(), start_prop);
                }
            }
        }
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
}

impl TKDisplayable for Cursor {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Cursor>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn set_theme(&mut self, _theme: &Theme) {
        // if let Some(label) = &mut self.label {
        //     label.layer.graphics.color = theme.fg_color;
        // }
    }

    fn update(&mut self) -> TKResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.apply_updates(&update.props);
            }
        }
        Ok(())
    }
}
