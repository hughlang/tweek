/// This helper module is a convenience when writing and testing examples.
///
/// mod helper1;
/// use helper1::*;
///

extern crate ggez;
extern crate tween;

use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::mint;

use tween::*;

pub enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
    Image(graphics::Rect),
    Text(graphics::Rect),
    Line(mint::Point2<f32>, mint::Point2<f32>),
}

pub struct ItemState {
    pub id: usize,
    pub shape: Shape,
    pub frame: graphics::Rect,
    pub graphics: graphics::DrawParam,
    pub tween: Option<Tween>,
    pub image: Option<graphics::Image>,
    pub text: Option<graphics::Text>,
    pub should_update: bool,
    pub should_render: bool,
}

impl ItemState {
    pub fn new(id: usize, shape: Shape) -> GameResult<ItemState> {
        let rect = match shape {
            Shape::Rectangle(rect) => rect,
            Shape::Circle(pt, r) => {
                graphics::Rect::new(pt.x - r, pt.y - r, r * 2.0, r * 2.0)
            },
            Shape::Image(rect) => rect,
            Shape::Text(rect) => rect,
            Shape::Line(pt1, pt2) => {
                graphics::Rect::new(pt1.x, pt1.y, pt2.x, pt2.y)
            },
        };


        Ok(ItemState {
            id: id,
            shape: shape,
            frame: rect,
            graphics: graphics::DrawParam::default(),
            tween: None,
            image: None,
            text: None,
            should_update: true,
            should_render: true,
        })
    }

    pub fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.tween {
            tween.tick();
            if let Some(update) = tween.get_update(&self.id) {
                self.frame.render_update(&update.props);
                self.graphics.render_update(&update.props);
            }
        }
        Ok(())
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        match self.shape {
            Shape::Rectangle(_) => {
                let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.frame, self.graphics.color)?;
                let pt = mint::Point2{x: self.frame.x, y: self.frame.y};
                let drawparams = graphics::DrawParam::new()
                    .dest(pt)
                    .rotation(self.graphics.rotation as f32)
                    .offset(mint::Point2{x: 0.5, y: 0.5});
                let _result = graphics::draw(ctx, &mesh, drawparams);
            },
            Shape::Circle(_, _) => {
                let r = self.frame.w / 2.0;
                let pt = mint::Point2{x: self.frame.x + r, y: self.frame.y + r};
                let mesh = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), pt, r, 0.2, self.graphics.color)?;
                let drawparams = graphics::DrawParam::new()
                    .offset(mint::Point2{x: 0.5, y: 0.5});
                let _result = graphics::draw(ctx, &mesh, drawparams);
            },
            Shape::Image(_) => {
                match &self.image {
                    Some(img) => {
                        let pt = mint::Point2{x: self.frame.x, y: self.frame.y};
                        let drawparams = graphics::DrawParam::new()
                            .dest(pt)
                            .rotation(self.graphics.rotation as f32)
                            .offset(mint::Point2{x: 0.5, y: 0.5})
                            .color(self.graphics.color);
                        let _result = graphics::draw(ctx, img, drawparams);
                    },
                    None => (),
                }
            },
            Shape::Text(_) => {
                match &self.text {
                    Some(txt) => {
                        let pt = mint::Point2{x: self.frame.x, y: self.frame.y};
                        let _result = graphics::draw(ctx, txt, (pt,));
                    },
                    None => (),
                }
            },
            Shape::Line(_, _) => {

            }
        }
        Ok(())
    }
}
