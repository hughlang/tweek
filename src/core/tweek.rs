extern crate ggez;

use std::{collections::HashMap};
use std::rc::Rc;
use std::cell::RefCell;


use super::property::*;
use super::timeline::*;
use super::tween::*;

//-- Base -----------------------------------------------------------------------

/// An attempt to create event callbacks using this example:
/// https://mattgathu.github.io/simple-events-hook-rust/
/// Not sure if it's useful
#[allow(unused_variables)]
pub trait Events {
    fn on_play(&mut self, caller: &Playable) {}
    fn on_start(&self, tween_id: usize) {}
    fn on_error(&self, tween_id: usize, err: &str) {}
    fn on_complete(&self, tween_id: usize) {}
}

pub trait Playable {
    fn play(&mut self);
    fn stop(&mut self);
    fn pause(&mut self);
    fn tick(&mut self, ctx: &mut TKContext);
    fn reset(&mut self);
    fn get_update(&mut self, id: &usize) -> Option<UIState>;
    // fn resume(&mut self);
    // fn seek(&mut self, pos: f64);
}

#[derive(Copy, Clone, Debug)]
pub enum TweenEvent {
    Completed(usize),
    Pause(usize),
    Play(usize),
	// case pending
	// case running
	// case idle
	// case cancelled
	// case completed
}

pub struct TKContext {
    tween_updates: Vec<usize>,
}

impl TKContext {
    pub fn new() -> Self {
        TKContext {
            tween_updates: Vec::new(),
        }
    }
}



//-- Main -----------------------------------------------------------------------

/// Tweek is the god class around here. It is meant to be the parent of all Tweens
/// and the receiver of all notification events about animation status.
/// The tween_db is an attempt to centralize ownership of Tweens in one place
/// when using a Timeline. TBD
pub struct Tweek {
    subscribers: Vec<Rc<Fn(TweenEvent, &str) + 'static>>,
    timelines: Vec<Rc<RefCell<Timeline>>>,
}

impl Tweek {
    pub fn new() -> Self {
        Tweek {
            subscribers: Vec::new(),
            timelines: Vec::new(),
        }
    }

    pub fn add_timeline(&mut self, timeline: Timeline) {
        self.timelines.push(Rc::new(RefCell::new(timeline)));
    }

    /// See: https://www.ralfj.de/projects/rust-101/part12.html
    /// This method should be called by a Timeline that wants to receive callbacks from
    /// Tweek.
    pub fn add_subscriber<C>(&mut self, cb: C) where C: Fn(TweenEvent, &str) + 'static {
        println!("Adding subscriber");
        self.subscribers.push(Rc::new(cb));
    }

    /// This method should be called by a Timeline that wants a Tween to send events
    /// to Tweek and then re-publish them back to the Timeline which has added itself as
    /// the subscribers list.
    /// Same as add_tween but without the lifetime marks
    pub fn register_tween(&mut self, tween: &mut Tween) {
        let subscribers = self.subscribers.clone();
        // let timelines = self.timelines.
        tween.add_callback(move |e, g| {
            println!("Tween callback: event={:?} id={}", e, g);
            for cb in subscribers.iter() {
                (&*cb)(e, g);
            }
        });


    }

    // Unused. Use register_tween instead
    pub fn add_tween<'a>(&'a self, tween: &'a mut Tween) {
        println!("add_tween for id={}", &tween.tween_id);
        let subscribers = self.subscribers.clone();
        tween.add_callback(move |e, g| {
            println!("Tween callback: event={:?} id={}", e, g);
            for cb in subscribers.iter() {
                (&*cb)(e, g);
            }
        });
    }


}

impl Playable for Tweek {
	fn play(&mut self) {
        for tl in &self.timelines {
            let mut timeline = tl.borrow_mut();
            (&mut *timeline).play();
        }
	}

    fn stop(&mut self) {
        for tl in &self.timelines {
            let mut timeline = tl.borrow_mut();
            (&mut *timeline).stop();
        }
	}

    fn pause(&mut self) {
        for tl in &self.timelines {
            let mut timeline = tl.borrow_mut();
            (&mut *timeline).pause();
        }
	}

    fn tick(&mut self, ctx: &mut TKContext) {
        for tl in &self.timelines {
            let mut timeline = tl.borrow_mut();
            (&mut *timeline).tick(ctx);
        }
	}

	fn reset(&mut self) {
        for tl in &self.timelines {
            let mut timeline = tl.borrow_mut();
            (&mut *timeline).reset();
        }
	}

    fn get_update(&mut self, id: &usize) -> Option<UIState> {
        for tl in &self.timelines {
            let mut timeline = tl.borrow_mut();
            if let Some(update) = (&mut *timeline).get_update(id) {
                return Some(update);
            }
        }
        None
    }

}
