/// This module is used throughout the examples and serves as a basic and generic system for rendering
/// shapes on screen. However, it isn't comprehensive and certain compromises are made to reduce
/// complexity. For example, it does not support Line graphics as borders for shapes. Only the Line
/// shape uses DrawMode::Stroke. Someday, it may get migrated into the ggez_support module.

extern crate ggez;
extern crate tweek;

use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::mint;

use tweek::prelude::*;

#[allow(dead_code)]
pub enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
    Image(graphics::Rect),
    Text(graphics::Rect),
    /// Parameters are start point, end point, and line width
    Line(mint::Point2<f32>, mint::Point2<f32>, f32),
}

pub struct ItemState {
    pub id: usize,
    pub shape: Shape,
    pub layer: TweenLayer,
    pub tween: Option<Tween>,
    pub image: Option<graphics::Image>,
    pub text: Option<graphics::Text>,
    pub should_update: bool,
    pub should_render: bool,
}

impl ItemState {

    #[allow(dead_code)]
    pub fn new(id: usize, shape: Shape) -> GameResult<ItemState> {
        let layer = match shape {
            Shape::Rectangle(rect) => {
                TweenLayer::new(rect, graphics::DrawParam::new())
            },
            Shape::Circle(pt, r) => {
                let rect = graphics::Rect::new(pt.x - r, pt.y - r, r * 2.0, r * 2.0);
                TweenLayer::new(rect, graphics::DrawParam::new())
            },
            Shape::Image(rect) => {
                TweenLayer::new(rect, graphics::DrawParam::new())
            },
            Shape::Text(rect) => {
                TweenLayer::new(rect, graphics::DrawParam::new())
            },
            Shape::Line(pt1, pt2, line_width) => {
                let rect = graphics::Rect::new(pt1.x, pt1.y, 1.0, line_width);
                let mut layer = TweenLayer::new(rect, graphics::DrawParam::new());
                layer.stroke = line_width;
                layer
            },
        };

        Ok(ItemState {
            id: id,
            shape: shape,
            layer: layer,
            tween: None,
            image: None,
            text: None,
            should_update: true,
            should_render: true,
        })
    }

    #[allow(dead_code)]
    pub fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.tween {
            tween.tick();
            if let Some(update) = tween.get_update(&self.id) {
                // println!("update props={:?}", update.props);
                self.layer.render_update(&update.props);
                self.layer.render_update(&update.props);
            }
        }
        Ok(())
    }

    /// This is an experimental/temporary means of getting Tween updates.
    /// The publishing of UIState updates may be handled by TKState in the
    /// future.
    #[allow(dead_code)]
    pub fn try_update(&mut self, tweek: &mut Tweek) -> GameResult {
        if let Some(update) = tweek.get_update(&self.id) {
            self.layer.render_update(&update.props);
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        match self.shape {
            Shape::Rectangle(_) => {
                let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.layer.frame, self.layer.graphics.color)?;
                let _result = graphics::draw(ctx, &mesh, self.layer.graphics);
            },
            Shape::Circle(_, _) => {
                let r = self.layer.frame.w / 2.0;
                let pt = mint::Point2{x: self.layer.frame.x + r, y: self.layer.frame.y + r};
                let mesh = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), pt, r, 0.2, self.layer.graphics.color)?;
                let _result = graphics::draw(ctx, &mesh, self.layer.graphics);
            },
            Shape::Image(_) => {
                match &self.image {
                    Some(img) => {
                        let pt = mint::Point2{x: self.layer.frame.x, y: self.layer.frame.y};
                        let _drawparams = graphics::DrawParam::new()
                            .dest(pt)
                            .rotation(self.layer.graphics.rotation as f32)
                            .offset(mint::Point2{x: 0.5, y: 0.5})
                            .color(self.layer.graphics.color);
                        let _result = graphics::draw(ctx, img, self.layer.graphics);
                    },
                    None => (),
                }
            },
            Shape::Text(_) => {
                match &self.text {
                    Some(txt) => {
                        let pt = mint::Point2{x: self.layer.frame.x, y: self.layer.frame.y};
                        let _result = graphics::draw(ctx, txt, (pt,));
                    },
                    None => (),
                }
            },
            Shape::Line(_, _, _) => {
                let points = vec![
                    mint::Point2{x: self.layer.frame.x, y: self.layer.frame.y},
                    mint::Point2{x: self.layer.frame.x + self.layer.frame.w, y: self.layer.frame.y},
                ];
                let mesh = graphics::Mesh::new_line(ctx, &points, self.layer.stroke, self.layer.graphics.color)?;
                let _result = graphics::draw(ctx, &mesh, self.layer.graphics);
            }
        }
        Ok(())
    }
}

