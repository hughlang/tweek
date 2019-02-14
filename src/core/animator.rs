/// An Animator has start and end properties that can be rendered in an animation
///
///
use std::{time::{Duration,Instant}};
use cgmath::*;

use super::property::*;
use super::easing::*;


/// An Animator represents state change from one UIState to another UIState state
pub struct Animator {
    pub id: usize,
    pub start_state: UIState,
    pub end_state: UIState,
    pub start_time: f64,
    pub end_time: f64,
    pub easing: Easing,
    pub debug: bool,
}

impl Animator {
    pub fn create(id: usize, props1: &Vec<Prop>, props2: &Vec<Prop>, ease: &Easing) -> Self {
        let start_state = UIState::create(id, props1.clone());
        let end_state = UIState::create(id, props2.clone());
        Animator {
            id: id,
            start_state: start_state,
            end_state: end_state,
            start_time: 0.0,
            end_time: 0.0,
            easing: ease.clone(),
            debug: false,
        }
    }

    pub fn schedule(mut self, start: f64, end: f64) -> Self {
        self.start_time = start;
        self.end_time = end;
        self
    }

    pub fn update(&self, start_time: Instant, duration: Duration, time_scale: f64) -> UIState {
        let mut props: Vec<Prop> = Vec::new();
        let elapsed = start_time.elapsed();
        let mut progress = 0.0 as f64;
        // if self.easing != Easing::Linear {
        //     let curve = self.easing.curve();
        //     let solver = BezierSolver::from(curve.clone());
        //     progress = solver.sstart_stateprogress);
        // }end_state
        if time_scale > 0.0 {
            progress = elapsed.as_float_secs() / duration.as_float_secs() * time_scale;
        } else {
            progress =  1.0 - elapsed.as_float_secs() / duration.as_float_secs() * time_scale.abs();
        }
        if progress > 0.0 && progress <= 1.0 {
            for (i, prop) in self.start_state.props.iter().enumerate() {
                let current = Animator::interpolate(prop, &self.end_state.props[i], progress);

                // println!("----------------------------------------------");
                // println!("elapsed={} progress={}", elapsed.as_float_secs(), progress);
                props.push(current);
            }
        }
        UIState::create(self.id, props)
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
            Prop::Alpha(v1) => {
                let v2 = unwrap_to!(target => Prop::Alpha);
                let out = v1.lerp(*v2, scale);
                // println!("Interpolated to: {}", out[0]);
                Prop::Alpha(out)

            },
            Prop::Position(m1) => {
                let m2 = unwrap_to!(target => Prop::Position);
                // println!("m1={:?} m2={:?}", m1, m2);
                let out = m1.lerp(*m2, scale);
                // println!("Interpolated to: x={} y={}", out[0], out[1]);
                Prop::Position(out)
            },
            Prop::Size(v1) => {
                let v2 = unwrap_to!(target => Prop::Size);
                let out = v1.lerp(*v2, scale);
                // println!("Interpolated to: {}", out[0]);
                Prop::Size(out)
            },
            _ => Prop::None,
        };


        return result;
    }
}

// #####################################################################################


// pub struct Animation {
//     start_time: Instant,
//     // end_time: Instant,
//     duration: Duration,
//     delay: Duration,
//     elapsed: Duration,
// 	// var state: AnimationState { get set }
// 	// var duration: TimeInterval { get set }
// 	// var delay: TimeInterval { get set }
// 	// var timeScale: Double { get set }
// 	// var progress: Double { get set }
// 	// var totalProgress: Double { get set }

// 	// var startTime: TimeInterval { get set }
// 	// var endTime: TimeInterval { get }
// 	// var totalDuration: TimeInterval { get }
// 	// var totalTime: TimeInterval { get }
// 	// var elapsed: TimeInterval { get }
// 	// var time: TimeInterval { get }

// 	// var timingFunction: TimingFunctionType { get }
// 	// var spring: Spring? { get }

// }

// impl Animation {
//     pub fn new() -> Self {
//         Animation{
//             start_time: Instant::now(),
//             duration: Duration::from_secs(0),
//             delay: Duration::from_secs(0),
//             elapsed: Duration::from_secs(0),
//         }
//     }

//     pub fn render() {

//     }
// }
