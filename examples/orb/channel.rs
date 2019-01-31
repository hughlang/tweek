extern crate orbrender;
extern crate tween;

use tween::*;
use orbrender::backend::Runner;
use orbrender::prelude::*;
use orbrender::render_objects::Rectangle;
use orbrender::window::WindowBuilder;
use std::time::{Duration};

use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;

// use crate::{
//     // orbrender::events::*,
//     // orbrender::render_objects::{Rectangle, RenderObject, Text, Image},
//     // orbrender::structs::*,
//     // orbrender::traits::Window,
//     orbrender::window::WindowBuilder,
// };

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
    &tween.play();
    let (tx, rx) = bounded::<Vec<UIState>>(0);

    std::thread::spawn(move || {
        loop {
            let updates = tween.get_updates();
            if &updates.len() > &0 {
                tx.send(updates).unwrap();
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    });

    state.index = window.insert_rectangle( square_id, rect1 );

    let arc_update = Arc::new(AtomicBool::new(true));

    Runner::new(Box::new(move || {
        if arc_update.load(atomic::Ordering::Acquire) {
            window.render();
            arc_update.store(false, atomic::Ordering::Release);
        }
        let rx_updates = rx.try_recv();
        if rx_updates.is_ok() {
            let updates = rx_updates.unwrap();
            for update in updates {
                if let Some(target) = window.get_mut_rectangle(update.id) {
                    target.render_update(&update.props);
                }
            }
            arc_update.store(true, atomic::Ordering::Release);
        }

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
        std::thread::sleep(Duration::from_millis(1));

        true
    })).run();



    // tween.to(vec![move_x(10.0), move_y(10.0)])
    //     .duration(5.0).play();

    Ok(())
}