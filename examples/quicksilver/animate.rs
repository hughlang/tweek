/// Animation demos
///
use tweek::prelude::*;

mod demo_helper;
use demo_helper::*;
use demo_helper::constants::*;

use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Background::Img, Color, FontStyle, Image, PixelFormat},
    input::{ButtonState, Key, MouseButton, MouseCursor},
    lifecycle::{run_with, Event, Settings, State, Window},
    Error, Result,
};

#[allow(unused_imports)]
use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, Rgba};

struct SceneBuilder {}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
impl SceneBuilder {

    fn animate_square(screen: Vector) -> Scene {
        let draw_area = DemoHelper::get_draw_area(screen);
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        let item_id = 1;
        let draw_area = DemoHelper::get_draw_area(screen);
        let frame = Rectangle::new((draw_area.pos.x, 200.0), (80.0, 80.0));
        let fill_color = Color::RED;
        // let mut mesh = DrawShape::rectangle(&frame, Some(fill_color), None, 2.0, 0.0);
        let mut square = ShapeView::new(frame, ShapeDef::Rectangle)
            .with_background(BackgroundStyle::Solid(fill_color));
        square.build();

        let target_x = draw_area.pos.x + draw_area.size.x - 120.0;
        let mut tween1 = Tween::with(item_id, &square.layer)
            .to(&[position(target_x, 400.0), size(120.0, 120.0), color(HexColors::Gold)])
            .duration(1.0)
            .ease(Ease::SineInOut)
            .repeat(9, 0.2)
            .yoyo();

        // square.layer.animate_with_props(propset.clone());
        square.layer.start_animation(tween1);
        scene.add_view(Box::new(square));

        scene
    }

    fn balloon(screen: Vector) -> Scene {
        let draw_area = DemoHelper::get_draw_area(screen);
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        let center_pt = Vector { x: screen.x / 2.0 - 140.0, y: screen.y / 2.0 - 40.0 };
        log::debug!("center_pt={:?} y={:?}", center_pt, 0);

        // Add a circle
        let frame = Rectangle::new(center_pt, (80.0, 80.0));
        let item_id = 2;
        let mut shape = ShapeView::new(frame, ShapeDef::Circle)
            .with_background(BackgroundStyle::Solid(Color::from_hex("#CD09AA")));
        shape.build();

        let mut tween = Tween::with(item_id, &shape.layer)
            .to(&[size(200.0, 200.0), alpha(0.9)])
            .duration(4.0)
            .to(&[position(700.0, draw_area.pos.y + 40.0), size(100.0, 100.0), alpha(0.8)])
            .duration(0.2)
            .ease(Ease::SineIn)
            .to(&[position(draw_area.pos.x + draw_area.size.x, 200.0), size(50.0, 50.0), alpha(0.7)])
            .duration(0.2)
            .to(&[position(650.0, draw_area.pos.y + draw_area.size.y), size(20.0, 20.0), alpha(0.6)])
            .duration(0.2)
            .to(&[position(400.0, 300.0), size(5.0, 5.0), alpha(0.2)])
            .duration(0.5)
            // .repeat(1, 2.0)
            ;

        shape.layer.start_animation(tween);
        scene.add_view(Box::new(shape));

        scene
    }

    /// An image that rolls across the screen
    fn rolling_image(screen: Vector) -> Scene {
        let draw_area = DemoHelper::get_draw_area(screen);
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        let item_id = 3;
        let image = Image::from_bytes(include_bytes!("../../static/tile.png")).unwrap();
        let rect = Rectangle::new((draw_area.pos.x, 400.0), (100.0, 100.0));
        let mut image_view = ImageView::new(rect, image);

        let mut tween = Tween::with(item_id, &image_view.layer)
            .to(&[shift_x(draw_area.width()), rotate(360.0)])
            .duration(3.0)
            .ease(Ease::BounceOut)
            .repeat(5, 0.5);

        image_view.layer.start_animation(tween);
        scene.add_view(Box::new(image_view));

        scene
    }

