/// This is the core Tween model and functions.
use super::animator::*;
use super::ease::*;
use super::property::*;
use super::rgb_from_hex;
use super::state::*;
use crate::events::*;

use cgmath::*;
use std::collections::HashSet;

//-- Prop functions -----------------------------------------------------------------------
/*
The following Prop functions are used as function parameters for the Tween::to method which
takes an array of functions that return a Prop. These serve as animation directives for the
specified Tweenable object.
 */

/// Set the position to x, y
pub fn position(x: f32, y: f32) -> Prop {
    Prop::Position(Point2D::new(x, y))
}

/// Shift the position by x, y
pub fn shift(x: f32, y: f32) -> Prop {
    Prop::Shift(Point2D::new(x, y))
}

/// Shift the position by x
pub fn shift_x(x: f32) -> Prop {
    Prop::Shift(Point2D::new(x, 0.0))
}

/// Shift the position by y
pub fn shift_y(y: f32) -> Prop {
    Prop::Shift(Point2D::new(0.0, y))
}

/// Set the size to w x h
pub fn size(w: f32, h: f32) -> Prop {
    Prop::Size(Frame2D::new(w, h))
}

/// This method increases or decreases the size of the object by the specified amounts.
/// Hence, it is an offset Prop like Prop::Shift
/// FIXME: Unused / untested
pub fn resize_by(w: f32, h: f32) -> Prop {
    Prop::Resize(Frame2D::new(w, h))
}

/// Change the alpha transparency (range 0.0..1.0). This will get merged with the Color
/// during Tween creation.
pub fn alpha(v: f32) -> Prop {
    Prop::Alpha(FloatProp::new(v))
}

/// Change the color to the specifed hex color (e.g 0xFFFFFF)
pub fn color(hex: &str) -> Prop {
    let rgb = rgb_from_hex(hex);
    Prop::Color(ColorRGBA::new(rgb.0, rgb.1, rgb.2, rgb.3))
}

/// Change the tint to the specifed hex color (e.g 0xFFFFFF)
pub fn tint(hex: &str) -> Prop {
    let rgb = rgb_from_hex(hex);
    Prop::Tint(ColorRGBA::new(rgb.0, rgb.1, rgb.2, rgb.3))
}

/// Rotate the object to the specified degrees (range 0.0..360.0)
pub fn rotate(degrees: f32) -> Prop {
    Prop::Rotate(FloatProp::new(degrees))
}

/// TODO: Move object along arc path by specified degrees, where 360 means full circle.
pub fn arc(_center_x: f32, _center_y: f32, _radius: f32, degrees: f32) -> Prop {
    Prop::Rotate(FloatProp::new(degrees))
}

/// Display a border for the object with specified width and color
pub fn border(hex: &str, width: f32, alpha: f32) -> Prop {
    let rgb = rgb_from_hex(hex);
    Prop::Border(Some(ColorRGBA::new(rgb.0, rgb.1, rgb.2, alpha)), FloatProp::new(width))
}

//-- Base -----------------------------------------------------------------------

/// The Tweenable trait defines what objects can be animated with Tween. The simple ability to
/// get or set UI properties allows the Tweek UI to animate objects by reading and writing values
/// that are displayed. The Tween code operates as a state machine within the specified timeframe(s)
/// to change the position and appearance of objects within a time duration.
pub trait Tweenable {
    /// Read the specified property from the graphics model
    fn get_prop(&self, prop: &Prop) -> Prop;
    /// Update the internal Transition property during animation. This is different from apply(), which
    /// is used to update the final properties of the layer after animation is complete.
    fn update_prop(&mut self, prop: &Prop);
    /// Update the specified props
    fn update_props(&mut self, props: &[Prop]) {
        for prop in props {
            self.update_prop(prop);
        }
    }
    /// Write the specified property to the graphics model
    fn apply(&mut self, prop: &Prop);
    /// Apply a bunch of Props
    fn apply_props(&mut self, props: &[Prop]) {
        for prop in props {
            self.apply(prop);
        }
    }
    fn init_props(&mut self);
    fn save_props(&mut self);
}

/// Enum type to define whether an animation plays start-to-finish only or
/// plays in reverse finish-to-start after the first run.
#[derive(PartialEq, Clone, Debug)]
pub enum AnimType {
    /// Plays once, start-to-finish
    Normal,
    /// Plays start-to-finish and reverses back to start
    Yoyo,
}

//-- Main -----------------------------------------------------------------------

