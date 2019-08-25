/// Test area for scene animation
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


/// The main function serves as an entrypoint into the event loop
fn main() {
    std::env::set_var("RUST_LOG", "main=debug,tweek=debug,tweek::gui=trace");

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::builder().default_format_timestamp(false).default_format_module_path(true).init();
    #[cfg(not(target_arch = "wasm32"))]
    color_backtrace::install();

    let screen = Vector::new(800, 600);
    run_with("Tweek UI", screen, Settings::default(), || MainApp::new(screen));
}

// *****************************************************************************************************
// MainApp is representative of a windowed-application that conforms to Quicksilver's run loop
// and event model
// *****************************************************************************************************

#[allow(dead_code)]
#[allow(unused_variables)]
struct MainApp {
    delegate: AppDelegate,
    screen: Vector,
    frames: usize,
}

impl MainApp {
    fn new(screen: Vector) -> Result<MainApp> {

        let mut delegate = AppDelegate::new(screen.clone());
        let screen_size = screen.clone();

        delegate.register_scene(move || {
            SceneBuilder::load_modals_scene(screen_size)
        });
        delegate.register_scene(move || {
            SceneBuilder::load_themes_demo(screen_size)
        });
        delegate.register_scene(move || {
            SceneBuilder::build_dots_demo(screen_size)
        });

        // Set the nav scene
        delegate.set_nav_scene(DemoHelper::build_nav_scene(screen));

        let mut app = MainApp {
            delegate,
            screen,
            frames: 0,
        };
        app.delegate.application_ready();
        Ok(app)
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl State for MainApp {
    fn new() -> Result<MainApp> {
        Err(Error::ContextError("Use run_with to execute custom new method".to_string()))
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.delegate.update(window)

    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.delegate.draw(window)
    }

    #[allow(unused_assignments)]
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        self.delegate.event(event, window)
    }
}

// ************************************************************************************
// SceneBuilder loads a scene to showcase functionality
// ************************************************************************************

struct SceneBuilder {}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
impl SceneBuilder {
    fn load_modals_scene(screen: Vector) -> Stage {
        let mut stage = Stage::new(Rectangle::new_sized(screen));
        stage.title = "Modal popup from bottom".to_string();
        let button_w = 120.0;
        let modal_w = 200.0;
        let modal_h = 200.0;
        let modal_x = (screen.x - modal_w) / 2.0;

        let frame = Rectangle::new_sized(screen);
        let mut bg_scene = Scene::new(frame).with_id(BG_SCENE, "Background");

        let mut xpos = (screen.x - button_w) / 2.0;
        let mut ypos = 80.0;

        let frame = Rectangle::new((xpos, ypos), (button_w, 50.0));
        let mut button = Button::new(frame)
            .background(BackgroundStyle::Solid(Color::from_hex(HexColors::DarkGreen)))
            .border(BorderStyle::SolidLine(Color::BLACK, 3.0))
            .with_text("Show modal");
        button.layer.corner_radius = 3.0;
        button.layer.font_style = FontStyle::new(12.0, Color::WHITE);

        ypos = 200.0;
        let mut command = Command::new(Box::new(button))
            .target(MAIN_SCENE, TypeId::of::<Scene>())
            .result(SceneEvent::Show(MAIN_SCENE))
            .transition(PropSet::new(vec![position(modal_x, ypos)], 0.5)
                .for_type(TweenType::Move)
                .ease(Ease::SineInOut)
                .delay(0.5)
            );
        bg_scene.add_command(command);
        stage.scenes.push(bg_scene);

        xpos = (screen.x - modal_w) / 2.0;
        ypos = screen.y; // Below window view

        let frame = Rectangle::new((xpos, ypos), (modal_w, modal_h));
        let mut modal_scene = Scene::new(frame).with_id(MAIN_SCENE, "Main");
        modal_scene.layer.apply_props(&[border("#333333", 2.0, 1.0)]);
        modal_scene.layer.bg_style = BackgroundStyle::Solid(Color::WHITE);
        modal_scene.layer.border_style = BorderStyle::SolidLine(Color::BLACK, 1.0);

        // Add title. Define frame relative to 0,0 origin.
        ypos = 0.0;
        let frame = modal_scene.sub_frame((0.0, 0.0), (modal_w, 50.0));
        let label = Label::new(frame, "Example");
        modal_scene.add_view(Box::new(label));

        // Add body text to modal scene
        ypos += 50.0;
        let frame = modal_scene.sub_frame((0.0, ypos), (modal_w, 100.0));
        println!("initial text frame={:?}", frame);
        let string = "This project will take some time to complete. We are dependent on future improvements to Quicksilver, \
        and so we have time to make as close to perfect as possible. Keep iterating and start building with it. \
        When it finally goes live at version 1.0, it will be fully tested and proven.";
        let mut text = Text::new(frame, string).margin(8.0, 5.0);
        text.layer.font_style = FontStyle::new(14.0, Color::BLACK);
        text.multiline = true;
        modal_scene.add_control(Box::new(text));

        // Add Close button to modal scene
        xpos = (modal_scene.layer.frame.width() - button_w) / 2.0;
        ypos += frame.height() + 5.0;
        let frame = modal_scene.sub_frame((xpos, ypos), (button_w, 40.0));
        let mut button = Button::new(frame).with_text("Close")
            .background(BackgroundStyle::Solid(Color::from_hex(HexColors::DarkRed)));
        button.layer.corner_radius = 3.0;
        button.layer.font_style = FontStyle::new(12.0, Color::WHITE);

        ypos = screen.y;

        let mut command = Command::new(Box::new(button))
            .target(MAIN_SCENE, TypeId::of::<Scene>())
            .result(SceneEvent::Hide(MAIN_SCENE))
            .transition(PropSet::new(vec![position(modal_x, ypos)], 0.5)
                .for_type(TweenType::Move)
                .ease(Ease::SineInOut)
                .delay(0.5)
            );
        modal_scene.add_command(command);
        stage.scenes.push(modal_scene);
        stage
    }

