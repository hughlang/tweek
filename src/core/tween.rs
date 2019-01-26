/// This is the core Tween model and functions.
extern crate orbrender;

// use enum_primitive_derive::*;
// use num_traits::{FromPrimitive, ToPrimitive};

use std::{any::TypeId, collections::HashMap, rc::Rc};
use na::*;

use super::animator::*;

// type ColorRGB = Matrix1x3<f64>;
type ColorRGBA = Matrix1x4<u8>;
type Point2D = Matrix1x2<f64>;
type Frame2D = Matrix1x2<f64>;

pub enum Prop {
    None,
    Alpha(f64),
    Color(ColorRGBA),
    Position(Point2D),
    Size(Frame2D),
}

impl Prop {
    pub fn prop_id(&self) -> u32 {
        get_prop_id(self)
    }
}

// Stupid shit helper method because Rust enums cannot emit a discriminator Int id if there are custom fields
pub fn get_prop_id(prop: &Prop) -> u32 {
    match prop {
        Prop::None => 0,
        Prop::Alpha(_) => 1,
        Prop::Color(_) => 2,
        Prop::Position(_) => 3,
        Prop::Size(_) => 4,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TweenMode {
    To,
    From,
    FromTo,
}

// pub trait Animation {
//     fn play();
//     fn duration(&self, _seconds: f64) -> Self;

// }

pub struct Tween {
    delay_s: f64,
    duration_s: f64,
    progress_s: f64,
    // props: Vec<Prop>,
    propsMap: HashMap<u32, Prop>,
}

// TODO: new constructor with Tweenable target
impl Default for Tween {
    fn default() -> Self {
        Tween {
            delay_s: 0.0,
            duration_s: 0.0,
            progress_s: 0.0,
            // props: Vec::new(),
            propsMap: HashMap::new(),
        }
    }
}

impl Tween {

    /// Execute all functions in the queue
    pub fn play(self) {
        let mut animator = Animator::default();

        // for each queued prop, construct animators that have the start and end state.
        for prop in self.propsMap.values() {

            // animator.start = Transition::From(prop);
            // animator.end = Transition::To(prop);
            // animator.current = Transition::State(prop);
        }
    }

    pub fn duration(mut self, _seconds: f64) -> Self {
        self.duration_s = _seconds;
        self
    }

    pub fn to(mut self, _props: Vec<Prop>) -> Self {
        for prop in _props {
            self.propsMap.insert(prop.prop_id(), prop);
        }
        self
    }

    fn build_animators() {

    }
}

pub fn position(x: f64, y: f64) -> Prop {
    Prop::Position(Frame2D::new(x, y))
}

pub fn move_x(v: f64) -> Prop {
    println!("Move x {}", v);
    Prop::Position(Frame2D::new(v, 0.0))
}

pub fn move_y(v: f64) -> Prop {
    println!("Move y {}", v);
    Prop::Position(Frame2D::new(0.0, v))
}


// #####################################################################################

pub trait Tweenable {
    // fn lerp(t: f64, end: Self) -> Self;
    // fn distance_to(other: Self) -> f64;
    fn get_key(&self) -> TypeId;
    fn get_prop(&self, prop: &Prop) -> Prop;
    fn apply(&mut self, prop: &Prop);
}

impl Tweenable for orbrender::render_objects::Rectangle {

    fn get_key(&self) -> TypeId { TypeId::of::<orbrender::render_objects::Rectangle>() }

    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Position(pos) => { self.position.x = pos[0]; self.position.y = pos[1] },
            _ => ()
        }
    }

    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Alpha(_) => { Prop::Alpha(1.0) },
            Prop::Color(_) => {
                if let Some(bgcolor) = self.background {
                    return Prop::Color(ColorRGBA::new(0, 0, 0, 0));
                } else {
                    return Prop::Color(ColorRGBA::new(0, 0, 0, 0));
                }
            },
            _ => Prop::None,
        }
        // Prop::None()
    }

}

