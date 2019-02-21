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


struct MainState {
    items: Vec<ItemState>,
    buttons: Vec<ButtonView>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        const BUTTON_WIDTH: f32 = 80.0;

        let font = graphics::Font::new(ctx, "/Roboto-Regular.ttf")?;

        let mut controls: Vec<ButtonView> = Vec::new();

        let mut ypos = screen_h;
        let mut xpos = screen_w/2.0 - BUTTON_WIDTH - 20.0;

        ypos -= 60.0;
        // Create play button
        let frame = graphics::Rect::new(xpos, ypos, 80.0, 32.0);
        let mut button = ButtonView::new(frame).with_title("Replay")
            .with_props(&vec![color(0xCD09AA)]);
        button.set_font(&font, &16.0, &graphics::Color::from_rgb_u32(0xFFFFFF));
        button.set_color(&graphics::Color::from_rgb_u32(0x999999));
        button.set_hover_animation(vec![color(0xFF8920)], 0.1);

        controls.push(button);

        xpos = screen_w/2.0 + 20.0;

        let icon = graphics::Image::new(ctx, "/icons/ios-pause.png")?;

        let frame = graphics::Rect::new(xpos, ypos, 80.0, 32.0);
        let mut button = ButtonView::new(frame).with_image(icon, 4.0);
        button.set_hover_animation(vec![color(HexColors::DarkGray)], 0.1);
        button.set_color(&graphics::Color::from_rgb_u32(0x999999));
        controls.push(button);

        let items: Vec<ItemState> = Vec::new();
        // items.push(item1);

        let s = MainState {
            items: items,
            buttons: controls,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for item in &mut self.items {
            item.update()?;
        }

        for control in &mut self.buttons {
            control.update()?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for item in &mut self.items {
            item.render(ctx)?;
        }

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
