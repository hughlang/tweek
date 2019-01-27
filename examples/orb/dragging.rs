use orbrender;
use orbrender::backend::Runner;
use orbrender::prelude::*;
// use time::PreciseTime;

use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;

struct State {
    pub mouse_delta: (f64, f64),
    pub mouse_position: (f64, f64),
    pub rect: (f64, f64, f64, f64),
    pub rect_pressed: bool,
    pub rect_index: usize,
}

fn main() {
    orbrender::initialize();

    let mut window = WindowBuilder::new()
        .with_title("OrbRender - dragging example")
        .with_background(Color::rgb(59, 67, 74))
        .with_size(Size::new(800.0, 600.0))
        .build();

    let mut example_state = State {
        mouse_delta: (0.0, 0.0),
        mouse_position: (0.0, 0.0),
        rect: (10.0, 40.0, 40.0, 40.0),
        rect_index: 0,
        rect_pressed: false,
    };

    example_state.rect_index = window.insert_rectangle(
        0,
        Rectangle::default()
            .with_size(Size::new(example_state.rect.2, example_state.rect.3))
            .with_position(Point::new(example_state.rect.0, example_state.rect.1))
            .with_background(Color::rgb(100, 123, 145)),
    );

    window.insert_text(
        1,
        Text::default()
            .with_text("Drag the rectangle")
            .with_position(Point::new(10.0, 20.0))
            .with_foreground(Color::rgb(204, 222, 237))
            .with_font(FontConfig::default().with_family("Roboto").with_size(14.0)),
    );

    let update = Arc::new(AtomicBool::new(true));

    Runner::new(Box::new(move || {
        if update.load(atomic::Ordering::Acquire) {
            window.render();
            update.store(false, atomic::Ordering::Release);
        }

        for event in window.events() {
            match event {
                Event::Mouse(event) => match event {
                    MouseEvent::Button { button, state } => {
                        if button == MouseButton::Left && state == ElementState::Pressed {
                            let rect = &example_state.rect;
                            let pos = &example_state.mouse_position;

                            if pos.0 >= rect.0
                                && pos.0 <= rect.0 + rect.2
                                && pos.1 >= rect.1
                                && pos.1 <= rect.1 + rect.3
                            {
                                example_state.rect_pressed = true;
                                if let Some(rect) =
                                    window.get_mut_rectangle(example_state.rect_index)
                                {
                                    rect.background = Some(Color::rgb(248, 222, 97));
                                    update.store(true, atomic::Ordering::Release);
                                }
                            } else {
                                example_state.rect_pressed = false;
                                example_state.mouse_delta = (0.0, 0.0);
                            }
                        } else if button == MouseButton::Left && state == ElementState::Released {
                            if example_state.rect_pressed {
                                if let Some(rect) = window.get_mut_rectangle(example_state.rect_index) {
                                    rect.background = Some(Color::rgb(100, 123, 145));
                                    update.store(true, atomic::Ordering::Release);
                                }
                            }
                            example_state.rect_pressed = false;

                        }
                    }
                    MouseEvent::Move(point) => {
                        let mouse_pos = &example_state.mouse_position;
                        example_state.mouse_delta = (mouse_pos.0 - point.x, mouse_pos.1 - point.y);
                        example_state.mouse_position = (point.x, point.y);
                        if example_state.rect_pressed {
                            example_state.rect.0 -= example_state.mouse_delta.0;
                            example_state.rect.1 -= example_state.mouse_delta.1;

                            let window_size = window.size();

                            if example_state.rect.0 + example_state.rect.2 > window_size.width {
                                example_state.rect.0 = window_size.width - example_state.rect.2;
                            } else if example_state.rect.0 < 0.0 {
                                example_state.rect.0 = 0.0;
                            }

                            if example_state.rect.1 + example_state.rect.3 > window_size.height {
                                example_state.rect.1 = window_size.height - example_state.rect.3;
                            } else if example_state.rect.1 < 0.0 {
                                example_state.rect.1 = 0.0;
                            }

                            if let Some(rect) = window.get_mut_rectangle(example_state.rect_index) {
                                rect.position.x = example_state.rect.0;
                                rect.position.y = example_state.rect.1;
                                update.store(true, atomic::Ordering::Release);
                            }
                        }
                    }
                    // _ => {}
                },
                Event::System(system_event) => match system_event {
                    SystemEvent::Quit => {
                        return false;
                    }
                    // _ => {}
                },
                // _ => {}
            }
        }

        true
    }))
    .run();
}
