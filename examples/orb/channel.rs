extern crate orbrender;
extern crate tween;

use tween::*;
use orbrender::backend::Runner;
use orbrender::prelude::*;
use std::time::{Duration, Instant};

use crossbeam_utils::thread;

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

    let square_id = 0;
    let rect1 = Rectangle::default()
            .with_size(Size::new(40.0, 40.0))
            .with_position(Point::new(10.0, 10.0))
            .with_background(Color::rgb(100, 123, 145));

    let mut tween = Tween::animate(&rect1, vec![position(300.0, 100.0)]).duration(4.0);

    let (tx, rx) = bounded::<Vec<Prop>>(1);

    thread::scope(move |scope| {
        scope.spawn(move |_| {
            &tween.play();
        });
    }).unwrap();

    state.index = window.insert_rectangle( square_id, rect1 );

    let update = Arc::new(AtomicBool::new(true));

    Runner::new(Box::new(move || {
        if update.load(atomic::Ordering::Acquire) {
            window.render();
            update.store(false, atomic::Ordering::Release);
        }

        // if let Some(target) = window.get_mut_rectangle(square_id) {
        //     &tween.render(target, &square_id);
        //     update.store(true, atomic::Ordering::Release);
        // }

        for event in window.events() {
            match event {
                Event::System(system_event) => match system_event {
                    SystemEvent::Quit => {
                        return false;
                    }
                    // _ => {}
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