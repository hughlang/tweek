extern crate ggez;

use std::rc::Rc;
use std::cell::RefCell;
// use std::{collections::HashMap};
use super::property::*;
use super::tweek::*;
use super::tween::*;

//-- Base -----------------------------------------------------------------------




//-- Main -----------------------------------------------------------------------

/// This class provides a way of coordinating multiple Tweens for playback,
/// either starting at the same time or sequentially.
/// See also: https://greensock.com/asdocs/com/greensock/TimelineLite.html
pub struct Timeline {
	tweek: Tweek,
    children: Vec<TweenRange>,

}

impl Timeline {
	pub fn new() -> Self {
		Timeline {
			tweek: Tweek::new(),
			children: Vec::new(),
		}
	}
	pub fn create(tweens: Vec<Tween>, align: TweenAlign) -> Self {
		let mut timeline = Timeline::new();
		timeline.tweek.add_subscriber(move |e, g| {
            println!("Tweek subscriber: event={:?} id={}", e, g);
        });
		match align {
			TweenAlign::Normal => {
				for mut tween in tweens {
					timeline.tweek.register_tween(&mut tween);
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

impl Playable for Timeline {

	fn play(&mut self) {
		for range in &self.children {
			// range.tween.borrow_mut().
			let mut tween = range.tween.borrow_mut();
			(&mut *tween).play();
		}
	}

    fn stop(&mut self) {

	}
    fn pause(&mut self) {

	}
    fn tick(&mut self) {

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
    tween: Rc<RefCell<Tween>>,
    start: f64, // The start time in float seconds
    end: f64,   // The end time in float seconds
}

impl TweenRange {
	fn new(tween: Tween, start: f64) -> Self {
		let end = start + tween.duration_s;
		TweenRange {
			tween: Rc::new(RefCell::new(tween)),
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

