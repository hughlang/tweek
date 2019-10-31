/// A wrapper for creating Command functions for buttons and other Responders
/// Status: Experimental
///
///
///
use crate::core::*;
use crate::events::*;
use crate::gui::*;

use std::any::{Any, TypeId};

/// A struct for building an action object to associate with a Button or other source
pub struct Command {
    /// The Button or other Responder
    pub source: Box<dyn Any>,
    /// Callback method for handling click action
    pub action: Option<Box<dyn FnMut(&mut AppState) + 'static>>,
    /// Animation directives
    pub transition: PropSet,
    /// The layer_id of the source object
    pub target_id: u32,
    /// The type_id of the target object
    pub target_type: TypeId,
    /// Holder of the event
    pub event: EventBox,
}

impl Command {
    /// Constructor that takes a Button or Responder object as the "source" of the command
    pub fn new(source: Box<dyn Any>) -> Self {
        Command {
            source,
            action: None,
            transition: PropSet::default(),
            target_id: 0,
            target_type: TypeId::of::<Scene>(),
            event: EventBox::new(NavEvent::Home),
        }
    }

    /// Define the target object given its layer.id and TypeId
    pub fn target(mut self, id: u32, type_id: TypeId) -> Self {
        self.target_id = id;
        self.target_type = type_id;
        self
    }

    /// Define the event that will be dispatched when triggered
    pub fn event<E: AnyEvent>(mut self, event: E) -> Self {
        self.action = Some(Box::new(move |state: &mut AppState| {
            state.event_bus.register_event(event);
        }));
        self.event = EventBox::new(event);
        self
    }

    /// Provide the display Props to manipulate or animate as a result of this command.
    pub fn animate(mut self, props: PropSet) -> Self {
        self.transition = props;
        self
    }

    /// This allows you to define the callback more explicitly
    pub fn callback<C>(mut self, cb: C) -> Self
    where
        C: FnMut(&mut AppState) + 'static,
    {
        self.action = Some(Box::new(cb));
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
