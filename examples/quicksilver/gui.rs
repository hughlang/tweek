/// Tweek GUI demos based on Quicksilver

use tweek::prelude::*;

#[allow(dead_code)]
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
    /// TODO:
    /// – image buttons
    /// – rounded rect button
    ///
    fn load_buttons_scene(screen: Vector) -> Scene {
        const BUTTON_WIDTH: f32 = 80.0;
        const BUTTON_HEIGHT: f32 = 40.0;

        // let style = FontStyle::new(16.0, Color::WHITE);
        // ---- Previous ---------------------

        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        let mut xpos = 200.0;
        let mut ypos = 200.0;


        scene
    }

    fn load_listbox_scene(screen: Vector) -> Scene {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        let numbers: Vec<u32> = (0..21).collect();
        let ds: Vec<String> = numbers.into_iter().map(|x| x.to_string()).collect();

        let frame = Rectangle::new((100.0, 200.0), (300.0, 200.0));
        let mut listbox = ListBox::new(frame).datasource(ds);
        scene.add_control(Box::new(listbox));

        scene
    }

    fn load_text_edit_scene(screen: Vector, theme: &mut Theme) -> Scene {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        let text: String = include_str!("../../static/lipsum.txt").into();
        let mut xpos = 200.0;
        let mut ypos = 200.0;

        let frame = Rectangle::new((xpos, ypos), (150.0, 40.0));
        let mut textfield = TextField::new(frame, theme, true);
        textfield.set_placeholder("Enter email address");
        // textfield.set_text("čćdđe ёєжзѕиіїйјк");  // FIXME: Special chars not accepted
        scene.add_control(Box::new(textfield));

        xpos += 170.0;
        let frame = Rectangle::new((xpos, ypos), (150.0, 40.0));
        let mut textfield = TextField::new(frame, theme, true).with_type(TextFieldType::Secure(MASK_TEXT_DOT));
        textfield.set_placeholder("Enter password");
        scene.add_control(Box::new(textfield));

        ypos += 80.0;

        let frame = Rectangle::new((xpos, ypos), (320.0, 200.0));
        let mut textarea = TextArea::new(frame, theme, true);
        // textarea.set_color(&Color::from_hex(HexColors::White));
        textarea.set_text(&text);
        scene.add_control(Box::new(textarea));
        scene
    }

    fn load_form_inputs_scene(screen: Vector) -> Scene {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        let mut xpos = 100.0;
        let mut ypos = 100.0;

        let frame = Rectangle::new((xpos, ypos), (300.0, 40.0));
        let checkbox = Checkbox::new(frame).with_text("Click the checkbox", false);
        scene.add_control(Box::new(checkbox));

        let options: Vec<(&str, bool)> = vec![
            ("This is option 1", false),
            ("This is option 2", false),
            ("This is option 3", false),
            ("This is option 4", false),
        ];

        // Radio buttons style
        ypos += 80.0;
        let frame = Rectangle::new((xpos, ypos), (300.0, 160.0));
        let mut group = OptionGroup::new(frame);
        group.multi_select = false;
        group.check_style = CheckStyle::Radio;
        group.set_options(options.clone());
        scene.add_control(Box::new(group));

        xpos = 450.0;
        let frame = Rectangle::new((xpos, ypos), (300.0, 160.0));
        let mut group = OptionGroup::new(frame);
        group.set_options(options.clone());
        group.multi_select = true;
        scene.add_control(Box::new(group));

        let options: Vec<(&str, bool)> = vec![
            ("Red", false),
            ("Green", false),
            ("Blue", false),
            ("Orange", false),
            ("Gray", false),
            ("Pink", false),
        ];

        xpos = 100.0;
        ypos += 180.0;
        let frame = Rectangle::new((xpos, ypos), (300.0, 160.0));
        let mut group = OptionGroup::new(frame);
        group.set_layout(OptionGroupLayout::HorizontalGrid(140.0, 10.0));
        group.set_options(options.clone());
        group.multi_select = true;
        scene.add_control(Box::new(group));

        xpos = 450.0;
        let frame = Rectangle::new((xpos, ypos), (300.0, 160.0));
        let mut group = OptionGroup::new(frame);
        group.set_layout(OptionGroupLayout::HorizontalWrap(10.0, 10.0));
        group.set_options(options.clone());
        group.multi_select = true;
        scene.add_control(Box::new(group));

        scene
    }

    /// Show off the different shape primitives and do some hover animation
    fn load_shapes_scene(screen: Vector) -> Scene {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        let mut xpos = 100.0;
        let mut ypos = 100.0;

        let frame = Rectangle::new((xpos, ypos), (120.0, 60.0));
        let fill_color = Color::from_hex("#CCCCCC");
        let line_color = Color::from_hex("#333333");
        let mut rounded = DrawShape::rectangle(&frame, Some(fill_color), Some(line_color), 2.0, 3.0);
        let shape = ShapeView::new(frame, ShapeDef::Rectangle).with_mesh(&mut rounded);
        scene.add_view(Box::new(shape));

        xpos += 180.0;
        let frame = Rectangle::new((xpos, ypos), (180.0, 60.0));
        let line_color = Color::from_hex("#333333");
        let mut mesh = DrawShape::rectangle(&frame, None, Some(line_color), 2.0, 0.0);
        let shape = ShapeView::new(frame, ShapeDef::Rectangle).with_mesh(&mut mesh);
        scene.add_view(Box::new(shape));

        ypos += 100.0;
        let frame = Rectangle::new((xpos, ypos), (200.0, 200.0));
        let fill_color = Color::from_hex("#CD5C5C");
        let line_color = Color::from_hex("#FFD700");
        let mut circle =
            DrawShape::circle(&frame.center(), &frame.width() / 2.0, Some(fill_color), Some(line_color), 2.0);
        let mut shape = ShapeView::new(frame, ShapeDef::Circle).with_mesh(&mut circle);
        shape.set_hover_animation(PropSet::new(vec![color(HexColors::HotPink)], 0.2).for_type(TweenType::Hover));
        scene.add_view(Box::new(shape));

        xpos += 250.0;
        ypos = 100.0;
        let frame = Rectangle::new((xpos, ypos), (200.0, 150.0));
        let fill_color = Color::from_hex("#FFD700");
        let mut ellipse = DrawShape::ellipse(&frame, Some(fill_color), None, 0.0, 30.0);
        let mut shape = ShapeView::new(frame, ShapeDef::Ellipse).with_mesh(&mut ellipse);
        scene.add_view(Box::new(shape));

        xpos = 200.0;
        ypos = 400.0;
        let pts: [&Vector; 3] = [
            &Vector::new(xpos, ypos),
            &Vector::new(xpos - 100.0, ypos + 100.0),
            &Vector::new(xpos + 100.0, ypos + 100.0),
        ];
        let line_color = Color::from_hex("#8B008B");
        let mut mesh = DrawShape::triangle(&pts, None, Some(line_color), 10.0);
        let shape = ShapeView::new(frame, ShapeDef::Triangle).with_mesh(&mut mesh);
        scene.add_view(Box::new(shape));

        xpos = 600.0;
        ypos = 400.0;
        let pts: [&Vector; 4] = [
            &Vector::new(xpos, ypos),
            &Vector::new(xpos + 50.0, ypos),
            &Vector::new(xpos + 20.0, ypos + 100.0),
            &Vector::new(xpos - 30.0, ypos + 100.0),
        ];
        let fill_color = Color::from_hex("#4169E1");
        let mut mesh = DrawShape::quad(&pts, Some(fill_color), None, 2.0);
        let shape = ShapeView::new(frame, ShapeDef::Quad).with_mesh(&mut mesh);
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
    Buttons,
    ListBox,
    TextEditor,
    FormInputs,
    Shapes,
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

        s.demo_list.push(Demo::Buttons);
        s.demo_list.push(Demo::ListBox);
        s.demo_list.push(Demo::TextEditor);
        s.demo_list.push(Demo::FormInputs);
        s.demo_list.push(Demo::Shapes);

        s.demo_index = 2;
        let demo = s.demo_list[s.demo_index].clone();
        s.load_demo(screen, &demo);
        Ok(s)
    }

    fn load_demo(&mut self, screen: Vector, demo: &Demo) {
        let mut scene = match demo {
            Demo::Buttons => SceneBuilder::load_buttons_scene(screen),
            Demo::ListBox => SceneBuilder::load_listbox_scene(screen),
            Demo::TextEditor => SceneBuilder::load_text_edit_scene(screen, &mut self.theme),
            Demo::FormInputs => SceneBuilder::load_form_inputs_scene(screen),
            Demo::Shapes => SceneBuilder::load_shapes_scene(screen),
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
    std::env::set_var("RUST_LOG", "main=debug,tweek=trace");

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::builder().default_format_timestamp(false).default_format_module_path(false).init();
    #[cfg(not(target_arch = "wasm32"))]
    color_backtrace::install();

    let screen = Vector::new(800, 600);
    run_with("Tweek UI", screen, Settings::default(), || MainState::new(screen));
}
