/// This is an example manager class that handles more complex animations and logic.
/// It is being used by ggrunner.rs by including the following statements at the top:
///
/// mod manager1;
/// use manager1::*;

extern crate ggez;
extern crate tween;

use ggez::graphics::{self};
use ggez::{Context, GameResult};
use ggez::mint;

use tween::*;

const SQUARE_ITEM_ID: usize = 100;
const ROUND_ITEM_ID: usize = 101;

//-- Base -----------------------------------------------------------------------


//-- Main -----------------------------------------------------------------------

pub struct GGManager {

}


//-- Support -----------------------------------------------------------------------

pub enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}

pub struct ItemState {
    id: usize,
    shape: Shape,
    pub bounds: graphics::Rect,
    pub fill_color: graphics::Color,
    pub tween: Option<Tween>,
}

impl ItemState {
    pub fn new(id: usize, shape: Shape) -> GameResult<ItemState> {
        let rect = match shape {
            Shape::Rectangle(rect) => rect,
            Shape::Circle(pt, r) => {
                graphics::Rect::new(pt.x - r, pt.y - r, r * 2.0, r * 2.0)
            },
        };

        Ok(ItemState {
            id: id,
            shape: shape,
            bounds: rect,
            fill_color: graphics::BLACK,
            tween: None,
        })
    }

    pub fn update(&mut self) -> GameResult {
        if let Some(tween) = &self.tween {
            if let Some(update) = tween.update_item(&self.id) {
                self.bounds.render_update(&update.props);
                self.fill_color.render_update(&update.props);
            }
        }
        Ok(())
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        match self.shape {
            Shape::Rectangle(_) => {
                let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.bounds, self.fill_color)?;
                let drawparams = graphics::DrawParam::new();
                let _result = graphics::draw(ctx, &mesh, drawparams);
            },
            Shape::Circle(_, r) => {
                let pt = mint::Point2{x: self.bounds.x + r, y: self.bounds.y + r};
                let mesh = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), pt, r, 1.0, self.fill_color)?;
                let drawparams = graphics::DrawParam::new();
                let _result = graphics::draw(ctx, &mesh, drawparams);
            },
        }
        Ok(())
    }
}
