/// This file will contain various helpers that will make it easier to use Tweek
/// in conjunction with ggez. Some ideas:
/// * A progress/timeline widget that can display timeline status information
/// * Buttons for play/pause/restart
///
///
extern crate ggez;

use crate::core::*;

use ggez::graphics::{self, DrawParam};
use ggez::mint;


//-- Base -----------------------------------------------------------------------

pub enum MouseState {
    None,
    Hover,
    Drag,
    Click,
}

// Unused
pub enum GGShape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
    Image(graphics::Rect),
    Text(graphics::Rect),
    Line(mint::Point2<f32>, mint::Point2<f32>),
}

//-- Main -----------------------------------------------------------------------


/// This also implements Tweenable
pub struct TweenLayer {
    pub frame: graphics::Rect,
    pub graphics: DrawParam,
    pub animation: Option<Tween>,
    pub stroke: f32,
    pub redraw: bool,
}

impl TweenLayer {
    pub fn new(frame: graphics::Rect, graphics: DrawParam) -> Self {
        TweenLayer {
            frame: frame,
            graphics: graphics,
            animation: None,
            stroke: 0.0,
            redraw: false,
        }
    }
}

/// This is a wrapper for the ggez properties that are tweenable. It is used as a convenient substitute
/// for having to manage multiple tweenables per displayed asset.
impl Tweenable for TweenLayer {
    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Alpha(val) => self.graphics.color.a = val[0] as f32,
            Prop::Color(rgb) => {
                self.graphics.color.r = rgb[0]/255.0;
                self.graphics.color.g = rgb[1]/255.0;
                self.graphics.color.b = rgb[2]/255.0;
            }
            Prop::Rotate(val) => self.graphics.rotation = val[0] as f32,
            Prop::Position(pos) => {
                self.frame.x = pos[0] as f32;
                self.frame.y = pos[1] as f32
            }
            Prop::Size(v) => {
                self.frame.w = v[0] as f32;
                self.frame.h = v[1] as f32
            }
            _ => (),
        }
    }
    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Alpha(_) => Prop::Alpha(FloatProp::new(self.graphics.color.a as f64)),
            Prop::Color(_) => {
                let (r, g, b) = self.graphics.color.to_rgb();
                Prop::Color(ColorRGB::new(r as f32, g as f32, b as f32))
            },
            Prop::Rotate(_) => Prop::Rotate(FloatProp::new(self.graphics.rotation as f64)),
            Prop::Position(_) => {
                Prop::Position(Point2D::new(self.frame.x as f64, self.frame.y as f64))
            }
            Prop::Size(_) => Prop::Size(Frame2D::new(self.frame.w as f64, self.frame.h as f64)),
            _ => Prop::None,
        }
    }
}




//-- Support -----------------------------------------------------------------------

