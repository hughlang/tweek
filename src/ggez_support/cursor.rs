/// Cursor used in text input controls
///
///
extern crate ggez;

use crate::core::*;

use ggez::graphics::{self, Rect};
use ggez::mint;
use ggez::{Context, GameResult};
use std::any::TypeId;
use std::collections::HashMap;

use super::*;

pub struct Cursor {
    pub layer: TweenLayer,
    // line: Option<graphics::Mesh>,
    defaults: HashMap<u32, Prop>,
}

impl Cursor {
    /// Create a new cursor at the specified points and line_width. A Rect is created so that any motion tweening
    /// can be applied.
    pub fn new(pt1: mint::Point2<f32>, pt2: mint::Point2<f32>, line_width: f32) -> Self {
        let rect = Rect::new(pt1.x, pt1.y, (pt2.x - pt1.x).abs(), (pt2.y - pt1.y).abs());
        let mut layer = TweenLayer::new(rect, graphics::DrawParam::new().color(graphics::BLACK));
        layer.stroke = line_width;

        Cursor {
            layer: layer,
            // line: None,
            defaults: HashMap::new(),
        }
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
}

impl TKDisplayable for Cursor {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Cursor>()
    }

    fn get_frame(&self) -> Rect {
        return self.layer.frame;
    }

    fn set_theme(&mut self, _theme: &Theme) {
        // if let Some(label) = &mut self.label {
        //     label.layer.graphics.color = theme.fg_color;
        // }
    }

    fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.apply_updates(&update.props);
            }
        }
        Ok(())
    }

    /// This render method expects the TextField/parent to tell it where the cursor render
    fn render_inside(&mut self, rect: &Rect, ctx: &mut Context) -> GameResult {
        let points = vec![
            mint::Point2 {
                x: rect.x,
                y: rect.y,
            },
            mint::Point2 {
                x: rect.x,
                y: rect.bottom(),
            },
        ];
        let mesh =
            graphics::Mesh::new_line(ctx, &points, self.layer.stroke, self.layer.graphics.color)?;
        let _result = graphics::draw(ctx, &mesh, self.layer.graphics);
        Ok(())
    }
}
