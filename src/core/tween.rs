/// This is the core Tween model and functions.
extern crate orbrender;

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

pub struct Tween {
    delay_s: f64,
    duration_s: f64,
    progress_s: f64,
    start_props: Vec<Prop>,
    end_props: Vec<Prop>,
    animators: HashMap<u32, Animator>,
    props_cache: HashMap<usize, Vec<Prop>>,
}

/// This is necessary for Orbrender Window to impose thread safety
unsafe impl Send for Tween {}

impl Tween {

    pub fn new() -> Self {
        Tween {
            delay_s: 0.0,
            duration_s: 0.0,
            progress_s: 0.0,
            start_props: Vec::new(),
            end_props: Vec::new(),
            animators: HashMap::new(),
            props_cache: HashMap::new(),
        }
    }

    pub fn duration(mut self, _seconds: f64) -> Self {
        self.duration_s = _seconds;
        self
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
    pub fn play(&mut self) {
        // for each queued prop, construct animators that have the start and end state.
        // let mut workers:Vec<Animator> = Vec::new();
        let animator = Animator::create(0, &self.start_props, &self.end_props, &self.duration_s);
        self.animators.insert(0, animator);
    }

    /// This should be called by a run loop to tell the animation to update itself
    pub fn update_async(&mut self) {

        let (tx, rx) = bounded::<Vec<Prop>>(1);
        thread::scope(|scope| {
                for animator in self.animators.values() {
                    let (tx, rx) = (tx.clone(), rx.clone());
                    scope.spawn(move |_| {
                        let ui_state = animator.update();
                        tx.send(ui_state.props).unwrap();
                    });
                    // let props = thread.join().unwrap();

                    let _props: Vec<Prop> = rx.recv().unwrap();
                    if _props.len() > 0 {
                        println!("props={:?}", _props);
                        self.props_cache.insert(animator.id, _props);
                        // self.animators.insert(animator.id, _props);
                        // Tween::save_props(animator.id, _props);
                    }
                }
        }).unwrap();

        // for animator in self.animators.values() {
        //     let props = thread::scope(|s| {
        //         let thread = s.spawn(|_| {
        //             animator.update()
        //         });
        //         thread.join().unwrap()
        //     }).unwrap();
        //     self.props_cache.insert(animator.id, props);
        // }

    }

    pub fn get_updates(&self) -> Vec<UIState> {
        let mut results: Vec<UIState> = Vec::new();
        for animator in self.animators.values() {
            let ui_state = animator.update();
            if ui_state.props.len() > 0 {
                results.push(ui_state);
            }
        }
        results
    }

    // pub fn render(&self, _target: &Tweenable, _id: &usize) {
    //     if let Some(props) = self.props_cache.get(_id) {
    //         _target.render(props);
    //     }
    // }
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
    fn get_prop(&self, prop: &Prop) -> Prop;
    fn apply(&mut self, prop: &Prop);
    fn apply_props(&mut self, props: Vec<Prop>) {
        for prop in props {
            self.apply(&prop);
        }
    }
    fn render(&mut self, props: &Vec<Prop>);
    fn render_update(&mut self, props: &Vec<Prop>);
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

    fn render(&mut self, props: &Vec<Prop>) {
        for prop in props {
            self.apply(prop);
        }
    }

    fn render_update(&mut self, props: &Vec<Prop>) {
        for prop in props {
            self.apply(prop);
        }
    }

}

