/// This file will contain various helpers that will make it easier to use Tweek
/// in conjunction with ggez. Some ideas:
/// * A progress/timeline widget that can display timeline status information
/// * Buttons for play/pause/restart
///
///
extern crate ggez;

use ggez::graphics::{self, DrawParam};
use ggez::{Context, GameResult};
use ggez::mint;

pub trait Responder {
    // fn hit_test(&self, pt: mint::Point2) -> bool;

}

pub struct TKView {

}

pub struct TKButton {
    frame: graphics::Rect,
    label: graphics::Text,
    graphics: DrawParam,
}

impl TKButton {
    pub fn new(frame: graphics::Rect, label: graphics::Text) -> Self {
        TKButton {
            frame: frame,
            label: label,
            graphics: DrawParam::default(),
        }
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.frame, self.graphics.color)?;
        let drawparams = DrawParam::new();
        let _result = graphics::draw(ctx, &mesh, drawparams);

        Ok(())
    }
}