/// The TweenLayer in ggez_support is a simple wrapper that makes ggez objects Tweenable.
/// That means it provides support for the get_prop() and apply() methods which simply
/// allow the framework to read and write values to graphical objects.
///
extern crate ggez;

use crate::core::*;

use ggez::graphics::{self, DrawParam};
use ggez::mint::{self};

//-- Main -----------------------------------------------------------------------

/// This also implements Tweenable
pub struct TweenLayer {
    pub frame: graphics::Rect,
    pub original: graphics::Rect,
    pub graphics: DrawParam,
    pub animation: Option<Tween>,
    pub stroke: f32,
}

impl TweenLayer {
    pub fn new(frame: graphics::Rect, graphics: DrawParam) -> Self {
        TweenLayer {
            frame: frame,
            original: frame.clone(),
            graphics: graphics,
            animation: None,
            stroke: 0.0,
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
                // The new position should take into account the specified offset for the graphic,
                // but only if it is between 0.0 and 1.0 for both x and y. You should not mess with
                // the scale property of DrawParam during live animation.
                let offset = self.graphics.offset;
                if offset.x > 0.0 && offset.x <= 1.0 && offset.y > 0.0 && offset.y <= 1.0 {
                    self.frame.x = pos[0] as f32 - offset.x * self.frame.w as f32;
                    self.frame.y = pos[1] as f32 - offset.y * self.frame.h as f32;
                } else {
                    self.frame.x = pos[0] as f32;
                    self.frame.y = pos[1] as f32;
                }
            }
            Prop::Size(size) => {
                self.frame.w = size[0] as f32;
                self.frame.h = size[1] as f32
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

