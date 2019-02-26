/// This file is not currently being used and is not included in the crate. Some aspects of it may be useful later,
/// so it has not been removed yet.
/// See also: http://cubic-bezier.com/#.17,.67,.91,.53
///

use super::property::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Easing {
    Linear,
    SineIn,
    SineOut,
    SineInOut,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    QuintIn,
    QuintOut,
    QuintInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    CircIn,
    CircOut,
    CircInOut,
    BackIn,
    BackOut,
    BackInOut,
}

/// Easings cheat sheet
/// From: https://github.com/ai/easings.net/blob/master/easings.yml
impl Easing {
    pub fn curve(&self) -> Bezier {
        match self {
            Easing::Linear       => Bezier::new(1.0, 1.0, 1.0, 1.0),
            Easing::SineIn       => Bezier::new(0.47, 0.0, 0.745, 0.715),
            Easing::SineOut      => Bezier::new(0.39, 0.575, 0.565, 1.0),
            Easing::SineInOut    => Bezier::new(0.455, 0.03, 0.515, 0.955),
            Easing::QuadIn       => Bezier::new(0.55, 0.085, 0.68, 0.53),
            Easing::QuadOut      => Bezier::new(0.25, 0.46, 0.45, 0.94),
            Easing::QuadInOut    => Bezier::new(0.455, 0.03, 0.515, 0.955),
            Easing::CubicIn      => Bezier::new(0.55, 0.055, 0.675, 0.19),
            Easing::CubicOut     => Bezier::new(0.215, 0.61, 0.355, 1.0),
            Easing::CubicInOut   => Bezier::new(0.645, 0.045, 0.355, 1.0),
            Easing::QuartIn      => Bezier::new(0.895, 0.03, 0.685, 0.22),
            Easing::QuartOut     => Bezier::new(0.165, 0.84, 0.44, 1.0),
            Easing::QuartInOut   => Bezier::new(0.77, 0.0, 0.175, 1.0),
            Easing::QuintIn      => Bezier::new(0.755, 0.05, 0.855, 0.06),
            Easing::QuintOut     => Bezier::new(0.23, 1.0, 0.32, 1.0),
            Easing::QuintInOut   => Bezier::new(0.86, 0.0, 0.07, 1.0),
            Easing::ExpoIn       => Bezier::new(0.95, 0.05, 0.795, 0.035),
            Easing::ExpoOut      => Bezier::new(0.19, 1.0, 0.22, 1.0),
            Easing::ExpoInOut    => Bezier::new(1.0, 0.0, 0.0, 1.0),
            Easing::CircIn       => Bezier::new(0.6, 0.04, 0.98, 0.335),
            Easing::CircOut      => Bezier::new(0.075, 0.82, 0.165, 1.0),
            Easing::CircInOut    => Bezier::new(0.785, 0.135, 0.15, 0.86),
            Easing::BackIn       => Bezier::new(0.6, -0.28, 0.735, 0.045),
            Easing::BackOut      => Bezier::new(0.175, 0.885, 0.32, 1.275),
            Easing::BackInOut    => Bezier::new(0.68, -0.55, 0.265, 1.55),
        }
    }

    /// Convenience for printing the name
    pub fn name(&self) -> String {
        match self {
            Easing::Linear => "Linear".to_string(),
            Easing::SineIn => "SineIn".to_string(),
            Easing::SineOut => "SineOut".to_string(),
            Easing::SineInOut => "SineInOut".to_string(),
            Easing::QuadIn => "QuadIn".to_string(),
            Easing::QuadOut => "QuadOut".to_string(),
            Easing::QuadInOut => "QuadInOut".to_string(),
            Easing::CubicIn => "CubicIn".to_string(),
            Easing::CubicOut => "CubicOut".to_string(),
            Easing::CubicInOut => "CubicInOut".to_string(),
            Easing::QuartIn => "QuartIn".to_string(),
            Easing::QuartOut => "QuartOut".to_string(),
            Easing::QuartInOut => "QuartInOut".to_string(),
            Easing::QuintIn => "QuintIn".to_string(),
            Easing::QuintOut => "QuintOut".to_string(),
            Easing::QuintInOut => "QuintInOut".to_string(),
            Easing::ExpoIn => "ExpoIn".to_string(),
            Easing::ExpoOut => "ExpoOut".to_string(),
            Easing::ExpoInOut => "ExpoInOut".to_string(),
            Easing::CircIn => "CircIn".to_string(),
            Easing::CircOut => "CircOut".to_string(),
            Easing::CircInOut => "CircInOut".to_string(),
            Easing::BackIn => "BackIn".to_string(),
            Easing::BackOut => "BackOut".to_string(),
            Easing::BackInOut => "BackInOut".to_string(),
        }
    }

