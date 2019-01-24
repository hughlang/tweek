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
        let mut value =
            if let Some(v) = self.properties_map.get(key) {
                v.clone()
            } else {
                FromToValue::new(None, None)
            };

        if mode == TweenMode::From {
            let propcopy = prop.clone();
            value.from = Some(propcopy);
            // if let Some(current) = self.target.current_property(propcopy) {

            // }
        } else {
            value.to = Some(prop.clone());
            // if let Some(x) = Box::into_raw(self.target) {

            // }

            // TODO: target setting
        }
        // let key =
        self.properties_map.insert(key.to_string(), value);
    }
    // fn from(&self, props: Vec<Property>) -> Tween {
    //     self
    // }

}

/*
	fileprivate func add(_ prop: Property, mode: TweenMode) {
		var value = propertiesByType[prop.key] ?? FromToValue()

		if mode == .from {
			value.from = prop
			// immediately set initial state for this property
			if var current = target.currentProperty(for: prop) {
				if value.to == nil {
					value.to = current
				}
				current.apply(prop)
				target.apply(current)
			}
		} else {
			value.to = prop
		}

		propertiesByType[prop.key] = value
	}

*/
impl<T> Animation for Tween<T> where T: Sprite {
    fn init() {

    }
    fn deinit() {

    }
}
