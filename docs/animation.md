# Animation

The Tweek project was originally created to provide animation capabilities for any kind of display object. This code is
found within the `/src/core` folder.

## Tween

Tweek was initially created as an animation framework for Rust, using the principles of Tween animation that were
well-known in the early days of web animation, primarily in Flash development. More specifically, it was popularized by
the [Tween and Timeline animation tools from Greensock](https://greensock.com/docs).

In this codebase, Tween refers to the Tween struct defined in [tween.rs](src/core/tween.rs), which has the responsibility for
managing animation transition for a single UI object, from one set of properties to another, within a defined number of
seconds. In some cases, the animation may be defined in multiple steps that execute sequentially. A Tween itself knows
almost nothing about the object it represents besides the visual Props that are being animated. Hence, it is a simple
time-based state machine for calculating visual Props.

## Tween Functions

A Tween Prop function is a Rust function that returns a Prop and an animation for an object can be defined with one or more
Prop functions. In the code snipped below, `position` and `size` are two Tween functions that specify the animation
changes to apply during a 0.5 second interval.

```rust
    let tween = Tween::with(id, &label.layer)
        .to(&[position(100.0, 200.0), size(120.0, 120.0)])
        .duration(0.5)
        .ease(Ease::SineInOut);
```

## Props

Props are animatable properties, such as position, size, color, etc. The Tween


## Timeline


