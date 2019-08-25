/// A wrapper for creating Command functions for buttons and other Responders
/// Status: Experimental
///
///
///
use crate::core::*;
use crate::events::*;
use crate::gui::*;

use std::{
    any::{Any, TypeId},
};

/// A struct for building an action object to associate with a Button or other control
pub struct Command {
    /// The Button or other Responder
    pub control: Box<dyn Any>,
    /// Callback method for handling click action
    pub action: Option<Box<dyn FnMut(&mut AppState) + 'static>>,
    /// Animation directives
    pub transition: PropSet,
    // /// The layer_id of the source object
    // pub source_id: u32,
    // /// type_id of the source object
    // pub source_type: TypeId,
    /// The layer_id of the target object
    pub target_id: u32,
    /// The type_id of the target object
    pub target_type: TypeId,
    /// Holder of the event
    pub result: EventBox,
}

impl Command {

    pub fn new(control: Box<dyn Any>) -> Self {
        // let source_id = control.get_id();
        // let source_type = control.get_type_id();
        Command {
            control: control,
            action: None,
            transition: PropSet::default(),
            // source_id,
            // source_type,
            target_id: 0,
            target_type: TypeId::of::<Scene>(),
            result: EventBox::new(NavEvent::Home),
        }
    }

    pub fn target(mut self, id: u32, type_id: TypeId) -> Self {
        self.target_id = id;
        self.target_type = type_id;
        self
    }

    pub fn result<E: AnyEvent>(mut self, event: E) -> Self {
        self.action = Some(Box::new(move |state: &mut AppState| {
            state.event_bus.register_event(event);
        }));
        self.result = EventBox::new(event);
        self
    }

    pub fn callback<C>(mut self, cb: C) -> Self
    where
        C: FnMut(&mut AppState) + 'static,
    {
        self.action = Some(Box::new(cb));
        self
    }

    pub fn transition(mut self, props: PropSet) -> Self {
        self.transition = props;
        self
    }
}

// ************************************************************************************
// Support
// ************************************************************************************

pub enum CommandEvent {
    Click,
    Hover,
}

pub enum CommandAction {
    None,
    Show,
    Hide,
}
