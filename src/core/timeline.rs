extern crate ggez;

// use std::{collections::HashMap};
// use super::animator::*;
use super::property::*;
use super::tween::*;

//-- Base -----------------------------------------------------------------------

/// An attempt to create event callbacks using this example:
/// https://mattgathu.github.io/simple-events-hook-rust/
#[allow(unused_variables)]
pub trait Events {
    fn on_start(&self) {}
    fn on_error(&self, err: &str) {}
    fn on_complete(&self) {}
}


/// This is an experimental implementation of the Events callback
pub struct Logger;

impl Events for Logger {
    fn on_start(&self) {
		println!("Started");
	}
    fn on_error(&self, err: &str) {
		println!("error: {}", err);
	}
    fn on_complete(&self) {
		println!("Finished");
	}
}

pub trait Animatable {
    fn play(&mut self);
    fn stop(&mut self);
    fn pause(&mut self);
    // fn resume(&mut self);
    // fn seek(&mut self, pos: f64);
	fn add_events_hook<E: Events + 'static>(&mut self, hook: E);

}

//-- Main -----------------------------------------------------------------------

/// This class provides a way of coordinating multiple Tweens for playback,
/// either starting at the same time or sequentially.
/// See also: https://greensock.com/asdocs/com/greensock/TimelineLite.html
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
	pub fn create(tweens: Vec<Tween>, align: TweenAlign) -> Self {
		let mut timeline = Timeline::new();
		match align {
			TweenAlign::Normal => {
				for tween in tweens {
					let range = TweenRange::new(tween, 0.0);
					timeline.children.push(range);
				}
			},
			TweenAlign::Sequence => {
				let mut pos = 0.0 as f64;
				for tween in tweens {
					let dur = tween.duration_s;
					let range = TweenRange::new(tween, pos);
					pos += dur;
					timeline.children.push(range);
				}
			},
			_ => (),
		}
		timeline
	}
	// pub fn add(&self, tween: Tween) -> Self {
	//
	// }

}

impl Animatable for Timeline {

	/// Main method for starting play of all tweens
    fn play(&mut self) {
        for hook in &self.hooks {
            hook.on_start();
        }
	}
    fn stop(&mut self) {

	}
    fn pause(&mut self) {

	}

    fn add_events_hook<E: Events + 'static>(&mut self, hook: E) {
        self.hooks.push(Box::new(hook));
    }

}

//-- Support -----------------------------------------------------------------------

/**
 * From Greensock AS3:
 * Options are: "sequence" (aligns them one-after-the-other in a sequence), "start"
 * (aligns the start times of all of the objects (ignoring delays)), and "normal"
 * (aligns the start times of all the tweens (honoring delays)). The default is "normal".
 */
pub enum TweenAlign {
    Normal,
    Sequence,
    Start,
}

pub struct TweenRange {
    tween: Tween,
    start: f64, // The start time in float seconds
    end: f64,   // The end time in float seconds
}

impl TweenRange {
	fn new(tween: Tween, start: f64) -> Self {
		let end = start + tween.duration_s;
		TweenRange {
			tween: tween,
			start: start,
			end: end,
		}
	}
}

pub enum AnimState {
    Pending,
    Running,
    Idle,
    Cancelled,
    Completed,
}

