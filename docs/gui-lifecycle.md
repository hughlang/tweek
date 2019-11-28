# GUI Lifecycle

The Tweek architecture


## Overview

Traits
* Displayable
* Responder


### Goals

* Clear and defined setup and runtime management
* Scene changes should define preferred theme or inherit



## Initialisation

### set_theme

### notify

### layer.start_animation

This calls tween.play() and sets layer.animation with the tween



## Run Loop

### update


### render

## Fonts



## Animation

This is a sample of a simple animation that increases the size of rectangle graphic over time with a SineOut easing speed. It also repeats and has a yoyo effect.

```rust
    let rect = Rect::new(xpos, ypos, 0.0, 20.0);

    let mut item = Item::new(item_id, Shape::Rectangle(rect))?;
    item.layer.graphics.color = Color::from_hex(HexColors::Orange);

    let tween = Tween::with(item_id, &item.layer)
        .to(&[size(target_width as f64, 20.0)])
        .duration(1.0)
        .ease(Ease::SineOut)
        .repeat(8, 0.2).yoyo()
        ;
```

The best place to experiment with animations is in the gg_demos.rs example file, which showcases several animation scenarios.


### Tweenable wrapper
Tweek provides a trait called *Tweenable* that makes it simple to add support for other graphics frameworks. Implementing it is pretty minimal. Here is a copy of the trait code itself. Essentially, it requires that you implement a wrapper around your basic graphics object(s) so that the system can read or write a specified "Prop" value.

```rust
pub trait Tweenable {
    fn get_prop(&self, prop: &Prop) -> Prop;
    fn apply(&mut self, prop: &Prop);
    fn apply_props(&mut self, props: &[Prop]) {
        for prop in props {
            self.apply(prop);
        }
    }
}
```

And this is the [Layer wrapper for Quicksilver that implements Tweenable](https://github.com/rayet-inc/tweek/blob/master/src/ui/layer.rs). It reads and writes the values you specify.

