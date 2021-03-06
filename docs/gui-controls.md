# GUI Controls

> This document is not finished. Please ignore for now.

* How buttons execute actions
* How text editor works
* How listbox works

## Responders

A Responder is a Rust trait that extends Displayable (all gui components) to define how to respond to user actions such as mouse clicks and keyboard input. We often call these "controls" to differentiate from the basic "views" that do not have complex user interaction. (Note: Displayables can still respond to mouse hover events and that's all)

Below are some of the categories of responders and there is often overlap, where some controls span multiple categories.

* Mouse click handlers: Button, Checkbox, etc
* Input fields: TextField, TextArea


## Buttons

In other GUI development frameworks, it is usually easy to associate a button click with a function that executes an action that affects the application state and the display of objects. In javascript, you have an `onclick` attribute that you can attach a function to. In iOS, you can bind a button in the Interface Builder to a field in a controller class. You can attach a function to that button with ease. In both cases, all of the logic is (hopefully) easy to interpret. In Rust, however, any function you attach to a button as a callback is limited in scope. It can only manipulate the parameters provided in the callback, which does not have easy access to global scope.

In Tweek, a button has access to the AppState object as a mutable reference and we can add an event to its `EventBus`. These events are `AnyEvent` enums that might carry numeric parameters that can be interpreted at a higher level. At the higher level, at the application scope, the events in the EventBus are evaluated in every cycle of the run loop. A button click can send a generic NavEvent to tell the application to go Next or Back and the application will try to take action on it. Hence, there is a separation between what the button says and how it is interpreted. In another scenario, a button might trigger the display of a modal and it can only provide an event like "show modal" with a number to specify which modal. (And yes, this whole system is glued together with magic numbers)

To close the divide between the ambiguous button action and the desired result, the Scene struct provides convenience methods that chain together functions that effectively say:

1. Send this event enum and maybe some numeric parameters...
2. Which means to execute a designated PropSet (which is an animation directive)...
3. And this is what it means within the Scene scope, so make it so.

## ListBox

A ListBox (aka "listbox"), displays a collection of rows that each contain simple text. It is designed to provide a UI
for selecting rows, similar to an HTML select element.

* Scrolling
* Highlighting with animation
* Unselect row as a public function
* Multi select
* Themeable


### Scrolling

A listbox supports vertical scrolling of content if the vertical size of the rows exceeds the size of the listbox. This
presents many challenges because rows can be partially visible at the top and bottom of the listbox. More specifically,
these rows need to be clipped to prevent rendering of content outside of the listbox.

Each row is represented by a ListBoxRow component.

### Rendering Layers

The following components need to be drawn in order, from back to front.

* Background: the background is just a rectangle with a default color.
* Row borders: These can be drawn as meshes at runtime and cached. However, constant transforms could be annoying. These
  can also be uploaded to the GPU as a texture.
* Highlighting: A highlighted row has a different background color and the color change should support basic color
  animation with Tween. The highlight rectangle is drawn above the background layer as Shape
* Text: The text layer is the hardest because text needs to clipped if it is overflowing the listbox frame. The easiest
  way to solve this is to render all text as images and upload as a texture into the GPU. However, if the dataset is
  huge, it will cause performance issues. In addition, textures may not support color changes without serious GL
  scripting.

* Live text:
  * Might be used for selected rows but clipping doesn't work.

### State Model

The state model helps define what actions are allowed.

* Idle: You can select a row
* Scrolling
* Selected active/inactive

### Events

* Row selected
* Row deselected


### Caching

Text objects and the creation of meshes