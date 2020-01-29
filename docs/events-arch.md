# Events Architecture

<!-- TOC -->

- [Events Architecture](#events-architecture)
- [Events System](#events-system)
    - [AnyEvent](#anyevent)
        - [Types](#types)
    - [EventBus](#eventbus)
    - [Dispatching Events](#dispatching-events)
- [Application Events](#application-events)
    - [Object Addressing Scheme](#object-addressing-scheme)
    - [Application/Stage/View Lifecycle](#applicationstageview-lifecycle)
        - [Startup](#startup)
    - [Event Listeners](#event-listeners)
        - [addlistener](#addlistener)
        - [Receiving notifications](#receiving-notifications)
    - [Notifications](#notifications)
- [TODO/Improvements](#todoimprovements)

<!-- /TOC -->

**Topics**

* AnyEvent trait
* EventBus
* Tween Events
* Mouse Events
* Event Listeners
* Notifications
* Observers
* Tags


# Events System

## AnyEvent
> Incomplete

Tweek uses the AnyEvent trait to allow for flexible definition and handling of any kind of event.
Several event types have already been defined. These can be found in the events module.


## EventBus
> Incomplete

The EventBus is a queue for AnyEvent objects. In the Tweek codebase, an EventBus is utilised in the
AppState object that is passed as a mutable reference in several of the Displayable and Responder
methods that are implemented by many gui objects.

## Dispatching Events
> Incomplete


# Application Events

Tweek's events system has been reengineered to provide better support for front-end applications. Specifically, that
means creating the following:

* Object addressing scheme – The system has an ECS-like mechanism for assigning unique numbers to every view object so
  that an application can handle events sent by view objects and take action on other objects.
* Event listeners – Like other GUI programming frameworks, an object can add a listener for an AnyEvent type and provide
  a callback function to execute when the event has occurred.
* Notifications – A system for posting string notifications that can be received by a controller and handled as needed.
* View tags – The system allows you to set a magic number for any gui object, so that you can reference it
  as needed.

## Object Addressing Scheme

Tweek provides a system of assigning numeric id values for all gui components on system startup. This depends on
initializing the view hierarchy on startup through the `view_will_load()` trait method of Displayable. The view hierarchy
is fairly flat, since most gui objects belong to a Scene or a Timeline. Below is an example of the console
output from the `Stage::print_tree()` method:

```
/Scene-1000
/Scene-1000/Shape-1001
/Scene-1000/Shape-1002
/Scene-1000/Shape-1003
/Scene-1000/Timeline-1004
/Scene-1000/Timeline-1004/Shape-1005
/Scene-1000/Timeline-1004/Shape-1006
/Scene-1000/Timeline-1004/Shape-1007
/Scene-1000/Timeline-1004/Shape-1008
/Scene-1000/Timeline-1004/Shape-1009
/Scene-1000/Timeline-1004/Shape-1010
```

The `NodePath` object is responsible for encoding the array of NodeIDs that define an object's location in the Stage.
Each gui object has a Layer field which contains its NodePath. The AppState object also has a BTreeMap directory of all
NodePath objects and can be used for locating an object Layer so that a string representation of its NodePath enables
object addressing and manipulation of objects.


## Application/Stage/View Lifecycle

The `AppState` object is mostly responsible for storing and handling all kinds of state information. This includes:

* EventBus – A queue of EventBox objects, each of which contains an AnyEvent object.
* node_tree – This is a BTreeMap that stores the entire directory of gui nodes.
* observer_map – A registry of components that asked to be notified of a specific string notification
  (TBD. This is not really being used yet. )

The `StageContext` object is a member of Stage and it holds the event listeners that the system checks whenever an event
is received. Currently, it mainly listens for TweenEvents that are emitted from `Layer::tween_update()` when animation
state changes occur.

### Startup

During startup, an external application must call `Stage::stage_ready()`, which will trigger the
`view_will_load()` initialization process. During this process, the node_tree directory is populated through calls to
`AppState::append_node()` for every view object. In addition, the system will load any Layer `event_listeners` and
`queued_observers` into the corresponding hashmaps. The sender NodePath is also saved so that it can be used to identify
the sender/subscriber for a specific event.

For convenience, Layer has `add_listener()` and `add_observer()` methods which will temporarily store the added data
into the view Layer. During `view_will_load()`, these data stores are drained and moved into their respective locations
in StageContext and AppState.


## Event Listeners and Notifications

### add_listener

This is an example where a shape object wants to provide a callback function to execute when the system encounters the
specified event. Currently, the callback function is used to post a string notification that another part of the
application expects to receive. Other ways of using the callback may be discovered in the future. In the example
below, LINE_6_COMPLETED is a string constant that another part of the code will wait for.

```rust
    line_6.get_layer_mut().add_listener(
        &TweenEvent::Completed.to_string(),
        Box::new(move |app_state| {
            app_state.post_notification(LINE_6_COMPLETED);
        }),
    );
```

### Receiving notifications

In the front-end application, it is expected that the controller or view delegate will participate in the run loop and
execute the update() method. In particular, it needs to pass in mutable references to AppState and Stage. In this
example, the code is asking whether a specific string constant notification was posted. Also, additional lookups take
place to find the view based on a magic number tag and manipulate the underlying Layer.

> TODO: Provide helper method to simplify this lookup.

```rust
    if let Some(notification) = app_state.lookup_notification(LINE_6_COMPLETED) {
        if let Some(node_path) = app_state.find_node_by_tag(SHAPE_1) {
            if let Some(layer) = &mut stage.find_view_by_path(node_path.clone()) {
                layer.visibility = Visibility::Visible;
            }
        }
    }
```



# TODO/Improvements

* Implement mouse and button events using new architecture
* Move logic from notify() trait method to handle_event()
  * This will remove one of the 3 notification systems
