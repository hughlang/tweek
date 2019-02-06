/// This is the core Tween model and functions.
extern crate orbrender;
extern crate ggez;

use crossbeam_channel::*;
use crossbeam_utils::thread;
use std::{collections::HashMap};
use std::time::{Duration, Instant};
// use std::any::Any;

use super::property::*;
use super::animator::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TweenMode {
    To,
    From,
    FromTo,
}

/// A Tween represents a group of animation Props that will be applied to the set of animators.
/// Only one duration timeline exists for all animators.
/// An AnimationState enum controls what animation can happen.
pub struct Tween {
    state: AnimState,
    delay_s: f64,
    duration_s: f64,
    progress_s: f64,
    start_props: Vec<Prop>,
    end_props: Vec<Prop>,
    animators: HashMap<usize, Animator>,
}

/// This is necessary for Orbrender Window to impose thread safety
unsafe impl Send for Tween {}

impl Tween {

    pub fn new() -> Self {
        Tween {
            state: AnimState::Idle,
            delay_s: 0.0,
            duration_s: 0.0,
            progress_s: 0.0,
            start_props: Vec::new(),
            end_props: Vec::new(),
            animators: HashMap::new(),
        }
    }

    pub fn duration(mut self, _seconds: f64) -> Self {
        self.duration_s = _seconds;
        self
    }

    pub fn to(_target: &Tweenable, _props: Vec<Prop>) -> Self {
        let mut tween = Tween::new();
        let mut props_map: HashMap<u32, Prop> = HashMap::new();

        // De-dupe props with a hashmap, just in case.
        for prop in _props {
            props_map.insert(prop.prop_id(), prop.clone());
        }

        for prop in props_map.values() {
            let start_prop = _target.get_prop(&prop);
            match start_prop {
                Prop::None => {},
                _ => {
                    tween.start_props.push(start_prop);
                    tween.end_props.push(prop.clone());
                }
            }
        }
        tween
    }

    /// Called externally. TODO: rename to "to"
    pub fn animate(_target: &Tweenable, _props: Vec<Prop>) -> Self {
        let mut tween = Tween::new();
        let mut props_map: HashMap<u32, Prop> = HashMap::new();

        // De-dupe props with a hashmap, just in case.
        for prop in _props {
            props_map.insert(prop.prop_id(), prop.clone());
        }

        for prop in props_map.values() {
            let start_prop = _target.get_prop(&prop);
            match start_prop {
                Prop::None => {},
                _ => {
                    tween.start_props.push(start_prop);
                    tween.end_props.push(prop.clone());
                }
            }
        }
        tween
    }

    /// Execute all functions in the queue
    pub fn play(&mut self) -> usize {
        let item_id = self.animators.len();
        let animator = Animator::create(item_id, &self.start_props, &self.end_props, &self.duration_s);
        self.animators.insert(item_id, animator);
        self.state = AnimState::Running;
        item_id
    }

    pub fn get_updates(&self) -> Vec<UIState> {
        let mut results: Vec<UIState> = Vec::new();
        match self.state {
            AnimState::Running => {
                for animator in self.animators.values() {
                    let ui_state = animator.update();
                    if ui_state.props.len() > 0 {
                        results.push(ui_state);
                    }
                }
            },
            _ => ()
        }
        results
    }


    pub fn update_all(&self) {
        // For each animator, do stuff
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
    fn get_prop(&self, prop: &Prop) -> Prop;
    fn apply(&mut self, prop: &Prop);
    // fn render_update(&mut self, props: &Vec<Prop>);
    fn render_update(&mut self, props: &Vec<Prop>) {
        for prop in props {
            self.apply(prop);
        }
    }

}

impl Tweenable for ggez::graphics::Rect {

    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Position(pos) => { self.x = pos[0] as f32; self.y = pos[1] as f32 },
            _ => ()
        }
    }

    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Alpha(_) => { Prop::Alpha(1.0) },
            // Prop::Color(_) => {
            //     if let Some(color) = self.background {
            //         return Prop::Color(ColorRGBA::new(color.r_f(), color.g_f(), color.b_f(), color.a_f()));
            //     } else {
            //         return Prop::Color(ColorRGBA::new(0.0, 0.0, 0.0, 0.0));
            //     }
            // },
            Prop::Position(_) => Prop::Position(Point2D::new(self.x as f64, self.y as f64)),
            Prop::Size(_) => Prop::Size(Frame2D::new(self.w as f64, self.h as f64)),
            _ => Prop::None,
        }
    }
}

impl Tweenable for orbrender::render_objects::Rectangle {

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
                if let Some(color) = self.background {
                    return Prop::Color(ColorRGBA::new(color.r_f(), color.g_f(), color.b_f(), color.a_f()));
                } else {
                    return Prop::Color(ColorRGBA::new(0.0, 0.0, 0.0, 0.0));
                }
            },
            Prop::Position(_) => Prop::Position(Point2D::new(self.position.x, self.position.y)),
            Prop::Size(_) => Prop::Size(Frame2D::new(self.size.width, self.size.height)),
            _ => Prop::None,
        }
    }


}