/// A Tween represents a group of animation Props that will be applied to the set of animators.
/// Only one duration timeline exists for all animators.
/// An AnimationState enum controls what animation can happen.
/// FIXME: Determine which fields can be private
#[derive(Clone, Debug)]
pub struct Tween {
    /// User defined number that can be used for debug purposes or matching a Tween with an object
    pub tween_id: u32,
    /// Time delay in seconds before starting play
    pub delay_s: f64,
    /// Epoch time in seconds when play started
    pub started_at: f64,
    /// Duration in seconds
    pub duration: f64,
    /// Current running state of the Tween
    pub state: PlayState,
    /// Number of plays completed. Used by repeat_count to determine when finished
    pub play_count: u32,
    /// Number of times to repeat the animation, where -1 = forever
    pub repeat_count: u32,
    /// Time delay in seconds before starting next repeat play
    pub repeat_delay: f64,
    /// FIXME: Unused
    pub loop_forever: bool,
    /// Time adjustment ratio to speed up or slow down animation speed. Mainly useful for previewing.
    pub time_scale: f32,
    /// Defines Normal or Yoyo animation type. Other types possible later.
    pub anim_type: AnimType,
    /// Should log debug information
    pub debug: bool,
    /// The starting properties of the object which is used for resets and reverse playback
    start_props: Vec<Prop>,
    /// The collection of animations for the object, which are generally sequential
    animators: Vec<Animator>,
}

impl Tween {
    /// Constructor
    pub fn new() -> Self {
        Tween {
            tween_id: 0,
            delay_s: 0.0,
            started_at: 0.0,
            duration: 0.0,
            state: PlayState::Waiting,
            play_count: 0,
            repeat_count: 0,
            repeat_delay: 0.0,
            loop_forever: false,
            time_scale: 1.0,
            anim_type: AnimType::Normal,
            debug: false,
            start_props: Vec::new(),
            animators: Vec::new(),
        }
    }

    /// Function to initialize a Tween with the vector of Tweenables
    /// The starting state of all Props are stored
    pub fn with(id: u32, tweenable: &dyn Tweenable) -> Self {
        let mut tween = Tween::new();
        tween.tween_id = id;
        let prop_list = Prop::get_prop_list();

        for prop in prop_list {
            let start_prop = tweenable.get_prop(&prop);
            // log::debug!("{}/ Start prop: {:?}", id, start_prop);
            match start_prop {
                Prop::None => {}
                _ => {
                    tween.start_props.push(start_prop);
                }
            }
        }
        tween
    }

    /// Builder method to apply a PropSet to this Tween
    pub fn using_props(self, propset: PropSet) -> Self {
        self.to(&propset.props).duration(propset.duration).delay(propset.delay).ease(propset.ease)
    }

    /// Static helper method to get the initial props for any Tweenable object
    /// This is particularly useful for GUI structs that implement Displayable trait.
    /// Examples: buttons, list boxes, text fields, etc.
    /// Usage:  Tween::load_props(button.layer);
    pub fn load_props(tweenable: &dyn Tweenable) -> Vec<Prop> {
        let mut results: Vec<Prop> = Vec::new();
        let prop_list = Prop::get_prop_list();
        for prop in prop_list {
            let start_prop = tweenable.get_prop(&prop);
            match start_prop {
                Prop::None => {}
                _ => {
                    results.push(start_prop);
                }
            }
        }
        results
    }

    /// The duration function can be chained after target Props have been added using
    /// the to() function. If duration is not specified, the Animator struct defaults
    /// to 1.0 seconds. Multiple animation segments in a single tween can be created by
    /// calling the to() function more than once. In this scenario, you can also call
    /// duration() after each to() call and this will set the duration for the last
    /// animation created.
    pub fn duration(mut self, secs: f64) -> Self {
        if self.animators.is_empty() {
            log::warn!("No animators created yet. Use the to() function first to add Props");
            return self;
        }
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
            animator.end_time = animator.start_time + animator.seconds as f64;
            time += animator.seconds as f64;
        }