    fn load_themes_demo(screen: Vector) -> Stage {
        let mut stage = Stage::new(Rectangle::new_sized(screen));
        stage.title = "Change themes".to_string();

        let mut xpos = 0.0;
        let mut ypos = 0.0;
        let frame = Rectangle::new((xpos, ypos), (screen.x, screen.y));
        let mut scene = Scene::new(frame).with_id(MAIN_SCENE, "Themes Scene");
        scene.layer.bg_style = BackgroundStyle::Solid(Color::WHITE);
        scene.layer.border_style = BorderStyle::SolidLine(Color::BLACK, 2.0);

        // Label
        xpos = 40.0;
        ypos = 80.0;
        // let frame = scene.sub_frame((xpos, ypos), (180.0, 40.0));
        // let label = Label::new(frame, "HELLO THERE");
        // scene.add_view(Box::new(label));

        // OK button
        ypos += 50.0;
        let frame = scene.sub_frame((xpos, ypos), (100.0, 50.0));
        let mut button = Button::new(frame).with_text("OK")
            .background(BackgroundStyle::Solid(Color::from_hex("#D2B48C")));
        button.layer.corner_radius = 3.0;
        scene.add_control(Box::new(button));

        // Checkbox
        ypos += 60.0;
        let frame = scene.sub_frame((xpos, ypos), (200.0, 40.0));
        let checkbox = Checkbox::new(frame).with_text("Click the checkbox", false);
        scene.add_control(Box::new(checkbox));

        // OptionGroup with radio buttons
        let list: Vec<(&str, bool)> = vec![
            ("This is option 1", false),
            ("This is option 2", false),
            ("This is option 3", false),
            ("This is option 4", false),
        ];

        ypos += 50.0;
        let frame = scene.sub_frame((xpos, ypos), (200.0, 105.0));
        let mut options = OptionGroup::new(frame);
        options.set_layout(OptionGroupLayout::HorizontalGrid(10.0, 5.0));
        options.multi_select = false;
        options.check_style = CheckStyle::Radio;
        options.set_options(list.clone());
        scene.add_control(Box::new(options));

        let numbers: Vec<u32> = (0..8).collect();
        let ds: Vec<String> = numbers.into_iter().map(|x| x.to_string()).collect();

        ypos += 130.0;
        let frame = scene.sub_frame((xpos, ypos), (200.0, 80.0));
        let mut listbox = ListBox::new(frame).datasource(ds);
        scene.add_control(Box::new(listbox));
        scene.layer.apply_props(&[border("#333333", 3.0, 1.0)]);

        // Buttons for selecting themes
        xpos = screen.x - 160.0;
        ypos = 100.0;
        let frame = scene.sub_frame((xpos, ypos), (100.0, 40.0));
        let mut button = Button::new(frame).with_text("Light theme");
        button.layer.corner_radius = 3.0;
        button.set_onclick(move |state| {
            state.event_bus.register_event(ThemeEvent::Change(LIGHT_THEME));
        });
        scene.add_control(Box::new(button));

        ypos += 50.0;
        let frame = scene.sub_frame((xpos, ypos), (100.0, 40.0));
        let mut button = Button::new(frame).with_text("Dark theme");
        button.layer.corner_radius = 3.0;
        button.set_onclick(move |state| {
            state.event_bus.register_event(ThemeEvent::Change(DARK_THEME));
        });
        scene.add_control(Box::new(button));

        stage.scenes.push(scene);
        stage
    }

