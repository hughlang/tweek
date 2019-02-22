/// Experiments with buttons
///
///
mod helper1;
use helper1::*;

extern crate ggez;
extern crate tweek;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::input::{ mouse};
use ggez::mint;

use std::env;
use std::path;

use tweek::prelude::*;

const SQUARE_ITEM_ID: usize = 100;
const BAR_WIDTH: f32 = 500.0;
const BUTTON_WIDTH: f32 = 60.0;
const BUTTON_GAP: f32 = 20.0;

struct StageHelper {}

#[allow(dead_code)]
#[allow(unused_mut)]
impl StageHelper {
    fn build_player_buttons(ctx: &mut Context) -> GameResult<Vec<ButtonView>> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let _font = graphics::Font::new(ctx, "/Roboto-Regular.ttf")?;

        let mut buttons: Vec<ButtonView> = Vec::new();
        let mut xpos = (screen_w - BAR_WIDTH)/2.0;

        let ypos = screen_h - 60.0;

        // ---- Skip Back ---------------------
        let frame = graphics::Rect::new(xpos, ypos, BUTTON_WIDTH, 32.0);
        let mut button = StageHelper::make_player_button(ctx, "/icons/md-skip-backward.png", frame)?;
        button.set_onclick(move |_action, _state| {
            // println!("Button onclick: action={:?}", action);

        });
        buttons.push(button);

        // ---- Play ---------------------
        xpos += BUTTON_WIDTH + BUTTON_GAP;
        let frame = graphics::Rect::new(xpos, ypos, BUTTON_WIDTH, 32.0);
        let mut button = StageHelper::make_player_button(ctx, "/icons/ios-play.png", frame)?;
        button.set_onclick(move |_action, _state| {
            println!("Button onclick: action={:?}", _action);

        });
        buttons.push(button);

        // ---- Skip Next ---------------------
        xpos += BUTTON_WIDTH + BUTTON_GAP;
        let frame = graphics::Rect::new(xpos, ypos, BUTTON_WIDTH, 32.0);
        let mut button = StageHelper::make_player_button(ctx, "/icons/md-skip-forward.png", frame)?;
        button.set_onclick(move |_action, _state| {
            // println!("Button onclick: action={:?}", action);

        });
        buttons.push(button);

        Ok(buttons)
    }

    fn make_player_button(ctx: &mut Context, file: &str, frame: graphics::Rect) -> GameResult<ButtonView> {
        let icon = graphics::Image::new(ctx, file.to_string())?;
        let mut button = ButtonView::new(frame).with_image(icon, 4.0);
        button.set_color(&graphics::Color::from_rgb_u32(0x999999));
        button.set_hover_animation(vec![color(0xFF8920)], 0.1);
        Ok(button)
    }

    /// This is a simple Sequence timeline where 4 independent tweens play sequentially,
    /// without any repeats.
    fn build_timeline_1() -> GameResult<(Timeline, Vec<ItemState>)> {
        let mut ypos = 50.0 as f32;
        let mut items: Vec<ItemState> = Vec::new();
        let mut tweens: Vec<Tween> = Vec::new();
        for i in 0..4 {
            let item_id = SQUARE_ITEM_ID + i as usize;
            let rect = graphics::Rect::new(50.0, ypos, 50.0, 50.0);
            let mut item1 = ItemState::new(item_id, Shape::Rectangle(rect))?;
            item1.layer.graphics.color = graphics::Color::from_rgb_u32(0x333333);

            let tween1 = Tween::with(item_id, &item1.layer)
                .to(vec![position(400.0, ypos as f64), size(80.0, 80.0)])
                .duration(0.5);
            ypos += 120.0;
            items.push(item1);
            tweens.push(tween1)
        }
        let timeline = Timeline::add(tweens)
            // .stagger(0.2)
            .align(TweenAlign::Sequence)
            ;
        Ok((timeline, items))
    }

    fn build_timeline_2() -> GameResult<(Timeline, Vec<ItemState>)> {
        let mut items: Vec<ItemState> = Vec::new();
        let mut tweens: Vec<Tween> = Vec::new();
        let mut ypos = 50.0 as f32;

        let rect = graphics::Rect::new(50.0, ypos, 50.0, 50.0);
        let mut item1 = ItemState::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.layer.graphics.color = graphics::Color::from_rgb_u32(0xCD09AA);

        let mut tween = Tween::with(SQUARE_ITEM_ID, &item1.layer)
            .to(vec![shift_x(400.0), shift_x(200.0), alpha(0.2)]).duration(1.0)
            .to(vec![shift_y(300.0), shift_x(-100.0), alpha(1.0)]).duration(0.5)
            .to(vec![position(200.0, 200.0), alpha(1.0)]).duration(0.5)
            .to(vec![size(200.0, 200.0)]).duration(1.0)
            // .repeat(4, 0.25)
            ;
        tweens.push(tween);

        items.push(item1);

        let timeline = Timeline::add(tweens)
            // .stagger(0.2)
            .align(TweenAlign::Sequence)
            ;
        Ok((timeline, items))

    }
}

struct MainState {
    tweek: Tweek,
    tk_state: TKState,
    items: Vec<ItemState>,
    buttons: Vec<ButtonView>,
    progress_bar: ProgressBarView,
}


impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let ypos = screen_h - 90.0;
        let xpos = (screen_w - BAR_WIDTH)/2.0;

        // Create progress bar
        let frame = graphics::Rect::new(xpos, ypos, BAR_WIDTH, 4.0);
        let mut progress = ProgressBarView::new(frame);
        progress.set_track_color(Color::from_rgb_u32(HexColors::MediumSlateBlue));
        progress.set_progress_color(Color::from_rgb_u32(HexColors::Azure));

        let buttons = StageHelper::build_player_buttons(ctx)?;

        // Here you can choose which timeline and animations to run
        // let (timeline, items) = StageHelper::build_timeline_1()?;
        let (timeline, items) = StageHelper::build_timeline_2()?;

        let mut tweek = Tweek::new();
        tweek.add_timeline(timeline);
        &tweek.play();

        let tk_state = TKState::new();

        let s = MainState {
            tweek: tweek,
            tk_state: tk_state,
            items: items,
            progress_bar: progress,
            buttons: buttons,
        };
        Ok(s)
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        self.tweek.update(&mut self.tk_state);
        let progress = self.tk_state.elapsed_time / self.tk_state.total_time;
        if progress <= 1.0 {
            self.progress_bar.set_progress(progress as f32);
        }

        for item in &mut self.items {
            item.try_update(&mut self.tweek)?;
        }

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
        for item in &mut self.items {
            item.render(ctx)?;
        }

        self.progress_bar.render(ctx)?;

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
