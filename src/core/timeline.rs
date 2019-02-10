extern crate ggez;

use std::rc::Rc;
use std::cell::RefCell;
use std::{collections::HashMap};
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
    children: HashMap<usize, TweenRange>,

}

impl Timeline {
	pub fn new() -> Self {
		Timeline {
			tweek: Tweek::new(),
			children: HashMap::new(),
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
					let id = tween.tween_id;
					timeline.tweek.register_tween(&mut tween);
					let range = TweenRange::new(tween, 0.0);
					timeline.children.insert(id, range);
				}
			},
			TweenAlign::Sequence => {
				let mut pos = 0.0 as f64;
				for mut tween in tweens {
					let id = tween.tween_id;
					let dur = tween.duration.as_float_secs();
					timeline.tweek.register_tween(&mut tween);
					let range = TweenRange::new(tween, pos);
					pos += dur;
					timeline.children.insert(id, range);
				}
			},
			_ => (),
		}
		timeline
	}

    pub fn get_update(&self, id: &usize) -> Option<UIState> {
		if let Some(range) = &self.children.get(id) {
			let tween = range.tween.borrow();
			return tween.update_item(id);
		}
        None
    }
}

impl Playable for Timeline {

	fn play(&mut self) {
		// for range in &self.children {
		// 	// range.tween.borrow_mut().
		// 	let mut tween = range.tween.borrow_mut();
		// 	(&mut *tween).play();
		// }
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
		let end = start + tween.duration.as_float_secs();
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

