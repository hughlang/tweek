/// Tweek GUI demos based on Quicksilver
extern crate tweek;
use tweek::prelude::*;

mod demo_helper;
use demo_helper::*;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused_imports)]
use quicksilver::{
    geom::{self, Circle, Line, Rectangle, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Color, Image},
    input::{ButtonState, Key, MouseButton, MouseCursor},
    lifecycle::{run, run_with, Asset, Event, Settings, State, Window},
    Error, Result,
};

struct SceneBuilder {}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
impl SceneBuilder {
    fn load_listbox_scene(screen: Vector) -> Scene {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(&frame);

        let numbers: Vec<u32> = (0..21).collect();
        let ds: Vec<String> = numbers.into_iter().map(|x| x.to_string()).collect();

        let frame = Rectangle::new((100.0, 200.0), (300.0, 200.0));
        let mut listbox = ListBox::new(&frame).datasource(ds);
        scene.controls.push(Rc::new(RefCell::new(listbox)));

        scene
    }

    fn load_text_edit_scene(screen: Vector) -> Scene {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(&frame);

        let text: String = include_str!("../../static/lipsum.txt").into();
        let mut xpos = 200.0;
        let mut ypos = 200.0;

        let frame = Rectangle::new((xpos, ypos), (150.0, 40.0));
        let mut textfield = TextField::new(frame, true);
        textfield.set_placeholder("Enter email address");
        scene.controls.push(Rc::new(RefCell::new(textfield)));

        xpos += 170.0;
        let frame = Rectangle::new((xpos, ypos), (150.0, 40.0));
        let mut textfield = TextField::new(frame, true);
        textfield.set_placeholder("Enter password");
        scene.controls.push(Rc::new(RefCell::new(textfield)));

        ypos += 80.0;

        let frame = Rectangle::new((xpos, ypos), (320.0, 200.0));
        let mut textarea = TextArea::new(frame, true);
        // textarea.set_color(&Color::from_rgb_u32(HexColors::White));
        textarea.set_text(&text);
        scene.controls.push(Rc::new(RefCell::new(textarea)));
        scene
    }

