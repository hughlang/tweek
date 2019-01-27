/// An Animator has start and end properties that can be rendered in an animation
///
///
use std::{time::{Duration,Instant}, collections::HashMap, rc::Rc};
use cgmath::*;

use super::property::*;
use super::tween::*;

#[allow(dead_code)]

pub trait Animatable {

    fn play();
    fn stop();
    fn pause();
    fn resume();
    fn seek();

	// func play()
	// func stop()
	// func pause()
	// func resume()
	// func seek(_ offset: TimeInterval) -> Self
	// func forward() -> Self
	// func reverse() -> Self
	// func restart(_ includeDelay: Bool)

}

/// An Animator represents state change from one ObjectState to another ObjectState state
pub struct Animator {
    pub id: u32,
    pub start: ObjectState,
    pub end: ObjectState,
    pub current: ObjectState,
    pub start_time: Instant,
    pub duration: Duration,
}

impl Animator {
    pub fn create(id: u32, props1: &Vec<Prop>, props2: &Vec<Prop>, seconds: &f64) -> Self {
        let current_state = ObjectState::create(props1);
        let start_state = ObjectState::create(props1);
        let end_state = ObjectState::create(props2);
        let time = Duration::from_float_secs(*seconds);
        Animator {
            id: id,
            start: start_state,
            end: end_state,
            current: current_state,
            start_time: Instant::now(),
            duration: time,
        }
    }

    pub fn render(&self) -> Vec<Prop> {
        let mut results: Vec<Prop> = Vec::new();
        let elapsed = self.start_time.elapsed();
        let progress = elapsed.as_float_secs() / self.duration.as_float_secs();
        // println!("elapsed={} progress={}", elapsed.as_float_secs(), progress);
        if progress > 0.0 && progress <= 1.0 {
            for (i, prop) in self.start.props.clone().iter().enumerate() {
                let target = self.end.props[i].clone();
                let current = Animator::interpolate(&prop, &target, progress);
                results.push(current);
            }
        }
        results
    }

    /// Given two Props of same type, calculate the interpolated state
    fn interpolate(initial: &Prop, target: &Prop, scale: f64) -> Prop {

        let result = match initial {
            // Prop::Alpha(_) => { Prop::Alpha(1.0) },
            // Prop::Color(_) => {
            //     if let Some(color) = self.background {
            //         return Prop::Color(ColorRGBA::new(color.r_f(), color.g_f(), color.b_f(), color.a_f()));
            //     } else {
            //         return Prop::Color(ColorRGBA::new(0.0, 0.0, 0.0, 0.0));
            //     }
            // },
            Prop::Position(m1) => {
                let m2 = unwrap_to!(target => Prop::Position);
                let out = m1.lerp(*m2, scale);
                Prop::Position(out)
            },
            // Prop::Size(_) => Prop::Size(Frame2D::new(self.size.width, self.size.height)),
            _ => Prop::None,
        };


        return result;
    }
}

// #####################################################################################


pub struct Animation {
    start_time: Instant,
    // end_time: Instant,
    duration: Duration,
    delay: Duration,
    elapsed: Duration,
	// var state: AnimationState { get set }
	// var duration: TimeInterval { get set }
	// var delay: TimeInterval { get set }
	// var timeScale: Double { get set }
	// var progress: Double { get set }
	// var totalProgress: Double { get set }

	// var startTime: TimeInterval { get set }
	// var endTime: TimeInterval { get }
	// var totalDuration: TimeInterval { get }
	// var totalTime: TimeInterval { get }
	// var elapsed: TimeInterval { get }
	// var time: TimeInterval { get }

	// var timingFunction: TimingFunctionType { get }
	// var spring: Spring? { get }

}

impl Animation {
    pub fn new() -> Self {
        Animation{
            start_time: Instant::now(),
            duration: Duration::from_secs(0),
            delay: Duration::from_secs(0),
            elapsed: Duration::from_secs(0),
        }
    }

    pub fn render() {

    }
}

impl Animatable for Animation {

    fn play() {

    }
    fn stop() {

    }
    fn pause() {

    }
    fn resume() {

    }
    fn seek() {

    }

}
