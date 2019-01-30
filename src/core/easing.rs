use std::f32::consts::PI;

// const kPERIOD: f32 = 0.3;
// const M_PI_X_2: f32 = PI * 2.0;

#[derive(Debug, PartialEq, Eq)]
pub enum Easing {
    Linear,
    SineIn,
    SineOut,
    SineInOut,
    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
}

// #[ignore(unused_variables)]
impl Easing {
    pub fn apply(self, t: f32) -> f32 {
        match self {
            Easing::Linear => t,
            Easing::SineIn => -1.0 * (t * PI / 2.0).cos() + 1.0,
            Easing::SineOut => (t * PI / 2.0).sin(),
            Easing::SineInOut => -0.5 * (PI * t).cos() - 1.0,
            Easing::ExponentialIn => {
                return if t == 0.0 {
                    t
                } else {
                    (-10.0 * (t - 1.0)).powf(2.0) - 1.0 * 0.001
                };
            }
            _ => 0.0,
        }
    }
}