    /// ********************************************************************************
    /// This is a template for creating a new animation.
    /// Copy it and try out different animation techniques.
    /// Add an entry to the Demo enum below to make it part of the Next/Previous cycle.
    fn empty_template(screen: Vector) -> Scene {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(&frame);

        // =====================================================
        // Create scene here
        // =====================================================

        scene
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
enum Demo {
    ListBox,
    TextEditor,
}

#[allow(dead_code)]
#[allow(unused_variables)]
struct MainState {
    grid: Grid,
    screen: Vector,
    scene: Scene,
    theme: Theme,
    buttons: Vec<Button>,
    tk_state: TKState,
    demo_index: usize,
    demo_list: Vec<Demo>,
    show_fps: bool,
    fps_view: Label,
    frames: usize,
    is_running: bool,
}

impl MainState {
    fn new(screen: Vector) -> Result<MainState> {
        let theme = StageHelper::load_theme();
        let buttons = StageHelper::make_next_prev_buttons(&screen, &theme);
        let fps = StageHelper::make_fps_counter(&screen, &theme);
        let grid = StageHelper::build_grid(screen.x, screen.y, 16.0, Color::from_hex("#CCCCCC"));
        let mut s = MainState {
            grid,
            screen,
            scene: SceneBuilder::empty_template(screen),
            theme: theme,
            buttons: buttons,
            tk_state: TKState::new(),
            demo_index: 0,
            demo_list: Vec::new(),
            show_fps: false,
            fps_view: fps,
            frames: 0,
            is_running: false,
        };

        s.demo_list.push(Demo::ListBox);
        s.demo_list.push(Demo::TextEditor);

        s.demo_index = 1;
        let demo = s.demo_list[s.demo_index].clone();
        s.load_demo(screen, &demo);
        Ok(s)
    }

    fn load_demo(&mut self, screen: Vector, demo: &Demo) {
        let mut scene = match demo {
            Demo::ListBox => SceneBuilder::load_listbox_scene(screen),
            Demo::TextEditor => SceneBuilder::load_text_edit_scene(screen),
            // _ => SceneBuilder::empty_template(screen),
        };
        scene.set_theme(&self.theme);
        self.scene = scene;
        self.scene.notify(&DisplayEvent::Ready);
    }
}

impl State for MainState {
    fn new() -> Result<MainState> {
        Err(Error::ContextError("Use run_with to execute custom new method".to_string()))
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        // if !self.is_running {
        //     self.scene.notify(&DisplayEvent::Ready, window);
        //     self.is_running = true;
        // }
        if let Some(click_id) = self.tk_state.click_target {
            self.tk_state.click_target = None;
            match click_id {
                NEXT_COMMAND => {
                    self.demo_index += 1;
                    if self.demo_index == self.demo_list.len() {
                        self.demo_index = 0;
                    }
                    let next = &self.demo_list[self.demo_index].clone();

                    &self.load_demo(window.screen_size(), next);
                    return Ok(());
                }
                PREV_COMMAND => {
                    if self.demo_index == 0 {
                        self.demo_index = self.demo_list.len() - 1;
                    } else {
                        self.demo_index -= 1;
                    }
                    let next = &self.demo_list[self.demo_index].clone();
                    &self.load_demo(window.screen_size(), next);
                    return Ok(());
                }
                _ => (),
            }
        }
        let _ = self.scene.update();

        for button in &mut self.buttons {
            let _ = button.update();
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Remove any lingering artifacts from the previous frame
        window.clear(Color::WHITE)?;

        for line in &self.grid.lines {
            window.draw_ex(&line.with_thickness(1.0), Col(self.grid.color), Transform::IDENTITY, 0);
        }

        for button in &mut self.buttons {
            let _ = button.render(&self.theme, window);
        }

        let _ = self.scene.render(&self.theme, window);

        if self.show_fps {
            self.frames += 1;
            if (self.frames % 20) == 0 {
                log::debug!("FPS: {}", window.current_fps());
            }
        }

        Ok(())
    }

    #[allow(unused_assignments)]
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Focused => {
                log::debug!("size={:?} y={:?}", window.screen_size(), 0);
            }
            Event::MouseMoved(pt) => {
                let mut hover: bool;
                for button in &mut self.buttons {
                    if button.handle_mouse_at(pt) {
                        hover = true;
                        break;
                    }
                }
                hover = self.scene.handle_mouse_at(pt);
                if hover {
                    window.set_cursor(MouseCursor::Hand);
                } else {
                    window.set_cursor(MouseCursor::Default);
                }
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                self.scene.handle_mouse_down(&window.mouse().pos(), &mut self.tk_state);
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Released) => {
                for button in &mut self.buttons {
                    if button.handle_mouse_up(&window.mouse().pos(), &mut self.tk_state) {
                        break;
                    }
                }
            }
            Event::MouseWheel(xy) => {
                self.scene.handle_mouse_scroll(xy, &mut self.tk_state);
            }
            Event::Key(key, ButtonState::Pressed) => match key {
                Key::Escape => {
                    window.close();
                }
                _ => {
                    self.scene.handle_key_command(key, window);
                }
            },
            Event::Typed(c) => {
                self.scene.handle_key_press(*c, window);
            }
            _ => {}
        };
        Ok(())
    }
}

// The main isn't that important in Quicksilver: it just serves as an entrypoint into the event
// loop
fn main() {
    std::env::set_var("RUST_LOG", "main=debug,tweek=debug");

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::builder().default_format_timestamp(false).default_format_module_path(false).init();
    #[cfg(not(target_arch = "wasm32"))]
    color_backtrace::install();

    let screen = Vector::new(1024, 768);
    run_with("Tweek UI", screen, Settings::default(), || MainState::new(screen));
}
