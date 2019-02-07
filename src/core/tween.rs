/// This is the core Tween model and functions.
extern crate ggez;

use std::{collections::HashMap};
use super::property::*;
use super::animator::*;
use super::easing::*;

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
    start_props: Vec<Prop>,
    end_props: Vec<Prop>,
    animators: HashMap<usize, Animator>,
    tween_id: usize,
    easing: Easing,
}

/// This is a sad Clone hack to get partial copy of a Tween.
/// Some fields are reset to empty. This is meant to be used for re-use of a Tween
/// because of borrowing issues
impl Clone for Tween {
    fn clone(&self) -> Self {
        Tween {
            state: AnimState::Idle,
            delay_s: 0.0,
            duration_s: 0.0,
            start_props: self.start_props.clone(),
            end_props: self.end_props.clone(),
            animators: HashMap::new(),
            tween_id: 0,
            easing: Easing::Linear,
        }
    }
}

/// This is necessary for Orbrender Window to impose thread safety
unsafe impl Send for Tween {}

impl Tween {

    pub fn new() -> Self {
        Tween {
            state: AnimState::Idle,
            delay_s: 0.0,
            duration_s: 0.0,
            start_props: Vec::new(),
            end_props: Vec::new(),
            animators: HashMap::new(),
            tween_id: 0,
            easing: Easing::Linear,
        }
    }

    /// Optional function to manually set the tween_id for a single object
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

    pub fn duration(mut self, _seconds: f64) -> Self {
        self.duration_s = _seconds;
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
    pub fn play(&mut self) -> usize {
        if self.tween_id == 0 {
            self.tween_id = self.animators.len();
        }
        println!("start={:?} end={:?}", &self.start_props, &self.end_props);
        let animator = Animator::create(self.tween_id, &self.start_props, &self.end_props, &self.duration_s, &self.easing);

        self.animators.insert(self.tween_id, animator);
        self.state = AnimState::Running;
        self.tween_id
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

    pub fn update_item(&self, id: &usize) -> Option<UIState> {
        if let Some(animator) = self.animators.get(id) {
            let ui_state = animator.update();
            return Some(ui_state);
        }
        None
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

// #####################################################################################

pub trait Tweenable {
    fn get_prop(&self, prop: &Prop) -> Prop;
    fn apply(&mut self, prop: &Prop);
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

