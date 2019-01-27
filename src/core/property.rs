/// A Command is a trait that allows Tween to manipulate it
///

use cgmath::*;

pub type ColorRGB = Vector3<f64>;
pub type ColorRGBA = Vector4<f64>;
pub type Point2D = Vector2<f64>;
pub type Frame2D = Vector2<f64>;

#[derive(Copy, Clone)]
pub enum Prop {
    None,
    Alpha(f64),
    Color(ColorRGBA),
    Position(Point2D),
    Size(Frame2D),
}

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
}

// TODO: implement Scale, Translation, and Rotation
pub struct ObjectState {
    pub props: Vec<Prop>,
}

impl ObjectState {
    pub fn create(_props: &Vec<Prop>) -> Self {
        ObjectState {
            props: _props.clone(),
        }
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


