# Tweek: A Simple Animation Kit for Rust

This is my attempt to write a Tween animation library for Rust. What's a "Tween", you ask? I can tell you it definitely isn't (this)[https://www.merriam-webster.com/dictionary/tween]. The term "tween" was well-known in the early days of web animation, primarily in Flash development. 






## Examples



* cargo run --bin ggmix
* cargo run --bin ggsequence


## UI Components and Widgets

The ggez_support folder has a number of custom components which are mostly designed to help with the examples. However, they also show how far you can go in building custom UIs with Tweek using GGEZ.


## Developer Notes


### Known Issues

* MacOS
  * A screen scaling and offset issue exists in ggez and only 1024x768 window size works accurately.
	
### Unit Tests (TODO)	
	
[ ] Test that end state props are expected based on forward or reverse time_scale
	
### Contributing

More details to come.	