        self
    }

    /// Builder method to define the start delay for the animation
    /// FIXME: Not used yet
    pub fn delay(mut self, seconds: f64) -> Self {
        self.delay_s = seconds;
        self
    }

    /// Builder method to define the repeat_count and delay
    pub fn repeat(mut self, count: u32, delay: f64) -> Self {
        self.repeat_count = count;
        self.repeat_delay = delay;
        self
    }

    /// Builder method to define the Ease type for the Animators.
    /// The default is Linear
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
    pub fn speed(mut self, scale: f32) -> Self {
        // prevent negative number for now
        self.time_scale = scale.abs();
        self
    }

    /// Run the animation to the end and reverses direction.
    /// Each playback in either direction counts as one play_count.
    pub fn yoyo(mut self) -> Self {
        self.anim_type = AnimType::Yoyo;
        // If repeat_count is zero, need to increase it to 1. TBD
        // FIXME: yoyo is currently broken and will be addressed later
        if self.repeat_count == 0 {
            self.repeat_count = 1
        }
        self
    }

    /// Calculate total playback time of all animators, which assumes sequential playback
    /// TODO: move this to Playable
    /// TODO: self.duration should be accurate. Use that instead?
    pub fn total_time(&self) -> f64 {
        let mut time = 0.0 as f64;
        for animator in &self.animators {
            time += animator.seconds;
        }

        // If infinite repeat_count, then only calculate one loop
        if self.repeat_count == u32::max_value() {
            return time + self.delay_s;
        }
        let total = time + self.delay_s + (self.repeat_count as f64) * (time + self.repeat_delay);
        total
    }

    pub fn get_runtime(&self) -> f64 {
        let mut time = self.delay_s;
        for animator in &self.animators {
            time += animator.seconds;
        }
        time
    }

    /// Function which reads the list of "to" props and finds the matching ones
    /// already saved in self.start_props to make sure that start_props and
    /// end_props have matching Prop types in the same order.
    pub fn to(mut self, props: &[Prop]) -> Self {
        // Some of the props may include offset types like Shift. These need to be separated
        // from the basic props
        let mut cleaned_props: Vec<Prop> = Vec::new();
        let mut sum_shift = Point2D::zero();
        let mut sum_resize = Frame2D::zero();

        for prop in props {
            match prop {
                Prop::Shift(v2) => {
                    sum_shift += v2.clone();
                }
                Prop::Resize(v2) => {
                    sum_resize += v2.clone();
                }
                _ => {
                    cleaned_props.push(prop.clone());
                }
            }
        }
        if sum_resize != Frame2D::zero() {
            log::debug!(">>>> Add prop: sum_resize={:?}", sum_resize);
            cleaned_props.push(Prop::Resize(sum_resize));
            // TODO: Shift left-top negative to keep it centered.
        }
        if sum_shift != Point2D::zero() {
            log::debug!(">>>> Add prop: sum_shift={:?}", sum_shift);
            cleaned_props.push(Prop::Shift(sum_shift));
        }

        let animator =
            Animator::create(&(self.tween_id as usize, self.animators.len()), &self.start_props, &cleaned_props);
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
                    let mut iter = animator
                        .end_state
                        .props
                        .iter_mut()
                        .filter(|x| x.lookup_parent_prop().prop_id() == begin_prop.prop_id());
                    if let Some(end_prop) = iter.next() {
                        match end_prop {
                            Prop::Shift(offset) => {
                                // calculate offset from begin_prop
                                match begin_prop {
                                    Prop::Position(pos) => {
                                        let sum_vec = pos.clone() + offset.clone();
                                        let sum_prop = Prop::Position(sum_vec);
                                        // log::trace!(">>>> Inserting sum_prop={:?}", sum_prop);
                                        end_props.push(sum_prop);
                                        keep_prop_ids.insert(begin_prop.prop_id());
                                    }
                                    _ => (),
                                }
                            }
                            Prop::Resize(offset) => {
                                // calculate offset from begin_prop
                                match begin_prop {
                                    Prop::Size(size) => {
                                        let sum_vec = size.clone() + offset.clone();
                                        let sum_prop = Prop::Size(sum_vec);
                                        // log::trace!(">>>> Inserting sum_prop={:?}", sum_prop);
                                        end_props.push(sum_prop);
                                        keep_prop_ids.insert(begin_prop.prop_id());
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                    } else {
                        end_props.push(begin_prop.clone());
                    }
                }
            }

            // Now, the end_props should be matching the list of start_state.props, so we can save that.
            // And then overwrite begin_props with end_props for the next loop.
            animator.end_state.props = end_props.clone();
            begin_props = end_props.clone();
        }

        // Step 2:
        // At this point, this dataset will work fine, but it includes props that have not
        // been mutated over time.  So, this next step trims down the props list to only the
        // props that have been mutated across all animators.
        for animator in &mut self.animators {
            // TODO: Implement FromIterator or whatever to make this less complex.
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
            if self.debug {
                log::debug!("Tween: [{}] start = {:?}", self.tween_id, &animator.start_state.props);
                log::debug!("Tween: [{}] end   = {:?}", self.tween_id, &animator.end_state.props);
            }
        }
    }

    /// Helper for debug output of Tween start props
    pub fn get_start_props(&self) -> Vec<Prop> {
        if let Some(animator) = self.animators.first() {
            return animator.start_state.props.clone();
        }
        Vec::new()
    }

    /// Helper for debug output of Tween end props
    pub fn get_end_props(&self) -> Vec<Prop> {
        if let Some(animator) = self.animators.last() {
            return animator.end_state.props.clone();
        }
        Vec::new()
    }
}

