/// A Command is a trait that allows Tween to manipulate it
///

use cgmath::*;

pub type FloatProp = Vector1<f64>;
pub type ColorRGB = Vector3<f32>;
pub type ColorRGBA = Vector4<f32>;
pub type Point2D = Vector2<f64>;
pub type Frame2D = Vector2<f64>;
pub type Bezier = Vector4<f64>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Prop {
    None,
    Alpha(FloatProp),
    Color(ColorRGB),
    Position(Point2D),
    Rotate(FloatProp),
    Size(Frame2D),
    Shift(Point2D), // offset the position by the specified x and y values
    Resize(Frame2D), // offset the size by the specified w and h values
}

impl Eq for Prop {}

impl Prop {
    /// Stupid shit helper method because Rust enums cannot emit a discriminator Int id if there are custom fields
    pub fn prop_id(&self) -> u32 {
        match self {
            Prop::None => 0,
            Prop::Alpha(_) => 1,
            Prop::Color(_) => 2,
            Prop::Position(_) => 3,
            Prop::Rotate(_) => 4,
            Prop::Size(_) => 5,
            Prop::Shift(_) => 6,
            Prop::Resize(_) => 7,
        }
    }

    /// Unfortunate helper method for doing reverse lookup of a prop based on its prop_id()
    /// All of the internal values are zero vectors so this is only useful for lookups and
    /// matching when inspecting Props
     /// Magic numbers FTW!
    pub fn from_prop_id(id: u32) -> Prop {
        match id {
            1 => Prop::Alpha(FloatProp::zero()),
            2 => Prop::Color(ColorRGB::zero()),
            3 => Prop::Position(Point2D::zero()),
            4 => Prop::Rotate(FloatProp::zero()),
            5 => Prop::Size(Frame2D::zero()),
            6 => Prop::Shift(Point2D::zero()),
            7 => Prop::Resize(Frame2D::zero()),
            _ => Prop::None,
        }
    }

    /// This builds a list of the core enums that are not offsets (i.e., Shift, Resize)
    /// All of the internal values are zero vectors so this is only useful for iterating
    /// the list of possible Props that need to be inspected.
    pub fn get_prop_list() -> Vec<Prop> {
        let mut results: Vec<Prop> = Vec::new();
        results.push(Prop::Alpha(FloatProp::zero()));
        results.push(Prop::Color(ColorRGB::zero()));
        results.push(Prop::Position(Point2D::zero()));
        results.push(Prop::Rotate(FloatProp::zero()));
        results.push(Prop::Size(Frame2D::zero()));
        results
    }

    /// Somewhat hacky, but useful helper method that defines which Props have parent props
    pub fn lookup_parent_prop(&self) -> Prop {
        match self {
            Prop::Shift(_) => Prop::Position(Point2D::zero()),
            Prop::Resize(_) => Prop::Size(Frame2D::zero()),
            _ => Prop::None,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct UIState {
    pub id: (usize, usize),
    pub props: Vec<Prop>,
    pub progress: f64,
}

impl UIState {
    pub fn create(_id: (usize, usize), _props: Vec<Prop>) -> Self {
        UIState {
            id: _id,
            props: _props,
            progress: 0.0,
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

#[derive(Default, Debug, Clone)]
pub struct UITransition {
    pub props: Vec<Prop>,
    pub seconds: f64,
}

impl UITransition {
    pub fn new(props: Vec<Prop>, seconds: f64) -> Self {
        UITransition {
            props,
            seconds,
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


