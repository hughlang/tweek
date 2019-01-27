extern crate orbrender;
extern crate tween;

use tween::*;
use orbrender::backend::Runner;
use orbrender::prelude::*;

use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;

use crate::{
    orbrender::events::*,
    orbrender::render_objects::{Rectangle, RenderObject, Text, Image},
    orbrender::structs::*,
    // orbrender::traits::Window,
    orbrender::window::WindowBuilder,
};

struct AnimState {
    pub frame: (f64, f64, f64, f64),
    pub index: usize,
}

pub fn main() -> Result<(), String> {
    // let
    let mut window = WindowBuilder::new()
        .with_title("Tween :: movedemo")
        .with_background(Color::rgb(59, 67, 74))
        .with_size(Size::new(800.0, 600.0))
        .build();

    let mut state = AnimState {
        frame: (10.0, 40.0, 40.0, 40.0),
        index: 0,
    };

    let rect1 = Rectangle::default()
            .with_size(Size::new(40.0, 40.0))
            .with_position(Point::new(10.0, 10.0))
            .with_background(Color::rgb(100, 123, 145));

    let tween = Tween::animate(&rect1, vec![move_x(10.0), move_y(10.0)]).duration(3.0);
    state.index = window.insert_rectangle( 0, rect1 );

    let update = Arc::new(AtomicBool::new(true));


    Runner::new(Box::new(move || {
        if update.load(atomic::Ordering::Acquire) {
            window.render();
            update.store(false, atomic::Ordering::Release);
        }
        tween.update();

        for event in window.events() {
            match event {
                Event::System(system_event) => match system_event {
                    SystemEvent::Quit => {
                        return false;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        true
    })).run();



    // tween.to(vec![move_x(10.0), move_y(10.0)])
    //     .duration(5.0).play();

    Ok(())
}