extern crate orbrender;
extern crate tween;

use tween::*;

use crate::{
    orbrender::events::*,
    orbrender::render_objects::{Rectangle, RenderObject, Text, Image},
    orbrender::structs::*,
    // orbrender::traits::Window,
    orbrender::window::WindowBuilder,
};

pub fn main() -> Result<(), String> {
    let mut window = WindowBuilder::new()
        .with_title("Tween :: movedemo")
        .with_background(Color::rgb(59, 67, 74))
        .with_size(Size::new(800.0, 600.0))
        .build();

    let rect1 = Rectangle::default()
            .with_size(Size::new(40.0, 40.0))
            .with_position(Point::new(10.0, 10.0))
            .with_background(Color::rgb(100, 123, 145));

    window.insert_rectangle( 0, rect1 );
    let mut running = true;
    let mut update = true;

    // while running {

    //     if update {
    //         window.render();
    //         update = false;
    //     }

    //     for event in window.events() {
    //         match event {
    //             Event::System(system_event) => match system_event {
    //                 SystemEvent::Quit => {
    //                     running = false;
    //                 }
    //                 _ => {}
    //             },
    //             _ => {}
    //         }
    //     }
    // }
    // rect1.t


    // let tween = Tween::new(&rect1);

    // tween.to(vec![move_x(10.0), move_y(10.0)])
    //     .duration(5.0).play();

    Ok(())
}