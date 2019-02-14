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
use super::tweek::*;

//-- Base -----------------------------------------------------------------------

pub fn position(x: f64, y: f64) -> Prop {
    Prop::Position(Point2D::new(x, y))
}

pub fn alpha(v: f64) -> Prop {
    Prop::Alpha(FloatProp::new(v))
}

pub fn size(w: f64, h: f64) -> Prop {
    Prop::Size(Frame2D::new(w, h))
}

/// The TweenState represents the animation state machine.
#[derive(PartialEq)]
pub enum TweenState {
    Pending,
    Running,
    Idle,
    Cancelled,
    Completed,
}

#[derive(PartialEq)]
pub enum AnimType {
    Normal,
    Yoyo,
}

//-- Main -----------------------------------------------------------------------

/// A Tween represents a group of animation Props that will be applied to the set of animators.
/// Only one duration timeline exists for all animators.
/// An AnimationState enum controls what animation can happen.
pub struct Tween {
    pub tween_id: usize,
    pub global_id: String,
    pub delay_s: Duration,
    pub start_time: Instant,
    pub duration: Duration,
    pub state: TweenState,
    pub repeat_count: i32, // -1 = forever. If > 0, decrement after each play until 0
    pub repeat_delay: Duration,
    pub time_scale: f64,
    pub anim_type: AnimType,
    start_props: Vec<Prop>,
    animators: HashMap<usize, Vec<Animator>>,
    easing: Easing,
    events: Vec<TKEvent>,
    callbacks: Vec<Box<FnMut(TKEvent, &mut TKContext) + 'static>>,
}

impl Tween {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        Tween {
            tween_id: 0,
            global_id: uuid.to_string(),
            delay_s: Duration::from_secs(0),
            start_time: Instant::now(),
            duration: Duration::from_secs(0),
            state: TweenState::Idle,
            repeat_count: 0,
            repeat_delay: Duration::from_secs(0),
            time_scale: 1.0,
            anim_type: AnimType::Normal,
            start_props: Vec::new(),
            animators: HashMap::new(),
            easing: Easing::Linear,
            events: Vec::new(),
            callbacks: Vec::new(),
        }
    }

    /// Function to initialize a Tween with the vector of Tweenables
    /// The starting state of all Props are stored
    pub fn with(id: usize, objects: &Vec<&Tweenable>) -> Self {
        let mut tween = Tween::new();
        tween.tween_id = id;
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

    /// Function which reads the list of "to" props and finds the matching ones
    /// already saved in self.start_props to make sure that start_props and
    /// end_props have matching Prop types in the same order.
    pub fn to(mut self, props:Vec<Prop>) -> Self {
        // let prop_ids: Vec<u32> = props.iter().map(|x| x.prop_id()).collect();
        let mut temp_map: HashMap<u32, Prop> = HashMap::new();
        for prop in &self.start_props {
            temp_map.insert(prop.prop_id(), prop.clone());
        }
        let mut match_props: Vec<Prop> = Vec::new();
        for prop in &props {
            if let Some(start_prop) = temp_map.get(&prop.prop_id()) {
                match_props.push(start_prop.clone());
            }
        }

        // if self.tween_id == 0 {
        //     self.tween_id = self.animators.len();
        // }
        println!("start={:?} \nend={:?}", &match_props, &props);

        let mut animator = Animator::create(&self.tween_id, &match_props, &props, &self.easing);
        animator.debug = true;
        let exists = &self.animators.contains_key(&self.tween_id);
        if *exists {
            &self.animators.get_mut(&self.tween_id).unwrap().push(animator);
            // if let Some(list) = &self.animators.get_mut(&self.tween_id) {
            //     list.push(animator);
            // }
        } else {
            &self.animators.insert(self.tween_id.clone(), vec![animator]);
        }
        self
    }

    pub fn duration(mut self, secs: f64) -> Self {
        self.duration = Duration::from_float_secs(secs);
        self
    }

    pub fn delay(mut self, _seconds: f64) -> Self {
        self.delay_s = Duration::from_float_secs(_seconds);
        self
    }

    pub fn repeat(mut self, count: i32, delay: f64) -> Self {
        self.repeat_count = count;
        self.repeat_delay = Duration::from_float_secs(delay);
        self
    }

    pub fn ease(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Set time_scale which modifies the speed of the animation,
    /// where 1.0 is considered normal time
    pub fn speed(mut self, scale: f64) -> Self {
        // prevent negative number for now
        self.time_scale = scale.abs();
        self
    }

    /// Run the animation to the end and reverse direction
    pub fn yoyo(mut self) -> Self {
        self.anim_type = AnimType::Yoyo;
        if self.repeat_count < 1 { self.repeat_count = 1 }
        self
    }

    pub fn add_callback<C>(&mut self, cb: C) where C: FnMut(TKEvent, &mut TKContext) + 'static {
        self.callbacks.push(Box::new(cb));
    }
}
impl Playable for Tween {

    fn play(&mut self) {
        self.start_time = Instant::now();
        self.state = TweenState::Running;



    }

    /// Probably use this to check the play status of each tween, based on the
    /// timeline, time elapsed, and duration, etc.
    fn tick(&mut self) {
        self.events.clear();
        match self.state {
            TweenState::Running => {
                if self.start_time.elapsed() > self.duration {
                    if self.repeat_count == 0 {
                        // If repeat_count is zero, tween is Completed.
                        self.state = TweenState::Completed;
                        self.events.push(TKEvent::Completed(self.tween_id));

                    } else {
                        // If it positive or negative, continue repeating
                        self.state = TweenState::Idle;
                    }
                }
            },
            TweenState::Idle => {
                if self.start_time.elapsed() > self.duration + self.repeat_delay
                {
                    if self.repeat_count > 0 {
                        self.repeat_count -= 1;
                        self.reset();
                    } else if self.repeat_count < 0 {
                        self.reset();
                    } else {
                        // self.state = TweenState::Completed;
                    }
                }
            },
            _ => (),
        }
    }

    fn stop(&mut self) {

    }

    fn pause(&mut self) {

    }

    fn reset(&mut self) {
        if self.anim_type == AnimType::Yoyo {
            if self.time_scale > 0.0 {
                self.time_scale *= -1.0;
            } else {
                self.time_scale = self.time_scale.abs();
            }
        }
        self.state = TweenState::Running;
        self.start_time = Instant::now();
    }

    fn get_update(&mut self, id: &usize) -> Option<UIState> {
        if self.state == TweenState::Running {
            if let Some(values) = self.animators.get(id) {

                // let ui_state = animator.update(self.start_time, self.duration, self.time_scale);
                // return Some(ui_state);
            }
        }
        None
    }

}

//-- Support -----------------------------------------------------------------------

pub trait Tweenable {
    fn get_prop(&self, prop: &Prop) -> Prop;
    fn apply(&mut self, prop: &Prop);
    fn render_update(&mut self, props: &Vec<Prop>) {
        for prop in props {
            self.apply(prop);
        }
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

