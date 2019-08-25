/// Easing formulas courtesy of GreenSock AS3 TweenLite code
/// See also: https://greensock.com/gsap-as, https://greensock.com/standard-license
use std::f32::consts::PI;

const PERIOD: f32 = 0.3;
const AMPLITUDE: f32 = 1.0;
const PI_2: f32 = PI * 2.0;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Ease {
    Linear,
    SineIn,
    SineOut,
    SineInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    // ***** The following are not supported yet. *****
    QuadIn, // power1
    QuadOut,
    QuadInOut,
    CubicIn, // power2
    CubicOut,
    CubicInOut,
    QuartIn, // power3 / quart
    QuartOut,
    QuartInOut,
    QuintIn, // power4 / strong
    QuintOut,
    QuintInOut,
    CircIn,
    CircOut,
    CircInOut,
}

/// The following provides the mathematical formulas that provide different timing ratios
/// for an animation. The progress parameter "p" represents the linear progress of a tween
/// animation based on the time elapsed divided by the duration to get a number between
/// 0.0 and 1.0. The functions below provide an adjusted ratio based on the timing formula
/// that applies to each.
#[cfg_attr(feature = "cargo-clippy", allow(clippy::needless_return))]
impl Ease {
    /// Method for calculating the ratio to apply to the linear interpolation of an animation based on
    /// the specified Ease type. For Linear, the parameter p returns exactly p. For other types, there
    /// is a formula copied from other easing formulas.
    pub fn get_ratio(self, p: f32) -> f32 {
        match self {
            Ease::Linear => p,
            Ease::SineIn => {
                // AS3: -Math.cos(p * _HALF_PI) + 1;
                return -1.0 * (p * PI / 2.0).cos() + 1.0;
            }
            Ease::SineOut => {
                // AS3: Math.sin(p * _HALF_PI);
                return (p * PI / 2.0).sin();
            }
            Ease::SineInOut => {
                // AS3: -0.5 * (Math.cos(Math.PI * p) - 1);
                return -0.5 * ((PI * p).cos() - 1.0);
            }
            Ease::ExpoIn => {
                // AS3: Math.pow(2, 10 * (p - 1)) - 0.001;
                return 2.0_f32.powf(10.0 * (p - 1.0)) - 0.001;
            }
            Ease::ExpoOut => {
                // AS3: 1 - Math.pow(2, -10 * p);
                return 1.0 - 2.0_f32.powf(-10.0 * p);
            }
            Ease::ExpoInOut => {
                // AS3: ((p*=2) < 1) ? 0.5 * Math.pow(2, 10 * (p - 1)) : 0.5 * (2 - Math.pow(2, -10 * (p - 1)));
                let x = p * 2.0;
                if x < 1.0 {
                    return 0.5 * 2.0_f32.powf(10.0 * (x - 1.0));
                } else {
                    return 0.5 * 2.0_f32.powf(-10.0 * (x - 1.0));
                }
            }
            Ease::BackIn => {
                // AS3: p * p * ((overshoot + 1) * p - overshoot);
                let overshoot = 1.70158;
                return p * p * ((overshoot + 1.0) * p - overshoot);
            }
            Ease::BackOut => {
                // AS3: ((p = p - 1) * p * ((overshoot + 1) * p + overshoot) + 1)
                let overshoot = 1.70158;
                let x = p - 1.0;
                return x * x * ((overshoot + 1.0) * x + overshoot) + 1.0;
            }
            Ease::BackInOut => {
                // AS3: ((p*=2) < 1) ? 0.5 * p * p * ((overshoot + 1) * p - overshoot) : 0.5 * ((p -= 2) * p * ((overshoot + 1) * p + overshoot) + 2)
                let overshoot = 1.70158 * 1.525;
                let x = p * 2.0;
                if x < 1.0 {
                    return 0.5 * x * x * ((overshoot + 1.0) * x - overshoot);
                } else {
                    let y = x - 2.0;
                    return y * y * ((overshoot + 1.0) * y + overshoot) + 2.0;
                }
            }
            Ease::BounceIn => {
                /*  AS3:
                   if ((p = 1 - p) < 1 / 2.75) {
                       return 1 - (7.5625 * p * p);
                   } else if (p < 2 / 2.75) {
                       return 1 - (7.5625 * (p -= 1.5 / 2.75) * p + .75);
                   } else if (p < 2.5 / 2.75) {
                       return 1 - (7.5625 * (p -= 2.25 / 2.75) * p + .9375);
                   } else {
                       return 1 - (7.5625 * (p -= 2.625 / 2.75) * p + .984375);
                   }
                */
                // let x = self.bounce_time(p);
                let mut p = 1.0 - p;
                if p < 1.0 / 2.75 {
                    return 1.0 - 7.5625 * p * p;
                } else if p < 2.0 / 2.75 {
                    p -= 1.5 / 2.75;
                    return 1.0 - (7.5625 * p * p + 0.75);
                } else if p < 2.5 / 2.75 {
                    p -= 2.25 / 2.75;
                    return 1.0 - (7.5625 * p * p + 0.9375);
                } else {
                    p -= 2.625 / 2.75;
                    return 1.0 - (7.5625 * p * p + 0.984_375);
                }
            }
            Ease::BounceOut => {
                /* AS3:
                    if (p < 1 / 2.75) {
                        return 7.5625 * p * p;
                    } else if (p < 2 / 2.75) {
                        return 7.5625 * (p -= 1.5 / 2.75) * p + .75;
                    } else if (p < 2.5 / 2.75) {
                        return 7.5625 * (p -= 2.25 / 2.75) * p + .9375;
                    } else {
                        return 7.5625 * (p -= 2.625 / 2.75) * p + .984375;
                    }
                */
                let mut p = p;
                if p < 1.0 / 2.75 {
                    return 7.5625 * p * p;
                } else if p < 2.0 / 2.75 {
                    p -= 1.5 / 2.75;
                    return 7.5625 * p * p + 0.75;
                } else if p < 2.5 / 2.75 {
                    p -= 2.25 / 2.75;
                    return 7.5625 * p * p + 0.9375;
                } else {
                    p -= 2.625 / 2.75;
                    return 7.5625 * p * p + 0.984_375;
                }
            }
            Ease::BounceInOut => {
                /* AS3:
                    var invert:Boolean;
                    if (p < 0.5) {
                        invert = true;
                        p = 1 - (p * 2);
                    } else {
                        p = (p * 2) - 1;
                    }
                    if (p < 1 / 2.75) {
                        p = 7.5625 * p * p;
                    } else if (p < 2 / 2.75) {
                        p = 7.5625 * (p -= 1.5 / 2.75) * p + .75;
                    } else if (p < 2.5 / 2.75) {
                        p = 7.5625 * (p -= 2.25 / 2.75) * p + .9375;
                    } else {
                        p = 7.5625 * (p -= 2.625 / 2.75) * p + .984375;
                    }
                    return invert ? (1 - p) * 0.5 : p * 0.5 + 0.5;
                */
                let mut p = p;
                let mut invert = false;
                if p < 0.5 {
                    invert = true;
                    p = 1.0 - (p * 2.0);
                } else {
                    p = (p * 2.0) - 1.0;
                }

                if p < 1.0 / 2.75 {
                    p = 7.5625 * p * p;
                } else if p < 2.0 / 2.75 {
                    p -= 1.5 / 2.75;
                    p = 7.5625 * p * p + 0.75;
                } else if p < 2.5 / 2.75 {
                    p -= 2.25 / 2.75;
                    p = 7.5625 * p * p + 0.9375;
                } else {
                    p -= 2.625 / 2.75;
                    p = 7.5625 * p * p + 0.984_375;
                }
                if invert {
                    return (1.0 - p) * 0.5;
                } else {
                    return p * 0.5 + 0.5;
                }
            }
            Ease::ElasticIn => {
                /* AS3:
                   _p1 = amplitude || 1;
                   _p2 = period || 0.3;
                   _p3 = _p2 / _2PI * (Math.asin(1 / _p1) || 0);

                   -(_p1 * Math.pow(2, 10 * (p -= 1)) * Math.sin( (p - _p3) * _2PI / _p2 ));
                */
                let curve = PERIOD / PI_2 * (1.0 / AMPLITUDE).asin();
                let p = p - 1.0;
                return -(AMPLITUDE * 2.0_f32.powf(10.0 * p) * (p - curve) * PI_2 / PERIOD).sin();
            }
            Ease::ElasticOut => {
                /* AS3:
                    _p1 = amplitude || 1;
                    _p2 = period || 0.3;
                    _p3 = _p2 / _2PI * (Math.asin(1 / _p1) || 0);

                    _p1 * Math.pow(2, -10 * p) * Math.sin( (p - _p3) * _2PI / _p2 ) + 1;
                */
                let curve = PERIOD / PI_2 * (1.0 / AMPLITUDE).asin();
                return AMPLITUDE * 2.0_f32.powf(-10.0 * p) * ((p - curve) * PI_2 / PERIOD).sin() + 1.0;
            }
            Ease::ElasticInOut => {
                /*
                    _p1 = amplitude || 1;
                    _p2 = period || 0.45;
                    _p3 = _p2 / _2PI * (Math.asin(1 / _p1) || 0);

                    return:
                    ((p*=2) < 1) ? -.5 * (_p1 * Math.pow(2, 10 * (p -= 1)) * Math.sin( (p - _p3) * _2PI / _p2))
                        : _p1 * Math.pow(2, -10 *(p -= 1)) * Math.sin( (p - _p3) * _2PI / _p2 ) *.5 + 1;
                */

                let period = 0.45;
                let curve = period / PI_2 * (1.0 / AMPLITUDE).asin();
                let mut p = p * 2.0;

                if p < 1.0 {
                    p -= 1.0;
                    return -0.5 * (AMPLITUDE * 2.0_f32.powf(10.0 * p)) * ((p - curve) * PI_2 / period).sin();
                } else {
                    p -= 1.0;
                    return AMPLITUDE * 2.0_f32.powf(-10.0 * p) * ((p - curve) * PI_2 / period).sin() * 0.5 + 1.0;
                }
            }
            _ => {
                log::warn!("Not implemented: {:?}", self);
                0.0
            }
        }
    }
}
