extern crate sdl2;


#[derive(Copy, Clone)]
pub enum TransformType {
    Int,
    Float,
    Point,
    Rect,

}

pub trait Interpolatable {
    fn get_type(self) -> TransformType;
    fn vectorize(self) -> Vec<f32>;
}

impl Interpolatable for f32 {
    fn get_type(self) -> TransformType { TransformType::Float }
    fn vectorize(self) -> Vec<f32> { vec![self] }
}

impl Interpolatable for i32 {
    fn get_type(self) -> TransformType { TransformType::Int }
    fn vectorize(self) -> Vec<f32> { vec![self as f32] }
}

impl Interpolatable for sdl2::rect::Point {
    fn get_type(self) -> TransformType { TransformType::Point }
    fn vectorize(self) -> Vec<f32> { vec![self.x() as f32, self.y() as f32] }
}

impl Interpolatable for sdl2::rect::Rect {
    fn get_type(self) -> TransformType { TransformType::Rect }
    fn vectorize(self) -> Vec<f32> { vec![self.x() as f32, self.y() as f32, self.width() as f32, self.height() as f32] }
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
