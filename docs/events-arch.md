# Events Architecture

> This document is not finished. Please ignore for now.

## AnyEvent

Tweek uses the AnyEvent trait to allow for flexible definition and handling of any kind of event.
Several event types have already been defined. These can be found in the events sub-crate.


## Types

* Display Events
* App Events
* Player Events

## EventBus

The EventBus is a queue for AnyEvent objects. In the Tweek codebase, an EventBus is utilised in the
AppState object that is passed as a mutable reference in several of the Displayable and Responder
methods that are implemented by many gui objects.

### Handling events



## Notifications

