/// A Property is a trait that allows Tween to manipulate it
///


#[derive(Copy, Clone)]
pub enum PropType {
    Int,
    Float,
    Point,
    Rect,

}

// ==============================================================

#[derive(Clone)]
pub struct Property {
    key: String,
    ptype: PropType,
    pub vectors: Vec<f32>,
}

impl Property {
    pub fn new(&self, _key: String, _ptype: PropType, _vectors: Vec<f32>) -> Self {
        Property { key: _key, ptype: _ptype, vectors: _vectors}
    }
    pub fn get_key(&self) -> &String { &self.key }
    pub fn apply_vectors(&mut self, in_vectors: Vec<f32>) {
        for (i, _) in in_vectors.iter().enumerate() {
            if in_vectors[i] > 0.0 { self.vectors[i] = in_vectors[i] }
        }
    }
    pub fn apply(&mut self, prop: Property) {
        self.apply_vectors(prop.vectors);
    }
}

pub struct X {
    key: String,
    vectors: Vec<f32>,

}

// pub struct X {

// }
// ==============================================================

#[derive(Clone)]
pub struct FromToValue {
	pub from: Option<Property>,
    pub to: Option<Property>,
}

impl FromToValue {

    pub fn new(_from: Option<Property>, _to: Option<Property>) -> Self {
        FromToValue { from: _from, to: _to }
    }
}


