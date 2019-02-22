/// This is the core Tween model and functions.
extern crate ggez;
extern crate uuid;

use cgmath::*;
use std::{collections::HashSet};
use std::{time::{Duration,Instant}};
use std::hash::{Hash, Hasher};
use uuid::Uuid;

use super::property::*;
use super::animator::*;
use super::ease::*;
use super::tweek::*;

//-- Helpers -----------------------------------------------------------------------

pub fn position(x: f64, y: f64) -> Prop {
    Prop::Position(Point2D::new(x, y))
}

pub fn shift_x(x: f64) -> Prop {
    Prop::Shift(Point2D::new(x, 0.0))
}

pub fn shift_y(y: f64) -> Prop {
    Prop::Shift(Point2D::new(0.0, y))
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

/// The TweenState represents the animation state machine.
#[derive(PartialEq)]
pub enum TweenState {
    Pending, // the initial state before play begins
    Running, // when play starts
    Idle,    // after play has completed and waiting for next instruction
    Cancelled,  // not in use
    Finishing,  // final state that allows one last update call to deliver tween end_state props
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
    pub play_count: u32,
    pub repeat_count: i32, // -1 = forever. If > 0, decrement after each play until 0
    pub repeat_delay: Duration,
	pub loop_forever: bool,
    pub time_scale: f64,
    pub anim_type: AnimType,
    start_props: Vec<Prop>,
    animators: Vec<Animator>,
    callbacks: Vec<Box<FnMut(TKEvent, &mut TKState) + 'static>>, // Don't need this.
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
            state: TweenState::Pending,
            play_count: 0,
            repeat_count: 0,
            repeat_delay: Duration::from_secs(0),
            loop_forever: false,
            time_scale: 1.0,
            anim_type: AnimType::Normal,
            start_props: Vec::new(),
            animators: Vec::new(),
            callbacks: Vec::new(),
        }
    }

    pub fn add_callback<C>(&mut self, cb: C) where C: FnMut(TKEvent, &mut TKState) + 'static {
        self.callbacks.push(Box::new(cb));
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

    pub fn duration(mut self, secs: f64) -> Self {
        // this gets recalculated on play() so the logic isn't too important
        if self.animators.len() > 0 {
            if let Some(animator) = self.animators.last_mut() {
                animator.seconds = secs;
            }
        }
        let mut time = 0.0 as f64;
        // If there are sequenced animators, set the start and end times
        // so the time ranges can be evaluated when getting updates
        for animator in &mut self.animators {
            animator.start_time = time;
            animator.end_time = animator.start_time + animator.seconds;
            time += animator.seconds;
        }
        self.duration = Duration::from_float_secs(time);

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

    pub fn ease(mut self, ease: Ease) -> Self {
        if self.animators.len() > 0 {
            if let Some(animator) = self.animators.last_mut() {
                animator.ease = ease;
            }
        }
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
        // TODO: if repeat_count is forever, this will force it to 1.
        if self.repeat_count < 1 { self.repeat_count = 1 }
        self
    }

    // TODO: move this to Playable
    // TODO: self.duration should be accurate. Use that instead.
    pub fn total_time(&self) -> f64 {
        let mut time = 0.0 as f64;
        for animator in &self.animators {
            time += animator.seconds;
        }

        // If no limit, then only calculate one loop
        if self.repeat_count < 1 {
            return time + self.delay_s.as_float_secs();
        }

        let total = time + self.delay_s.as_float_secs() +
             (self.repeat_count as f64) * (time + self.repeat_delay.as_float_secs());

        total
    }

    /// When this is called from a parent Timeline, it needs...
    /// When playing in reverse, the time_scale is < 0, so the playhead needs to be opposite
    pub fn update(&mut self) -> Option<UIState> {
        match self.state {
            TweenState::Running => {
                let elapsed = self.started_at.elapsed().as_float_secs();
                let total_seconds = self.duration.as_float_secs();
                // println!("duration={:?} // elapsed={}", self.duration, elapsed);
                for animator in &mut self.animators {
                    if self.time_scale > 0.0 {
                        if animator.start_time < elapsed && animator.end_time >= elapsed {
                            let playhead = elapsed - animator.start_time;
                            let ui_state = animator.update(playhead, self.time_scale);
                            return Some(ui_state);
                        }
                    } else {
                        // Calculate elapsed in reverse direction by subtracting from total seconds
                        let elapsed = total_seconds - elapsed;
                        if animator.start_time < elapsed && animator.end_time >= elapsed {
                            // Calculate playhead in reverse by subtracting elapsed from animator end_time
                            let playhead = animator.end_time - elapsed;
                            let ui_state = animator.update(playhead, self.time_scale);
                            return Some(ui_state);
                        }
                    }
                }
            },
            TweenState::Finishing => {
                if let Some(animator) = self.animators.last_mut() {
                    self.state = TweenState::Completed;
                    if self.time_scale >= 0.0 {
                        return Some(animator.end_state.clone());
                    } else {
                        return Some(animator.start_state.clone());
                    }
                }
            }
            _ => ()
        }
        // if self.state == TweenState::Running {
        //     // For now, this assumes that animators do not overlap and are purely sequential
        //     for animator in &mut self.animators {
        //         let elapsed = self.started_at.elapsed().as_float_secs();
        //         if animator.start_time < elapsed && animator.end_time >= elapsed {
        //             let ui_state = animator.update(self.started_at, self.time_scale);
        //             return Some(ui_state);
        //         }
        //     }
        // }
        None
    }

    /// Function which reads the list of "to" props and finds the matching ones
    /// already saved in self.start_props to make sure that start_props and
    /// end_props have matching Prop types in the same order.
    pub fn to(mut self, props:Vec<Prop>) -> Self {

        // Some of the props may include offset types like Shift. These need to be separated
        // from the basic props
        let mut cleaned_props: Vec<Prop> = Vec::new();
        let mut sum_shift = Point2D::zero();

        for prop in &props {
            match prop {
                Prop::Shift(v2) => {
                    sum_shift += v2.clone();
                },
                _ => {
                    cleaned_props.push(prop.clone());
                },
            }
        }
        if sum_shift != Point2D::zero() {
            println!(">>>> Add prop: sum_shift={:?}", sum_shift);
            cleaned_props.push(Prop::Shift(sum_shift));
        }

        let animator = Animator::create(&self.tween_id, &self.start_props, &cleaned_props);
        self.animators.push(animator);
        self
    }


    /// An awkward but necessary function to align all the start and end props in
    /// all animators so that they have matching props in the same order.
    /// This is necessary to support the chaining of to() functions to arbitrarily
    /// create a sequence of animations in a single Tween
    fn sync_animators(&mut self) {

        let mut keep_prop_ids: HashSet<u32> = HashSet::new();
        let mut begin_props: Vec<Prop> = Vec::new();
        let mut end_props: Vec<Prop> = Vec::new();

        if let Some(first) = &self.animators.first_mut() {
            begin_props = first.start_state.props.clone();
        }

        println!("[{}] -------------------------------------------------------------------------------", self.tween_id);
        for animator in &mut self.animators {
            if !&end_props.is_empty() {
                animator.start_state.props = end_props.clone();
            }
            &end_props.clear();

            // Step 1:
            // start_state.props should always have the full list of available props.
            // Use this list to populate end_props in this priority order:
            // a) Find exact match in end_state.props and insert into end_props
            // b) Find offset matches (like Shift) in end_state.props and insert into end_props
            // c) Copy unchanged prop from start_state.props to end_state.props

            for begin_prop in begin_props {
                let mut iter = animator.end_state.props.iter_mut().filter(|x| x.prop_id() == begin_prop.prop_id());
                if let Some(end_prop) = iter.next() {
                    end_props.push(end_prop.clone());
                    keep_prop_ids.insert(begin_prop.prop_id());
                } else {
                    let mut iter = animator.end_state.props.iter_mut()
                        .filter(|x| x.lookup_parent_prop().prop_id() == begin_prop.prop_id());
                    if let Some(end_prop) = iter.next() {
                        match end_prop {
                            Prop::Shift(offset) => {
                                // calculate offset from begin_prop
                                match begin_prop {
                                    Prop::Position(pos) => {
                                        let sum_vec = pos.clone() + offset.clone();
                                        let sum_prop = Prop::Position(sum_vec);
                                        println!(">>>> Inserting sum_prop={:?}", sum_prop);
                                        end_props.push(sum_prop);
                                        keep_prop_ids.insert(begin_prop.prop_id());
                                    },
                                    _ => (),
                                }
                            },
                            _ => (),
                        }
                    } else {
                        end_props.push(begin_prop.clone());
                    }
                }

            }

            // Now, the end_props should be matching list of against start_state.props.
            // Set end_state.props with end_props
            // At this point, this dataset will work fine, but it includes props that have not
            // been mutated over time.
            animator.end_state.props = end_props.clone();
            begin_props = end_props.clone();
            // println!("start={:?} \nend={:?}", &animator.start_state.props, &animator.end_state.props);
        }

        // Step 2:
        // Iterate through animators again and only include props that were mutated
        for animator in &mut self.animators {
            // TODO: Implement FromIterator or whatever to make this simple
            let mut iter = animator.start_state.props.iter_mut().filter(|x| keep_prop_ids.contains(&x.prop_id()));
            let mut start_props: Vec<Prop> = Vec::new();
            while let Some(prop) = iter.next() {
                start_props.push(prop.clone());
            }
            animator.start_state.props = start_props;

            let mut iter = animator.end_state.props.iter_mut().filter(|x| keep_prop_ids.contains(&x.prop_id()));
            let mut end_props: Vec<Prop> = Vec::new();
            while let Some(prop) = iter.next() {
                end_props.push(prop.clone());
            }
            animator.end_state.props = end_props;
            println!("start: {:?} \nend   : {:?}", &animator.start_state.props, &animator.end_state.props);
        }

    }
}

