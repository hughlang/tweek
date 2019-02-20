/// Experiments with buttons
///
///
mod helper1;
use helper1::*;

extern crate ggez;
extern crate tweek;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::input::{ mouse};

use std::env;
use std::path;

use tweek::prelude::*;

const SQUARE_ITEM_ID: usize = 100;

struct MainState {
    tk_state: TKState,
    buttons: Vec<GGButton>,
    progress_bar: GGProgressBar,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // Add a rectangle
        let rect = graphics::Rect::new(0.0, 0.0, 50.0, 50.0);
        let mut item1 = ItemState::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.layer.graphics.color = graphics::Color::from_rgb_u32(0x333333);

        let mut tween1 = Tween::with(SQUARE_ITEM_ID, &item1.layer)
            .to(vec![position(400.0, 100.0), size(100.0, 100.0), alpha(0.2)])
            .duration(1.0)
            .repeat(7, 0.25)
            .yoyo();

        &tween1.play();
        item1.tween = Some(tween1);

        let font = graphics::Font::new(ctx, "/Roboto-Regular.ttf")?;

        let mut controls: Vec<GGButton> = Vec::new();

        let frame = graphics::Rect::new(300.0, 300.0, 120.0, 50.0);
        let mut button = GGButton::new(frame).with_title("Play")
            .with_props(&vec![color(0xCD09AA)]);
        button.set_font(&font, &24.0, &graphics::Color::from_rgb_u32(0xFFFFFF));
        button.set_color(&graphics::Color::from_rgb_u32(0x999999));
        button.set_on_hover(vec![color(0xFF8920)], 0.1);
        button.set_onclick(move |action, state| {
            println!("Button onclick: action={:?}", action);
        });

        controls.push(button);
        let frame = graphics::Rect::new(50.0, 500.0, 500.0, 8.0);
        let progress = GGProgressBar::new(frame);


        let items: Vec<ItemState> = Vec::new();
        // items.push(item1);
        let tk_state = TKState::new();

        let s = MainState {
            tk_state: tk_state,
            progress_bar: progress,
            buttons: controls,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for control in &mut self.buttons {
            control.update()?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for control in &mut self.buttons {
            control.render(ctx)?;
        }

        graphics::present(ctx)?;

        // self.frames += 1;
        // if (self.frames % 10) == 0 {
        //     println!("FPS: {}", ggez::timer::fps(ctx));
        // }

        // timer::yield_now();
        Ok(())
    }

    /// A mouse button was pressed
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        println!("Mouse down at: x={} y={}", x, y);
    }

    /// A mouse button was released
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        for button in &mut self.buttons {
            let did_click = button.handle_mouse_up(_x, _y, &mut self.tk_state);

        }

    }

    /// The mouse was moved; it provides both absolute x and y coordinates in the window,
    /// and relative x and y coordinates compared to its last position.
    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        for button in &mut self.buttons {
            if button.handle_mouse_at(x, y) {
                mouse::set_cursor_type(ctx, mouse::MouseCursor::Hand);
            } else {
                mouse::set_cursor_type(ctx, mouse::MouseCursor::Default);
            }
            // control.render(ctx)?;
        }

    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("tween0", "tweenkit")
        .window_setup(conf::WindowSetup::default().title("Tween test"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(1024.0, 768.0)
                .hidpi(true),
        )
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("HIDPI: {}", graphics::os_hidpi_factor(ctx));

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