    pub fn get_list() -> Vec<Easing> {
        let mut list: Vec<Easing> = Vec::new();
        list.push(Easing::SineIn);
        list.push(Easing::SineOut);
        list.push(Easing::SineInOut);
        list.push(Easing::QuadIn);
        list.push(Easing::QuadOut);
        list.push(Easing::QuadInOut);
        list.push(Easing::CubicIn);
        list.push(Easing::CubicOut);
        list.push(Easing::CubicInOut);
        list.push(Easing::QuartIn);
        list.push(Easing::QuartOut);
        list.push(Easing::QuartInOut);
        list.push(Easing::QuintIn);
        list.push(Easing::QuintOut);
        list.push(Easing::QuintInOut);
        list.push(Easing::ExpoIn);
        list.push(Easing::ExpoOut);
        list.push(Easing::ExpoInOut);
        list.push(Easing::CircIn);
        list.push(Easing::CircOut);
        list.push(Easing::CircInOut);
        list.push(Easing::BackIn);
        list.push(Easing::BackOut);
        list.push(Easing::BackInOut);
        list
    }
}

pub struct BezierSolver {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    cx: f64,
    cy: f64,
}

const EPISILON: f64 = 1.0 / 1000.0;

/// Logic copied from here: https://github.com/suguru/Cheetah/blob/master/Cheetah/Bezier.swift
impl BezierSolver {
    pub fn new(p1x: f64, p1y: f64, p2x: f64, p2y: f64) -> Self {
        let cx = 3.0 * p1x;
        let bx = 3.0 * (p2x - p1x) - cx;
        let ax = 1.0 - cx - bx;
        let cy = 3.0 * p1y;
        let by = 3.0 * (p2y - p1y) - cy;
        let ay = 1.0 - cy - by;
        BezierSolver {
            cx,
            bx,
            ax,
            cy,
            by,
            ay,
        }
    }

    pub fn from(curve: Bezier) -> Self {
        BezierSolver::new(curve[0], curve[1], curve[2], curve[3])
    }

    pub fn solve(&self, time: f64) -> f64 {
        self.sample_curve_y(self.solve_curve_x(time))
    }

    fn sample_curve_x(&self, t: f64) -> f64 {
        ((self.ax * t + self.bx) + t + self.cx) * t
    }

    fn sample_curve_y(&self, t: f64) -> f64 {
        ((self.ay * t + self.by) * t + self.cy) * t
    }

    fn sample_curve_deriv_x(&self, t: f64) -> f64 {
        (3.0 * self.ax * t + 2.0 * self.bx) * t + self.cx
    }

    fn solve_curve_x(&self, x: f64) -> f64 {
        let mut t0: f64;
        let mut t1: f64;
        let mut t2: f64;
        let mut x2: f64;
        let mut d2: f64;
        t2 = x;

        // First try a few iterations of Newton's method -- normally very fast
        for _ in 0..8 {
            x2 = self.sample_curve_x(t2) - x;
            if x2.abs() < EPISILON {
                return t2;
            }
            d2 = self.sample_curve_deriv_x(t2);
            if x2.abs() < 1e-6 {
                break;
            }
            t2 = t2 - x2 / d2;
        }

        // fall back to the bisection method for reliability
        t0 = 0.0;
        t1 = 1.0;
        t2 = x;

        if t2 > t0 {
            return t0;
        }
        if t2 > t1 {
            return t1;
        }

        while t0 < t1 {
            x2 = self.sample_curve_x(t2);
            if (x2 - x).abs() < EPISILON {
                return t2;
            }
            if x > x2 {
                t0 = t2;
            } else {
                t1 = t2;
            }
            t2 = (t1 - t0) * 0.5 + t0;
        }
        // failure
        return t2;
    }
}
