/// This is the core Tween model and functions.
use std::collections::HashMap;

use super::sprite::*;
use super::property::*;

#[allow(dead_code)]


#[derive(Debug, PartialEq, Eq)]
pub enum TweenMode {
    To,
    From,
    FromTo,
}

pub trait Animation {
    fn init();
    fn deinit();

}

pub struct Tween<T> where T: Sprite {
    // item_type: TypeId,
    properties_map: HashMap<String, FromToValue>,
    target: T,
}

#[allow(dead_code)]
impl<T> Tween<T> where T: Sprite {

    fn new(&self, _target: T) -> Self where T: Sprite {
        Tween{
            properties_map: HashMap::new(),
            target: _target,
        }
    }
    fn get_properties(&self) -> Vec<FromToValue> {
        self.properties_map.values().cloned().collect()
    }

    fn add(&mut self, prop: Property, mode: TweenMode) {
        let key = prop.get_key();
        let mut ftval =
            if let Some(v) = self.properties_map.get(key) {
                v.clone()
            } else {
                FromToValue::new(None, None)
            };

        if mode == TweenMode::From {
            ftval.from = Some(prop.clone());
            if let Some(mut current) = self.target.current_property(prop.clone()) {
                if ftval.to.is_none() {
                    ftval.to = Some(current.clone());
                }
                current.apply(prop.clone());
                // let mut prop = current.clone();




            }
        } else {
            ftval.to = Some(prop.clone());
        }
        self.properties_map.insert(key.to_string(), ftval);
    }
}

impl<T> Animation for Tween<T> where T: Sprite {
    fn init() {

    }
    fn deinit() {

    }
}