impl Playable for Tween {

    fn play(&mut self) {
        println!("Play?");
        if self.state == TweenState::Pending {
            self.sync_animators();
        }
        // self.print_timeline();

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
                    self.play_count += 1;
                    if self.play_count > self.repeat_count as u32 {
                        // If repeat_count is zero, tween is Completed.
                        self.state = TweenState::Finishing;
                        events.push(TKEvent::Completed(self.tween_id));

                    } else {
                        // set state=Idle means wait for repeat_delay to finish
                        self.state = TweenState::Idle;
                    }
                }
            },
            TweenState::Idle => {
                // If repeat_delay > 0, tween should wait until time elapsed passes it
                if self.started_at.elapsed() > self.duration + self.repeat_delay
                {
                    if self.repeat_count < 0 {
                        self.reset();
                    } else if self.play_count <= self.repeat_count as u32 {
                        self.reset();
                    } else {
                        self.state = TweenState::Completed;
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

    fn stop(&mut self) {

    }

    fn pause(&mut self) {

    }

    /// Reset is used to move the playhead back to the start and set state to Running
    fn reset(&mut self) {
        println!("Reset?");

        if self.anim_type == AnimType::Yoyo {
            // If configured as yoyo animation, reverse the timescale so that the next play
            // is in reverse.
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

impl Drop for Tween {
    fn drop(&mut self) {
        println!("Dropping: {}", self.tween_id);
    }
}
