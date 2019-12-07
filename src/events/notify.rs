/// NOTE: This file does not define AnyEvent types. Instead, it is a utility for Notifications using a local queue
/// instead of the global EventBus.
///
/// This event system follows the Observer design pattern described here:
/// https://blog.rom1v.com/2017/09/gnirehtet-rewritten-in-rust/#observer
/// In this code, the Storage struct is called Notifications
use crate::events::*;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// An Listener that handles alerts
pub trait Listener {
    /// TBD: Can the default implementation be overridden?
    fn on_event(&mut self, event: EventBox);
}

impl<F: FnMut(EventBox)> Listener for F {
    fn on_event(&mut self, event: EventBox) {
        self(event);
    }
}

/// A struct that holds Listeners
pub struct Notifier {
    listeners: Vec<Box<dyn Listener>>,
}

impl Notifier {
    /// Constructor
    pub fn new() -> Self {
        Self { listeners: Vec::new() }
    }

    /// Add a new Listener
    pub fn register<T: Listener + 'static>(&mut self, listener: T) {
        self.listeners.push(Box::new(listener));
    }

    /// Notify all listeners using the specified AnyEvent enum
    pub fn notify<E: AnyEvent>(&mut self, event: E) {
        for listener in &mut self.listeners {
            listener.on_event(EventBox::new(event));
        }
    }
}

// *****************************************************************************************************
// Model objects for passing around alert info
// *****************************************************************************************************

/**
* A struct that can receive an AnyEvent enum and store it.
* 1/ Add Notifications field to your struct and initialise it.
   notifications: Rc<RefCell<Notifications>>,
   notifications: Notifications::new(),

* 2/ Sending a notification:
   let mut notifier = Notifier::new();
   self.notifications.borrow_mut().attach(&mut notifier);
   notifier.notify(<AnyEvent>);  // example
*/
pub struct Notifications {
    weak_self: Weak<RefCell<Notifications>>,
    pub events: EventBus,
}

impl Notifications {
    /// Constructor that holds a weak_self reference to support Observer pattern
    pub fn new() -> Rc<RefCell<Self>> {
        let rc = Rc::new(RefCell::new(Self {
            weak_self: Weak::new(), // initialize empty
            events: EventBus::default(),
        }));
        rc.borrow_mut().weak_self = Rc::downgrade(&rc);
        rc
    }

    /// Registering a notifier and storing the alert
    pub fn attach(&mut self, notifier: &mut Notifier) {
        let rc = self.weak_self.upgrade().unwrap();
        notifier.register(move |msg| {
            rc.borrow_mut().store(msg);
        })
    }

    /// Method to store an alert
    pub fn store(&mut self, msg: EventBox) {
        self.events.add_event(msg);
    }

    /// Clear all alerts
    pub fn clear(&mut self) {
        self.events.event_queue.clear();
    }
}

// ************************************************************************************
// Dispatcher trait
// ************************************************************************************

/// A trait that requires methods to pass along a mutable Notifier reference. This is useful in scenarios where a result
/// object is expected, along with notifications to trigger other actions. In the Layer-Tween relationship, the Tween
/// object needs to notify state changes such as PlayState::Completed. The parent Layer reviews the notifications queue
/// and publishes new events to the parent Displayable.
///
/// A future use case will be to capture GPU Texture state events and metadata to better manage GPU actions.
pub trait NotifyDispatcher {
    /// Associated type for the Result. Example: type Update = PropSet;
    type Update;
    /// Associated type for Params
    type Params;
    /// In the runloop update() call, the status method tells the object to evaluate its internal state before
    /// calling the request_update method. Both methods can emit useful notifications that can be evaluated at runtime.
    fn status(&mut self, notifier: &mut Notifier, params: Box<Self::Params>);
    /// This is a generic method to get an expected Update object, along with notifications dispatched during the
    /// request_update() call.
    fn request_update(&mut self, notifier: &mut Notifier, params: Box<Self::Params>) -> Option<Box<Self::Update>>;
}
