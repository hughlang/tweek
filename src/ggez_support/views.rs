/// Basic views that conform to GGEZView, but are not required to implement TKResponder
///
///
extern crate ggez;

use crate::core::*;

use ggez::graphics::{self, DrawParam};
use ggez::mint::{Point2, Vector2};
use ggez::{Context, GameResult};

use super::base::*;

pub trait Displayable {
    fn update(&mut self) -> GameResult;
    fn render(&mut self, ctx: &mut Context) -> GameResult;
    fn render_inside(&mut self, _rect: &graphics::Rect, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

// Not finished and currently unused
pub struct BoxView {
    pub layer: TweenLayer,
    pub subviews: Vec<Box<Displayable>>,  // TBD
}

impl BoxView {
    pub fn new(frame: graphics::Rect) -> Self {
        let layer = TweenLayer::new(frame, DrawParam::new() );
        BoxView {
            layer: layer,
            subviews: Vec::new(),
        }
    }
}

impl Displayable for BoxView {

    fn update(&mut self) -> GameResult {
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.layer.frame,
            self.layer.graphics.color,
        )?;
        graphics::draw(ctx, &mesh, self.layer.graphics)?;

        // TODO: Iterate through subviews and render_inside?

        Ok(())
    }
}

//-- Label -----------------------------------------------------------------------

pub struct LabelView {
    pub layer: TweenLayer,
    pub title: String,
    pub text: graphics::Text,
}

impl LabelView {
    pub fn new(frame: &graphics::Rect, text: &str) -> Self {
        let layer = TweenLayer::new(
            frame.clone(),
            DrawParam::new().color(graphics::WHITE),
        );

        LabelView {
            layer: layer,
            title: text.to_string(),
            text: graphics::Text::new(text.to_string()),
        }
    }

    pub fn set_font(&mut self, font: &graphics::Font, size: &f32) {
        self.text = graphics::Text::new((self.title.clone(), font.clone(), size.clone()));
    }

    pub fn set_color(&mut self, color: &graphics::Color) {
        self.layer.graphics.color = color.clone();
    }
}

impl Displayable for LabelView {

    fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.render_update(&update.props);
                self.layer.redraw = true;
            }
        }
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context) -> GameResult {
        let _result = graphics::draw(ctx, &self.text, self.layer.graphics);
        Ok(())
    }

    fn render_inside(&mut self, rect: &graphics::Rect, ctx: &mut Context) -> GameResult {
        let (width, height) = self.text.dimensions(ctx);
        let pt = Point2{x: rect.x + (rect.w - width as f32)/2.0 , y: rect.y + (rect.h - height as f32)/2.0 };
        // println!("inside={:?} // dest={:?}", rect,  pt);
        let _result = graphics::draw(ctx, &self.text, self.layer.graphics.dest(pt));
        Ok(())
    }
}

//-- Image -----------------------------------------------------------------------

pub struct ImageView {
    pub layer: TweenLayer,
    pub scale: f32,
    // pub mesh: Option<graphics::Mesh>,
    pub image: graphics::Image,
}

impl ImageView {
    pub fn new(frame: graphics::Rect, image: graphics::Image) -> Self {
        let layer = TweenLayer::new(frame, DrawParam::new());
        ImageView {
            layer: layer,
            scale: 1.0,
            image: image,
        }
    }
}

impl Displayable for ImageView {
    fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.render_update(&update.props);
                self.layer.redraw = true;
            }
        }
        Ok(())
    }
    fn render(&mut self, ctx: &mut Context) -> GameResult {
        let pt = Point2{x: self.layer.frame.x, y: self.layer.frame.y};
        let drawparams = graphics::DrawParam::new()
            .dest(pt)
            .rotation(self.layer.graphics.rotation as f32)
            .offset(Point2{x: 0.5, y: 0.5})
            .color(self.layer.graphics.color);
        let _result = graphics::draw(ctx, &self.image, drawparams);
        Ok(())
    }

    fn render_inside(&mut self, rect: &graphics::Rect, ctx: &mut Context) -> GameResult {
        let pt = Point2{x: rect.x + rect.w/2.0 , y: rect.y + rect.h/2.0 };
        let scale = Vector2{x: self.scale, y: self.scale};
        let drawparams = graphics::DrawParam::new()
            .dest(pt)
            .scale(scale)
            .rotation(self.layer.graphics.rotation as f32)
            .offset(Point2{x: 0.5, y: 0.5})
            .color(self.layer.graphics.color);
        let _result = graphics::draw(ctx, &self.image, drawparams);
        Ok(())
    }

}
