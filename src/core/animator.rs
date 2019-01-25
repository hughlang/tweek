/// An Animator has start and end properties that can be rendered in an animation
///
///
use std::{any::TypeId, collections::HashMap, rc::Rc};

use super::sprite::*;
use super::property::*;

#[allow(dead_code)]

#[derive(Default, Clone, Debug)]
pub struct Animator {
    start: Transition,
    end: Transition,
    current: Transition,
    duration: f32,
}

impl Animator {
    pub fn create(_start: Property, _end: Property, _duration) -> Self {
        Animator { start: _start, end: _end, current: start, duration: _duration, }
    }

    pub fn render() {

    }
}

// Unused
struct Interpolaendr {
    id: u32,
    progress: f32,
    finished: bool,

}
