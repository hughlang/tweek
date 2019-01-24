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
    pub value: LerpValue,
}

impl Property {
    pub fn apply(value: LerpValue) {}
    pub fn get_key(&self) -> &String { &self.key }
}

// pub struct X {

// }

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

