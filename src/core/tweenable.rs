/// Tweenable is a trait that makes things animatable
///
///
extern crate sdl2;
use std::any::Any;

use super::property::*;
use super::tween::*;

pub trait Tweenable: Any {
    type Item: Tweenable;
    // fn tween(&self) -> Tween;
    fn apply(prop: Property);
    fn current_property(prop: Property) -> Option<Property>;
}

pub struct Sprite {

}

impl Tweenable for Sprite {
    type Item = Sprite;
    fn apply(prop: Property) {

    }
    fn current_property(prop: Property) -> Option<Property> {
        None
    }
}

impl Tweenable for sdl2::rect::Rect {
    type Item = sdl2::rect::Rect;
    fn apply(prop: Property) {

    }
    fn current_property(prop: Property) -> Option<Property> {
        None
    }
}

// impl Tweenable {
//     // pub fn tween() -> Tween {

//     // }
// }