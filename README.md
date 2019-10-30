# Tweek UI: A GUI framework for Rust with Animation

Tweek is a Rust framework for building cross-platform GUI applications in Rust, with an ongoing goal of building for WebAssembly and WebGL. The goal is to deliver the "write once, run anywhere" vision using a modern and safe toolchain.  Tweek was initially created as an animation framework for Rust, using the principles of Tween animation that were well-known in the early days of web animation, primarily in Flash development. More specifically, it was popularized by the [Tween and Timeline animation tools from Greensock](https://greensock.com/docs).

In this release (version 0.2.0), Tweek takes on the bigger challenge of becoming a complete UI framework. That includes
a suite of GUI components that actually work and can be used for building applications. Hence, that becomes a challenge
of creating an application framework where GUI controls can communicate with the overall application and participate in
a lifecycle where user actions are captured and translated into results.

## Status

Tweek is still under development and has not been released as a crate yet. In the initial 0.1.0 release, we
experimented with a number of backend graphics engines (SDL2, OrbTk, GGEZ) in order to prototype the animation and
display of shapes, images, and text. In the current 0.2.0 release, we migrated to Quicksilver for the backend, because
of its support of Wasm and WebGL. Even so, major changes to Quicksilver were required to support variable GPU
rendering pipelines and this release depends on a personal fork of Quicksilver.

In the next major release, we will migrate to yet another backend. Probably something based on the gfx -> rendy -> wgpu
chain of libraries. In the short time since early 2019 and now, the realm of graphics libraries on Rust has evolved
quickly and we should finally have the desired tech stack. The concrete goal with version 0.3.0 is to publish a
crate that will be stable and usable by anyone.

In other words, version 0.2.0 is an interim release that is meant to be feature complete. And yet, much of it will be
replaced in the next release. Hence, the code and documentation are not perfect, and many bugs still exist. However, the
hard part of making the GUI components work in a reliable way is mostly there. Since the graphics backend will get
replaced, it doesn't make sense to over-develop the current version.

## Goals

This architecure leans towards an [immediate mode GUI](https://en.wikipedia.org/wiki/Immediate_Mode_GUI) over retained
mode with an [Entity Component System (ECS)](https://en.wikipedia.org/wiki/Entity_component_system). The ECS frameworks
available today in Rust seem to demand full adoption of a particular architecture and are geared towards game
development. The goal of Tweek is to build a GUI framework for applications and game dev architectures have other interests in mind.

And yet, immediate mode GUI frameworks are often so barebones that you might question if they are usable in a real application.  Hence, the Tweek UI framework includes features that will hopefully be usable right away. For instance, the current codebase includes features that support the full application lifecycle including event notifications, UI transitions, and stateful data display and capture. It is also extensible, so that you can write your own components that conform to the required traits.

However, the endgame is quite clear. Rust is currently the language of choice when targeting WebAssembly (aka, Wasm) and Wasm-enabled browsers. Specifically, I'm talking about pure WebGL graphics and not a hybrid solution that requires HTML. When this is achieved, it will be possible to deliver the rich animations that work in nearly all browsers and bring back the "write once, run anywhere" vision.

## Examples
There are several demos you can try out in the examples directory that showcase various GUI and animation scenarios. Conveniently, most of them are bundled into a small set of files where you can browse demos with the Next and Previous buttons.

* GUI components
    * cargo run --example gui
    * cargo run --example animate


## Developer Notes

* To start, make sure you are using the latest stable Rust version. Currently 1.38.0
* [Docs](docs/README.md)

### Known Issues

* GPU rendering of text is working, but may still be buggy.

### Performance

* In release mode, frame rate is well over 120 fps. In debug mode, it is close to 60 fps.

## License

[See License file](LICENSE.txt)

## Author

* [Hugh Lang](@hughlang)