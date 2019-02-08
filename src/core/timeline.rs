extern crate ggez;

use std::{collections::HashMap};
use super::animator::*;
use super::property::*;
use super::tween::*;


/// An attempt to create event callbacks using this example:
/// https://mattgathu.github.io/simple-events-hook-rust/
#[allow(unused_variables)]
pub trait Events {
    fn on_start(&self) {}
    fn on_error(&self, err: &str) {}
    fn on_complete(&self) {}
}

pub struct Timeline {
    hooks: Vec<Box<Events>>,
    children: Vec<TweenRange>,

}

impl Timeline {
	pub fn new() -> Self {
		Timeline {
			hooks: Vec::new(),
			children: Vec::new(),
		}
	}
	// pub fn add(&self, tween: Tween) -> Self {
	// 	
	// }
}

//-- Support -----------------------------------------------------------------------

/**
 * From Greensock AS3:
 * Options are: "sequence" (aligns them one-after-the-other in a sequence), "start"
 * (aligns the start times of all of the objects (ignoring delays)), and "normal"
 * (aligns the start times of all the tweens (honoring delays)). The default is "normal".
 */
pub enum TimeAlign {
    Normal,
    Sequence,
    Start,
}

pub struct TweenRange {
    tween: Tween,
    start: f32, // The start time in float seconds
    end: f32,   // The end time in float seconds
}

impl TweenRange {
	fn new(tween: Tween, start: f32, end: f32) -> Self {
		TweenRange {
			tween,
			start,
			end,
		}
	}
}

