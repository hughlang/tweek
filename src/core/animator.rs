/// An Animator has start and end properties that can be rendered in an animation
///
///
use cgmath::*;

use super::property::*;
use super::ease::*;

/// An Animator represents state change from one UIState to another UIState state
pub struct Animator {
    /// The id is a weird composite of the parent tween_id and the position within the animators vector.
    /// Not important yet, but it's better to have a grouping differentiator in the future.
    pub id: (usize, usize),
    pub start_state: UIState,
    pub end_state: UIState,
    pub start_time: f64,
    pub end_time: f64,
    pub seconds: f64,
    pub ease: Ease,
    pub debug: bool,
}

impl Animator {
    pub fn create(id: &(usize, usize), props1: &Vec<Prop>, props2: &Vec<Prop>) -> Self {
        let tween_id = id.clone();
        let start_state = UIState::create(tween_id, props1.clone());
        let end_state = UIState::create(tween_id, props2.clone());
        Animator {
            id: tween_id,
            start_state: start_state,
            end_state: end_state,
            start_time: 0.0,
            end_time: 0.0,
            seconds: 1.0,
            ease: Ease::Linear,
            debug: false,
        }
    }

    pub fn schedule(mut self, start: f64, end: f64) -> Self {
        self.start_time = start;
        self.end_time = end;
        self
    }

    /// Parameters:
    /// >> playhead = the relative seconds that have elapsed for this animator within the tween. For a tween playing
    /// in reverse with multiple animators, it is important to calculate this correctly against the overall timeline.
    /// >> time_scale = The is 1.0 for forward playback at normal speed and -1.0 for reverse playback. Faster and slower
    /// adjustment can be configured by increasing or decreasing this number.
    #[allow(unused_assignments)]
    pub fn update(&self, playhead: f64, time_scale: f64) -> UIState {
        let mut props: Vec<Prop> = Vec::new();
        // let mut elapsed = 0.0 as f64;
        let mut progress = 0.0 as f64;
        if time_scale > 0.0 {
            progress = playhead / self.seconds * time_scale;
        } else {
            progress =  1.0 - playhead / self.seconds * time_scale.abs();
        }
        let ratio = self.ease.clone().get_ratio(progress as f32);
        progress = ratio as f64;

        for (i, prop) in self.start_state.props.iter().enumerate() {
            if prop ==  &self.end_state.props[i] {
                // println!("Unchanged start={:?} end={:?}", prop, &self.end_state.props[i]);
                props.push(prop.clone());
                continue;
            }
            let current = Animator::interpolate(prop, &self.end_state.props[i], progress);

            if self.debug {
                println!("[{}.{}] from={:?} to={:?} >>> now={:?}", self.id.0, self.id.1, prop, &self.end_state.props[i], current);
            }
            props.push(current);
        }
        UIState::create(self.id, props)
    }

    /// Given two Props of same type, calculate the interpolated state
    fn interpolate(initial: &Prop, target: &Prop, scale: f64) -> Prop {
        if initial.prop_id() != target.prop_id() { return initial.clone() }

        let result = match initial {
            Prop::Alpha(v1) => {
                let v2 = unwrap_to!(target => Prop::Alpha);
                let out = v1.lerp(*v2, scale);
                // println!("Interpolated to: {}", out[0]);
                Prop::Alpha(out)

            },
            Prop::Color(m1) => {
                let m2 = unwrap_to!(target => Prop::Color);
                let out = m1.lerp(*m2, scale as f32);
                // println!("Interpolated to: r={} g={} b={}", out[0], out[1], out[2]);
                Prop::Color(out)
            },
            Prop::Position(m1) => {
                let m2 = unwrap_to!(target => Prop::Position);
                let out = m1.lerp(*m2, scale);
                // println!("Interpolated to: x={} y={}", out[0], out[1]);
                Prop::Position(out)
            },
            Prop::Rotate(v1) => {
                let v2 = unwrap_to!(target => Prop::Rotate);
                let out = v1.lerp(*v2, scale);
                // println!("Interpolated to: {}", out[0]);
                Prop::Rotate(out)

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
