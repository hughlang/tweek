//! The simplest possible example that does something.
mod shape_helper;
use shape_helper::*;

extern crate ggez;
extern crate tweek;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::input::{mouse};
use ggez::graphics::{self, DrawParam, Color};
use ggez::mint::{self};
use ggez::nalgebra as na;
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;
use tweek::prelude::*;


struct DemoHelper {}

#[allow(dead_code)]
#[allow(unused_mut)]
impl DemoHelper {

    fn make_buttons(ctx: &mut Context) -> GameResult<Vec<ButtonView>> {
        const BUTTON_WIDTH: f32 = 90.0;
        const BUTTON_HEIGHT: f32 = 40.0;
        const BUTTON_GAP: f32 = 20.0;
        let screen_w = ctx.conf.window_mode.width;

        let font = graphics::Font::new(ctx, "/Roboto-Bold.ttf")?;

        let mut buttons: Vec<ButtonView> = Vec::new();
        let xpos = 30.0;
        let ypos = 30.0;

        // ---- Previous ---------------------
        let frame = graphics::Rect::new(xpos, ypos, BUTTON_WIDTH, BUTTON_HEIGHT);
        let mut button = ButtonView::new(frame).with_title("Previous");
        button.set_font(&font, &18.0, &Color::from_rgb_u32(0xFFFFFF));
        button.set_color(&graphics::Color::from_rgb_u32(HexColors::Tan));
        button.set_hover_animation(vec![color(HexColors::Chocolate)], 0.1);
        button.set_onclick(move |_action, _state| {
            // println!("Button onclick: action={:?}", action);

        });
        buttons.push(button);

        // ---- Next ---------------------
        let frame = graphics::Rect::new(screen_w - BUTTON_WIDTH - 30.0, ypos, BUTTON_WIDTH, BUTTON_HEIGHT);
        let mut button = ButtonView::new(frame).with_title("Next");
        button.set_font(&font, &18.0, &Color::from_rgb_u32(0xFFFFFF));
        button.set_color(&graphics::Color::from_rgb_u32(HexColors::Tan));
        button.set_hover_animation(vec![color(HexColors::Chocolate)], 0.1);
        button.set_onclick(move |_action, _state| {
            // println!("Button onclick: action={:?}", action);

        });
        buttons.push(button);

        Ok(buttons)
    }
    fn build_arc_demo(ctx: &mut Context) -> GameResult<(Timeline, Vec<ItemState>)> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;
        let center_pt = mint::Point2{ x: screen_w/2.0, y: screen_h / 2.0 };
        let mut items: Vec<ItemState> = Vec::new();
        let mut tweens: Vec<Tween> = Vec::new();

        let dot_count = 8;
        let dot_radius = 10.0;
        let scene_radius = 96.0;

        for i in 0..dot_count {
            let item_id = i + 10 as usize;

            let mut item1 = ItemState::new(item_id, Shape::Circle(mint::Point2{x: center_pt.x, y: center_pt.y - scene_radius}, dot_radius))?;
            item1.layer.graphics.color = graphics::Color::from_rgb_u32(HexColors::Red);
            let alpha = 1.0 - (i as f32 / dot_count as f32)/2.0;
            item1.layer.graphics.color.a = alpha;
            item1.layer.graphics.offset = na::Point2::new(center_pt.x, center_pt.y);

            let tween1 = Tween::with(item_id, &item1.layer)
                .to(vec![rotate(360.0)])
                .duration(1.8)
                .ease(Ease::SineInOut)
                .repeat(-1, 0.8)
                ;
            items.push(item1);
            tweens.push(tween1)
        }

        let timeline = Timeline::add(tweens)
            .stagger(0.12)
            ;
        Ok((timeline, items))
    }

}

struct MainState {
    grid: graphics::Mesh,
    tweek: Tweek,
    tk_state: TKState,
    items: Vec<ItemState>,
    buttons: Vec<ButtonView>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let buttons = DemoHelper::make_buttons(ctx)?;
        let gridmesh = GGTools::build_grid(ctx, screen_w, screen_h, 32.0, graphics::Color::from_rgb_u32(0xCCCCCC))?;

        let (timeline, items) = DemoHelper::build_arc_demo(ctx)?;

        let mut tweek = Tweek::new();
        tweek.add_timeline(timeline);
        &tweek.play();

        let tk_state = TKState::new();

        let s = MainState {
            grid: gridmesh,
            tweek: tweek,
            tk_state: tk_state,
            items: items,
            buttons: buttons,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        self.tweek.update(&mut self.tk_state);

        for item in &mut self.items {
            item.try_update(&mut self.tweek)?;
        }
        for button in &mut self.buttons {
            button.update()?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(ctx, &self.grid, DrawParam::default())?;
        for button in &mut self.buttons {
            button.render(ctx)?;
        }

        self.tweek.update(&mut self.tk_state);

        for item in &mut self.items {
            item.render(ctx)?;
        }

        graphics::present(ctx)?;

        // timer::yield_now();
        Ok(())
    }

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
            let _did_click = button.handle_mouse_up(_x, _y, &mut self.tk_state);
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
        .window_mode(conf::WindowMode::default().dimensions(1024.0, 768.0).hidpi(false))
        .add_resource_path(resource_dir);


    let (ctx, events_loop) = &mut cb.build()?;

    println!("HIDPI: {}", graphics::os_hidpi_factor(ctx));

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
