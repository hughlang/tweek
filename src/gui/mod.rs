//! The gui module contains the base UI components using Quicksilver as the backend.
//! It will not contain widget components that are really a composition of base gui components
//! that have advanced logic. However, some widget-like components (like OptionGroup) are necessary
//! to support basic UI behaviors that one would expect in a web interface, like radio buttons.
//!
//! rules

pub use self::base::*;
pub use self::button::*;
pub use self::checkbox::*;
pub use self::cursor::*;
pub use self::image::*;
pub use self::label::*;
pub use self::layer::*;
pub use self::list_box::*;
pub use self::option_group::*;
pub use self::scene::*;
pub use self::shape::*;
pub use self::stage::*;
pub use self::text::*;
pub use self::text_area::*;
pub use self::text_field::*;
pub use self::theme::*;

mod base;
mod button;
mod checkbox;
mod cursor;
mod image;
mod label;
mod layer;
mod list_box;
mod option_group;
mod scene;
mod shape;
mod stage;
mod text;
mod text_area;
mod text_field;
mod theme;

use crate::core::Timeline;
use std::any::TypeId;
use std::collections::HashMap;

use quicksilver::graphics::Color;

/// A wrapper for Font styling. Removing dependency on Quicksilver fonts
#[derive(Clone, Copy, Debug)]
pub struct FontStyle {
    pub(crate) size: f32,
    pub(crate) color: Color,
}

impl FontStyle {
    /// Create a new instantce of a font style
    pub fn new(size: f32, color: Color) -> FontStyle {
        FontStyle { size, color }
    }

    /// Getter for size. Returns f32
    pub fn get_size(&self) -> f32 {
        self.size
    }

    /// Getter for Color, which has r,g,b,a values of 0.0 to 1.0
    pub fn get_color(&self) -> Color {
        self.color
    }
}

// HashMap that contains registry of all UI components in Tweek. This is used to
// look up the string name of a component based on its TypeId
lazy_static! {
    #[allow(missing_docs)]
    pub static ref GUI_TYPES_MAP: HashMap<TypeId, &'static str> = {
        let mut map = HashMap::new();
        map.insert(TypeId::of::<Button>(), "Button");
        map.insert(TypeId::of::<Checkbox>(), "Checkbox");
        map.insert(TypeId::of::<Cursor>(), "Cursor");
        map.insert(TypeId::of::<ImageView>(), "Image");
        map.insert(TypeId::of::<Label>(), "Label");
        map.insert(TypeId::of::<ListBox>(), "ListBox");
        map.insert(TypeId::of::<OptionGroup>(), "OptionGroup");
        map.insert(TypeId::of::<Scene>(), "Scene");
        map.insert(TypeId::of::<Stage>(), "Stage");
        map.insert(TypeId::of::<ShapeView>(), "Shape");
        map.insert(TypeId::of::<TextArea>(), "TextArea");
        map.insert(TypeId::of::<TextField>(), "TextField");
        map.insert(TypeId::of::<Text>(), "Text");
        map.insert(TypeId::of::<Timeline>(), "Timeline");
        map
    };
}

lazy_static! {
    #[allow(missing_docs)]
    pub static ref GUI_NAMES_MAP: HashMap<&'static str, TypeId> = {
        let mut map = HashMap::new();
        map.insert("Button", TypeId::of::<Button>());
        map.insert("Checkbox", TypeId::of::<Checkbox>());
        map.insert("Cursor", TypeId::of::<Cursor>());
        map.insert("Image", TypeId::of::<ImageView>());
        map.insert("Label", TypeId::of::<Label>());
        map.insert("ListBox", TypeId::of::<ListBox>());
        map.insert("OptionGroup", TypeId::of::<OptionGroup>());
        map.insert("Scene", TypeId::of::<Scene>());
        map.insert("Stage", TypeId::of::<Stage>());
        map.insert("Shape", TypeId::of::<ShapeView>());
        map.insert("TextArea", TypeId::of::<TextArea>());
        map.insert("TextField", TypeId::of::<TextField>());
        map.insert("Text", TypeId::of::<Text>());
        map.insert("Timeline", TypeId::of::<Timeline>());
        map
    };
}

lazy_static! {
    #[allow(missing_docs)]
    pub static ref GUI_RESPONDERS: Vec<TypeId> = {
        let mut array = Vec::new();
        array.push(TypeId::of::<Button>());
        array.push(TypeId::of::<Checkbox>());
        array.push(TypeId::of::<ListBox>());
        array.push(TypeId::of::<OptionGroup>());
        array.push(TypeId::of::<Scene>());
        array.push(TypeId::of::<TextArea>());
        array.push(TypeId::of::<TextField>());
        array.push(TypeId::of::<Text>());
        array
    };
}

lazy_static! {
    #[allow(missing_docs)]
    pub static ref GUI_INPUTS: Vec<TypeId> = {
        let mut array = Vec::new();
        array.push(TypeId::of::<Checkbox>());
        array.push(TypeId::of::<ListBox>());
        array.push(TypeId::of::<TextArea>());
        array.push(TypeId::of::<TextField>());
        array.push(TypeId::of::<OptionGroup>());
        array
    };
}

// lazy_static! {
//     #[allow(missing_docs)]
//     pub static ref GUI_PANELS: Vec<TypeId> = {
//         let mut array = Vec::new();
//         array.push(TypeId::of::<Scene>());
//         array
//     };
// }

/// Helper to get the string for a given GUI component
pub fn gui_print_type(type_id: &TypeId) -> &'static str {
    if let Some(name) = GUI_TYPES_MAP.get(type_id) {
        name
    } else {
        "Unknown"
    }
}
