# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/wasm-network/tweek-rust/compare/0.1.0...HEAD

### Changes to come
– Replace Quicksilver with a more complete graphics backend.
– Redesign the Theme system
– Fix timeline animation

## [0.2.0] - 2019.10.31

### Added
– Suite of GUI components including buttons, text/labels, text field, text area, list box, and checkboxes.
– GPU font rendering using glyph_brush and rusttype.
– EventBus notifications with variable AnyEvent types
– Scene struct which acts as a parent object for a collection of view objects and controls.
– Theme support for all components (WIP)
– Prototype AppDelegate that translates Quicksilver run loop events/calls to Tweek
– Support for WebAssembly and WebGL display (WIP)
– Instrumentation that prints detailed info about GUI layout, styling, and animation.

### Changed
– Added Quicksilver as the backend 2D graphics engine (plus windowing, inputs, etc) to replace GGEZ.
– Timeline animation is somewhat broken now.

### Removed
– GGEZ was removed as a dependency.

## [0.1.0] - 2019-02-28

This was a prototype release to demonstrate Tween animation using Rust. The "core" module consists of a state machine
for calculating interpolations over time for a given set of animation directives (e.g., position, size, color,
rotation).

### Added
- Prototype animation framework using Tween principles found in Greensock AS3.

