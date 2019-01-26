/// A Command is a trait that allows Tween to manipulate it
///

use na::*;

#[derive(Copy, Clone)]
pub enum PropType {
    Int,
    Float,
    Point,
    Rect,
}


pub trait Property {
    fn get_key(&self) -> String;
    fn get_type(&self) -> PropType;
    // fn get_matrix(&self) -> Matrix<N: Scalar, R: U, C: U, S: Storage>;
}

// ==============================================================


#[derive(Clone, Debug)]
pub struct XPos {
    key: String,
    vectors: Matrix1<f32>,
}

impl XPos {
    pub fn new(v: f32) -> Self {
        XPos {
            key: "frame.x".to_string(),
            vectors: Matrix1::new(v),
        }
    }
}

impl Property for XPos {
    fn get_key(&self) -> String { self.key.clone() }
    fn get_type(&self) -> PropType { PropType::Float }
}

#[derive(Clone, Debug)]
pub struct YPos {
    key: String,
    vectors: Matrix1<f32>,
}

impl Property for YPos {
    fn get_key(&self) -> String { self.key.clone() }
    fn get_type(&self) -> PropType { PropType::Float }
}

impl YPos {
    pub fn new(v: f32) -> Self {
        YPos {
            key: "frame.y".to_string(),
            vectors: Matrix1::new(v),
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


