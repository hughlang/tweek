/// An Animator has start and end properties that can be rendered in an animation
///
///
use cgmath::*;

use super::ease::*;
use super::property::*;

/// An Animator represents state change from one set of Props to a matching set of Props.
/// For example, a Tween might contain the directives to move and resize an object in the same time frame
/// and thus, it will contain Prop::Position and Prop::Size values in the start_state and end_state objects.
/// Usually, a Tween has one Animator, but it can have several to represent a sequence or even concurrent
/// set of animations for a given Tweenable component.
#[derive(Clone, Debug)]
pub struct Animator {
    /// The id is a weird composite of the parent tween_id and the position within the animators vector.
    /// Currently, it is only used for debug printing to differentiate between Animator segments
    pub id: (usize, usize),
    /// Stores the Props for the initial state of the Tweenable component.
    pub start_state: PropSet,
    /// Stores the target Props for the Tweenable component
    pub end_state: PropSet,
    /// The zero-based start time of this animator.
    pub start_time: f64,
    /// The zero-based end time of this animator
    pub end_time: f64,
    /// The duration in seconds for this animator
    pub seconds: f64,
    /// The motion adjustment property to specify how quickly or slowly an object is animated during its
    /// animation duration. By default, an animation is linear so that the interpolation of animation is the
    /// same in every moment/frame. With other Ease types, the interpolation of speed is modified through a
    /// formula in the Ease module.
    pub ease: Ease,
}

impl Animator {
    /// This constructor method takes the Tween id and the sequential animator index as useful values for
    /// debug purposes. The props1 and props2 arrays represent the start_state and end_state props that are
    /// used for the animation.
    pub fn create(id: &(usize, usize), props1: &[Prop], props2: &[Prop]) -> Self {
        let tween_id = id.clone();
        let start_state = PropSet::new(props1.to_vec(), 0.0);
        let end_state = PropSet::new(props2.to_vec(), 0.0);
        Animator {
            id: tween_id,
            start_state,
            end_state,
            start_time: 0.0,
            end_time: 0.0,
            seconds: 1.0,
            ease: Ease::Linear,
        }
    }

    // /// Unused
    // pub fn schedule(mut self, start: f64, end: f64) -> Self {
    //     self.start_time = start;
    //     self.end_time = end;
    //     self
    // }

    /// Parameters:
    /// >> playhead = the relative seconds that have elapsed for this animator within the tween. For a tween playing
    /// in reverse with multiple animators, it is important to calculate this correctly against the overall timeline.
    /// >> time_scale = The is 1.0 for forward playback at normal speed and -1.0 for reverse playback. Faster and slower
    /// adjustment can be configured by increasing or decreasing this number.
    #[allow(unused_assignments)]
    pub fn update(&self, playhead: f64, time_scale: f64) -> PropSet {
        let mut props: Vec<Prop> = Vec::new();
        let mut progress: f64 = 0.0;
        if time_scale > 0.0 {
            progress = playhead / self.seconds * time_scale;
        } else {
            progress = 1.0 - playhead / self.seconds * time_scale.abs();
        }
        let ratio = self.ease.clone().get_ratio(progress as f32);
        progress = ratio as f64;

        // log::trace!("progress={} playhead={} total secs={}", progress, playhead, self.seconds);
        for (i, prop) in self.start_state.props.iter().enumerate() {
            // FIXME: This will crash if end_state.props index out of bounds
            if i >= self.end_state.props.len() {
                log::error!("Index out of bounds for i={} and end_state.props len={}", i, self.end_state.props.len());
                break;
            }
            if prop == &self.end_state.props[i] {
                // log::trace!("Unchanged start={:?} end={:?}", prop, &self.end_state.props[i]);
                props.push(prop.clone());
                continue;
            }
            let current = Animator::interpolate(prop, &self.end_state.props[i], progress as f32);
            // log::trace!(
            //     "[{}.{}] FROM={:?}  TO={:?}  >>  NOW={:?}",
            //     self.id.0,
            //     self.id.1,
            //     prop,
            //     &self.end_state.props[i],
            //     current
            // );
            props.push(current);
        }
        PropSet::new(props, 0.0)
    }

    /// Given two Props of same type, calculate the interpolated state
    fn interpolate(initial: &Prop, target: &Prop, scale: f32) -> Prop {
        if initial.prop_id() != target.prop_id() {
            return initial.clone();
        }

        let result = match initial {
            Prop::Alpha(v1) => {
                let v2 = unwrap_to!(target => Prop::Alpha);
                let out = v1.lerp(*v2, scale);
                Prop::Alpha(out)
            }
            Prop::Color(m1) => {
                let m2 = unwrap_to!(target => Prop::Color);
                let out = m1.lerp(*m2, scale as f32);
                Prop::Color(out)
            }
            Prop::Tint(m1) => {
                let m2 = unwrap_to!(target => Prop::Tint);
                let out = m1.lerp(*m2, scale as f32);
                Prop::Tint(out)
            }
            Prop::Position(m1) => {
                let m2 = unwrap_to!(target => Prop::Position);
                let out = m1.lerp(*m2, scale);
                Prop::Position(out)
            }
            Prop::Rotate(v1) => {
                let v2 = unwrap_to!(target => Prop::Rotate);
                let out = v1.lerp(*v2, scale);
                Prop::Rotate(out)
            }
            Prop::Size(v1) => {
                let v2 = unwrap_to!(target => Prop::Size);
                let out = v1.lerp(*v2, scale);
                Prop::Size(out)
            }
            // TODO: Handle border animation strictly for color
            _ => {
                log::debug!("Prop not handled: {:?}", initial);
                Prop::None
            }
        };
        result
    }
}
