/// Image
extern crate ggez;

use crate::core::*;

use ggez::graphics::{self, DrawParam, Rect};
use ggez::mint::{Point2, Vector2};
use ggez::{Context, GameResult};

use std::any::TypeId;
use std::f32;

use super::*;

//-- Image -----------------------------------------------------------------------

pub struct ImageView {
    pub layer: TweenLayer,
    pub scale: f32,
    // pub mesh: Option<graphics::Mesh>,
    pub image: graphics::Image,
}

impl ImageView {
    pub fn new(frame: Rect, image: graphics::Image) -> Self {
        let layer = TweenLayer::new(frame, DrawParam::new());
        ImageView { layer: layer, scale: 1.0, image: image }
    }
}

impl TKDisplayable for ImageView {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ImageView>()
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
        let pt = Point2 { x: self.layer.frame.x, y: self.layer.frame.y };
        let drawparams = graphics::DrawParam::new()
            .dest(pt)
            .rotation(self.layer.graphics.rotation as f32)
            .offset(Point2 { x: 0.5, y: 0.5 })
            .color(self.layer.graphics.color);
        let _result = graphics::draw(ctx, &self.image, drawparams);
        Ok(())
    }

    fn render_inside(&mut self, rect: &Rect, ctx: &mut Context) -> GameResult {
        let pt = Point2 { x: rect.x + rect.w / 2.0, y: rect.y + rect.h / 2.0 };
        let scale = Vector2 { x: self.scale, y: self.scale };
        let drawparams = graphics::DrawParam::new()
            .dest(pt)
            .scale(scale)
            .rotation(self.layer.graphics.rotation as f32)
            .offset(Point2 { x: 0.5, y: 0.5 })
            .color(self.layer.graphics.color);
        let _result = graphics::draw(ctx, &self.image, drawparams);
        Ok(())
    }
}