impl Playable for Tween {
    fn play(&mut self) {
        match self.state {
            PlayState::Waiting => {
                self.state = PlayState::Pending;
            }
            _ => (),
        }
    }

    fn stop(&mut self) {}

    fn pause(&mut self) {}

    /// Reset is used to move the playhead back to the start and set state to Running
    fn reset(&mut self) {
        if self.anim_type == AnimType::Yoyo {
            // If configured as yoyo animation, reverse the timescale so that the next play
            // is in reverse.
            if self.time_scale > 0.0 {
                self.time_scale *= -1.0;
            } else {
                self.time_scale = self.time_scale.abs();
            }
        }
        self.state = PlayState::Waiting;
    }
}

impl NotifyDispatcher for Tween {
    type Update = PropSet;
    type Params = f64; // Not in use yet, but possibly use this to request PropSet for specific time in seconds

    /// This replaces the tick() method which was used to tell Tween to check if it's state is changing based on the
    /// time elapsed. The Layer expects to receive notifications when state changes to PlayState::Starting
    fn status(&mut self, notifier: &mut Notifier, params: Box<Self::Params>) {
        let duration = self.get_runtime();
        let current = *params;
        let elapsed = current - self.started_at;

        match self.state {
            PlayState::Pending => {
                if elapsed > self.delay_s {
                    self.state = PlayState::Starting;
                }
            }
            PlayState::Starting => {
                notifier.notify(TweenEvent::Started);
                self.sync_animators();
                self.started_at = current;
                self.state = PlayState::Running;
            }
            PlayState::Running => {
                if elapsed > duration {
                    self.play_count += 1;
                    if self.play_count > self.repeat_count {
                        // If repeat_count is zero, tween is Completed.
                        self.state = PlayState::Finishing;
                    } else {
                        // set state=Idle means wait for repeat_delay to finish
                        self.state = PlayState::Idle;
                    }
                }
            }
            PlayState::Idle => {
                // If repeat_delay > 0, tween should wait until time elapsed passes it
                if elapsed > (duration + self.repeat_delay) as f64 {
                    // if self.debug {
                    log::trace!("repeats={:?} plays={:?}", self.repeat_count, self.play_count);
                    // }
                    if self.play_count < self.repeat_count {
                        notifier.notify(TweenEvent::Restarting);
                        self.state = PlayState::Pending;
                    } else {
                        self.state = PlayState::Completed;
                    }
                }
            }
            _ => (),
        }
    }

    /// This call requests a PropSet response about the current Props that are animating so that the parent Layer can
    /// update the display. The Layer expects to receive notifications when state changes to PlayState::Completed
    fn request_update(&mut self, notifier: &mut Notifier, params: Box<Self::Params>) -> Option<Box<Self::Update>> {
        let current = *params;
        match self.state {
            PlayState::Running => {
                let elapsed = current - self.started_at;
                let total_seconds = self.total_time();
                for animator in &mut self.animators {
                    if self.time_scale > 0.0 {
                        if animator.start_time < elapsed && animator.end_time >= elapsed {
                            let playhead = elapsed - animator.start_time;
                            let ui_state = animator.update(playhead, self.time_scale as f64);
                            // TODO: log event
                            if self.debug {
                                log::trace!(
                                    "request_update [{}.{}]  >>  {:?}",
                                    animator.id.0,
                                    animator.id.1,
                                    ui_state.props
                                );
                            }
                            return Some(Box::new(ui_state));
                        }
                    } else {
                        // Calculate elapsed in reverse direction by subtracting from total seconds
                        let elapsed = total_seconds - elapsed;
                        if animator.start_time < elapsed && animator.end_time >= elapsed {
                            // Calculate playhead in reverse by subtracting elapsed from animator end_time
                            let playhead = animator.end_time - elapsed;
                            let ui_state = animator.update(playhead, self.time_scale as f64);
                            return Some(Box::new(ui_state));
                        }
                    }
                }
            }
            PlayState::Finishing => {
                // FIXME: Getting the last animator does not mean it is always the last one to finish
                log::trace!("request_update {:?}", self.state);
                if let Some(animator) = self.animators.last_mut() {
                    self.state = PlayState::Completed;
                    // if self.debug {
                    // }
                    // ======== Notify Completed =======
                    notifier.notify(TweenEvent::Completed);

                    if self.time_scale >= 0.0 {
                        return Some(Box::new(animator.end_state.clone()));
                    } else {
                        return Some(Box::new(animator.start_state.clone()));
                    }
                }
            }
            _ => (),
        }
        None
    }
}

//-- Support -----------------------------------------------------------------------

impl Drop for Tween {
    fn drop(&mut self) {
        if self.debug {
            log::trace!("Dropping: {}", self.tween_id);
        }
    }
}
