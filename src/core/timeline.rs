extern crate ggez;

use std::rc::Rc;
use std::cell::RefCell;
use std::{collections::HashMap};
use std::{time::{Duration,Instant}};

use super::property::*;
use super::tweek::*;
use super::tween::*;

//-- Base -----------------------------------------------------------------------


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


//-- Main -----------------------------------------------------------------------

/// This class provides a way of coordinating multiple Tweens for playback,
/// either starting at the same time or sequentially.
/// See also: https://greensock.com/asdocs/com/greensock/TimelineLite.html
pub struct Timeline {
    children: HashMap<usize, TweenRange>,
    tl_start: Instant,
}

impl Timeline {
	pub fn new() -> Self {
		Timeline {
			children: HashMap::new(),
			tl_start: Instant::now(),
		}
	}

	pub fn create(tweens: Vec<Tween>, align: TweenAlign, tweek: &mut Tweek) -> Self {
		let mut timeline = Timeline::new();

		match align {
			TweenAlign::Normal => {
				for mut t in tweens {
					let id = t.tween_id;
					tweek.add_tween(&mut t);
					let range = TweenRange::new(t, 0.0);
					timeline.children.insert(id, range);
				}
			},
			TweenAlign::Sequence => {
				let mut pos = 0.0 as f64;
				for mut t in tweens {
					let id = t.tween_id;
					let dur = t.duration.as_float_secs();
					tweek.add_tween(&mut t);
					let range = TweenRange::new(t, pos);
					pos += dur;
					timeline.children.insert(id, range);
				}
			},
			_ => (),
		}
		timeline.setup(tweek)
	}

	pub fn setup(mut self, tweek: &mut Tweek) -> Self {
		tweek.add_subscriber( |e, g| {
            println!("Tweek subscriber: event={:?} id={}", e, g);
			match e {
				TweenEvent::Completed(id) => {
					// &self.play();
					// for (i, range) in &self.children {
					// 	println!("play – {}", i);
						// let elapsed = &self.tl_start.elapsed().as_float_secs();
					// 	if range.start < elapsed && range.end > elapsed {
					// 		let mut tween = range.tween.borrow_mut();
					// 		(&mut *tween).play();
					// 	}
					// }

				},
				_ => (),
			}
        });
		self
	}

}

impl Playable for Timeline {

	/// The Timeline play method should only play the tweens where the start time
	/// is not greater than the current elapsed time.
	fn play(&mut self) {
		for (i, range) in &self.children {
			println!("play – {}", i);
			let elapsed = self.tl_start.elapsed().as_float_secs();
			if range.start < elapsed && range.end > elapsed {
				let mut tween = range.tween.borrow_mut();
				(&mut *tween).play();
			}
		}
	}

    fn stop(&mut self) {

	}

    fn pause(&mut self) {

	}

    fn tick(&mut self) {
		for (_, range) in &self.children {
			let mut tween = range.tween.borrow_mut();
			(&mut *tween).tick();
		}

	}

	fn reset(&mut self) {
		self.tl_start = Instant::now();
		for (_, range) in &self.children {
			let mut tween = range.tween.borrow_mut();
			(&mut *tween).reset();
		}

	}

    fn get_update(&mut self, id: &usize) -> Option<UIState> {
		if let Some(range) = &self.children.get(id) {
			let mut tween = range.tween.borrow_mut();
			return (&mut *tween).get_update(id);
		}
        None
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

#[derive(PartialEq)]
pub enum AnimState {
    Pending,
    Running,
    Idle,
    Cancelled,
    Completed,
}

