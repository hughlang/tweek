# Instrumentation

## Overview

* Scene debugging
* Tween animations

## Goals

* Print scene hierachy to console based on-demand special mouse and keyboard actions.
* Show values for each view: layer_id, type, frame, props, etc
* Non-repetitive. Only print changed objects once.
* Use EventBus to publish logging events

# Scene hierarchy

**Example:**

			<Scene> [0] Pos(20.0,80.0) Size(760.0,500.0)
			| <Label> [1] Pos(40.0,180.0) Size(180.0,40.0)
			| <Button> [2] Pos(40.0,230.0) Size(100.0,50.0)
			| <Checkbox> [3] Pos(40.0,290.0) Size(200.0,40.0)
			| <OptionGroup> [4] Pos(40.0,340.0) Size(200.0,105.0)
			| | <Checkbox> [0] Pos(40.0,350.0) Size(200.0,20.0)
			| | <Checkbox> [1] Pos(40.0,380.0) Size(200.0,20.0)
			| | <Checkbox> [2] Pos(40.0,410.0) Size(200.0,20.0)
			| | <Checkbox> [3] Pos(40.0,440.0) Size(200.0,20.0)

Each Displayable should implement a method that outputs its debug output string. A Scene hierarchy should be a maximum
of 3 levels deep. However, that's not a hard rule. Nonetheless, any object that contains another Displayable object is
responsible for printing it's layer info in the format above. For example, an OptionGroup has a collection of Checkboxes
and it should print out information for each checkbox. The Scene will have a method to iterate through all of its
objects and print them out on request.

- [x] Completed


# Animation events

**Objectives:**

* Starting with a Displayable, provide set_debug(bool) which modifies the layer.debug value. Any animation for the
  object will have detailed trace logging.
* Better state tracking: pending, starting, running, finishing, finished
* Better interpolation printing through the returned PropSet

## Notifications

The EventBus architecture is also used for the Notifications system that is designed for event handling at the Displayable/Layer level. If you recall, a Displayable is a graphical component that can receive calls during the run loop to update and render itself. Each Displayable has a Layer that composes much of the properties and functionality for component rendering. This also includes an optional Tween animation property that is responsible for animations. 

One of the challenges of building a GUI where things can animate and move is ensuring that the animation begins and ends successfully. In Rust and in this code, it is difficult to bind functions that need to manipulate external scope and make the graphical changes. In this document, we address how the Notifications system helps to communicate events from the lowest level where Tween animations are calculated to higher level Displayable objects.

A Tween animation comprises a list of Props to change in a specified timespan, such as position, color, etc. During the animation, a Tweenable layer rapidly calculates the changes to its display using the Tween object. At the end of the animation duration, a signal must be received so that the Displayable can record its final state (or reset it to its original state, in the case of a Hover or Click animation).



This is an AnyEvent that is designed to carry an event from a lower level and combined with other data at a higher
level. For example, a Tween object does not know anything about its parent Layer or Displayable and therefore, it cannot
emit useful logging data.

### LogEvent

A LogEvent is generically a message from a lower level of the code placed on the EventBus to get logged at a higher
level. This will allow for better control of what gets logged. For example, a Layer may have debug = true and thus it
can decide what gets logged.

## GPU Rendering

**Objectives:**

* Collect debug messages from GPU and record the count of sequential duplicates.
* Better GPU event lifecycle tracking so that new GPU tasks are identified and reported back to Tweek.


## Debug lifecycle

* Create scene with id
*
* Set theme
* On notify ready