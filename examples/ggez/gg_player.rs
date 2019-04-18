/// This demo tries to showcase a large variety of Timeline animation scenarios and provides
/// player controls to help you review the animations in detail. The other examples provided
/// generally demonstrate simple Tween animations without a timeline.
///
/// STATUS: This example was created before gg_basics and gg_demos and therefore is a little outdated.
/// For now, this file is used for testing timeline stuff. It needs:
/// * a Next/Previous browsing scheme
/// * Buttons for slower, faster, replay that work.
///
///
mod shape_helper;
use shape_helper::*;

extern crate ggez;
extern crate tweek;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self, Color};
use ggez::input::mouse;
use ggez::{Context, ContextBuilder, GameResult};
// use ggez::mint;

use std::env;
use std::path;

use tweek::prelude::*;

const SQUARE_ITEM_ID: usize = 100;
const BAR_WIDTH: f32 = 500.0;
const BUTTON_WIDTH: f32 = 60.0;
const BUTTON_GAP: f32 = 20.0;

/// The StageHelper generates most of the objects used in this demo. There are helper functions for creating
/// the player buttons and progress bar. And there are helper functions that create different animation scenarios
/// that you can test by editing the MainState code below StageHelper and picking the function you want to try.
/// Notable differences from animating a standalone Tween:
/// * Don't call tween.play().  Instead, call tweek.play() after adding one or more timelines to it.
/// * Don't call tween.tick().  Instead, call tweek.update() in the run loop.
/// * The player buttons are instances of ggez_support::ButtonView, which implement Displayable and thus have update()
/// and render() functions. They are also wrappers for the Tweenable GGLayer, so they support internal Tween animations.
/// They also implement TKResponder, so they support the update() method which allows a TKState
/// object to be passed around and mutated.
struct StageHelper {}

#[allow(dead_code)]
#[allow(unused_mut)]
impl StageHelper {
    fn build_player_buttons(ctx: &mut Context) -> GameResult<Vec<ButtonView>> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let _font = graphics::Font::new(ctx, "/Roboto-Regular.ttf")?;

        let mut buttons: Vec<ButtonView> = Vec::with_capacity(3);
        let mut xpos = (screen_w - BAR_WIDTH) / 2.0;

        let ypos = screen_h - 60.0;

        // ---- Skip Back ---------------------
        let frame = graphics::Rect::new(xpos, ypos, BUTTON_WIDTH, 32.0);
        let mut button =
            StageHelper::make_player_button(ctx, "/icons/md-skip-backward.png", frame)?;
        button.set_onclick(move |_action, _state| {
            log::trace!("Button onclick: action={:?}", _action);
        });
        buttons.push(button);

        // ---- Play ---------------------
        xpos += BUTTON_WIDTH + BUTTON_GAP;
        let frame = graphics::Rect::new(xpos, ypos, BUTTON_WIDTH, 32.0);
        let mut button = StageHelper::make_player_button(ctx, "/icons/ios-play.png", frame)?;
        button.set_onclick(move |_action, tk| {
            log::trace!("Button onclick: action={:?}", _action);
            tk.requests.push(TKRequest::Play);
        });
        buttons.push(button);

        // ---- Skip Next ---------------------
        xpos += BUTTON_WIDTH + BUTTON_GAP;
        let frame = graphics::Rect::new(xpos, ypos, BUTTON_WIDTH, 32.0);
        let mut button = StageHelper::make_player_button(ctx, "/icons/md-skip-forward.png", frame)?;
        button.set_onclick(move |_action, _state| {
            log::trace!("Button onclick: action={:?}", _action);
        });
        buttons.push(button);

