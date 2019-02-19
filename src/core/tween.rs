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

pub fn size(w: f64, h: f64) -> Prop {
    Prop::Size(Frame2D::new(w, h))
}

pub fn alpha(v: f64) -> Prop {
    Prop::Alpha(FloatProp::new(v))
}

pub fn color(c: u32) -> Prop {
    let rp = ((c & 0x00FF_0000u32) >> 16) as f32;
    let gp = ((c & 0x0000_FF00u32) >> 8) as f32;
    let bp = (c & 0x0000_00FFu32) as f32;
    Prop::Color(ColorRGB::new(rp, gp, bp))
}

pub fn rotate(degrees: f64) -> Prop {
    Prop::Rotate(FloatProp::new(degrees.to_radians()))
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
    pub started_at: Instant,
    pub duration: Duration,
    pub state: TweenState,
    pub repeat_count: i32, // -1 = forever. If > 0, decrement after each play until 0
    pub repeat_delay: Duration,
    pub time_scale: f64,
    pub anim_type: AnimType,
    start_props: Vec<Prop>,
    animators: Vec<Animator>,
    easing: Easing,
    callbacks: Vec<Box<FnMut(TKEvent, &mut TKContext) + 'static>>,
}

impl Tween {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        Tween {
            tween_id: 0,
            global_id: uuid.to_string(),
            delay_s: Duration::from_secs(0),
            started_at: Instant::now(),
            duration: Duration::from_secs(0),
            state: TweenState::Idle,
            repeat_count: 0,
            repeat_delay: Duration::from_secs(0),
            time_scale: 1.0,
            anim_type: AnimType::Normal,
            start_props: Vec::new(),
            animators: Vec::new(),
            easing: Easing::Linear,
            callbacks: Vec::new(),
        }
    }

    /// Function to initialize a Tween with the vector of Tweenables
    /// The starting state of all Props are stored
    pub fn with(id: usize, tweenable: &Tweenable) -> Self {
        let mut tween = Tween::new();
        tween.tween_id = id;
        let prop_list = Prop::get_prop_list();

        for prop in prop_list {
            let start_prop = tweenable.get_prop(&prop);
            match start_prop {
                Prop::None => {},
                _ => {
                    tween.start_props.push(start_prop);
                }
            }
        }
        tween
    }

    pub fn init(id: usize, props: Vec<Prop>) -> Self {
        let mut tween = Tween::new();
        tween.tween_id = id;

        for prop in props {
            tween.start_props.push(prop);
        }
        tween
    }

    /// Function which reads the list of "to" props and finds the matching ones
    /// already saved in self.start_props to make sure that start_props and
    /// end_props have matching Prop types in the same order.
    pub fn to(mut self, props:Vec<Prop>) -> Self {
        // let prop_ids: Vec<u32> = props.iter().map(|x| x.prop_id()).collect();

        let mut start_map: HashMap<u32, Prop> = HashMap::new();
        for prop in &self.start_props {
            start_map.insert(prop.prop_id(), prop.clone());
        }
        let mut last_props: Vec<Prop> = Vec::new();
        let mut end_props: Vec<Prop> = Vec::new();

        if self.animators.len() == 0 {
            for prop in &props {
                // technically, it would be a bug if the corresponding prop was not found in start_props.
                if let Some(start_prop) = start_map.get(&prop.prop_id()) {
                    last_props.push(start_prop.clone());
                }
            }

            let animator = Animator::create(&self.tween_id, &last_props, &props, &self.easing);
            self.animators.push(animator);

        } else {
            if let Some(previous) = self.animators.last() {

                let mut all_props: HashMap<u32, Prop> = HashMap::new();
                let mut new_props: Vec<Prop> = Vec::new();

                // Fill hashmap with last props and overwrite matching ones
                for prop in &previous.end_state.props {
                    all_props.insert(prop.prop_id(), prop.clone());
                }
                for prop in &props {
                    all_props.insert(prop.prop_id(), prop.clone());
                }

                for (id, prop) in all_props {
                    let last_prop = previous.end_state.get_prop_value(id);
                    if last_prop != Prop::None {
                        last_props.push(last_prop);
                        end_props.push(prop.clone());
                    } else {
                        if let Some(start_prop) = start_map.get(&id) {
                            last_props.push(start_prop.clone());
                            end_props.push(prop.clone());

                            // A new prop was added. Need to write this value back to all previous animators
                            new_props.push(start_prop.clone());
                        } else {
                            // Ignore
                        }
                    }
                }

                for animator in &mut self.animators {
                    for new_prop in &new_props {
                        animator.start_state.props.push(new_prop.clone());
                        animator.end_state.props.push(new_prop.clone());
                    }
                }
                let animator = Animator::create(&self.tween_id, &last_props, &end_props, &self.easing);
                self.animators.push(animator);
            }
        }
        self
    }

    pub fn duration(mut self, secs: f64) -> Self {
        // this gets recalculated on play() so the logic isn't too important
        self.duration = self.duration + Duration::from_float_secs(secs);
        if self.animators.len() > 0 {
            if let Some(animator) = self.animators.last_mut() {
                animator.seconds = secs;
            }
        }
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

    pub fn total_duration(&self) -> f64 {
        let total = (self.duration + self.repeat_delay).as_float_secs() * (self.repeat_count + 1) as f64;

        total
    }

    pub fn update(&mut self) -> Option<UIState> {
        if self.state == TweenState::Running {
            // For now, this assumes that animators do not overlap and are purely sequential
            for animator in &mut self.animators {
                let elapsed = self.started_at.elapsed().as_float_secs();
                if animator.start_time < elapsed && animator.end_time >= elapsed {
                    let ui_state = animator.update(self.started_at, self.time_scale);
                    return Some(ui_state);
                }
            }
        }
        None
    }
}
impl Playable for Tween {

    fn play(&mut self) {
        let mut time = 0.0 as f64;
        // If there are sequenced animators, set the start and end times
        // so the time ranges can be evaluated when getting updates
        for animator in &mut self.animators {
            animator.start_time = time;
            animator.end_time = animator.start_time + animator.seconds;
            time += animator.seconds;
            println!("start={:?} \nend={:?}", &animator.start_state.props, &animator.end_state.props);

        }
        self.duration = Duration::from_float_secs(time);
        self.started_at = Instant::now();
        self.state = TweenState::Running;
    }

    /// Probably use this to check the play status of each tween, based on the
    /// timeline, time elapsed, and duration, etc.
    fn tick(&mut self) -> Vec<TKEvent> {
        let mut events: Vec<TKEvent> = Vec::new();
        match self.state {
            TweenState::Running => {
                if self.started_at.elapsed() > self.duration {
                    if self.repeat_count == 0 {
                        // If repeat_count is zero, tween is Completed.
                        self.state = TweenState::Completed;
                        events.push(TKEvent::Completed(self.tween_id));

                    } else {
                        // If it positive or negative, continue repeating
                        self.state = TweenState::Idle;
                    }
                }
            },
            TweenState::Idle => {
                if self.started_at.elapsed() > self.duration + self.repeat_delay
                {
                    if self.repeat_count > 0 {
                        self.repeat_count -= 1;
                        self.reset();
                    } else if self.repeat_count < 0 {
                        self.reset();
                    }
                }
            },
            _ => (),
        }
        events
    }

    fn get_update(&mut self, _id: &usize) -> Option<UIState> {
        return self.update();
    }

    fn sync(&mut self, ctx: &mut TKContext) {

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
        self.started_at = Instant::now();
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