    fn build_dots_demo(screen: Vector) -> Stage {
        let mut stage = Stage::new(Rectangle::new_sized(screen));
        stage.title = "Rotating dots".to_string();

        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame.clone());

        let draw_area = DemoHelper::get_draw_area(screen);
        let center_pt = Vector { x: screen.x / 2.0, y: screen.y / 2.0 };
        // let start_pt = center_pt - Vector::new(100.0, 100.0);
        let start_pt = Vector::new(0, -5.0);

        let dot_radius = 10.0;
        let scene_radius = 96.0;
        let dot_count = 8;

        let mut shapes: Vec<Box<dyn Displayable>> = Vec::with_capacity(dot_count);
        let mut tweens: Vec<Tween> = Vec::with_capacity(dot_count);
        let mut timeline = Timeline::new(frame);

        for i in 0..dot_count {
            let frame = Rectangle::new(start_pt, (dot_radius * 2.0, dot_radius * 2.0));

            let item_id = i as u32;
            let mut color = Color::RED;
            let alpha = 1.0 - (i as f32 / dot_count as f32) / 2.0;
            color.a = alpha;
            let mut dot = ShapeView::new(frame, ShapeDef::Circle)
                .with_background(BackgroundStyle::Solid(color));
            // dot.layer.anchor_pt = Vector::new(0.0, 100.0);
            dot.layer.anchor_pt = center_pt;
            dot.build();
            let mut tween = Tween::with(item_id, &dot.layer)
                .to(&[rotate(360.0)])
                .duration(1.8)
                .ease(Ease::SineInOut)
                .repeat(-1, 0.8);
            tween.state = PlayState::Pending;
            dot.layer.set_animation(tween);
            timeline.add_sprite(Box::new(dot), 0.0);
        }
        timeline.stagger(0.125);
        &timeline.play();
        scene.set_timeline(timeline);

        // let timeline = Timeline::add(tweens).stagger(0.12);
        // stage.timelines.push(timeline);
        stage.scenes.push(scene);

        stage
    }
    /// ********************************************************************************
    /// This is a template for creating a new animation.
    /// Copy it and try out different animation techniques.
    /// Add an entry to the Demo enum below to make it part of the Next/Previous cycle.
    fn empty_template(screen: Vector) -> Vec<Scene> {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        // =====================================================
        // Create scene here
        // =====================================================

        vec![scene]
    }
}