        Ok(buttons)
    }

    fn make_player_button(
        ctx: &mut Context,
        file: &str,
        frame: graphics::Rect,
    ) -> GameResult<ButtonView> {
        let icon = graphics::Image::new(ctx, file.to_string())?;
        let mut button = ButtonView::new(frame).with_image(icon, 4.0);
        button.set_color(&graphics::Color::from_rgb_u32(0x999999));
        button.set_hover_animation(&[color(0xFF8920)], 0.1);
        Ok(button)
    }

    //---- 1 ----------------------------------------------------------------------
    /// This is a simple Sequence timeline where 4 independent tweens play sequentially,
    /// without any repeats.
    fn build_timeline_1(_ctx: &mut Context) -> GameResult<(Timeline, Vec<Item>)> {
        let mut ypos = 50.0 as f32;
        let mut items: Vec<Item> = Vec::with_capacity(4);
        let mut tweens: Vec<Tween> = Vec::with_capacity(4);
        for i in 0..4 {
            let item_id = SQUARE_ITEM_ID + i as usize;
            let rect = graphics::Rect::new(50.0, ypos, 50.0, 50.0);
            let mut item1 = Item::new(item_id, Shape::Rectangle(rect))?;
            item1.layer.graphics.color = graphics::Color::from_rgb_u32(0x333333);

            let tween1 = Tween::with(item_id, &item1.layer)
                .to(&[position(400.0, ypos as f64), size(80.0, 80.0)])
                .duration(0.5)
                .yoyo();
            ypos += 120.0;
            items.push(item1);
            tweens.push(tween1)
        }

        // Testing variations:
        // * The stagger function tells the timeline to offset the start time of each tween by
        // a fixed time (in seconds).
        // * The align function can support Sequence playback of the tweens (one after the other),
        // while the default behavior is to run them all simultaneously.
        let timeline = Timeline::add(tweens)
            // .align(TweenAlign::Sequence)
            .stagger(0.2);
        Ok((timeline, items))
    }

    ///---- 2 ----------------------------------------------------------------------
    /// This is a timeline with a single tween that repeats. A repeat_count of 1 means it
    /// play twice.
    fn build_timeline_2(_ctx: &mut Context) -> GameResult<(Timeline, Vec<Item>)> {
        let mut items: Vec<Item> = Vec::with_capacity(1);
        let mut tweens: Vec<Tween> = Vec::with_capacity(1);
        let mut ypos = 50.0 as f32;

        let rect = graphics::Rect::new(50.0, ypos, 50.0, 50.0);
        let mut item1 = Item::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.layer.graphics.color = graphics::Color::from_rgb_u32(0xCD09AA);

        // FYI: the Tween props below show how you can dynamically build sequences of animations
        // in a single tween by chaining together "to" function calls. It transparently
        // creates new animators in the Tween.
        // Also, this shows the usage of shift_x and shift_y, which are offset functions that
        // manipulate the previous state. Just for fun, it shows that two shift_x props are
        // handled fine by adding them together.
        //
        // Testing variations:
        // * Tween repeat (without yoyo) should repeat the number of times you specify
        // * Yoyo repeat should go back and forth smoothly based on repeat_count (default=1)
        let mut tween = Tween::with(SQUARE_ITEM_ID, &item1.layer)
            .to(&[shift_x(400.0), shift_x(200.0), alpha(0.2)])
            .duration(1.0)
            .to(&[shift_y(300.0), shift_x(-100.0), alpha(1.0)])
            .duration(0.5)
            .to(&[position(200.0, 200.0), alpha(1.0)])
            .duration(0.5)
            .to(&[size(200.0, 200.0)])
            .duration(1.0)
            .repeat(1, 0.25)
            .yoyo();

        tweens.push(tween);

        items.push(item1);

        let timeline = Timeline::add(tweens).align(TweenAlign::Sequence);
        Ok((timeline, items))
    }

    ///---- 3 ----------------------------------------------------------------------
    /// play twice.
    fn build_timeline_3(ctx: &mut Context) -> GameResult<(Timeline, Vec<Item>)> {
        let mut items: Vec<Item> = Vec::with_capacity(1);
        let mut tweens: Vec<Tween> = Vec::with_capacity(1);
        let mut ypos = 50.0 as f32;

        const TEXT_ITEM_ID: usize = 10;
        let font = graphics::Font::new(ctx, "/Roboto-Regular.ttf")?;
        let text = graphics::Text::new(("Tweek Player", font, 48.0));

        let rect = graphics::Rect::new(20.0, 20.0, 200.0, 40.0);
        let mut item4 = Item::new(TEXT_ITEM_ID, Shape::Text(rect))?;
        item4.text = Some(text);

        let mut tween4 = Tween::with(TEXT_ITEM_ID, &item4.layer)
            .to(&[position(400.0, 20.0), alpha(0.2)])
            .duration(3.0);
        &tween4.play();
        item4.tween = Some(tween4);

        let rect = graphics::Rect::new(50.0, ypos, 50.0, 50.0);
        let mut item1 = Item::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.layer.graphics.color = graphics::Color::from_rgb_u32(0xCD09AA);

        // FYI: the Tween props below show how you can dynamically build sequences of animations
        // in a single tween by chaining together "to" function calls. It transparently
        // creates new animators in the Tween.
        // Also, this shows the usage of shift_x and shift_y, which are offset functions that
        // manipulate the previous state. Just for fun, it shows that two shift_x props are
        // handled fine by adding them together.
        //
        // Testing variations:
        // * Tween repeat (without yoyo) should repeat the number of times you specify
        // * Yoyo repeat should go back and forth smoothly based on repeat_count (default=1)
        let mut tween = Tween::with(SQUARE_ITEM_ID, &item1.layer)
            .to(&[shift_x(400.0), shift_x(200.0), alpha(0.2)])
            .duration(1.0)
            .to(&[shift_y(300.0), shift_x(-100.0), alpha(1.0)])
            .duration(0.5)
            .to(&[position(200.0, 200.0), alpha(1.0)])
            .duration(0.5)
            .to(&[size(200.0, 200.0)])
            .duration(1.0)
            .repeat(1, 0.25)
            .yoyo();

        tweens.push(tween);

        items.push(item1);

        let timeline = Timeline::add(tweens).align(TweenAlign::Sequence);
        Ok((timeline, items))
    }
}

