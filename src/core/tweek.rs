/// Tweek acts as the coordinator when there are multiple tweens being animated with one or more timelines.
///
extern crate ggez;

use std::cell::RefCell;
use std::rc::Rc;

use super::property::*;
use super::timeline::*;
use super::tween::*;

//-- Base -----------------------------------------------------------------------

pub type TweenRef = Rc<RefCell<Tween>>;
pub type UserCommand = u32;

/// The Playable trait provides support for basic animation updating and control
pub trait Playable {
    fn play(&mut self);

    // TODO: Return an Option<TKEvent> instead.
    fn tick(&mut self) -> Vec<TKEvent>;
    fn get_update(&mut self, id: &usize) -> Option<UIState>;
    fn stop(&mut self);
    fn pause(&mut self);
    fn reset(&mut self);
    // fn resume(&mut self);
    // fn seek(&mut self, pos: f64);
}

/// This is an experimental trait with the intention of passing around a mutable TKState
/// which other code can use.
///
pub trait TimelineAware {
    // fn tk_play(&mut self, ctx: &mut TKState);
    fn update(&mut self, ctx: &mut TKState);
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
    pub click_target: Option<usize>,
    pub row_target: Option<usize>,
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
            click_target: None,
            row_target: None,
        }
    }
}

pub type TKResult<T = ()> = Result<T, &'static str>;

//-- Main -----------------------------------------------------------------------

/// Tweek acts as a coordinator when multiple tween animations are added to a Timeline
/// for animation.
pub struct Tweek {
    subscribers: Vec<Rc<Fn(TKEvent, &mut TKState) + 'static>>,
    timelines: Vec<Rc<RefCell<Timeline>>>,
}

impl Tweek {
    pub fn new() -> Self {
        Tweek { subscribers: Vec::new(), timelines: Vec::new() }
    }

    pub fn add_timeline(&mut self, timeline: Timeline) {
        self.timelines.push(Rc::new(RefCell::new(timeline)));
    }

    /// See: https://www.ralfj.de/projects/rust-101/part12.html
    /// This method should be called by a Timeline that wants to receive callbacks from
    /// Tweek.
    pub fn add_subscriber<C>(&mut self, cb: C)
    where
        C: Fn(TKEvent, &mut TKState) + 'static,
    {
        log::debug!("Adding subscriber");
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
            log::debug!("Tween callback: event={:?}", e);
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
                    }
                    _ => (),
                }
            }
            &ctx.requests.clear();
        }
    }
}
