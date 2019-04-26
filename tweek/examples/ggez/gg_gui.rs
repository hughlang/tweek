/// This is a test environment for prototyping UI components based ggez.
///
///
mod shape_helper;
use shape_helper::*;

extern crate ggez;
extern crate tweek;

use std::cell::RefCell;
use std::rc::Rc;

use ggez::conf;
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color, DrawParam, Rect};
use ggez::input::mouse;
// use ggez::mint::{Point2};
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;
use tweek::prelude::*;

const STAGE_WIDTH: f32 = 800.0;
const STAGE_HEIGHT: f32 = 500.0;

struct SceneHelper {}

// #[allow(dead_code)]
impl SceneHelper {
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

    fn load_theme(ctx: &mut Context) -> GameResult<Theme> {
        let mut theme = Theme::default();
        let hidpi_factor = graphics::hidpi_factor(ctx);
        // let hidpi_factor = 2.0;
        log::debug!("graphics::hidpi_factor: {}", hidpi_factor);
        theme.load_normal_font("/resources", "/Roboto-Regular.ttf", ctx);
        theme.load_title_font("/resources", "/Roboto-Bold.ttf", ctx);
        theme.font_size = 14.0 * hidpi_factor;
        theme.title_font_size = 16.0 * hidpi_factor;

        Ok(theme)
    }

    fn load_text_inputs_scene(ctx: &mut Context) -> GameResult<(Scene)> {
        // let tool = TestTool::new();
        // log::debug!("glyph_brush: ===========================");
        // tool.evaluate_glyph_brush();
        // log::debug!("rusttype: ===========================");
        // tool.evaluate_rusttype();

        let (screen_w, screen_h, _stage) = SceneHelper::get_stage(ctx);
        let frame = Rect::new(0.0, 0.0, screen_w, screen_h);
        let mut scene = Scene::new(&frame);

        let xpos = 200.0;
        let mut ypos = 240.0;
        let frame = Rect::new(xpos, ypos, 300.0, 40.0);
        let mut input = TextField::new(frame);
        input.set_text("ABC A B C i 1 2 3");
        input.set_color(HexColors::LightYellow);
        input.set_hover_animation(&[color(HexColors::LightBlue)], 0.1);
        // input.set_placeholder("Click here to enter some text");
        input.set_onclick(move |_action, _tk| {
            // TODO: TKState should know what mouse cursor should be
            // tk.commands.push(PREV_COMMAND);
        });
        scene.controls.push(Rc::new(RefCell::new(input)));

        ypos += 100.0;
        let frame = Rect::new(xpos, ypos, 300.0, 40.0);
        let mut input = TextField::new(frame);
        input.set_text("");
        input.set_color(HexColors::LightYellow);
        input.set_hover_animation(&[color(HexColors::LightBlue)], 0.1);
        input.set_placeholder("Click here to enter some text");
        input.set_onclick(move |_action, _tk| {
            // TODO: TKState should know what mouse cursor should be
            // tk.commands.push(PREV_COMMAND);
        });
        scene.controls.push(Rc::new(RefCell::new(input)));

        let text: String = include_str!("lipsum.txt").into();
        ypos += 100.0;
        let frame = Rect::new(xpos, ypos, 300.0, 200.0);
        let mut textarea = TextArea::new(frame, true);
        textarea.set_color(&Color::from_rgb_u32(HexColors::White));
        textarea.set_text(&text);
        scene.controls.push(Rc::new(RefCell::new(textarea)));

        Ok(scene)
    }

    fn load_listbox_scene(ctx: &mut Context) -> GameResult<(Scene)> {
        let (screen_w, screen_h, _stage) = SceneHelper::get_stage(ctx);
        let frame = Rect::new(0.0, 0.0, screen_w, screen_h);
        let mut scene = Scene::new(&frame);

        let numbers: Vec<u32> = (0..21).collect();
        let ds: Vec<String> = numbers.into_iter().map(|x| x.to_string()).collect();

        let frame = Rect::new(100.0, 200.0, 300.0, 200.0);
        let mut listbox = ListBox::new(&frame, ctx).datasource(ds);
        listbox.set_color(&Color::from_rgb_u32(0xCCCCCC));
        scene.controls.push(Rc::new(RefCell::new(listbox)));

        Ok(scene)
    }

    /// ********************************************************************************
    /// This is a template for creating a new animation.
    /// Copy it and try out different animation techniques.
    /// Add an entry to the Demo enum below to make it part of the Next/Previous cycle.
    fn empty_template(ctx: &mut Context) -> GameResult<(Scene)> {
        let (screen_w, screen_h, _stage) = SceneHelper::get_stage(ctx);
        let frame = Rect::new(0.0, 0.0, screen_w, screen_h);
        let scene = Scene::new(&frame);

        // =====================================================
        // Customize here
        // =====================================================

        Ok(scene)
    }
}

