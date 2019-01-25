/// A Command is a trait that allows Tween to manipulate it
///


#[derive(Copy, Clone)]
pub enum PropType {
    Int,
    Float,
    Point,
    Rect,

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


#[derive(Clone, Debug)]
pub struct XPos {
    key: String,
    vectors: Vec<f32>,
}

impl XPos {
    pub fn new(&self, x: f32) -> Self {
        XPos {
            key: "frame.x".to_string(),
            vectors: vec![x],
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
    vectors: Vec<f32>,
}

impl Property for YPos {
    fn get_key(&self) -> String { self.key.clone() }
    fn get_type(&self) -> PropType { PropType::Float }
}

impl YPos {
    pub fn new(&self, y: f32) -> Self {
        YPos {
            key: "frame.y".to_string(),
            vectors: vec![y],
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


