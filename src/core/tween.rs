/// This is the core Tween model and functions.
extern crate orbrender;

use std::{any::TypeId, collections::HashMap, rc::Rc};
use na::*;

use super::property::*;
use super::animator::*;

type ColorRGB = Matrix1x3<f32>;
type Point2D = Matrix1x2<f32>;
type Frame2D = Matrix1x2<f32>;

pub enum Prop {
    None,
    Alpha(f32),
    Color(ColorRGB),
    Position(Point2D),
    Rect(Frame2D),
    Size(Frame2D),
}



#[derive(Debug, PartialEq, Eq)]
pub enum TweenMode {
    To,
    From,
    FromTo,
}

pub trait Animation {
    fn play();
    fn duration(&self, _seconds: f32) -> Self;

}

pub struct Tween {
    delay_s: f32,
    duration_s: f32,
    progress_s: f32,
    props: Vec<Box<Property>>,
    animators: HashMap<TypeId, Prop>,
}

// TODO: new constructor with Tweenable target
impl Default for Tween {
    fn default() -> Self {
        Tween {
            delay_s: 0.0,
            duration_s: 0.0,
            progress_s: 0.0,
            props: Vec::new(),
            animators: HashMap::new(),
        }
    }
}

impl Tween {

    /// Execute all functions in the queue
    pub fn play(self) {
        let mut animator = Animator::default();

        for prop in self.props {
            animator.start = Transition::From(prop);
            // animator.end = Transition::To(prop);
            // animator.current = Transition::State(prop);
        }
    }

    pub fn duration(mut self, _seconds: f32) -> Self {
        self.duration_s = _seconds;
        self
    }

    pub fn to(mut self, _props: Vec<Box<Property>>) -> Self {
        self.props = _props;
        self
    }



}

pub fn position(x: f32, y: f32) -> Prop {
    Prop::Position(Frame2D::new(x, y))
}

pub fn move_x(v: f32) -> Box<Property> {
    println!("Move x {}", v);
    Box::new(XPos::new(v))
}

pub fn move_y(v: f32) -> Box<Property> {
    println!("Move y {}", v);
    Box::new(YPos::new(v))
}


// #####################################################################################

pub trait Tweenable {
    // fn lerp(t: f32, end: Self) -> Self;
    // fn distance_to(other: Self) -> f32;
    fn get_key(&self) -> TypeId;
    fn apply(&self, prop: &Prop);
    fn get_prop(&self, key: &String) -> Prop;
}

impl Tweenable for orbrender::render_objects::Rectangle {

    fn get_key(&self) -> TypeId { TypeId::of::<orbrender::render_objects::Rectangle>() }

    fn apply(&self, prop: &Prop) {
        match prop {
            Prop::Position(xy) => (),
            _ => ()
        }

    }
    fn get_prop(&self, key: &String) -> Prop {
        Prop::None
    }

}
// impl Tweenable for sdl2::rect::Rect {
//     // type Item = sdl2::rect::Rect;
//     fn get_key(&self) -> TypeId { TypeId::of::<sdl2::rect::Rect>() }

//     fn apply(&self, prop: &Prop) {
//         match prop {
//             Prop::Position(xy) => (),
//             _ => ()
//         }

//     }
//     fn get_prop(&self, key: &String) -> Prop {
//         Prop::None
//     }
// }


// pub enum Anim {
//     None,
//     MoveX(f32),

// }
// pub struct Tween<T> where T: Sprite {
//     // item_type: TypeId,
//     properties_map: HashMap<String, FromToValue>,
//     target: T,
// }

// #[allow(dead_code)]
// impl<T> Tween<T> where T: Sprite {

//     fn new(&self, _target: T) -> Self where T: Sprite {
//         Tween{
//             properties_map: HashMap::new(),
//             target: _target,
//         }
//     }
//     fn get_properties(&self) -> Vec<FromToValue> {
//         self.properties_map.values().cloned().collect()
//     }

//     fn add(&mut self, prop: &Property, mode: TweenMode) {
//         let key = prop.get_key();
//         let mut ftval =
//             if let Some(v) = self.properties_map.get(&key) {
//                 v.clone()
//             } else {
//                 FromToValue::new(None, None)
//             };

//         if mode == TweenMode::From {
//             ftval.from = Some(prop.to_owned());
//             if let Some(mut current) = self.target.current_property(prop.clone()) {
//                 if ftval.to.is_none() {
//                     ftval.to = Some(current.clone());
//                 }
//                 current.apply(prop.clone());
//                 // let mut prop = current.clone();




//             }
//         } else {
//             ftval.to = Some(prop.clone());
//         }
//         self.properties_map.insert(key.to_string(), ftval);
//     }
// }

// impl<T> Animation for Tween<T> where T: Sprite {
//     fn init() {

//     }
//     fn deinit() {

//     }
// }