    fn pink_snake(screen: Vector) -> Scene {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        let draw_area = DemoHelper::get_draw_area(screen);
        let frame = Rectangle::new((draw_area.pos.x, draw_area.pos.y), (20.0, 20.0));

        let w = 640.0;
        let h = 400.0;

        let mut shape = ShapeView::new(frame, ShapeDef::Rectangle)
            .with_background(BackgroundStyle::Solid(Color::from_hex(HexColors::HotPink)));
        shape.build();

        let item_id = 1;
        let mut tween = Tween::with(item_id, &shape.layer)
            .to(&[size(w, 20.0)])
            .duration(1.0)
            .ease(Ease::ElasticIn)
            .to(&[size(20.0, 20.0), shift_x(w - 20.0)])
            .duration(1.0)
            .ease(Ease::ElasticOut)
            .to(&[size(20.0, h)])
            .duration(1.0)
            .ease(Ease::BackIn)
            .to(&[size(20.0, 20.0), shift_y(h - 20.0)])
            .duration(1.0)
            .ease(Ease::BackOut)
            .to(&[size(w, 20.0), shift_x(-w + 20.0)])
            .duration(1.0)
            .ease(Ease::BounceIn)
            .to(&[size(20.0, 20.0)])
            .duration(1.0)
            .ease(Ease::BounceOut)
            .to(&[size(20.0, h), shift_y(-h + 20.0)])
            .duration(1.0)
            .ease(Ease::SineIn)
            .to(&[size(20.0, 20.0)])
            .duration(1.0)
            .ease(Ease::SineOut)
            .repeat(-1, 0.2);

        shape.layer.start_animation(tween);
        scene.add_view(Box::new(shape));

        scene
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
    Square,
    Balloon,
    RollingImage,
    Snake,
}

#[allow(dead_code)]
#[allow(unused_variables)]
struct MainState {
    grid: Grid,
    screen: Vector,
    scene: Scene,
    nav_scene: Scene,
    theme: Theme,
    app_state: AppState,
    demo_index: usize,
    demo_list: Vec<Demo>,
    is_running: bool,
    frames: usize,
}

impl MainState {
    fn new(screen: Vector) -> Result<MainState> {
        let mut theme = DemoHelper::load_theme();
        let grid = DemoHelper::build_grid(screen.x, screen.y, 16.0, Color::from_hex("#CCCCCC"));
        let mut s = MainState {
            grid,
            screen,
            scene: SceneBuilder::empty_template(screen),
            nav_scene: DemoHelper::build_nav_scene(screen),
            theme: theme,
            app_state: AppState::new(),
            demo_index: 0,
            demo_list: Vec::new(),
            is_running: false,
            frames: 0,
        };

        s.demo_list.push(Demo::Square);
        s.demo_list.push(Demo::Balloon);
        s.demo_list.push(Demo::RollingImage);
        s.demo_list.push(Demo::Snake);

        s.demo_index = 1;
        let demo = s.demo_list[s.demo_index].clone();
        s.load_demo(screen, &demo);
        Ok(s)
    }

    fn load_demo(&mut self, screen: Vector, demo: &Demo) {
        let mut scene = match demo {
            Demo::Square => SceneBuilder::animate_square(screen),
            Demo::RollingImage => SceneBuilder::rolling_image(screen),
            Demo::Balloon => SceneBuilder::balloon(screen),
            Demo::Snake => SceneBuilder::pink_snake(screen),
            // _ => SceneBuilder::empty_template(screen),
        };
        scene.set_theme(&mut self.theme);
        self.scene = scene;
        self.scene.notify(&DisplayEvent::Ready);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl State for MainState {
    fn new() -> Result<MainState> {
        Err(Error::ContextError("Use run_with to execute custom new method".to_string()))
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        for event in self.app_state.event_bus.into_iter() {
            if let Ok(evt) = event.downcast_ref::<SceneEvent>() {
                log::debug!("SceneEvent={:?}", evt);
                self.scene.handle_event(evt);
            }
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

        let _ = self.scene.update(window, &mut self.app_state);
        // Sorry, bad hacks
        self.app_state.zero_offset();
        let _ = self.nav_scene.update(window, &mut self.app_state);

        self.frames += 1;
        if (self.frames % FPS_FRAMES_INTERVAL) == 0 {
            let out = format!("FPS: {:.2}", window.current_fps());
            self.nav_scene.set_field_value(&FieldValue::Text(out), TypeId::of::<Text>(), FPS_TAG);
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Remove any lingering artifacts from the previous frame
        window.clear(Color::WHITE)?;

        for line in &self.grid.lines {
            window.draw_ex(&line.with_thickness(1.0), Col(self.grid.color), Transform::IDENTITY, 0);
        }

        let _ = self.nav_scene.render(&mut self.theme, window);
        let _ = self.scene.render(&mut self.theme, window);

        Ok(())
    }

    #[allow(unused_assignments)]
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Focused => {
                log::debug!("size={:?} y={:?}", window.screen_size(), 0);
            }
            Event::MouseMoved(pt) => {
                let hover1 = self.nav_scene.handle_mouse_at(pt);
                let hover2 = self.scene.handle_mouse_at(pt);
                if hover1 || hover2 {
                    window.set_cursor(MouseCursor::Hand);
                } else {
                    window.set_cursor(MouseCursor::Default);
                }
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                self.scene.handle_mouse_down(&window.mouse().pos(), &mut self.app_state);
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Released) => {
                self.scene.handle_mouse_up(&window.mouse().pos(), &mut self.app_state);
                self.nav_scene.handle_mouse_up(&window.mouse().pos(), &mut self.app_state);
            }
            Event::MouseWheel(xy) => {
                self.scene.handle_mouse_scroll(xy, &mut self.app_state);
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

    let screen = Vector::new(800, 600);
    run_with("Tweek UI", screen, Settings::default(), || MainState::new(screen));
}