/// ##########################################################################################
/// MainState is where the stage setup occurs and the creation of the Tweek objects that will
/// manage the animations. It also implements the EventHandler trait which is the run loop
/// in ggez.
/// ##########################################################################################

struct MainState {
    tweek: Tweek,
    tk_state: TKState,
    items: Vec<Item>,
    buttons: Vec<ButtonView>,
    progress_bar: ProgressBarView,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let ypos = screen_h - 90.0;
        let xpos = (screen_w - BAR_WIDTH) / 2.0;

        // Create progress bar
        let frame = graphics::Rect::new(xpos, ypos, BAR_WIDTH, 4.0);
        let mut progress = ProgressBarView::new(frame);
        progress.set_track_color(Color::from_rgb_u32(HexColors::MediumSlateBlue));
        progress.set_progress_color(Color::from_rgb_u32(HexColors::Azure));

        let buttons = StageHelper::build_player_buttons(ctx)?;

        // ########################################################
        // Here you can choose which timeline and animations to run
        // ########################################################
        let (timeline, items) = StageHelper::build_timeline_1(ctx)?;
        // let (timeline, items) = StageHelper::build_timeline_2(ctx)?;

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
        // This should be called here at the beginning of each run loop. It is responsible for
        // coordinating and all timeline and tween updates. The mutable TKState parameter is used to
        // store and share events and requests among all of the structs that implement TimelineAware.
        self.tweek.update(&mut self.tk_state);

        let progress = self.tk_state.elapsed_time / self.tk_state.total_time;
        if progress <= 1.0 {
            self.progress_bar.set_progress(progress as f32);
        }

        for item in &mut self.items {
            item.timeline_update(&mut self.tweek)?;
        }

        for control in &mut self.buttons {
            control.update()?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);

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
        //     log::debug!("FPS: {}", ggez::timer::fps(ctx));
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
        log::debug!("Mouse down at: x={} y={}", x, y);
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
    std::env::set_var("RUST_LOG", "main=debug,tweek=debug");
    env_logger::init();

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

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
