extern crate ggez;

use std::{collections::HashMap};
use super::property::*;
use super::animator::*;
use super::timeline::*;
use super::tween::*;


//-- Base -----------------------------------------------------------------------

/// An attempt to create event callbacks using this example:
/// https://mattgathu.github.io/simple-events-hook-rust/
/// It doesn't seem very useful at the moment, but I will leave it in for now
#[allow(unused_variables)]
pub trait Events {
    fn on_start(&self) {}
    fn on_error(&self, err: &str) {}
    fn on_complete(&self) {}
}

//-- Main -----------------------------------------------------------------------


// pub trait
pub struct Tweek {
    tween_db: HashMap<usize, Tween>,
}

impl Tweek {
    fn new() -> Self {
        Tweek {
            tween_db: HashMap::new(),
        }
    }
}

impl Events for Tweek {
    fn on_start(&self) {
		println!("Started");
	}
    fn on_error(&self, err: &str) {
		println!("error: {}", err);
	}
    fn on_complete(&self) {
		println!("Finished");
	}
}
