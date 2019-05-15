> Notice: The Tweek library is currently undergoing major changes. We have removed support for GGEZ, as they have decided to drop support for MacOS. In the current version 0.2.0, we have successfully migrated to Quicksilver. However, some of the examples have not been migrated yet. Specifically, the timelines

# Tweek: A Tween Animation Kit for Rust

"Tween" is a term that was well-known in the early days of web animation, primarily in Flash development. More specifically, it was popularized by the [Tween and Timeline animation tools from Greensock](https://greensock.com/docs). Originally created for Flash and ActionScript, the Tween tools made it easy for any developer to build rich animations with minimal effort. Later, as Flash usage declined, the Greensock team ported the entire codebase to Javascript and HTML.


## Goals

Hence, this project aspires to deliver the same level of power and simplicity to the Rust community. That's a very ambitious goal, given that Rust has a steep learning curve and can be very intimidating because of its ultra-strict rules and syntax. Moreover, graphical user interfaces in Rust are still being developed and many GUI frameworks are not ready to build on top of.

However, the endgame is quite clear to me. Rust is currently the language of choice when targeting WebAssembly (aka, Wasm) and Wasm-enabled browsers. Specifically, I'm talking about pure WebGL graphics and not a hybrid solution that requires HTML. When this is achieved, it will be possible to deliver the rich animations that work in nearly all browsers and bring back the "write once, run anywhere" vision.

## Examples
There are several demos you can try out in the examples directory that showcase various animation scenarios. Conveniently, most of them are bundled into a small set of files where you can browse demos with the Next and Previous buttons.

* Basic Tween animations
    * cargo run --example basics
* Work-in-progress GUI library
    * cargo run --example gui

To learn more about these examples, please also read the following pages:

* [Basics](docs/1-basics.md)


## Basic Usage

* To start, make sure you are using the latest stable Rust version. Currently 1.34.0

This is a sample of a simple animation that increases the size of rectangle graphic over time with a SineOut easing speed. It also repeats and has a yoyo effect.

```rust
    let rect = Rect::new(xpos, ypos, 0.0, 20.0);

    let mut item = Item::new(item_id, Shape::Rectangle(rect))?;
    item.layer.graphics.color = Color::from_rgb_u32(HexColors::Orange);

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
    fn apply_updates(&mut self, props: &[Prop]) {
        for prop in props {
            self.apply(prop);
        }
    }
}
```

And this is the [TweenLayer wrapper for Quicksilver that implements Tweenable](https://github.com/wasm-network/tweek-rust/blob/master/src/quicksilver_ui/layer.rs). It reads and writes the values you specify.


### UI Components

By itself, the Tweenable trait provides the basic support needed for Tweek integration. However, it's hard to resist going further and adding as many helpers and utilities to make UI building easier. And so, Tweek provides a number of experimental View components in the quicksilver_ui folder. Things like buttons, labels, images, and a progress bar. These are almost done, but not very pretty yet.

Naturally, these components also have Tweenable graphics, which provides "internal tweening" capabilities for different behaviors. For example, the Button struct can animate a color change for the *on hover* event, like this:

```rust
        let mut button = Button::new(frame).with_title("Previous");
        button.set_color(&Color::from_rgb_u32(HexColors::Tan));
        button.set_hover_animation(&[color(HexColors::Chocolate)], 0.1);
        button.set_onclick(move |_action, tk| {
            tk.commands.push(PREV_COMMAND);
        });
```



### Performance

* In release mode, frame rate is well over 120 fps. In debug mode, it is close to 60 fps.



## Developer Notes


### Known Issues

* GPU rendering of text is working, but may still be buggy.


### Unit Tests (TODO)

[ ] Test that end state props are expected based on forward or reverse time_scale

### Contributing

More details to come.


## License

[See License file](LICENSE.txt)

## Author

Hugh Lang