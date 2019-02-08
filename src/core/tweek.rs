extern crate ggez;

use std::{collections::HashSet};
use uuid::Uuid;

use super::property::*;
use super::animator::*;
use super::timeline::*;
use super::tween::*;

//-- Base -----------------------------------------------------------------------

/// An attempt to create event callbacks using this example:
/// https://mattgathu.github.io/simple-events-hook-rust/
/// Not sure if it's useful
#[allow(unused_variables)]
pub trait Events {
    fn on_play(&mut self, caller: &Animatable) {}
    fn on_start(&self, tween_id: usize) {}
    fn on_error(&self, tween_id: usize, err: &str) {}
    fn on_complete(&self, tween_id: usize) {}
}

pub trait Playable {

}

/// The Animatable trait is used by Tween to...
pub trait Animatable {
    /// Animatibles need to check on their player state and report changes to
    fn tick(&mut self);
    fn play(&mut self);
    fn stop(&mut self);
    fn pause(&mut self);
    // fn resume(&mut self);
    // fn seek(&mut self, pos: f64);
	// fn add_events_hook<E: Events + 'static>(&mut self, hook: E);

}

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
    tween_db: HashSet<Tween>,
}

impl Tweek {
    pub fn new() -> Self {
        Tweek {
            tween_db: HashSet::new(),
        }
    }

    fn add_tween(tween: &Tween) {


    }

    pub fn player_event_handler(&self, event: TweenEvent, caller: &Playable) {

    }


}

