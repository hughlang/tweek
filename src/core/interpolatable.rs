extern crate sdl2;


#[derive(Copy, Clone)]
pub enum LerpType {
    Int,
    Float,
    Point,
    Rect,

}

pub trait Lerp {
    fn get_type(self) -> LerpType;
    fn vectorize(self) -> LerpValue;
    // fn interpolate(&self, to: Lerp, progress: f32) -> Lerp {
    //     let value = self.vectorize();

    // }
}

impl Lerp for f32 {
    fn get_type(self) -> LerpType { LerpType::Float }
    fn vectorize(self) -> LerpValue {
        LerpValue { i_type: self.get_type(), vectors: vec![self]}
    }
}

impl Lerp for u32 {
    fn get_type(self) -> LerpType { LerpType::Int }
    fn vectorize(self) -> LerpValue {
        LerpValue { i_type: self.get_type(), vectors: vec![self as f32]}
    }
}

impl Lerp for sdl2::rect::Point {
    fn get_type(self) -> LerpType { LerpType::Point }
    fn vectorize(self) -> LerpValue {
        LerpValue { i_type: self.get_type(),
            vectors: vec![self.x() as f32, self.y() as f32]}
    }
}

impl Lerp for sdl2::rect::Rect {
    fn get_type(self) -> LerpType { LerpType::Rect }
    fn vectorize(self) -> LerpValue {
        LerpValue { i_type: self.get_type(),
            vectors: vec![self.x() as f32, self.y() as f32, self.width() as f32, self.height() as f32]}
    }
}

/// =====================================================================================

// pub struct LerpValue {
//     pub i_type: LerpType,
//     pub vectors: Vec<f32>,
// }

// impl Clone for LerpValue {
//     fn clone(&self) -> Self {
//         LerpValue {
//             i_type: self.i_type,
//             vectors: self.vectors.clone(),
//         }
//     }
// }

// impl LerpValue {
//     pub fn new(&self, value: LerpValue) -> Self {
//         LerpValue { i_type: value.i_type, vectors: value.vectors }
//     }

//     pub fn interpolate(&self, to: LerpValue, progress: f32) -> LerpValue {
//         let mut diff: Vec<f32> = Vec::new();
//         for i in 0..self.vectors.len() {
//             let val = self.vectors[i] + (to.vectors[i] - self.vectors[i]) * progress;
//             diff.push(val)
//         }
//         LerpValue {
//             i_type: self.i_type.clone(),
//             vectors: diff,
//         }
//     }

//     // pub fn to_Lerp() -> Lerp {

//     //     let x: f32 = 0.0;
//     //     x
//     // }


// }
