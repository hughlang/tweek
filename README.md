# Tweek: A Simple Animation Kit for Rust

This is my attempt to write a Tween animation library for Rust. What's a "Tween", you ask? I can tell you it definitely isn't (this)[https://www.merriam-webster.com/dictionary/tween]. The term "tween" was well-known in the early days of web animation, primarily in Flash development. 



## Support for GGEZ


### Tweenable wrapper



### UI Components and Widgets

The ggez_support folder has a number of custom components which are mostly designed to help with the examples. However, they also show how far you can go in building custom UIs with Tweek using GGEZ.



## Examples



* cargo run --bin ggmix
* cargo run --bin ggease
* cargo run --bin gg_player

### Performance

* In release mode, frame rate is over 120 fps. In debug mode, it is close to 60 fps, unless you are animating text.



## Developer Notes


### Known Issues

* MacOS
  * A screen scaling and offset issue exists in ggez and only 1024x768 window size works accurately.
	
### Unit Tests (TODO)	
	
[ ] Test that end state props are expected based on forward or reverse time_scale
	
### Contributing

More details to come.	