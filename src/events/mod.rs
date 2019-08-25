//! Tweek Events
//!
//! The Events module provides a flexible system of propagating events in the GUI hierarchy.
//! The EventBus is the holder of the queue of EventBox objects, which all implement the AnyEvent
//! trait. Methods are provided for type-checking of events.
//!
//! The events architecture is still a work-in-progress and is used in Examples code and other
//! external repos to test its usefulness.
//!
//! Disclosure: Some of this code was copied/derived from OrbTk EventQueue and related code.
//! TODO: discuss how to attribute code fragments copied from other projects.
//!
//! See the documentation in /docs/events-arch.md


// pub use self::app::*;
pub use self::app::*;
pub use self::display::*;
pub use self::notify::*;
pub use self::player::*;
// pub use self::system::*;
pub use self::types::*;

// mod app;
mod app;
mod display;
mod notify;
mod player;
// mod system;
mod types;

use std::{
    any::{Any, TypeId},
};

// *****************************************************************************************************
// This module file contains the core code for Events
// * AnyEvent trait and EventBox wrapper
// * EventBus for queued events
// *****************************************************************************************************

/// Used to define an event trait.
pub trait AnyEvent: Any + Copy {}

/// Enum for error events
#[derive(Debug)]
pub enum EventError {
    /// An attempt to downcast an AnyEvent to the specified event type failed.
    WrongType(TypeId),
}

/// A holder of a boxed AnyEvent and its TypeId.
#[derive(Debug)]
pub struct EventBox {
    event: Box<dyn Any>,
    event_type: TypeId,
}

impl EventBox {
    /// Constructor
    pub fn new<E: AnyEvent>(event: E) -> Self {
        EventBox { event: Box::new(event), event_type: TypeId::of::<E>() }
    }

    /// Determine if the event type matches
    pub fn is_type<E: AnyEvent>(&self) -> bool {
        self.event_type == TypeId::of::<E>()
    }

    /// Getter for event_type
    pub fn event_type(&self) -> TypeId {
        self.event_type
    }

    /// Attempt to convert the type into the specified type and use it
    pub fn downcast<E: AnyEvent>(self) -> Result<E, EventError> {
        if self.event_type == TypeId::of::<E>() {
            return Ok(*self.event.downcast::<E>().unwrap());
        }
        Err(EventError::WrongType(TypeId::of::<E>()))
    }

    /// Attempt to convert the event into the specified type as a reference
    pub fn downcast_ref<E: Any>(&self) -> Result<&E, EventError> {
        if self.event_type == TypeId::of::<E>() {
            return Ok(&*self.event.downcast_ref::<E>().unwrap());
        }
        Err(EventError::WrongType(TypeId::of::<E>()))
    }
}

/// This is the event queue which is intended to propagate events up the GUI hierarchy.
#[derive(Default, Debug)]
pub struct EventBus {
    /// An array of Boxed events
    pub event_queue: Vec<EventBox>,
}

impl EventBus {
    /// Add an array of events to the queue
    pub fn append(&mut self, other: &mut Vec<EventBox>) {
        self.event_queue.append(other);
    }

    pub fn add_event(&mut self, event: EventBox) {
        self.event_queue.push(event);
    }

    /// Register an event?
    pub fn register_event<E: AnyEvent>(&mut self, event: E) {
        self.event_queue.push(EventBox::new::<E>(event));
    }

    /// Pop an event from the queue
    pub fn dequeue(&mut self) -> Option<EventBox> {
        if !self.event_queue.is_empty() {
            return Some(self.event_queue.remove(0));
        }
        None
    }

    /// Number of queued events
    pub fn len(&self) -> usize {
        self.event_queue.len()
    }

    /// Find the events matching the specified TypeId and return them in a Vec.
    /// This does not dequeue/remove from the event_queue.
    pub fn filter<E: AnyEvent>(&mut self) -> Vec<E> {
        let mut results: Vec<E> = Vec::new();
        for event in &self.event_queue {
            if let Ok(evt) = event.downcast_ref::<E>() {
                results.push(evt.clone());
            }
        }
        results
    }

    // pub fn query<E: AnyEvent>(&mut self) -> Vec<E> {
    //     let mut results: Vec<E> = Vec::new();
    //     for event in &self.event_queue {
    //         if let Ok(evt) = event.downcast_ref::<E>() {
    //             results.push(evt.clone());
    //         }
    //     }
    //     results
    // }
}

impl<'a> IntoIterator for &'a mut EventBus {
    type Item = EventBox;
    type IntoIter = EventBusIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EventBusIterator { event_queue: self }
    }
}

/// A convenience iterator
pub struct EventBusIterator<'a> {
    event_queue: &'a mut EventBus,
}

impl<'a> Iterator for EventBusIterator<'a> {
    type Item = EventBox;

    fn next(&mut self) -> Option<EventBox> {
        self.event_queue.dequeue()
    }
}


