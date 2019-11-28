# Tweek UI: A GUI framework for Rust with Animation

Tweek is a Rust framework for building cross-platform GUI applications, with an ongoing goal of building for WebAssembly and WebGL. The goal is to deliver the "write once, run anywhere" vision using a modern and safe toolchain.  Tweek was initially created as an animation framework for Rust, using the principles of Tween animation that were well-known in the early days of web animation, primarily in Flash development. More specifically, it was popularized by the [Tween and Timeline animation tools from Greensock](https://greensock.com/docs).

In this release (version 0.2.0), Tweek takes on the bigger challenge of becoming a complete UI framework. That includes
a suite of GUI components that actually work and can be used for building applications. Hence, that becomes a challenge
of creating an application framework where GUI controls can communicate with the overall application and participate in
a lifecycle where user actions are captured and translated into results.

## Status

Tweek is still under active development and has not been released as a crate yet. In the initial 0.1.0 release, we
experimented with a number of backend graphics engines (SDL2, OrbTk, GGEZ) in order to prototype the animation and
display of shapes, images, and text. In the current 0.2.0 release, we migrated to Quicksilver for the backend, because
of its support of Wasm and WebGL. Even so, major changes to Quicksilver were required to support variable GPU
rendering pipelines and this release depends on a personal fork of Quicksilver.

In the next major release, we will migrate to yet another backend. Probably something based on the gfx -> rendy -> wgpu
chain of libraries. In the short time since early 2019 and now (Oct 2019), the realm of graphics libraries on Rust has evolved
quickly and we should finally have the desired tech stack. The concrete goal with version 0.3.0 is to publish a
crate that will be stable and usable by anyone.

In other words, version 0.2.0 is an interim release that is meant to be feature complete, but much of the backend will be
replaced in the next release. Hence, the code and documentation are not perfect, and many bugs still exist. However, the
hard part of making the GUI components work in a reliable way is mostly there. Since the graphics backend will get
replaced, it doesn't make sense to over-develop the current version.

## Goals

* A complete suite of fully themeable GUI components
* ... with the ability to animate anything using simple function parameters
* ... that integrates into a complete application framework
* ... which supports event handling and stateful data capture
* ... that can run on any operating system and even in a web browser
* ... and performs well.

This architecure resembles an [immediate mode GUI](https://en.wikipedia.org/wiki/Immediate_Mode_GUI) with a full
application model, instead of a retained mode GUI with an [Entity Component System
(ECS)](https://en.wikipedia.org/wiki/Entity_component_system). The ECS frameworks available today in Rust seem to demand
full adoption of a particular architecture and most are geared towards game development.

However, immediate mode GUI frameworks are often so barebones that you might question if they are usable in a real
application.  Hence, the Tweek UI framework includes features that will hopefully make it usable right away. For instance,
the current codebase provides features that support the full application lifecycle including event notifications, UI
transitions, and stateful data display and capture. The examples use an AppDelegate model that can be adapted as needed.
It is also extensible, so that you can write your own components that conform to the required traits.

## Examples
There are several demos you can try out in the examples directory that showcase various GUI and animation scenarios.
Conveniently, most of them are bundled into a small set of files where you can browse demos with the Next and Previous
buttons. These serve as a good example of how real applications can be developed using Tweek.

* cargo run --example gui
* cargo run --example animate
* cargo run --example modals
    * *this demo is limited*

The examples can also run via WebAssembly, but there is a crashing bug that still needs to be fixed. Still, you can at
least see the first demo. Use the bash script (if you are not on windows), like this:

* ./runweb.sh gui
* ./runweb.sh animate
* ./runweb.sh modals

For windows, use the full command: `cargo web start --target=wasm32-unknown-unknown --auto-reload --example gui`


## Developer Notes

* To start, make sure you are using the latest stable Rust version. Currently 1.38.0
* [Developer Docs](docs/README.md)
* [Change Log](CHANGELOG.md)

### Performance

In release mode, frame rate is well over 120 fps. In debug mode, it is close to 60 fps (on a very old macbook pro). On a
faster PC with a decent graphics card, you easily get >120 fps in debug mode.

Due to the need for Wasm compatibility, the code does not make use of any async threads, but this is something we would
like to support in the next release for non-wasm.

## License

[See License file](LICENSE.txt)

## Author

* [Hugh Lang](https://github.com/hughlang)