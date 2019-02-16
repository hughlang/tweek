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

use super::property::*;


pub trait Responder {
    fn hit_test(&self, pt: mint::Point2<f64>) -> bool;

}

/// Expected struct properties:
/// – position
/// – frame
///
pub trait TKMovable {

    fn set_frame(frame: mint::Vector4<f64>);


}

pub struct TKLayer {
    pub frame: graphics::Rect,
    pub graphics: DrawParam,
}

impl TKLayer {
    pub fn new(frame: graphics::Rect, graphics: DrawParam )-> Self  {
        TKLayer{ frame: frame, graphics: graphics }
    }
}

pub struct TKLabel {
    text: String,
}

impl TKLabel {
    pub fn new(text: String) -> Self {
        TKLabel {
            text,
        }
    }
}
pub struct TKButton {
    pub layer: TKLayer,
    pub graphics: DrawParam,
}

impl TKButton {
    pub fn new(frame: graphics::Rect, label: graphics::Text) -> Self {
        let layer = TKLayer{ frame: frame, graphics: DrawParam::default() };
        TKButton {
            layer: layer,
            graphics: DrawParam::default(),
        }
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.layer.frame, self.graphics.color)?;
        let drawparams = DrawParam::new();
        let _result = graphics::draw(ctx, &mesh, drawparams);

        Ok(())
    }
}