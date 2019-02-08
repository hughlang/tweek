extern crate ggez;

// use std::{collections::HashMap};
// use super::animator::*;
use super::property::*;
use super::tweek::*;
use super::tween::*;

//-- Base -----------------------------------------------------------------------




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
					// tween.add_events_hook(Timeline);
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
	pub fn notify(&self, event: TweenEvent, id: &str) {

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

