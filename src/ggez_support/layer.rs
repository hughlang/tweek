/// The TweenLayer in ggez_support is a simple wrapper that makes ggez objects Tweenable.
/// That means it provides support for the get_prop() and apply() methods which simply
/// allow the framework to read and write values to graphical objects.
///
extern crate ggez;

use ggez::graphics::{self, Color, DrawParam, Font, Rect};
use ggez::mint;
use std::any::TypeId;

use super::*;
use crate::core::*;

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
    pub frame: graphics::Rect,
    pub original: graphics::Rect,
    pub defaults: Vec<Prop>,
    pub graphics: DrawParam,
    pub theme: Theme,
    pub font: Font,
    pub font_size: f32,
    pub animation: Option<Tween>,
    pub stroke: f32,
    pub border_width: f32,
    pub border_color: Option<Color>,
    pub on_hover: Option<UITransition>,
    pub mouse_state: MouseState,
}

impl TweenLayer {
    pub fn new(frame: graphics::Rect, graphics: DrawParam) -> Self {
        TweenLayer {
            frame: frame,
            original: frame.clone(),
            defaults: Vec::new(),
            graphics: graphics,
            theme: Theme::default(),
            font: Font::default(),
            font_size: 14.0,
            animation: None,
            stroke: 0.0,
            border_width: 0.0,
            border_color: None,
            on_hover: None,
            mouse_state: MouseState::None,
        }
    }

    pub fn handle_mouse_over(&mut self, x: f32, y: f32) -> bool {
        if self.frame.contains(mint::Point2 { x, y }) {
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
    pub fn inset_by(&self, left: f32, top: f32, right: f32, bottom: f32) -> Rect {
        Rect::new(self.frame.x + left, self.frame.y + top, self.frame.w - left - right, self.frame.h - top - bottom)
    }

    /// This returns a rect relative to the current self.frame coordinates using
    /// the coordinate system the parent of the this TweenLayer object.
    pub fn offset_by(&self, left: f32, top: f32, right: f32, bottom: f32) -> Rect {
        Rect::new(self.frame.x + left, self.frame.y + top, self.frame.w - left - right, self.frame.h - top - bottom)
    }

    pub fn scrollable_types() -> Vec<TypeId> {
        vec![TypeId::of::<ListBox>(), TypeId::of::<TextArea>(), TypeId::of::<TextField>()]
    }

    pub fn is_scrollable(type_id: &TypeId) -> bool {
        TweenLayer::scrollable_types().contains(type_id)
    }

    /**
    * Method which returns the list of Rect areas outside of the object frame.
    * Up to four Rect objects are returned based on the outside Rect param provided.
    * The diagram below illustrates how the Rect areas are defined.
    * Top and Bottom, full width. Left and Right, in between.
    *
       +--------------+
       |              |
       +--------------+
       |  |********|  |
       |  |********|  |
       |  |********|  |
       +--------------+
       |              |
       +--------------+
    */
    pub fn get_perimeter_blocks(frame: &Rect, outside: &Rect) -> Vec<Rect> {
        if outside.x == frame.x && outside.y == frame.y && outside.w == frame.w && outside.h == frame.h {
            return Vec::new();
        }

        let mut blocks: Vec<Rect> = Vec::new();
        if frame.y - outside.y > 0.0 {
            blocks.push(Rect::new(outside.x, outside.y, outside.w, frame.y - outside.y - 1.0));
        }
        if outside.bottom() - frame.bottom() > 0.0 {
            blocks.push(Rect::new(outside.x, frame.bottom() + 1.0, outside.w, outside.bottom() - frame.bottom()));
        }
        if frame.x - outside.x > 0.0 {
            blocks.push(Rect::new(outside.x, frame.top(), frame.x - outside.x - 1.0, frame.bottom() - frame.top()));
        }
        if outside.right() - frame.right() > 0.0 {
            blocks.push(Rect::new(
                frame.right() + 1.0,
                frame.top(),
                outside.right() - frame.right() + 1.0,
                frame.bottom() - frame.top(),
            ));
        }
        blocks
    }
}

impl Tweenable for TweenLayer {
    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Alpha(val) => self.graphics.color.a = val[0] as f32,
            Prop::Color(rgb) => {
                self.graphics.color.r = rgb[0] / 255.0;
                self.graphics.color.g = rgb[1] / 255.0;
                self.graphics.color.b = rgb[2] / 255.0;
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
            }
            Prop::Rotate(_) => Prop::Rotate(FloatProp::new(self.graphics.rotation as f64)),
            Prop::Position(_) => Prop::Position(Point2D::new(self.frame.x as f64, self.frame.y as f64)),
            Prop::Size(_) => Prop::Size(Frame2D::new(self.frame.w as f64, self.frame.h as f64)),
            _ => Prop::None,
        }
    }
}
