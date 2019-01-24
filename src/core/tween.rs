/// This is the core Tween model and functions.
use std::{any::TypeId, collections::HashMap, rc::Rc};

use super::sprite::*;
use super::property::*;

#[allow(dead_code)]


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
    delay: f32,
    duration_s: f32,
    progress: f32,
    fn_queue: Vec<fn() -> Property>,
}

impl Default for Tween {
    fn default() -> Self {
        Tween {
            delay: 0.0,
            duration_s: 0.0,
            progress: 0.0,
            fn_queue: Vec::new(),
        }
    }
}

impl Tween {
    fn play() {

    }
    fn duration(mut self, _seconds: f32) -> Self {
        self.duration_s = _seconds;
        self
    }

    fn to(mut self, props: Vec<fn() -> Property>) -> Self {
        self.fn_queue = props;
        self
    }

    // fn from(mut self, props: Vec<Property>) {

    // }

}

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
