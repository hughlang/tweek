extern crate ggez;

use std::{collections::HashMap};
use std::rc::Rc;
use std::cell::RefCell;


// use super::property::*;
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
    fn tick(&mut self);
    // fn resume(&mut self);
    // fn seek(&mut self, pos: f64);

}

#[derive(Copy, Clone, Debug)]
pub enum TweenEvent {
    Play(usize),
    Pause(usize),
}

//-- Main -----------------------------------------------------------------------

/// Tweek is the god class around here. It is meant to be the parent of all Tweens
/// and the receiver of all notification events about animation status.
/// The tween_db is an attempt to centralize ownership of Tweens in one place
/// when using a Timeline. TBD
pub struct Tweek {
    tween_db: HashMap<String, Rc<RefCell<Tween>>>,
    subscribers: Vec<Rc<Fn(TweenEvent, &str) + 'static>>,
}

impl Tweek {
    pub fn new() -> Self {
        Tweek {
            tween_db: HashMap::new(),
            subscribers: Vec::new(),
        }
    }

    /// See: https://www.ralfj.de/projects/rust-101/part12.html
    /// This method should be called by a Timeline that wants to receive callbacks from
    /// Tweek.
    pub fn add_subscriber<C>(&mut self, cb: C) where C: Fn(TweenEvent, &str) + 'static {
        self.subscribers.push(Rc::new(cb));
    }

    /// This method should be called by a Timeline that wants a Tween to send events
    /// to Tweek and then re-publish them back to the Timeline which has added itself as
    /// the subscribers list.
    /// Same as add_tween but without the lifetime marks
    pub fn register_tween(&mut self, tween: &mut Tween) {
        let subscribers = self.subscribers.clone();
        tween.add_callback(move |e, g| {
            println!("Tween callback: event={:?} id={}", e, g);
            for cb in subscribers.iter() {
                (&*cb)(e, g);
            }
        });
        // self.tween_db.insert(tween.global_id, Rc::new(RefCell::new(tween)));


    }

    // Unused. Use register_tween instead
    pub fn add_tween<'a>(&'a self, tween: &'a mut Tween) {
        let subscribers = self.subscribers.clone();
        tween.add_callback(move |e, g| {
            println!("Tween callback: event={:?} id={}", e, g);
            for cb in subscribers.iter() {
                (&*cb)(e, g);
            }
        });
    }


    pub fn player_event_handler(&self, event: TweenEvent) {

    }


}

