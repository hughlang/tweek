/// A Timeline represents a group of Tween animations that each have a start and stop time in seconds
/// in the overall timeline.
extern crate ggez;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use super::property::*;
use super::tweek::*;
use super::tween::*;

//-- Base -----------------------------------------------------------------------

pub struct TweenRange {
    tween: TweenRef,
    pub start: f64, // The start time in float seconds
    pub end: f64,   // The end time in float seconds
    pub state: TweenState,
}

impl TweenRange {
    fn new(tween: Tween, start: f64) -> Self {
        let end = start + &tween.total_time();
        TweenRange { tween: Rc::new(RefCell::new(tween)), start: start, end: end, state: TweenState::Pending }
    }
}

//-- Main -----------------------------------------------------------------------

/// This struct provides a way of coordinating multiple Tweens for playback,
/// either starting at the same time or sequentially.
/// See also: https://greensock.com/asdocs/com/greensock/TimelineLite.html
pub struct Timeline {
    children: HashMap<usize, TweenRange>,
    tween_ids: Vec<usize>,
    tl_start: Instant,
    pub repeat_count: u32,
    pub repeat_delay: Duration,
    pub loop_forever: bool,
}

impl Timeline {
    pub fn new() -> Self {
        Timeline {
            children: HashMap::new(),
            tween_ids: Vec::new(),
            tl_start: Instant::now(),
            repeat_count: 0,
            repeat_delay: Duration::from_secs(0),
            loop_forever: false,
        }
    }

    pub fn add(tweens: Vec<Tween>) -> Self {
        let mut timeline = Timeline::new();
        let start = 0.0 as f64;

        for t in tweens {
            let id = t.tween_id;
            timeline.tween_ids.push(id);
            let range = TweenRange::new(t, start);
            timeline.children.insert(id.clone(), range);
        }
        timeline
    }

    pub fn align(mut self, alignment: TweenAlign) -> Self {
        let mut start = 0.0 as f64;
        for id in &self.tween_ids {
            if let Some(range) = self.children.get_mut(&id) {
                let tween = range.tween.borrow();
                let total_secs = (&*tween).total_time();
                range.start = start;
                range.end = range.start + total_secs;
                log::debug!("align start={} end={}", range.start, range.end);

                match alignment {
                    TweenAlign::Normal => {}
                    TweenAlign::Sequence => {
                        start += total_secs;
                    }
                    _ => (),
                }
            }
        }
        self
    }

    pub fn stagger(mut self, offset: f64) -> Self {
        for (index, id) in self.tween_ids.iter().enumerate() {
            if let Some(range) = self.children.get_mut(&id) {
                let mut tween = range.tween.borrow_mut();
                let total_secs = (&mut *tween).total_time();
                range.start = index as f64 * offset;
                range.end = range.start + total_secs;
                log::debug!("stagger start={} end={}", range.start, range.end);
            }
        }
        self
    }

    pub fn repeat(mut self, count: u32, delay: f64) -> Self {
        self.repeat_count = count;
        self.repeat_delay = Duration::from_secs_f64(delay);
        self
    }

    pub fn notify(&mut self, event: &TKEvent) {
        log::debug!("notify event={:?}", event);
    }

    pub fn total_time(&self) -> f64 {
        let floats: Vec<f64> = self.children.values().map(|x| x.end).collect();
        if let Some(max) = floats.iter().cloned().max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN")) {
            return max;
        }
        0.0
    }
}

impl Playable for Timeline {
    /// The Timeline play method should only play the tweens where the start time
    /// is not greater than the current elapsed time.
    fn play(&mut self) {
        self.tl_start = Instant::now();
        for (id, range) in &self.children {
            let elapsed = self.tl_start.elapsed().as_secs_f64();
            if range.start <= elapsed && range.end > elapsed {
                log::debug!("timeline play id={}", id);
                let mut tween = range.tween.borrow_mut();
                (&mut *tween).play();
                // range.state = TweenState::Running;
            }
        }
    }

    // Deprecate this to a no-op
    #[allow(unused_mut)]
    fn tick(&mut self) -> Vec<TKEvent> {
        log::warn!("Timeline.tick is deprecated");
        Vec::new()
    }

    fn stop(&mut self) {}

    fn pause(&mut self) {}

    fn reset(&mut self) {
        self.tl_start = Instant::now();
        for (_, range) in &self.children {
            let mut tween = range.tween.borrow_mut();
            (&mut *tween).reset();
        }
    }

    fn get_update(&mut self, id: &usize) -> Option<UIState> {
        if let Some(range) = &self.children.get(id) {
            let mut tween = range.tween.borrow_mut();
            return (&mut *tween).get_update(id);
        }
        None
    }
}

#[allow(unused_variables)]
impl TimelineAware for Timeline {
    /// This is called by Tweek.update() which is continuously called from the run loop.
    ///
    /// information updates to TKState
    /// 1. Each tween handles its own repeats and will set its state to Idle just before repeating.
    fn update(&mut self, ctx: &mut TKState) {
        for (_, range) in &self.children {
            //
            let elapsed = self.tl_start.elapsed().as_secs_f64();
            if range.start <= elapsed && range.end > elapsed {
                let mut tween = range.tween.borrow_mut();
                match tween.state {
                    TweenState::Pending => {
                        (&mut *tween).play();
                    }
                    _ => {
                        let mut events = (&mut *tween).tick();
                        ctx.events.append(&mut events);
                    }
                }
            } else {
                let mut tween = range.tween.borrow_mut();
                let mut events = (&mut *tween).tick();
                ctx.events.append(&mut events);
            }
        }
        ctx.elapsed_time = self.tl_start.elapsed().as_secs_f64();
        ctx.total_time = self.total_time();
    }
}

//-- Support -----------------------------------------------------------------------

/**
 * From Greensock AS3:
 * Options are: "sequence" (aligns them one-after-the-other in a sequence), "start"
 * (aligns the start times of all of the objects (ignoring delays)), and "normal"
 * (aligns the start times of all the tweens (honoring delays)). The default is "normal".
 */
pub enum TweenAlign {
    Normal,
    Sequence,
    Start,
}
