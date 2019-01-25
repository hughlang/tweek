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


#[derive(Clone)]
pub struct Command {
    key: String,
    ptype: PropType,
    pub vectors: Vec<f32>,
}

impl Command {
    pub fn new(&self, _key: String, _ptype: PropType, _vectors: Vec<f32>) -> Self {
        Command { key: _key, ptype: _ptype, vectors: _vectors}
    }
    // pub fn get_key(&self) -> String { self.key }
    pub fn apply_vectors(&mut self, in_vectors: Vec<f32>) {
        for (i, _) in in_vectors.iter().enumerate() {
            if in_vectors[i] > 0.0 { self.vectors[i] = in_vectors[i] }
        }
    }
    pub fn apply(&mut self, prop: Command) {
        self.apply_vectors(prop.vectors);
    }
}

#[derive(Default, Clone)]
pub struct X {
    key: String,
    vectors: Vec<f32>,
}

impl X {
    pub fn new(&self, x: f32) -> Self {
        X {
            key: "frame.x".to_string(),
            vectors: vec![x],
        }
    }
}

impl Property for X {
    fn get_key(&self) -> String { self.key.clone() }
    fn get_type(&self) -> PropType { PropType::Float }
}

#[derive(Default, Clone)]
pub struct Y {
    key: String,
    vectors: Vec<f32>,
}

impl Property for Y {
    fn get_key(&self) -> String { self.key.clone() }
    fn get_type(&self) -> PropType { PropType::Float }
}

impl Y {
    pub fn new(&self, y: f32) -> Self {
        Y {
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


