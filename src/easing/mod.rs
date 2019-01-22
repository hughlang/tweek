use std::f32::consts::PI;

const kPERIOD: f32 = 0.3;
const M_PI_X_2: f32 = PI * 2.0;
//
// let kPERIOD: Double = 0.3
// let M_PI_X_2: Double = Double.pi * 2.0

#[derive(Debug, PartialEq, Eq)]
pub enum Easing {
  Linear,
  SineIn,
  SineOut,
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
      Linear => t,
      SineIn => -1.0 * (t * PI / 2.0).cos() + 1.0,
      SineOut => (t * PI/2.0).sin(),
      SineInOut => -0.5 * (PI * t).cos() - 1.0,
      ExponentialIn => {
          if t == 0.0 {
              return 0.0;
          } else {
              return (-10.0 * (t-1.0)).powf(2.0) - 1.0 * 0.001;
          }
      },
      _ => 0.0,
    }
  }
}
