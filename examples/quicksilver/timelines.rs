/// Tweek Basics demo with Quicksilver
use tweek::prelude::*;

mod demo_helper;
use demo_helper::*;

use std::any::TypeId;
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

struct DemoBuilder {}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
impl DemoBuilder {
    /// This demo shows a collection of dots rotating around in a circle
    fn build_dots_demo(screen: Vector) -> (Timeline, Scene) {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        let draw_area = DemoHelper::get_draw_area(screen);
        let center_pt = Vector { x: screen.x / 2.0, y: screen.y / 2.0 };
        let start_pt = center_pt - Vector::new(0.0, 100.0);

        let dot_radius = 10.0;
        let scene_radius = 96.0;
        let dot_count = 8;

        let mut shapes: Vec<ShapeView> = Vec::with_capacity(dot_count);
        let mut tweens: Vec<Tween> = Vec::with_capacity(dot_count);

        for i in 0..dot_count {
            let frame = Rectangle::new(center_pt, (dot_radius * 2.0, dot_radius * 2.0));

            let item_id = i;
            let mut color = Color::RED;
            let alpha = 1.0 - (i as f32 / dot_count as f32) / 2.0;
            color.a = alpha;
            let mut dot = ShapeView::new(frame, ShapeDef::Circle)
                .with_background(BackgroundStyle::Solid(color));
            // dot.layer.anchor_pt = Vector::new(0.0, 100.0);
            dot.layer.anchor_pt = Vector::new(0.0, -100.0);
            dot.build();
            let tween1 = Tween::with(item_id, &dot.layer)
                .to(&[rotate(360.0)])
                .duration(1.8)
                .ease(Ease::SineInOut)
                .repeat(-1, 0.8);
            scene.add_view(Box::new(dot));
            tweens.push(tween1)
        }

        let timeline = Timeline::add(tweens).stagger(0.12);
        (timeline, scene)
    }


    /// ********************************************************************************
    /// This is a template for creating a new animation.
    /// Copy it and try out different animation techniques.
    /// Add an entry to the Demo enum below to make it part of the Next/Previous cycle.
    fn empty_template(screen: Vector) -> Scene {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        // =====================================================
        // Create scene here
        // =====================================================

        scene
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
enum Demo {
    DotCircle,
}

#[allow(dead_code)]
#[allow(unused_variables)]
struct MainState {
    grid: Grid,
    screen: Vector,
    scene: Scene,
    theme: Theme,
    tweek: Tweek,
    app_state: AppState,
    buttons: Vec<Button>,
    demo_index: usize,
    demo_list: Vec<Demo>,
    show_fps: bool,
    // buttons: Vec<Button>,
}

impl MainState {
    fn new(screen: Vector) -> Result<MainState> {
        let theme = DemoHelper::load_theme();
        let buttons = DemoHelper::make_next_prev_buttons(&screen, &theme);

        let grid = DemoHelper::build_grid(screen.x, screen.y, 32.0, Color::from_hex("#CCCCCC"));
        let scene = DemoBuilder::empty_template(screen);

        let mut s = MainState {
            grid,
            screen,
            scene,
            theme,
            tweek: Tweek::new(),
            app_state: AppState::new(),
            buttons: buttons,
            demo_index: 0,
            demo_list: Vec::new(),
            show_fps: false,
        };

        s.demo_list.push(Demo::DotCircle);

        s.demo_index = 0;
        let demo = s.demo_list[s.demo_index].clone();
        s.load_demo(screen, &demo);
        Ok(s)
    }

    fn load_demo(&mut self, screen: Vector, demo: &Demo) {
        // self.app_state.commands.clear();
        let (timeline, mut scene) = match demo {
            Demo::DotCircle => DemoBuilder::build_dots_demo(screen),
            // _ => DemoBuilder::empty_template(screen),
        };
        let mut tweek = Tweek::new();
        tweek.add_timeline(timeline);
        &tweek.play();

        let app_state = AppState::new();
        self.app_state = app_state;
        self.tweek = tweek;

        scene.set_theme(&mut self.theme);
        self.scene = scene;
        self.scene.notify(&DisplayEvent::Ready);
    }
}

impl State for MainState {
    fn new() -> Result<MainState> {
        Err(Error::ContextError("Use run_with to execute custom new method".to_string()))
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        for event in self.app_state.event_bus.into_iter() {
            // if let Ok(evt) = event.downcast_ref::<SceneEvent>() {
            //     log::debug!("SceneEvent={:?}", evt);
            //     self.scene.handle_event(evt);
            // }
            if let Ok(evt) = event.downcast_ref::<NavEvent>() {
                log::debug!("NavEvent={:?}", evt);
                match evt {
                    NavEvent::Next => {
                        self.demo_index += 1;
                        if self.demo_index == self.demo_list.len() {
                            self.demo_index = 0;
                        }
                        let next = &self.demo_list[self.demo_index].clone();

                        &self.load_demo(window.screen_size(), next);
                        return Ok(());

                    }
                    NavEvent::Back => {
                        if self.demo_index == 0 {
                            self.demo_index = self.demo_list.len() - 1;
                        } else {
                            self.demo_index -= 1;
                        }
                        let next = &self.demo_list[self.demo_index].clone();
                        &self.load_demo(window.screen_size(), next);
                        return Ok(());
                    }
                    _ => ()
                }
            }
        }
        self.tweek.update(&mut self.app_state);

        let _ = self.scene.update(window, &mut self.app_state);

        for button in &mut self.buttons {
            let _ = button.update(window, Vector::ZERO);
        }

        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::MouseMoved(pt) => {
                let mut hover = false;
                for button in &mut self.buttons {
                    if button.handle_mouse_at(pt) {
                        hover = true;
                        break;
                    }
                }
                if hover {
                    window.set_cursor(MouseCursor::Hand);
                } else {
                    window.set_cursor(MouseCursor::Default);
                }
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Released) => {
                for button in &mut self.buttons {
                    if button.handle_mouse_up(&window.mouse().pos(), &mut self.app_state) {
                        break;
                    }
                }
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => {}
        };
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Remove any lingering artifacts from the previous frame
        window.clear(Color::WHITE)?;

        for line in &self.grid.lines {
            window.draw_ex(&line.with_thickness(1.0), Col(self.grid.color), Transform::IDENTITY, 0);
        }

        let _ = self.scene.render(&mut self.theme, window);

        for button in &mut self.buttons {
            let _ = button.render(&mut self.theme, window);
        }

        Ok(())
    }
}

// The main isn't that important in Quicksilver: it just serves as an entrypoint into the event
// loop
fn main() {
    std::env::set_var("RUST_LOG", "main=trace,tweek=trace");

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::builder().default_format_timestamp(false).default_format_module_path(false).init();

    let screen = Vector::new(1024, 768);
    run_with("Tweek Basics", screen, Settings::default(), || MainState::new(screen));
}
