/// A Command is a trait that allows Tween to manipulate it
///

use cgmath::*;

pub type FloatProp = Vector1<f64>;
pub type ColorRGB = Vector3<f64>;
pub type ColorRGBA = Vector4<f64>;
pub type Point2D = Vector2<f64>;
pub type Frame2D = Vector2<f64>;
pub type Bezier = Vector4<f64>;

#[derive(Copy, Clone, Debug)]
pub enum Prop {
    None,
    Alpha(FloatProp),
    Color(ColorRGBA),
    Position(Point2D),
    Size(Frame2D),
}

// pub const PROP_LIST: Vec<Prop> = vec![Prop::Alpha, Prop::Color, Prop::Position:: Prop::Size];

impl Prop {
    /// Stupid shit helper method because Rust enums cannot emit a discriminator Int id if there are custom fields
    pub fn prop_id(&self) -> u32 {
        match self {
            Prop::None => 0,
            Prop::Alpha(_) => 1,
            Prop::Color(_) => 2,
            Prop::Position(_) => 3,
            Prop::Size(_) => 4,
        }
    }
    pub fn get_prop_list() -> Vec<Prop> {
        let mut results: Vec<Prop> = Vec::new();
        results.push(Prop::Alpha(FloatProp::zero()));
        results.push(Prop::Color(ColorRGBA::zero()));
        results.push(Prop::Position(Point2D::zero()));
        results.push(Prop::Size(Frame2D::zero()));
        results
    }
}

// impl
// TODO: implement Scale, Translation, and Rotation

#[derive(Default)]
pub struct UIState {
    pub id: usize,
    pub props: Vec<Prop>,
}

impl UIState {
    pub fn create(_id: usize, _props: Vec<Prop>) -> Self {
        UIState {
            id: _id,
            props: _props,
        }
    }

    pub fn get_prop_value(&self, prop_id: u32) -> Prop {
        let mut iter = self.props.iter().filter( |x| x.prop_id() == prop_id );
        if let Some(item) = &iter.next() {
            return *item.clone();
        }
        Prop::None
    }
}


#[derive(Copy, Clone)]
pub enum PropType {
    Int,
    Float,
    Point,
    Rect,
}


// #####################################################################################


