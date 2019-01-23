extern crate sdl2;

use sdl2::rect::*;


#[derive(Copy, Clone)]
pub enum InterpolatableType {
    Int,
    Float,
    Point,
    Rect,

}

pub trait Interpolatable {
    fn get_type(self) -> InterpolatableType;
    fn vectorize(self) -> InterpolatableValue;
    // fn interpolate(&self, to: Interpolatable, progress: f32) -> Interpolatable {
    //     let value = self.vectorize();

    // }
}

impl Interpolatable for f32 {
    fn get_type(self) -> InterpolatableType { InterpolatableType::Float }
    fn vectorize(self) -> InterpolatableValue {
        InterpolatableValue { i_type: self.get_type(), vectors: vec![self]}
    }
}

impl Interpolatable for u32 {
    fn get_type(self) -> InterpolatableType { InterpolatableType::Int }
    fn vectorize(self) -> InterpolatableValue {
        InterpolatableValue { i_type: self.get_type(), vectors: vec![self as f32]}
    }
}

impl Interpolatable for sdl2::rect::Point {
    fn get_type(self) -> InterpolatableType { InterpolatableType::Point }
    fn vectorize(self) -> InterpolatableValue {
        InterpolatableValue { i_type: self.get_type(),
            vectors: vec![self.x() as f32, self.y() as f32]}
    }
}

impl Interpolatable for sdl2::rect::Rect {
    fn get_type(self) -> InterpolatableType { InterpolatableType::Rect }
    fn vectorize(self) -> InterpolatableValue {
        InterpolatableValue { i_type: self.get_type(),
            vectors: vec![self.x() as f32, self.y() as f32, self.width() as f32, self.height() as f32]}
    }
}

pub struct InterpolatableValue {
    pub i_type: InterpolatableType,
    pub vectors: Vec<f32>,
}

impl Clone for InterpolatableValue {
    fn clone(&self) -> Self {
        InterpolatableValue {
            i_type: self.i_type,
            vectors: self.vectors.clone(),
        }
    }
}

impl InterpolatableValue {
    pub fn new(mut self, value: InterpolatableValue) -> Self {
        self.i_type = value.i_type;
        self.vectors = value.vectors;
        self
    }

    pub fn interpolate(&self, to: InterpolatableValue, progress: f32) -> InterpolatableValue {
        let mut diff: Vec<f32> = Vec::new();
        for i in 0..self.vectors.len() {
            let val = self.vectors[i] + (to.vectors[i] - self.vectors[i]) * progress;
            diff.push(val)
        }
        InterpolatableValue {
            i_type: self.i_type.clone(),
            vectors: diff,
        }
    }
}
