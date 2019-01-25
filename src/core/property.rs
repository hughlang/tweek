/// A Command is a trait that allows Tween to manipulate it
///

use na::{Matrix4, Point3, Vector3};

#[derive(Copy, Clone)]
pub enum PropType {
    Int,
    Float,
    Point,
    Rect,
}

pub enum Transition {
    None,
    From(Box<Property>),
    To(Box<Property>),
    State(Box<Property>),
}

pub trait Tweenable {
    fn lerp(t: f32, end: Self) -> Self;
    fn distance_to(other: Self) -> f32;
}

pub trait Property {
    fn get_key(&self) -> String;
    fn get_type(&self) -> PropType;
}

// ==============================================================


#[derive(Default, Clone, Debug)]
pub struct XPos {
    key: String,
    // vectors: Vec<f32>,
}

impl XPos {
    pub fn new(v: f32) -> Self {
        XPos {
            key: "frame.x".to_string(),
            // vectors: vec![v],
        }
    }
}

impl Property for XPos {
    fn get_key(&self) -> String { self.key.clone() }
    fn get_type(&self) -> PropType { PropType::Float }
}

#[derive(Default, Clone, Debug)]
pub struct YPos {
    key: String,
    // vectors: Vec<f32>,
}

impl Property for YPos {
    fn get_key(&self) -> String { self.key.clone() }
    fn get_type(&self) -> PropType { PropType::Float }
}

impl YPos {
    pub fn new(v: f32) -> Self {
        YPos {
            key: "frame.y".to_string(),
            // vectors: vec![v],
        }
    }
}


// pub struct X {

// }
// ==============================================================

// #[derive(Clone)]
// pub struct FromToValue {
// 	pub from: Property,
//     pub to: Property,
// }

// impl FromToValue {

//     // pub fn new(_from: Option<Command>, _to: Option<Command>) -> Self {
//     //     FromToValue { from: _from, to: _to }
//     // }
// }


