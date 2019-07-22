pub use self::base::*;
pub use self::button::*;
pub use self::checkbox::*;
pub use self::cursor::*;
pub use self::image::*;
pub use self::label::*;
pub use self::layer::*;
pub use self::list_box::*;
pub use self::option_group::*;
pub use self::progress_bar::*;
pub use self::scene::*;
pub use self::shape::*;
pub use self::text::*;
pub use self::text_area::*;
pub use self::text_field::*;
pub use self::theme::*;
pub use self::toolbar::*;

mod base;
mod button;
mod checkbox;
mod cursor;
mod image;
mod label;
mod layer;
mod list_box;
mod option_group;
mod progress_bar;
mod scene;
mod shape;
mod text;
mod text_area;
mod text_field;
mod theme;
mod toolbar;

use std::any::TypeId;
use std::collections::HashMap;

lazy_static! {
    pub static ref GUI_TYPES_MAP: HashMap<TypeId, &'static str> = {
        let mut map = HashMap::new();
        map.insert(TypeId::of::<Button>(), "Button");
        map.insert(TypeId::of::<Checkbox>(), "Checkbox");
        map.insert(TypeId::of::<Cursor>(), "Cursor");
        map.insert(TypeId::of::<ImageView>(), "ImageView");
        map.insert(TypeId::of::<Label>(), "Label");
        map.insert(TypeId::of::<ListBox>(), "ListBox");
        map.insert(TypeId::of::<OptionGroup>(), "OptionGroup");
        map.insert(TypeId::of::<ProgressBarView>(), "ProgressBar");
        map.insert(TypeId::of::<Scene>(), "Scene");
        map.insert(TypeId::of::<ShapeView>(), "ShapeView");
        map.insert(TypeId::of::<TextArea>(), "TextArea");
        map.insert(TypeId::of::<TextField>(), "TextField");
        map.insert(TypeId::of::<Text>(), "Text");
        map.insert(TypeId::of::<Toolbar>(), "Toolbar");
        map
    };
}