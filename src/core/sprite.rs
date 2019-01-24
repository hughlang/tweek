/// Sprite is a trait that makes things animatable
///
///
extern crate sdl2;

use super::property::*;
use super::tween::*;

pub trait Sprite {
    // type Item: Sprite;
    // fn tween(&self) -> Tween;
    fn apply(&self, prop: Property);
    fn current_property(&self, prop: Property) -> Option<Property>;
}

impl Sprite for sdl2::rect::Rect {
    // type Item = sdl2::rect::Rect;
    fn apply(&self, prop: Property) {

    }
    fn current_property(&self, prop: Property) -> Option<Property> {
        None
    }
}

// impl Sprite {
//     // pub fn tween() -> Tween {

//     // }
// }