/// This is the core Tween model and functions.
use std::collections::HashMap;
use super::tweenable::*;
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

pub struct Tween {
    propertiesMap: HashMap<String, FromToValue>,
}

#[allow(dead_code)]
impl Tween {

    fn new(&self, _target: impl Tweenable) -> Self {
        Tween{
            propertiesMap: HashMap::new(),
        }
    }
    fn get_properties(&self) -> Vec<FromToValue> {
        self.propertiesMap.values().cloned().collect()
    }

    fn add(&mut self, prop: Property, mode: TweenMode) {
        let key = prop.get_key();
        let mut value =
            if let Some(v) = self.propertiesMap.get(key) {
                v.clone()
            } else {
                FromToValue::new(None, None)
            };

        if mode == TweenMode::From {
            value.from = Some(prop.clone());
        } else {
            value.to = Some(prop.clone());

            // TODO: target setting
        }
        // let key =
        self.propertiesMap.insert(key.to_string(), value);
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
impl Animation for Tween {
    fn init() {

    }
    fn deinit() {

    }
}