/// This file contains most of the model objects used to read and write values in Tweenable objects.
///
use super::ease::*;
use crate::events::*;
use cgmath::*;
use std::fmt;

/*
The following type aliases are used for human readable names for cgmath Vector types that are used in the
Animator for interpolation math.
 */
/// A single float value
pub type FloatProp = Vector1<f32>;
/// A triplet of float values for RGB colors in the range 0 to 255. Float is used for consistency, instead of int
pub type ColorRGB = Vector3<f32>;
/// Represents four values of RGBA. Currently unused
pub type ColorRGBA = Vector4<f32>;
/// Float values for x-y coordinates
pub type Point2D = Vector2<f32>;
/// Float values for width and height
pub type Frame2D = Vector2<f32>;
/// Unused. Intended for bezier curve paths. TODO?
pub type Bezier = Vector4<f32>;

/// The Prop enum contains a cgmath::Vector instance that is interpolated in the Animator update() method
/// based on the initial and target Prop values of the same type.
#[derive(Copy, Clone, PartialEq)]
pub enum Prop {
    /// The None type means that the requested Prop value does not exist in the object of specific code logic
    None,
    /// The transparency of an object in the range 0.0 to 1.0
    Alpha(FloatProp),
    /// Property with RGB values as f32 values in range 0 to 255
    Color(ColorRGBA),
    /// The position of an object as x-y coordinates
    Position(Point2D),
    /// The rotation of an object in degrees with range 0.0 to 360.0
    Rotate(FloatProp),
    /// The size of an object as w-h values
    Size(Frame2D),
    /// A special type to represent the position offset by the specified x-y values
    /// This is translated into a Position type during Tween pre-processing
    Shift(Point2D),
    /// A special type to represent the resize offset. Not really useful yet.
    /// Translates to a Size type during Tween pre-processing
    Resize(Frame2D),
    /// A special type used to apply a border as an animation directive
    Border(Option<ColorRGBA>, FloatProp),
    /// Tint color refers to the foreground color of some nested objects (ie, Text)
    Tint(ColorRGBA),
}

impl fmt::Debug for Prop {
    /// Special debug output that trims the extra Vector wrappers.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prop::Alpha(val) => write!(f, "Alpha({:.2})", val[0]),
            Prop::Color(rgba) => write!(f, "Color({}, {}, {}) Alpha({})", rgba[0], rgba[1], rgba[2], rgba[3]),
            Prop::Tint(rgba) => write!(f, "Color({}, {}, {}) Alpha({})", rgba[0], rgba[1], rgba[2], rgba[3]),
            Prop::Rotate(val) => write!(f, "Rotate({:.2})", val[0]),
            Prop::Position(pos) => write!(f, "Position({:.2}, {:.2})", pos[0], pos[1]),
            Prop::Size(size) => write!(f, "Size({:.2}, {:.2})", size[0], size[1]),
            Prop::Border(rgba, width) => {
                if let Some(rgba) = rgba {
                    write!(
                        f,
                        "Border(width({}), color({}, {}, {}) alpha({}))",
                        width[0], rgba[0], rgba[1], rgba[2], rgba[3]
                    )
                } else {
                    write!(f, "Border(None)")
                }
            }
            Prop::Shift(pos) => write!(f, "Shift({:.2}, {:.2})", pos[0], pos[1]),
            Prop::Resize(size) => write!(f, "Resize({:.2}, {:.2})", size[0], size[1]),
            _ => write!(f, "{:?}", self),
        }
    }
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
            Prop::Border(_, _) => 8,
            Prop::Tint(_) => 9,
        }
    }

    /// Unfortunate helper method for doing reverse lookup of a prop based on its prop_id()
    /// All of the internal values are zero vectors so this is only useful for lookups and
    /// matching when inspecting Props
    /// Magic numbers FTW!
    pub fn from_prop_id(id: u32) -> Prop {
        match id {
            1 => Prop::Alpha(FloatProp::zero()),
            2 => Prop::Color(ColorRGBA::zero()),
            3 => Prop::Position(Point2D::zero()),
            4 => Prop::Rotate(FloatProp::zero()),
            5 => Prop::Size(Frame2D::zero()),
            6 => Prop::Shift(Point2D::zero()),
            7 => Prop::Resize(Frame2D::zero()),
            8 => Prop::Border(None, FloatProp::zero()),
            9 => Prop::Tint(ColorRGBA::zero()),
            _ => Prop::None,
        }
    }

    /// This builds a list of the core enums that are not offsets (i.e., Shift, Resize)
    /// All of the internal values are zero vectors so this is only useful for iterating
    /// the list of possible Props that need to be inspected.
    pub fn get_prop_list() -> Vec<Prop> {
        let list = vec![
            Prop::Alpha(FloatProp::zero()),
            Prop::Color(ColorRGBA::zero()),
            Prop::Position(Point2D::zero()),
            Prop::Rotate(FloatProp::zero()),
            Prop::Size(Frame2D::zero()),
            Prop::Border(None, FloatProp::zero()),
            Prop::Tint(ColorRGBA::zero()),
        ];
        list
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

/// A wrapper to hold an array of Props used in Animator for Tween animation
#[derive(Debug, Clone, PartialEq)]
pub struct PropSet {
    /// Array of Props based on user-specified animation directives
    pub props: Vec<Prop>,
    /// Duration in seconds
    pub duration: f64,
    /// Delay in seconds
    pub delay: f64,
    /// Easing formula
    pub ease: Ease,
    // /// Number of times to repeat.
    // pub repeat_count: u32,
    // /// Seconds between repeat
    // pub repeat_delay: f64,
    // /// repeats forever
    // pub repeat_loop: bool,
    /// The type of transition
    pub event: TweenType, // Because type is a reserved word
}

impl Default for PropSet {
    fn default() -> Self {
        PropSet { props: Vec::new(), duration: 0.0, delay: 0.0, ease: Ease::Linear, event: TweenType::None }
    }
}

impl PropSet {
    /// Constructor
    pub fn new(props: Vec<Prop>, secs: f64) -> Self {
        PropSet { props: props, duration: secs, delay: 0.0, ease: Ease::Linear, event: TweenType::Animation }
    }

    /// Builder method to define the TweenType type.
    pub fn for_type(mut self, evt: TweenType) -> Self {
        self.event = evt;
        self
    }
    /// Builder method to set the delay
    pub fn delay(mut self, secs: f64) -> Self {
        self.delay = secs;
        self
    }
    /// Builder method to set the ease formula
    pub fn ease(mut self, ease: Ease) -> Self {
        self.ease = ease;
        self
    }

    // pub fn repeat(mut self, )

    /// Get a specific Prop based on the numeric prop_id
    pub fn get_prop_value(&self, prop_id: u32) -> Prop {
        let mut iter = self.props.iter().filter(|x| x.prop_id() == prop_id);
        if let Some(item) = iter.next() {
            return *item;
        }
        Prop::None
    }
}

// #####################################################################################

/*
trait Cartesian<T: PartialEq> {
    fn x(&self) -> &T;
    fn y(&self) -> &T;
}

impl<T> Cartesian<T> for Vector2<T> where T: PartialEq {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

fn cartesian_cmp<T>(lhs: T, rhs: T) -> bool where T: Cartesian<T> + PartialEq {
    lhs.x() == rhs.x() && lhs.y() == rhs.y()
}
*/
