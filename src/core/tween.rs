/// This is the core Tween model and functions.
extern crate ggez;
extern crate uuid;

use std::{collections::HashMap};
use std::{time::{Duration,Instant}};
use std::hash::{Hash, Hasher};
use uuid::Uuid;

use super::property::*;
use super::animator::*;
use super::easing::*;
use super::timeline::*;
use super::tweek::*;

//-- Base -----------------------------------------------------------------------

pub trait Tweenable {
    fn get_prop(&self, prop: &Prop) -> Prop;
    fn apply(&mut self, prop: &Prop);
    fn render_update(&mut self, props: &Vec<Prop>) {
        for prop in props {
            self.apply(prop);
        }
    }
}

pub fn position(x: f64, y: f64) -> Prop {
    Prop::Position(Point2D::new(x, y))
}

pub fn alpha(v: f64) -> Prop {
    Prop::Alpha(FloatProp::new(v))
}

pub fn size(w: f64, h: f64) -> Prop {
    Prop::Size(Frame2D::new(w, h))
}

//-- Main -----------------------------------------------------------------------

/// A Tween represents a group of animation Props that will be applied to the set of animators.
/// Only one duration timeline exists for all animators.
/// An AnimationState enum controls what animation can happen.
pub struct Tween {
    pub tween_id: usize,
    pub global_id: String,
    pub delay_s: f64,
    pub start_time: Instant,
    pub duration: Duration,
    state: AnimState,
    start_props: Vec<Prop>,
    end_props: Vec<Prop>,
    animators: HashMap<usize, Animator>,
    easing: Easing,
    callbacks: Vec<Box<Fn(TweenEvent, &str) + 'static>>,
}

impl Tween {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        Tween {
            tween_id: 0,
            global_id: uuid.to_string(),
            delay_s: 0.0,
            start_time: Instant::now(),
            duration: Duration::from_secs(0),
            state: AnimState::Idle,
            start_props: Vec::new(),
            end_props: Vec::new(),
            animators: HashMap::new(),
            easing: Easing::Linear,
            callbacks: Vec::new(),
        }
    }

    // /// Optional function to manually set the tween_id for a single object
    pub fn with_id(mut self, id: usize) -> Self {
        self.tween_id = id;
        self
    }

    /// Function to initialize a Tween with the vector of Tweenables
    /// The starting state of all Props are stored
    pub fn with(objects: &Vec<&Tweenable>) -> Self {
        let mut tween = Tween::new();
        let prop_list = Prop::get_prop_list();

        for prop in prop_list {
            for object in objects {
                let start_prop = object.get_prop(&prop);
                match start_prop {
                    Prop::None => {},
                    _ => {
                        tween.start_props.push(start_prop);
                    }
                }
            }
        }
        tween
    }

    /// Function which reads the list of props and finds the matching ones
    /// already saved in self.start_props.
    pub fn to(mut self, props:Vec<Prop>) -> Self {
        // let prop_ids: Vec<u32> = props.iter().map(|x| x.prop_id()).collect();
        let mut temp_map: HashMap<u32, Prop> = HashMap::new();
        for prop in self.start_props {
            temp_map.insert(prop.prop_id(), prop.clone());
        }
        let mut match_props: Vec<Prop> = Vec::new();
        for prop in &props {
            if let Some(start_prop) = temp_map.get(&prop.prop_id()) {
                match_props.push(start_prop.clone());
            }
        }
        self.end_props = props;
        self.start_props = match_props;
        self
    }

    pub fn duration(mut self, secs: f64) -> Self {
        self.duration = Duration::from_float_secs(secs);
        self
    }

    pub fn delay(mut self, _seconds: f64) -> Self {
        self.delay_s = _seconds;
        self
    }

    pub fn ease(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Execute all functions in the queue

    pub fn get_updates(&self) -> Vec<UIState> {
        let mut results: Vec<UIState> = Vec::new();
        match self.state {
            AnimState::Running => {
                for animator in self.animators.values() {
                    let ui_state = animator.update(self.start_time, self.duration);
                    if ui_state.props.len() > 0 {
                        results.push(ui_state);
                    }
                }
            },
            _ => ()
        }
        results
    }

    pub fn update_item(&self, id: &usize) -> Option<UIState> {
        if self.state == AnimState::Running {
            if let Some(animator) = self.animators.get(id) {
                let ui_state = animator.update(self.start_time, self.duration);
                return Some(ui_state);
            }
        }
        None
    }

    pub fn add_callback<C>(&mut self, cb: C) where C: Fn(TweenEvent, &str) + 'static {
        self.callbacks.push(Box::new(cb));
    }
}

impl Hash for Tween {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.global_id.hash(state);
    }
}

impl PartialEq for Tween {
    fn eq(&self, other: &Tween) -> bool {
        self.global_id == other.global_id
    }
}

impl Eq for Tween {}

impl Playable for Tween {

    /// Probably use this to check the play status of each tween, based on the
    /// timeline, time elapsed, and duration, etc.
    fn tick(&mut self) {
        if self.state == AnimState::Running && self.start_time.elapsed() > self.duration {
            self.state = AnimState::Completed;
            for cb in self.callbacks.iter() {
                (&*cb)(TweenEvent::Completed(self.tween_id), &self.global_id);
            }
        }
    }

    fn play(&mut self) {

        for cb in self.callbacks.iter() {
            (&*cb)(TweenEvent::Play(self.tween_id), &self.global_id);
        }

        if self.tween_id == 0 {
            self.tween_id = self.animators.len();
        }
        println!("start={:?} \nend={:?}", &self.start_props, &self.end_props);
        let animator = Animator::create(self.tween_id, &self.start_props, &self.end_props, &self.easing);

        self.animators.insert(self.tween_id, animator);
        self.state = AnimState::Running;
    }

    fn stop(&mut self) {

    }

    fn pause(&mut self) {

    }

    fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}

//-- TODO: Move to separate file -----------------------------------------------------------------------
// The plan is to have ggez support as a feature and would have its own module in this lib

impl Tweenable for ggez::graphics::Rect {
    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Position(pos) => { self.x = pos[0] as f32; self.y = pos[1] as f32 },
            Prop::Size(v) => { self.w = v[0] as f32; self.h = v[1] as f32 },
            _ => ()
        }
    }
    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Position(_) => Prop::Position(Point2D::new(self.x as f64, self.y as f64)),
            Prop::Size(_) => Prop::Size(Frame2D::new(self.w as f64, self.h as f64)),
            _ => Prop::None,
        }
    }
}

/// With ggez, the graphics objects do not contain their own color attributes, so
/// we need to apply tween updates to the Color object separately.
impl Tweenable for ggez::graphics::Color {
    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Alpha(val) => { self.a = val[0] as f32 },
            _ => ()
        }
    }
    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Alpha(_) => { Prop::Alpha(FloatProp::new(self.a as f64)) },
            _ => Prop::None,
        }
    }
}

