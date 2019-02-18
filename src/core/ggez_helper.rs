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
use super::tween::*;

//-- Base -----------------------------------------------------------------------
// The plan is to have ggez support as a feature and would have its own module in this lib

impl Tweenable for ggez::graphics::Rect {
    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Position(pos) => { self.x = pos[0] as f32; self.y = pos[1] as f32 },
            Prop::Size(v) => { self.w = v[0] as f32; self.h = v[1] as f32 },
            _ => ()
        }
    }
    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Position(_) => Prop::Position(Point2D::new(self.x as f64, self.y as f64)),
            Prop::Size(_) => Prop::Size(Frame2D::new(self.w as f64, self.h as f64)),
            _ => Prop::None,
        }
    }
}

impl Tweenable for ggez::graphics::DrawParam {
    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Alpha(val) => { self.color.a = val[0] as f32 },
            Prop::Rotate(val) => { self.rotation = val[0] as f32 },
            _ => ()
        }
    }
    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Alpha(_) => { Prop::Alpha(FloatProp::new(self.color.a as f64)) },
            Prop::Rotate(_) => { Prop::Rotate(FloatProp::new(self.rotation as f64)) },
            _ => Prop::None,
        }
    }
}

/// This is a wrapper for the ggez properties that are tweenable. It is used as a convenient substitute
/// for having to manage multiple tweenables per displayed asset.
impl Tweenable for GGLayer {
    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Alpha(val) => { self.graphics.color.a = val[0] as f32 },
            Prop::Rotate(val) => { self.graphics.rotation = val[0] as f32 },
            Prop::Position(pos) => { self.frame.x = pos[0] as f32; self.frame.y = pos[1] as f32 },
            Prop::Size(v) => { self.frame.w = v[0] as f32; self.frame.h = v[1] as f32 },
            _ => ()
        }
    }
    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Alpha(_) => { Prop::Alpha(FloatProp::new(self.graphics.color.a as f64)) },
            Prop::Rotate(_) => { Prop::Rotate(FloatProp::new(self.graphics.rotation as f64)) },
            Prop::Position(_) => Prop::Position(Point2D::new(self.frame.x as f64, self.frame.y as f64)),
            Prop::Size(_) => Prop::Size(Frame2D::new(self.frame.w as f64, self.frame.h as f64)),
            _ => Prop::None,
        }
    }
}

/// This will implement Tweenable
pub struct GGLayer {
    pub frame: graphics::Rect,
    pub graphics: DrawParam,
}

impl GGLayer {
    pub fn new(frame: graphics::Rect, graphics: DrawParam )-> Self  {
        GGLayer{ frame: frame, graphics: graphics }
    }
}



pub trait TKResponder {
    fn hit_test(&self, pt: mint::Point2<f64>) -> bool;

}

/// Expected struct properties:
/// – position
/// – frame
///
pub trait TKMovable {

    fn set_frame(frame: mint::Vector4<f64>);


}

pub struct TKLabel {
    pub layer: GGLayer,
    pub text: String,
}

// impl TKLabel {
//     pub fn new(text: String) -> Self {
//         TKLabel {
//             text: text,
//             layer:
//         }
//     }
// }

pub struct TKButton {
    pub layer: GGLayer,
    pub graphics: DrawParam,
}

impl TKButton {
    pub fn new(frame: graphics::Rect, label: graphics::Text) -> Self {
        let layer = GGLayer{ frame: frame, graphics: DrawParam::default() };
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