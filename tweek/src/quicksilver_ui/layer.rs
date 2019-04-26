/// TweenLayer for Quicksilver
///
extern crate quicksilver;

use super::*;
use crate::core::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{run, Settings, State, Window},
};

//-- Base -----------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseState {
    None,
    Click,
    Drag,
    Focus,
    Hover,
    Select,
}

//-- Main -----------------------------------------------------------------------

/// This is a wrapper for the ggez properties that are tweenable. It is used as a convenient substitute
/// for having to manage multiple tweenables per displayed asset.
pub struct TweenLayer {
    pub frame: Rectangle,
    pub original: Rectangle,
    pub rotation: f32,
    pub defaults: Vec<Prop>,
    pub color: Color,
    pub theme: Option<Theme>,
    // pub font: Font,
    // pub font_size: f32,
    pub animation: Option<Tween>,
    pub stroke: f32,
    pub border_width: f32,
    pub border_color: Option<Color>,
    pub on_hover: Option<UITransition>,
    pub mouse_state: MouseState,
}

impl TweenLayer {
    pub fn new(frame: Rectangle) -> Self {
        TweenLayer {
            frame: frame,
            original: frame,
            rotation: 0.0,
            theme: None,
            defaults: Vec::new(),
            color: Color::WHITE,
            animation: None,
            stroke: 1.0,
            border_width: 0.0,
            border_color: None,
            on_hover: None,
            mouse_state: MouseState::None,
        }
    }

    pub fn handle_mouse_over(&mut self, pt: &Vector) -> bool {
        if pt.overlaps_rectangle(&self.frame) {
            match self.mouse_state {
                MouseState::None => {
                    // change state to hover and start animations
                    self.mouse_state = MouseState::Hover;

                    if let Some(transition) = &self.on_hover {
                        if transition.seconds > 0.0 {
                            let mut tween = Tween::with(0, self).to(&transition.props).duration(transition.seconds);
                            &tween.play();
                            self.animation = Some(tween);
                        } else {
                            self.apply_updates(&transition.props.clone());
                        }
                    }
                }
                _ => (),
            }
            return true;
        } else {
            match self.mouse_state {
                MouseState::Hover => {
                    // log::debug!("Mouse out at: x={} y={}", x, y);
                    self.apply_updates(&self.defaults.clone());
                    self.mouse_state = MouseState::None;
                    self.animation = None;
                }
                _ => (),
            }
        }
        false
    }

    /// Returns a Rect within the parent rect padded by the specified values, using
    /// the coordinate system of this TweenLayer object. That is, the origin is based on (0.0, 0.0)
    /// in this self.frame
    ///
    /// TBD: Should we follow CSS pattern or Apple's UIEdgeInsets pattern?
    /// – CSS margins: https://developer.mozilla.org/en-US/docs/Web/CSS/margin
    /// – Apple iOS insets: https://developer.apple.com/documentation/uikit/1624475-uiedgeinsetsmake
    /// It seems more logical to use top-left-right-bottom, so mapping the x, y of a Rect is more obvious.
    pub fn inset_by(&self, left: f32, top: f32, right: f32, bottom: f32) -> Rectangle {
        Rectangle::new(
            (self.frame.pos.x + left, self.frame.pos.y + top),
            (self.frame.size.x - left - right, self.frame.size.y - top - bottom),
        )
    }

    /// This returns a rect relative to the current self.frame coordinates using
    /// the coordinate system the parent of the this TweenLayer object.
    pub fn offset_by(&self, left: f32, top: f32, right: f32, bottom: f32) -> Rectangle {
        Rectangle::new(
            (self.frame.pos.x + left, self.frame.pos.y + top),
            (self.frame.size.x - left - right, self.frame.size.y - top - bottom),
        )
    }

    pub fn get_border_lines(&self, width: f32) -> Vec<Line> {
        let lines = UITools::make_border_lines(&self.frame, width);
        lines
    }
}

impl Tweenable for TweenLayer {
    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Alpha(_) => Prop::Alpha(FloatProp::new(self.color.a as f64)),
            Prop::Color(_) => Prop::Color(ColorRGB::new(
                self.color.r * 255.0 as f32,
                self.color.g * 255.0 as f32,
                self.color.b * 255.0 as f32,
            )),
            Prop::Rotate(_) => Prop::Rotate(FloatProp::new(self.rotation as f64)),
            Prop::Position(_) => Prop::Position(Point2D::new(self.frame.pos.x as f64, self.frame.pos.y as f64)),
            Prop::Size(_) => Prop::Size(Frame2D::new(self.frame.size.x as f64, self.frame.size.y as f64)),
            _ => Prop::None,
        }
    }
    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Alpha(val) => self.color.a = val[0] as f32,
            Prop::Color(rgb) => {
                self.color.r = rgb[0] / 255.0;
                self.color.g = rgb[1] / 255.0;
                self.color.b = rgb[2] / 255.0;
            }
            Prop::Rotate(val) => self.rotation = val[0] as f32,
            Prop::Position(pos) => {
                self.frame.pos.x = pos[0] as f32;
                self.frame.pos.y = pos[1] as f32;
            }
            Prop::Size(size) => {
                self.frame.size.x = size[0] as f32;
                self.frame.size.y = size[1] as f32
            }
            _ => (),
        }
    }
}
