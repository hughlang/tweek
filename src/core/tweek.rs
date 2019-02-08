extern crate ggez;

use std::{collections::HashMap};
use uuid::Uuid;

use super::property::*;
use super::animator::*;
use super::timeline::*;
use super::tween::*;

//-- Base -----------------------------------------------------------------------

/// An attempt to create event callbacks using this example:
/// https://mattgathu.github.io/simple-events-hook-rust/
/// At the moment, the plan is to publish events from Tweens to the Tweek manager
/// which will take care of managing Timelines, etc.
#[allow(unused_variables)]
pub trait Events {
    fn on_start(&self, tween_id: usize) {}
    fn on_error(&self, tween_id: usize, err: &str) {}
    fn on_complete(&self, tween_id: usize) {}
}

pub trait Animatable {
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
    tween_db: HashMap<String, Tween>,
}

impl Tweek {
    fn new() -> Self {
        Tweek {
            tween_db: HashMap::new(),
        }
    }

    // fn add_tween(tween: Tween) -> String {
    //     let uuid = Uuid::new_v4();

    // }
}

