extern crate ggez;

use std::rc::Rc;
use std::cell::RefCell;
use std::{collections::HashMap};
use std::{time::{Duration,Instant}};

use super::property::*;
use super::tweek::*;
use super::tween::*;

//-- Base -----------------------------------------------------------------------

type TweenRef = Rc<RefCell<Tween>>;

pub struct TweenRange {
    tween: TweenRef,
    pub start: f64, // The start time in float seconds
    pub end: f64,   // The end time in float seconds
	pub state: TweenState,
}

impl TweenRange {
	fn new(tween: Tween, start: f64) -> Self {
		let end = start + tween.duration.as_float_secs();
		TweenRange {
			tween: Rc::new(RefCell::new(tween)),
			start: start,
			end: end,
			state: TweenState::Pending,
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
    pub repeat_delay: Duration,
}

impl Timeline {
	pub fn new() -> Self {
		Timeline {
			children: HashMap::new(),
			tl_start: Instant::now(),
            repeat_count: 0,
            repeat_delay: Duration::from_secs(0),
		}
	}

	pub fn create(tweens: Vec<Tween>, align: TweenAlign) -> Self {
		let mut timeline = Timeline::new();
		let mut start = 0.0 as f64;

		for mut t in tweens {
			let id = t.tween_id;
			let dur = t.duration.as_float_secs();
			t.add_callback(move |e, ctx| {
				println!("OG callback: event={:?}", e);
				match e {
					TKEvent::Completed(id) => {
						// Inform ctx that playback has completed
						ctx.events.push(e);
					},
					_ => (),
				}

			});

			let range = TweenRange::new(t, start);

			match align {
				TweenAlign::Sequence => {
					start += dur;
				},
				_ => (),
			}
			timeline.children.insert(id, range);
		}

		timeline
	}

	// fn setup(self, ctx: &mut TKContext) -> Self {
	// 	tweek.add_subscriber( |e, g| {
    //         println!("Tweek subscriber: event={:?}", e);
	// 		match e {
	// 			TKEvent::Completed(id) => {
	// 				// if let Some(tween) = self.children.get(&id) {

	// 				// }
	// 				// &self.play();
	// 				// for (i, range) in &self.children {
	// 				// 	println!("play – {}", i);
	// 					// let elapsed = &self.tl_start.elapsed().as_float_secs();
	// 				// 	if range.start < elapsed && range.end > elapsed {
	// 				// 		let mut tween = range.tween.borrow_mut();
	// 				// 		(&mut *tween).play();
	// 				// 	}
	// 				// }

	// 			},
	// 			_ => (),
	// 		}
    //     });
	// 	self
	// }

    pub fn repeat(mut self, count: i32, delay: f64) -> Self {
        self.repeat_count = count;
        self.repeat_delay = Duration::from_float_secs(delay);
        self
    }

	pub fn notify(&mut self, event:&TKEvent) {
		println!("notify event={:?}", event);
	}

	// pub fn get_total_duration(&self) -> f64 {
	// 	// let mut max = 0 as f64;
	// 	// for range in self.children {

	// 	// }
	// 	let x = self.children.values().max_by_key(|v| v.end );


	// }

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
				// range.state = TweenState::Running;
			}
		}
	}

    fn tick(&mut self) -> Vec<TKEvent> {
        let mut events: Vec<TKEvent> = Vec::new();
		for (_, range) in &self.children {
			let elapsed = self.tl_start.elapsed().as_float_secs();
			if range.start < elapsed && range.end > elapsed {
				let mut tween = range.tween.borrow_mut();
				match tween.state {
					TweenState::Idle | TweenState::Pending => {
						(&mut *tween).play();
					},
					_ => {
						let mut ticks = (&mut *tween).tick();
			            events.append(&mut ticks);
					}
				}
			} else {
				let mut tween = range.tween.borrow_mut();
				let mut ticks = (&mut *tween).tick();
				events.append(&mut ticks);
			}
		}

		// Now read the context for events
		for event in &events {
			match event {
				TKEvent::Completed(id) => {
					// Decide: repeat?
					if let Some(range) = &self.children.get(id) {

						// self.reset();
						// let mut tween = range.tween.borrow_mut();
						// (&mut *tween).reset();
					}

				}
				_ => (),
			}
		}
		events
	}

    fn sync(&mut self, ctx: &mut TKContext) {
		for (_, range) in &self.children {
			let mut tween = range.tween.borrow_mut();
			(&mut *tween).sync(ctx);
		}
    }

    fn stop(&mut self) {

	}

    fn pause(&mut self) {

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


