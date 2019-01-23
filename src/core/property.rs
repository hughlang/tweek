/// A Property is a trait that allows Tween to manipulate it
///

use super::interpolatable::*;


// pub trait Property {
//     fn get_key() -> String;
//     fn get_value() -> String;
//     fn set_value(value: String);
// }

#[derive(Clone)]
pub struct Property {
    key: String,
    value: InterpolatableValue,
}

impl Property {
    pub fn apply(value: InterpolatableValue) {}
}

pub struct X {

}

#[derive(Clone)]
pub struct FromToValue {
	from: Option<Property>,
    to: Option<Property>,
}

impl FromToValue {
    fn new(&self, from: Option<Property>, to: Option<Property>) -> Self {
        FromToValue { from: None, to: None }
    }
}

