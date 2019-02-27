/// This is a collection of basic Tween examples that show individual tween animations without a timeline.
/// This is a good reference for exploring the basic tween helper commands, which can be found at the
/// top of the file: /src/core/tween.rs
///
mod shape_helper;
use shape_helper::*;

extern crate ggez;
extern crate tweek;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self, Color, DrawParam, Rect};
use ggez::input::mouse;
use ggez::mint::{Point2};
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;
use tweek::prelude::*;

const STAGE_WIDTH: f32 = 900.0;
const STAGE_HEIGHT: f32 = 500.0;

struct DemoHelper {}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
impl DemoHelper {

    fn test_square_1(ctx: &mut Context) -> GameResult<(Vec<ItemState>)> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;
        let draw_area = Rect::new(
            (screen_w - STAGE_WIDTH) / 2.0,
            120.0,
            STAGE_WIDTH,
            STAGE_HEIGHT,
        );

        let rect = Rect::new(draw_area.x, 200.0, 80.0, 80.0);

        let item_id = 1;
        let mut item1 = ItemState::new(item_id, Shape::Rectangle(rect))?;
        item1.layer.graphics.color = Color::from_rgb_u32(HexColors::Red);

        let mut tween1 = Tween::with(item_id, &item1.layer)
            .to(vec![position(840.0, 200.0), size(120.0, 120.0), color(HexColors::Gold)])
            .duration(1.0)
            .ease(Ease::SineIn)
            .repeat(8, 0.2);

        &tween1.play();
        item1.tween = Some(tween1);
        Ok(vec![item1])
    }

    fn test_circle_1(ctx: &mut Context) -> GameResult<(Vec<ItemState>)> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;
        let draw_area = Rect::new(
            (screen_w - STAGE_WIDTH) / 2.0,
            120.0,
            STAGE_WIDTH,
            STAGE_HEIGHT,
        );

        let item_id = 2;
        // Add a circle
        let mut item2 = ItemState::new(item_id, Shape::Circle(Point2{x: 500.0, y: 200.0}, 40.0))?;
        item2.layer.graphics.color = graphics::Color::from_rgb_u32(0xCD09AA);

        let mut tween2 = Tween::with(item_id, &item2.layer)
            .to(vec![position(40.0, 400.0), alpha(0.2)]).duration(1.0)
            .to(vec![position(40.0, 40.0), alpha(1.0)]).duration(0.5)
            .to(vec![position(300.0, 40.0), alpha(1.0)]).duration(0.5)
            .to(vec![size(200.0, 200.0)]).duration(1.0)
            .repeat(-1, 0.25);

        &tween2.play();
        item2.tween = Some(tween2);
        Ok(vec![item2])
    }

    /// ********************************************************************************
    /// This is a template for creating a new animation.
    /// Copy it and try out different animation techniques.
    /// Add an entry to the Demo enum below to make it part of the Next/Previous cycle.
    fn empty_template(ctx: &mut Context) -> GameResult<(Vec<ItemState>)> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        // =====================================================
        // Create item and tween here
        // =====================================================

        Ok(vec![])
    }
}

/// This enum is a list of all the loadable demo animations.
#[derive(Copy, Clone, Debug)]
enum Demo {
    Square1,
    Circle1,
}

/// ##########################################################################################
/// MainState is where the stage setup occurs and the creation of the Tweek objects that will
/// manage the animations. It also implements the EventHandler trait which is the run loop
/// in ggez.
/// ##########################################################################################

struct MainState {
    grid: graphics::Mesh,
    frames: usize,
    items: Vec<ItemState>,
    buttons: Vec<ButtonView>,
    tk_state: TKState,
    demo_index: usize,
    demo_list: Vec<Demo>,
    show_fps: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let buttons = ShapeHelper::make_next_prev_buttons(ctx)?;
        let gridmesh =
            ShapeHelper::build_grid(ctx, screen_w, screen_h, 16.0, Color::from_rgb_u32(0xCCCCCC))?;

        let mut s = MainState {
            grid: gridmesh,
            frames: 0,
            items: Vec::new(),
            buttons: buttons,
            tk_state: TKState::new(),
            demo_index: 0,
            demo_list: Vec::new(),
            show_fps: false,
        };

        // s.show_fps = true;

        // ===== If you are adding a new animation to try out, add it to the demo_list here. =====
        s.demo_list.push(Demo::Square1);
        s.demo_list.push(Demo::Circle1);

        // Pick which demo to start with.
        s.demo_index = 1;
        let demo = s.demo_list[s.demo_index].clone();
        s.load_demo(ctx, &demo)?;

        Ok(s)
    }

    #[allow(unreachable_patterns)]
    /// This method takes a Demo enum as a parameter to identify which DemoHelper function
    /// to call and replace the current timeline animation.
    fn load_demo(&mut self, ctx: &mut Context, demo: &Demo) -> GameResult {
        self.tk_state.commands.clear();
        let items = match demo {
            Demo::Square1 => DemoHelper::test_square_1(ctx)?,
            Demo::Circle1 => DemoHelper::test_circle_1(ctx)?,
            _ => DemoHelper::empty_template(ctx)?,
        };
        self.items = items;
        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if &self.tk_state.commands.len() > &0 {
            // This is not a good way to do this. FIXME
            let cmd = &self.tk_state.commands[0];
            match *cmd {
                NEXT_COMMAND => {
                    self.demo_index += 1;
                    if self.demo_index == self.demo_list.len() {
                        self.demo_index = 0;
                    }
                    let next = &self.demo_list[self.demo_index].clone();


                    &self.load_demo(ctx, next);
                    return Ok(());
                }
                PREV_COMMAND => {
                    if self.demo_index == 0 {
                        self.demo_index = self.demo_list.len() - 1;
                    } else {
                        self.demo_index -= 1;
                    }
                    let next = &self.demo_list[self.demo_index].clone();
                    &self.load_demo(ctx, next);
                    return Ok(());
                }
                _ => (),
            }
        }

        for item in &mut self.items {
            item.update()?;
        }
        for button in &mut self.buttons {
            button.update()?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);

        // To hide the grid, comment out this line.
        graphics::draw(ctx, &self.grid, DrawParam::default())?;

        if self.show_fps {
            self.frames += 1;
            if (self.frames % 20) == 0 {
                println!("FPS: {}", ggez::timer::fps(ctx));
            }
        }

        for button in &mut self.buttons {
            button.render(ctx)?;
        }

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
    // println!("HIDPI: {}", graphics::os_hidpi_factor(ctx));

    let cb = ContextBuilder::new("tween0", "tweenkit")
        .window_setup(conf::WindowSetup::default().title("Tween Basics"))
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
