extern crate ggez;

use std::{collections::HashMap};
use std::rc::Rc;
use std::cell::RefCell;

use super::property::*;
use super::timeline::*;
use super::tween::*;

//-- Base -----------------------------------------------------------------------

pub type TweenRef = Rc<RefCell<Tween>>;
pub type UserCommand = u32;

pub trait Playable {
    fn play(&mut self);
    fn tick(&mut self) -> Vec<TKEvent>;
    fn get_update(&mut self, id: &usize) -> Option<UIState>;
    fn stop(&mut self);
    fn pause(&mut self);
    fn reset(&mut self);
    // fn resume(&mut self);
    // fn seek(&mut self, pos: f64);
}


#[derive(Copy, Clone, Debug)]
pub enum TKEvent {
    None,
    Completed(usize),
    Pause(usize),
    Play(usize),
	// case pending
	// case running
	// case idle
	// case cancelled
	// case completed
}

#[derive(Copy, Clone, Debug)]
pub enum TKAction {
    Click,
    Hover,
}

pub enum TKRequest {
    Play,
    Replay,
    Pause,
    Reverse,
    SkipForward,

}

pub struct TKState {
    pub time_scale: f64,
    pub elapsed_time: f64,
    pub total_time: f64,
    pub events: Vec<TKEvent>,
    pub requests: Vec<TKRequest>,
    /// user defined u32 values that can be used for any purpose.
    pub commands: Vec<UserCommand>,
    tween_store: HashMap<usize, TweenRef>,
}

impl TKState {
    pub fn new() -> Self {
        TKState {
            time_scale: 1.0,
            elapsed_time: 0.0,
            total_time: 0.0,
            events: Vec::new(),
            requests: Vec::new(),
            commands: Vec::new(),
            tween_store: HashMap::new(),
        }
    }

    pub fn get_update(&mut self, id: &usize) -> Option<UIState> {
        if let Some(rc) = self.tween_store.get(id) {
            let mut tween = rc.borrow_mut();
            let update = (&mut *tween).update();
            return update;
        }
        None
    }

    pub fn register_tween(&mut self, tween: Tween) {
        self.tween_store.insert(tween.tween_id.clone(), Rc::new(RefCell::new(tween)));
    }
}


pub trait TKResponder {
    fn handle_mouse_at(&mut self, _x: f32, _y: f32) -> bool {
        false
    }
    fn handle_mouse_down(&mut self, _x: f32, _y: f32, _state: &mut TKState) -> bool {
        false
    }
    fn handle_mouse_up(&mut self, _x: f32, _y: f32, _state: &mut TKState) -> bool {
        false
    }
}


//-- Main -----------------------------------------------------------------------

/// Tweek is the god class around here. It is meant to be the parent of all Tweens
/// and the receiver of all notification events about animation status.
/// The tween_db is an attempt to centralize ownership of Tweens in one place
/// when using a Timeline. TBD
pub struct Tweek {
    subscribers: Vec<Rc<Fn(TKEvent, &mut TKState) + 'static>>,
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
    pub fn add_subscriber<C>(&mut self, cb: C) where C: Fn(TKEvent, &mut TKState) + 'static {
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
            println!("Tween callback: event={:?}", e);
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
            println!("Tween callback: event={:?}", e);
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

    fn tick(&mut self) -> Vec<TKEvent> {
        let mut events: Vec<TKEvent> = Vec::new();
        for tl in &self.timelines {
            let mut timeline = tl.borrow_mut();
            let mut ticks = (&mut *timeline).tick();
            events.append(&mut ticks);
        }
        events
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

/// This is an experimental trait with the intention of passing around a mutable TKState
/// which other code can use. TKState has a shared tween_store where all tweens are registered.
/// Some ideas
/// * allow other code to add callback functions that execute when specific events happen?
///
pub trait TimelineAware {
    // fn tk_play(&mut self, ctx: &mut TKState);
    fn update(&mut self, ctx: &mut TKState);

}

impl TimelineAware for Tweek {

    fn update(&mut self, ctx: &mut TKState) {

        if ctx.requests.is_empty() {
            ctx.events.clear();
            for tl in &self.timelines {
                let mut timeline = tl.borrow_mut();
                (&mut *timeline).update(ctx);
            }
        } else {
            for request in &ctx.requests {
                match request {
                    TKRequest::Play => {
                        for tl in &self.timelines {
                            let mut timeline = tl.borrow_mut();
                            (&mut *timeline).reset();
                        }
                    },
                    _ => (),
                }
            }
            &ctx.requests.clear();
        }
    }
}
