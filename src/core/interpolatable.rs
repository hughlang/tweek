
#[derive(Copy, Clone)]
pub enum InterpolatableType {
    Int,
    Float,
}

pub trait Interpolatable: Sized {
    fn get_type(&self) -> InterpolatableType;
    fn vectorize(&self) -> InterpolatableType;
    // fn interpolate(&self, to: Interpolatable, progress: f32) {
    //     let value = self.vectorize();

    // }
}


// #[derive(Clone)]
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
        InterpolatableValue { i_type: self.i_type.clone(), vectors: diff }
    }
}