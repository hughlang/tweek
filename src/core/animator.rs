/// An Animator has start and end properties that can be rendered in an animation
///
///
use std::{any::TypeId, collections::HashMap, rc::Rc};

use super::sprite::*;
use super::property::*;

#[allow(dead_code)]

pub struct Animator {
    pub start: Transition,
    pub end: Transition,
    pub current: Transition,
    pub duration: f32,
}

impl Default for Animator {
    fn default() -> Self {
        Animator {
            start: Transition::None,
            end: Transition::None,
            current: Transition::None,
            duration: 0.0,
        }
    }
}

impl Animator {
    // pub fn create(_start: Property, _end: Property, _duration: f32) -> Self {
    //     Animator { start: _start, end: _end, current: start, duration: _duration, }
    // }

    pub fn render() {

    }
}

// Unused
struct Interpolator {
    id: u32,
    progress: f32,
    finished: bool,

}
