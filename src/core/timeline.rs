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
    pub start: f64, // The start time in float seconds
    pub end: f64,   // The end time in float seconds
	pub state: AnimState,
}

impl TweenRange {
	fn new(tween: Tween, start: f64) -> Self {
		let end = start + tween.duration.as_float_secs();
		TweenRange {
			tween: Rc::new(RefCell::new(tween)),
			start: start,
			end: end,
			state: AnimState::Pending,
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
    pub repeat_count: i32, // -1 = forever
    pub repeat_delay: f64,
}

impl Timeline {
	pub fn new() -> Self {
		Timeline {
			children: HashMap::new(),
			tl_start: Instant::now(),
            repeat_count: 0,
            repeat_delay: 0.0,
		}
	}

	// pub fn init(tweek: Tweek) -> Self {

	// }

	pub fn create(tweens: Vec<Tween>, align: TweenAlign, tweek: &mut Tweek) -> Self {
		let mut timeline = Timeline::new();
		let mut start = 0.0 as f64;

		for mut t in tweens {
			let id = t.tween_id;
			let dur = t.duration.as_float_secs();
			t.add_callback(move |e, g| {
				println!("OG callback: event={:?} id={}", e, g);
				match e {
					TweenEvent::Completed(id) => {
						// &*timeline.play();
					},
					_ => (),
				}

			});
			// tweek.add_tween(&mut t);
			let range = TweenRange::new(t, start);
			match align {
				TweenAlign::Sequence => {
					start += dur;
				},
				_ => (),
			}
			timeline.children.insert(id, range);
		}

		timeline.setup(tweek)
	}

	fn setup(self, tweek: &mut Tweek) -> Self {
		tweek.add_subscriber( |e, g| {
            println!("Tweek subscriber: event={:?} id={}", e, g);
			match e {
				TweenEvent::Completed(id) => {
					// if let Some(tween) = self.children.get(&id) {

					// }
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

    pub fn repeat(mut self, count: i32, delay: f64) -> Self {
        self.repeat_count = count;
        self.repeat_delay = delay;
        self
    }

	pub fn notify(&mut self, event:&TweenEvent) {
		println!("notify event={:?}", event);


	}

}

impl Playable for Timeline {

	/// The Timeline play method should only play the tweens where the start time
	/// is not greater than the current elapsed time.
	fn play(&mut self) {
		for (_, range) in &self.children {
			let elapsed = self.tl_start.elapsed().as_float_secs();
			if range.start < elapsed && range.end > elapsed {
				let mut tween = range.tween.borrow_mut();
				(&mut *tween).play();
				// range.state = AnimState::Running;
			}
		}
	}

    fn stop(&mut self) {

	}

    fn pause(&mut self) {

	}

    fn tick(&mut self) {
		for (_, range) in &self.children {
			let elapsed = self.tl_start.elapsed().as_float_secs();
			if range.start < elapsed && range.end > elapsed {
				let mut tween = range.tween.borrow_mut();
				match tween.state {
					AnimState::Idle | AnimState::Pending => {
						(&mut *tween).play();
					},
					_ => {
						(&mut *tween).tick();
					}
				}
			} else {
				let mut tween = range.tween.borrow_mut();
				(&mut *tween).tick();
			}
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

