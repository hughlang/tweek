/// This is a test environment for using gfx with offscreen canvas to render stuff and clip it.
/// Ignore this example for now.
///
mod shape_helper;
use shape_helper::*;

extern crate ggez;
extern crate tweek;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self, *};
use ggez::input::{mouse};
use ggez::mint::{Point2};
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;
use tweek::prelude::*;

const STAGE_WIDTH: f32 = 800.0;
const STAGE_HEIGHT: f32 = 500.0;

struct DemoHelper {}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
impl DemoHelper {

    fn get_stage(ctx: &mut Context) -> (f32, f32, Rect) {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;
        let draw_area = Rect::new(
            (screen_w - STAGE_WIDTH) / 2.0,
            120.0,
            STAGE_WIDTH,
            STAGE_HEIGHT,
        );
        (screen_w, screen_h, draw_area)
    }

    fn build_bars_demo(ctx: &mut Context) -> GameResult<(Vec<Item>, Rect)> {
        let (screen_w, screen_h, draw_area) = DemoHelper::get_stage(ctx);

        let mut items: Vec<Item> = Vec::new();

        const BAR_HEIGHT: f32 = 40.0;
        let line_count = 9;

        for i in 0..line_count {
            let item_id = i as usize;
            let ypos = i as f32 * BAR_HEIGHT + draw_area.top();

            let rect = Rect::new(draw_area.left(), ypos, draw_area.w, BAR_HEIGHT);

            let mut item = Item::new(item_id, Shape::Rectangle(rect))?;

            if i % 3 == 0 {
                item.layer.graphics.color = Color::from_rgb_u32(HexColors::Tomato);
            } else if i % 3 == 1 {
                item.layer.graphics.color = Color::from_rgb_u32(HexColors::Gold);
            } else {
                item.layer.graphics.color = Color::from_rgb_u32(HexColors::Gray);
            }

            items.push(item);
        }
        let container = Rect::new(draw_area.x, draw_area.y, draw_area.w, BAR_HEIGHT * line_count as f32);

        Ok((items, container))
    }
    /// ********************************************************************************
    /// This is a template for creating a new animation.
    /// Copy it and try out different animation techniques.
    /// Add an entry to the Demo enum below to make it part of the Next/Previous cycle.
    fn empty_template(ctx: &mut Context) -> GameResult<(Vec<Item>)> {
        let (screen_w, screen_h, draw_area) = DemoHelper::get_stage(ctx);

        // =====================================================
        // Create item and tween here
        // =====================================================

        Ok(vec![])
    }
}

#[allow(dead_code)]
/// This enum is a list of all the loadable demo animations.
#[derive(Copy, Clone, Debug)]
enum Demo {
    Square1,
}

/// ##########################################################################################
/// MainState is where the stage setup occurs and the creation of the Tweek objects that will
/// manage the animations. It also implements the EventHandler trait which is the run loop
/// in ggez.
/// ##########################################################################################

struct MainState {
    grid: graphics::Mesh,
    canvas: graphics::Canvas,
    container: Rect,
    frames: usize,
    items: Vec<Item>,
    inputs: Vec<TextField>,
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
            ShapeHelper::build_grid(ctx, screen_w, screen_h, 50.0, Color::from_rgb_u32(0xCCCCCC))?;


        let canvas = graphics::Canvas::with_window_size(ctx)?;

        let (items, rect) = DemoHelper::build_bars_demo(ctx)?;
        log::debug!("bars={:?} rect={:?}", items.len(), rect);
        let s = MainState {
            grid: gridmesh,
            canvas: canvas,
            container: rect,
            frames: 0,
            items: items,
            inputs: Vec::new(),
            buttons: buttons,
            tk_state: TKState::new(),
            demo_index: 0,
            demo_list: Vec::new(),
            show_fps: false,
        };

        // s.show_fps = true;

        // ===== If you are adding a new animation to try out, add it to the demo_list here. =====
        // s.demo_list.push(Demo::Square1);

        // // Pick which demo to start with.
        // s.demo_index = 0;
        // let demo = s.demo_list[s.demo_index].clone();
        // s.load_demo(ctx, &demo)?;

        Ok(s)
    }

    #[allow(unreachable_patterns)]
    #[allow(unused_variables)]
    /// This method takes a Demo enum as a parameter to identify which DemoHelper function
    /// to call and replace the current timeline animation.
    fn load_demo(&mut self, ctx: &mut Context, demo: &Demo) -> GameResult {
        self.tk_state.commands.clear();
        // let items = match demo {
        //     Demo::Square1 => DemoHelper::make_text_field(ctx)?,
        //     _ => DemoHelper::empty_template(ctx)?,
        // };
        // self.items = items;
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

        for input in &mut self.inputs {
            input.update()?;
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

        // Render stuff offscreen first
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into()); // No idea what this color is

        for item in &mut self.items {
            item.render(ctx)?;
        }
        // log::debug!("canvas={:?}x{:?}", self.canvas.image().width(), self.canvas.image().height());
        graphics::set_canvas(ctx, None);
        graphics::clear(ctx, graphics::WHITE);

        // To hide the grid, comment out this line.
        graphics::draw(ctx, &self.grid, DrawParam::default())?;

        if self.show_fps {
            self.frames += 1;
            if (self.frames % 20) == 0 {
                log::debug!("FPS: {}", ggez::timer::fps(ctx));
            }
        }
        for input in &mut self.inputs {
            input.render(ctx)?;
        }

        for button in &mut self.buttons {
            button.render(ctx)?;
        }
        let canvas_w = self.canvas.image().width() as f32;
        let canvas_h = self.canvas.image().height() as f32;

        // Calculate the reverse y: canvas height - rect bottom y
        // 768.0 - 480.0 = 288.0
        let flip_y = canvas_h - self.container.bottom();

        let clip = Rect::new(
            self.container.x / canvas_w,
            flip_y / canvas_h,
            self.container.w / canvas_w,
            self.container.h / canvas_h,
        );
        // Must flip y to -1
        let offset = Point2{x: 0.0, y: -1.0};
        let origin = self.container.point();

        let params = DrawParam::default()
                .dest(origin)
                .src(clip)
                .offset(offset)
                ;

        graphics::draw(
            ctx,
            &self.canvas,
            params
        )?;

        // for item in &mut self.items {
        //     item.render(ctx)?;
        // }

        graphics::present(ctx)?;

        // timer::yield_now();
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }

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
        for input in &mut self.inputs {
            let _did_click = input.handle_mouse_up(_x, _y, &mut self.tk_state);
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
        for input in &mut self.inputs {
            if input.handle_mouse_at(x, y) {
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
    // log::debug!("HIDPI: {}", graphics::os_hidpi_factor(ctx));

    let cb = ContextBuilder::new("tween0", "tweenkit")
        .window_setup(conf::WindowSetup::default().title("Tween Basics"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(1024.0, 768.0)
                .hidpi(true),
        )
        .add_resource_path(resource_dir);
    let (ctx, events_loop) = &mut cb.build()?;
    // ggez::graphics::set_resolution(ctx, 2048.0, 1536.0)?;
    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}

// ############################# CANVAS JUNK ##################################

