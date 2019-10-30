# Themes and Fonts

> This document is not finished. Please ignore for now.

The Theme struct has a huge influence on the entire GUI architecture and is pervasive in creating an app.


## Theme Definition

A Theme struct has a collection of fields that define an entire look and feel for a user interface. That includes a bunch of basic values the define colors and sizes of things in the UI. In addition, it holds pre-defined animation behaviors that are applied to the UI such as button hover and click behaviors. These are encapsulated in a PropSet object, which is used with the  Tween animation in Tweek core.

### Fonts

The Theme struct has a Fonts field, which is basically a storage wrapper for fonts that have been loaded by the system. Essentially, it's a HashMap for storing Vec<u8> bytes for loaded fonts, so they can be re-used later. The hashmap uses a String as the key and a few default strings have been defined so that a font data can be retrieved using its common name.

For example, to retrieve the bytes for the default font, you can do this:

	let data = theme.data_for_font(Theme::DEFAULT_FONT).clone();

The other pre-defined font names include: regular, bold, and italic. The Fonts wrapper only stores the fonts that have been added through the `Theme::add_named_font`	method which takes a string name and a path to load a truetype file from.


### DrawFont

A DrawFont is responsible for holding and drawing a single font, regardless of size or color. It is a wrapper around some GlyphBrush utilities that can calculate, draw, and render text. A DrawFont is initialized with font bytes obtained as a Vec<u8> data, sometimes obtained from the Fonts hashmap discussed above.

A Theme also has a `default_font`	DrawFont field that is accessed by various GUI components during the render lifecycle.
When a Theme is created, the `default_font`	field is initialised with a built-in Roboto font.

### Animation



## Theme Lifecycle

* AppDelegate
* set_theme
* render




## Theme Rules

### Button

The Button component has the most capabilities in the user interface and has special considerations for animation in Tweek.

* Hover
* Click

**Rules**

* Theme hover_effect applies to all buttons unless restricted by Layer.lock_style
* After click, restore to previous state



### Input Fields

* Background is not transparent
* Border should use theme border fields
* Hover animation may be used when mouse




## Theme Picker

