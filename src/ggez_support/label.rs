extern crate ggez;

use crate::core::*;

use ggez::graphics::{self, DrawParam, Rect};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use std::any::TypeId;
use std::f32;

use super::*;

//-- Label -----------------------------------------------------------------------

pub struct LabelView {
    pub layer: TweenLayer,
    pub string: String,
    pub text: graphics::Text,
    pub align: TextAlign,
}

impl LabelView {
    pub fn new(frame: &Rect, text: &str) -> Self {
        let layer = TweenLayer::new(frame.clone(), DrawParam::new().color(graphics::BLACK));

        LabelView {
            layer: layer,
            string: text.to_string(),
            text: graphics::Text::new(text.to_string()),
            align: TextAlign::Center,
        }
    }

    pub fn set_color(&mut self, color: &graphics::Color) {
        self.layer.graphics.color = color.clone();
    }

    // FIXME: This is probably needed, but it doesn't set the font properly
    // pub fn set_text(&mut self, text: &str) {
    //     self.text = graphics::Text::new(text.to_string());
    // }
}

impl TKDisplayable for LabelView {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<LabelView>()
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

    fn render(&mut self, ctx: &mut Context) -> GameResult {
        let _result = graphics::draw(ctx, &self.text, self.layer.graphics);
        Ok(())
    }

    /// The rect parameter should already be inset from the outer parent frame
    fn render_inside(&mut self, rect: &Rect, ctx: &mut Context) -> GameResult {
        self.text = graphics::Text::new((
            self.string.clone(),
            self.layer.font.clone(),
            self.layer.font_size,
        ));

        let (width, height) = self.text.dimensions(ctx);
        let origin = match self.align {
            TextAlign::Left => Point2 {
                x: rect.x,
                y: rect.y + (rect.h - height as f32) / 2.0,
            },
            TextAlign::Center => Point2 {
                x: rect.x + (rect.w - width as f32) / 2.0,
                y: rect.y + (rect.h - height as f32) / 2.0,
            },
            TextAlign::Right => Point2 {
                x: rect.x + (rect.w - width as f32) / 2.0,
                y: rect.y + (rect.h - height as f32) / 2.0,
            }, // FIXME
        };

        // NOTE: The queue_text method is used here, so draw_queued_text needs to be called later.
        graphics::queue_text(ctx, &self.text, origin, Some(self.layer.graphics.color));
        Ok(())
    }
}
