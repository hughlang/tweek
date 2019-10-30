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

use std::any::TypeId;
use std::collections::HashMap;

// HashMap that contains registry of all UI components in Tweek. This is used to
// look up the string name of a component based on its TypeId
lazy_static! {
    #[allow(missing_docs)]
    pub static ref GUI_TYPES_MAP: HashMap<TypeId, &'static str> = {
        let mut map = HashMap::new();
        map.insert(TypeId::of::<Button>(), "Button");
        map.insert(TypeId::of::<Checkbox>(), "Checkbox");
        map.insert(TypeId::of::<Cursor>(), "Cursor");
        map.insert(TypeId::of::<ImageView>(), "ImageView");
        map.insert(TypeId::of::<Label>(), "Label");
        map.insert(TypeId::of::<ListBox>(), "ListBox");
        map.insert(TypeId::of::<OptionGroup>(), "OptionGroup");
        map.insert(TypeId::of::<Scene>(), "Scene");
        map.insert(TypeId::of::<Stage>(), "Stage");
        map.insert(TypeId::of::<ShapeView>(), "ShapeView");
        map.insert(TypeId::of::<TextArea>(), "TextArea");
        map.insert(TypeId::of::<TextField>(), "TextField");
        map.insert(TypeId::of::<Text>(), "Text");
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
        // array.push(TypeId::of::<Scene>());
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

pub fn print_full_path(nodes: Vec<Node>) -> String {
    let paths: Vec<String> = nodes.iter().map(|x| x.id_string() ).collect();
    format!("/{}", paths.join("/"))
}
