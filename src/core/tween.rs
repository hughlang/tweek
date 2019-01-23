/// This is the core Tween model and functions.
use std::collections::HashMap;
use super::interpolatable::*;
use super::property::*;


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

impl Tween {

    fn new(&self) -> Self {
        Tween{
            propertiesMap: HashMap::new(),
        }
    }
    fn get_properties(&self) -> Vec<FromToValue> {
        self.propertiesMap.values().cloned().collect()
     }
    fn add(&self, prop: Property, mode: TweenMode) {

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