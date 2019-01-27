use orbrender;
use orbrender::backend::Runner;
use orbrender::prelude::*;
use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;

fn main() {
    orbrender::initialize();

    // #[cfg(target_arch = "wasm32")]
    // let res_path = "res/";

    // #[cfg(not(target_arch = "wasm32"))]
    // let res_path = "static/res/";

    let mut window = WindowBuilder::new()
        .with_title("OrbRender - drawing example")
        .with_background(Color::rgb(59, 67, 74))
        .with_size(Size::new(800.0, 600.0))
        .build();

    window.insert_rectangle(
        0,
        Rectangle::default()
            .with_size(Size::new(40.0, 40.0))
            .with_position(Point::new(10.0, 10.0))
            .with_background(Color::rgb(100, 123, 145)),
    );

    window.insert_rectangle(
        1,
        Rectangle::default()
            .with_size(Size::new(40.0, 40.0))
            .with_position(Point::new(60.0, 10.0))
            .with_background(Color::rgb(100, 123, 145))
            .with_border_color(Color::rgb(173, 179, 184))
            .with_border_thickness(
                Thickness::default()
                    .with_left_right(4.0)
                    .with_top_bottom(8.0),
            )
            .with_radius(4.0),
    );

    // z index example
    window.insert_rectangle_with_z(
        2,
        Rectangle::default()
            .with_size(Size::new(20.0, 20.0))
            .with_position(Point::new(120.0, 20.0))
            .with_background(Color::rgb(248, 222, 97)),
        10,
    );

    window.insert_rectangle(
        3,
        Rectangle::default()
            .with_size(Size::new(40.0, 40.0))
            .with_position(Point::new(110.0, 10.0))
            .with_background(Color::rgb(100, 123, 145)),
    );

    window.insert_text(
        4,
        Text::default()
            .with_text("OrbRender")
            .with_position(Point::new(10.0, 80.0))
            .with_foreground(Color::rgb(204, 222, 237))
            .with_font(FontConfig::default().with_family("Roboto").with_size(22.0)),
    );

    // window.insert_image(
    //     5,
    //     Image::default()
    //         .with_position(Point::new(10.0, 100.0))
    //         .with_source(format!("{}orbtk-space.png", res_path)),
    // );

    let update = Arc::new(AtomicBool::new(true));

    Runner::new(Box::new(move || {
        if update.load(atomic::Ordering::Acquire) {
            window.render();
            update.store(false, atomic::Ordering::Release);
        }

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
}
