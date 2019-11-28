/// Test area for scene animation
mod helper;
use helper::*;
use tweek::prelude::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Background::Img, Color, Image, PixelFormat},
    input::{ButtonState, Key, MouseButton, MouseCursor},
    lifecycle::{run_with, Asset, Event, Settings, State, Window},
    load_file, Error, Result,
};

#[allow(unused_imports)]
use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, Rgba};

/// The main function serves as an entrypoint into the event loop
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    std::env::set_var("RUST_LOG", "main=debug,tweek=trace,tweek::gui=trace");

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

        delegate.add_stage_builder(move || StageBuilder::rolling_image(screen_size));
        delegate.add_stage_builder(move || StageBuilder::pink_snake(screen_size));
        delegate.add_stage_builder(move || StageBuilder::balloon_demo(screen_size));
        delegate.add_stage_builder(move || StageBuilder::animate_square(screen_size));

        // Set the nav scene
        delegate.set_nav_scene(DemoHelper::build_nav_scene(screen));

        let mut app = MainApp { delegate, screen, frames: 0 };
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
// StageBuilder loads scenes and returns a Stage object
// ************************************************************************************

struct StageBuilder {}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
impl StageBuilder {
    fn animate_square(screen: Vector) -> Stage {
        let frame = Rectangle::new_sized(screen);

        let mut stage = Stage::new(frame.clone());
        stage.title = "Animate Square".to_string();
        let mut scene = Scene::new(frame);

        let draw_area = DemoHelper::get_draw_area(screen);
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame);

        let item_id = 1;
        let draw_area = DemoHelper::get_draw_area(screen);
        let frame = Rectangle::new((draw_area.pos.x, 200.0), (80.0, 80.0));
        let fill_color = Color::RED;
        let mut square = ShapeView::new(frame, ShapeDef::Rectangle).with_background(BackgroundStyle::Solid(fill_color));
        square.build();

        let target_x = draw_area.pos.x + draw_area.size.x - 120.0;

        // let propset = PropSet::new(vec![shift(2.0, 2.0)], 0.1).for_type(TweenType::Click);

        let mut tween1 = Tween::with(item_id, &square.layer)
            .to(&[position(target_x, 400.0), size(120.0, 120.0), color(HexColors::Gold)])
            .duration(1.0)
            .ease(Ease::SineInOut)
            .repeat(-1, 0.2)
            .yoyo();

        square.layer.start_animation(tween1);
        // square.layer.tween_type = TweenType::Move;
        scene.add_view(Box::new(square));

        stage.add_scene(scene);
        stage
    }

    fn balloon_demo(screen: Vector) -> Stage {
        let frame = Rectangle::new_sized(screen);

        let mut stage = Stage::new(frame.clone());
        stage.title = "Balloon?".to_string();
        let mut scene = Scene::new(frame);

        let draw_area = DemoHelper::get_draw_area(screen);

        let center_pt = Vector { x: screen.x / 2.0 - 140.0, y: screen.y / 2.0 - 40.0 };
        log::debug!("center_pt={:?} y={:?}", center_pt, 0);

        // Add a circle
        let frame = Rectangle::new(center_pt, (80.0, 80.0));
        let item_id = 2;
        let mut shape =
            ShapeView::new(frame, ShapeDef::Circle).with_background(BackgroundStyle::Solid(Color::from_hex("#CD09AA")));
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

        stage.add_scene(scene);
        stage
    }

    /// An image that rolls across the screen
    fn rolling_image(screen: Vector) -> Stage {
        let frame = Rectangle::new_sized(screen);

        let mut stage = Stage::new(frame.clone());
        stage.title = "Rolling Image".to_string();
        let mut scene = Scene::new(frame);

        let draw_area = DemoHelper::get_draw_area(screen);

        let item_id = 3;
        let path = "tile.png";
        let asset = Asset::new(load_file(path));
        let rect = Rectangle::new((draw_area.pos.x, 400.0), (100.0, 100.0));
        let mut image_view = ImageView::new(rect, asset);

        let mut tween = Tween::with(item_id, &image_view.layer)
            .to(&[shift_x(draw_area.width() - 50.0), rotate(360.0)])
            .duration(3.0)
            .ease(Ease::BounceOut)
            .repeat(5, 0.5);

        image_view.layer.start_animation(tween);
        scene.add_view(Box::new(image_view));

        stage.add_scene(scene);
        stage
    }

    fn pink_snake(screen: Vector) -> Stage {
        let frame = Rectangle::new_sized(screen);

        let mut stage = Stage::new(frame.clone());
        stage.title = "Pink Snake".to_string();
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

        stage.add_scene(scene);
        stage
    }
    /// This is a template for creating a new stage.
    fn empty_template(screen: Vector) -> Stage {
        let frame = Rectangle::new_sized(screen);

        let mut stage = Stage::new(frame.clone());
        stage.title = "Example".to_string();
        let mut scene = Scene::new(frame);

        // =====================================================
        // Create scene here
        // =====================================================

        stage.add_scene(scene);
        stage
    }
}