#[allow(dead_code)]
/// This enum is a list of all the loadable demo animations.
#[derive(Copy, Clone, Debug)]
enum Demo {
    ListBox,
    TextField,
}

/// ##########################################################################################
/// MainState is where the stage setup occurs and the creation of the Tweek objects that will
/// manage the animations. It also implements the EventHandler trait which is the run loop
/// in ggez.
/// ##########################################################################################

struct MainState {
    grid: graphics::Mesh,
    scene: Scene,
    theme: Theme,
    frames: usize,
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

        let gridmesh =
            ShapeHelper::build_grid(ctx, screen_w, screen_h, 16.0, Color::from_rgb_u32(0xCCCCCC))?;

        let theme = SceneHelper::load_theme(ctx)?;
        let mut buttons = ShapeHelper::make_next_prev_buttons(ctx)?;
        for button in &mut buttons {
            button.set_theme(&theme);
        }

        let scene = SceneHelper::empty_template(ctx)?;

        let mut s = MainState {
            grid: gridmesh,
            scene: scene,
            theme: theme,
            frames: 0,
            buttons: buttons,
            tk_state: TKState::new(),
            demo_index: 0,
            demo_list: Vec::new(),
            show_fps: false,
        };
        s.show_fps = true;

        // ===== If you are adding a new animation to try out, add it to the demo_list here. =====
        s.demo_list.push(Demo::ListBox);
        s.demo_list.push(Demo::TextField);

        // Pick which demo to start with.
        s.demo_index = 1;
        let demo = s.demo_list[s.demo_index].clone();
        s.load_demo(ctx, &demo)?;

        Ok(s)
    }

    #[allow(unreachable_patterns)]
    #[allow(unused_variables)]
    /// This method takes a Demo enum as a parameter to identify which SceneHelper function
    /// to call and replace the current timeline animation.
    fn load_demo(&mut self, ctx: &mut Context, demo: &Demo) -> GameResult {
        self.tk_state.commands.clear();

        let mut scene = match demo {
            Demo::ListBox => SceneHelper::load_listbox_scene(ctx),
            Demo::TextField => SceneHelper::load_text_inputs_scene(ctx),
            _ => SceneHelper::empty_template(ctx),
        }?;
        scene.set_theme(&self.theme);
        self.scene = scene;
        self.scene.notify(&DisplayEvent::Ready);
        Ok(())
    }
}

// *****************************************************************************************************
// Event Loop
// *****************************************************************************************************

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
        self.scene.update()?;

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
                log::debug!("FPS: {}", ggez::timer::fps(ctx));
            }
        }

        self.scene.render(ctx)?;

        for button in &mut self.buttons {
            button.render(ctx)?;
        }

        graphics::present(ctx)?;

        // timer::yield_now();
        Ok(())
    }

    fn text_input_event(&mut self, ctx: &mut Context, c: char) {
        if c.is_ascii_control() {
            return;
        }
        let _ = self.scene.handle_key_press(c, ctx);
    }

    /// TODO: queue all backspace events and execute on key up?
    // fn key_down_event(
    //     &mut self,
    //     ctx: &mut Context,
    //     keycode: KeyCode,
    //     _keymods: KeyMods,
    //     _repeat: bool,
    // ) {
    //     log::debug!("{:?}", keycode);

    //     if keycode == KeyCode::Escape {
    //         // event::quit(ctx);
    //     }
    // }

    fn key_up_event(&mut self, ctx: &mut Context, code: KeyCode, keymods: KeyMods) {
        if TEXT_KEY_COMMANDS.contains(&code) {
            self.scene.handle_key_command(code, keymods, ctx);
        } else {
            // TODO: handle other use cases
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        log::debug!("#### Mouse down at: x={} y={}", x, y);
    }

    /// A mouse button was released
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.scene.handle_mouse_up(_x, _y, &mut self.tk_state);

        for button in &mut self.buttons {
            let _did_click = button.handle_mouse_up(_x, _y, &mut self.tk_state);
        }
    }

    /// The mouse was moved; it provides both absolute x and y coordinates in the window,
    /// and relative x and y coordinates compared to its last position.
    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        let _ = self.scene.handle_mouse_at(x, y);
        for button in &mut self.buttons {
            if button.handle_mouse_at(x, y) {
                mouse::set_cursor_type(ctx, mouse::MouseCursor::Hand);
            } else {
                mouse::set_cursor_type(ctx, mouse::MouseCursor::Default);
            }
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.scene.handle_mouse_scroll(x, y, &mut self.tk_state);
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {
        log::debug!("resize_event w={} h={}", _width, _height);
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
                .hidpi(false)
                .resizable(false),
        )
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;
    // log::debug!("Game resource path: {:?}", ctx.filesystem);

    let (size_w, size_h) = graphics::size(ctx);
    log::debug!("size_w={:?} size_h={:?}", size_w, size_h);

    let (size_w, size_h) = graphics::drawable_size(ctx);
    log::debug!("draw_w={:?} draw_h={:?}", size_w, size_h);

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
