/// A Property is a trait that allows Tween to manipulate it
///
///

// pub trait Property {
//     fn get_key() -> String;
//     fn get_value() -> String;
//     fn set_value(value: String);
// }

#[derive(Default, Clone)]
pub struct Property {
    key: String,
    value: u32,
}

impl Property {
    fn apply(value: u32) {}
}

