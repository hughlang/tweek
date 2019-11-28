/// Test area for scene animation
mod helper;
use helper::*;
use tweek::prelude::*;

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::Color,
    lifecycle::{run_with, Asset, Event, Settings, State, Window},
    load_file, Error, Result,
};

#[cfg(target_arch = "wasm32")]
use stdweb::console;

// use std::char;
// use unicode_normalization::char::compose;
// use unicode_normalization::UnicodeNormalization;

/// The main function serves as an entrypoint into the event loop
fn main() {
    // You can configure the amount of debug output for each module here. In this example, the default
    // log level for the tweek crate is debug
    #[cfg(not(target_arch = "wasm32"))]
    std::env::set_var("RUST_LOG", "main=debug,tweek=debug,quicksilver=debug");

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
        // Use the AppDelegate to manage the app events and display
        let mut delegate = AppDelegate::new(screen.clone());
        let screen_size = screen.clone();

        // This is where all the demos are loaded. Each builder function returns a container object that
        // can lazy load a scene that is displayed through the AppDelegate. Each demo builder appears in
        // sequence by navigating with the Previous/Next buttons.
        delegate.add_stage_builder(move || StageBuilder::shapes_demo(screen_size));
        delegate.add_stage_builder(move || StageBuilder::listbox_demo(screen_size));
        delegate.add_stage_builder(move || StageBuilder::text_editor_demo(screen_size));
        // delegate.add_stage_builder(move || StageBuilder::buttons_demo(screen_size));
        delegate.add_stage_builder(move || StageBuilder::checkboxes_demo(screen_size));

        // Set the nav scene
        delegate.set_nav_scene(DemoHelper::build_nav_scene(screen));

        let mut app = MainApp { delegate, screen, frames: 0 };
        // Signal to all display objects that the application is ready.
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
    fn listbox_demo(screen: Vector) -> Stage {
        let frame = Rectangle::new_sized(screen);
        let mut stage = Stage::new(frame.clone());
        stage.title = "Listbox Demo".to_string();

        let mut scene = Scene::new(frame);

        let numbers: Vec<u32> = (0..21).collect();
        let ds: Vec<String> = numbers.into_iter().map(|x| x.to_string()).collect();

        let frame = Rectangle::new((100.0, 200.0), (300.0, 200.0));
        let mut listbox = ListBox::new(frame);
        listbox.set_datasource(ds);
        listbox.row_border_style = BorderStyle::SolidLine(Color::from_hex("#EEEEEE"), 1.0);
        scene.add_control(Box::new(listbox));

        /* Ignore: This is just an experiment in text clipping */
        // let frame = Rectangle::new((500.0, 200.0), (200.0, 30.0));
        // let mut text = Text::new(frame, "Clip this title");
        // text.layer.font_style = FontStyle::new(20.0, Color::BLACK);
        // text.layer.lock_style = true;
        // text.text_align = TextAlign::Left;
        // text.vert_align = VertAlign::Bottom;
        // text.layer.debug = true;
        // text.layer.border_style = BorderStyle::SolidLine(Color::from_hex("#CCCCCC"), 0.5);
        // let subframe = Rectangle::new((500.0, 220.0), (200.0, 10.0));
        // text.subframe = Some(subframe);
        // scene.add_control(Box::new(text));

        stage.add_scene(scene);
        stage
    }

    /// Types of buttons:
    /// * Standard text button
    /// * Image button
    /// * Image and text button
    /// * Rounded corners and borders
    /// * Transparent
    /// * Animated text
    /// * Grouped toggle buttons
    // fn buttons_demo(screen: Vector) -> Stage {
    //     let frame = Rectangle::new_sized(screen);
    //     let mut stage = Stage::new(frame.clone());
    //     stage.title = "Buttons Demo".to_string();

    //     const GRID_COLUMN_INTERVAL: f32 = 120.0;
    //     const GRID_ROW_INTERVAL: f32 = 80.0;
    //     const BUTTON_W: f32 = 100.0;
    //     const BUTTON_H: f32 = 40.0;
    //     const TITLE_H: f32 = 30.0;
    //     const ROW_GAP: f32 = 20.0;
    //     let mut scene = Scene::new(frame);

    //     let mut xpos = 100.0;
    //     let mut ypos = 100.0;

    //     // Button 1 ---------------------
    //     let frame = scene.sub_frame((xpos, ypos), (BUTTON_W, TITLE_H));
    //     let mut text = Text::new(frame, "Text button");
    //     text.layer.font_style = FontStyle::new(12.0, Color::WHITE);
    //     text.text_align(TextAlign::Left);
    //     scene.add_control(Box::new(text));

    //     ypos += TITLE_H;
    //     let frame = scene.sub_frame((xpos, ypos), (BUTTON_W, BUTTON_H));
    //     let mut button = Button::new(frame).with_text("Continue");
    //     // button.layer.corner_radius = 3.0;
    //     scene.add_control(Box::new(button));

    //     ypos += frame.height() + ROW_GAP;
    //     // Button 2 ---------------------
    //     let frame = scene.sub_frame((xpos, ypos), (BUTTON_W, TITLE_H));
    //     let mut text = Text::new(frame, "Image buttons");
    //     text.layer.font_style = FontStyle::new(12.0, Color::WHITE);
    //     text.text_align(TextAlign::Left);
    //     scene.add_control(Box::new(text));

    //     ypos += TITLE_H;

    //     // Image only
    //     let bytes = DrawImage::load_file_bytes("icons/png/ios-heart.png");

    //     let frame = scene.sub_frame((xpos, ypos), (100.0, 100.0));
    //     let mut label = Label::new(frame.clone());
    //     if let Ok(img) = image::load_from_memory(bytes.as_slice()) {
    //         label.set_image(img);
    //     }
    //     label.display = LabelDisplay::Image;
    //     label.layer.font_style = FontStyle::new(16.0, Color::WHITE);

    //     let mut button = Button::new(frame);
    //     button.set_label(label);
    //     button.layer.corner_radius = 3.0;
    //     scene.add_control(Box::new(button));

    //     // Image Top
    //     xpos += frame.width() + 5.0;

    //     let frame = scene.sub_frame((xpos, ypos), (100.0, 100.0));
    //     let mut label = Label::new(frame.clone());
    //     if let Ok(img) = image::load_from_memory(bytes.as_slice()) {
    //         label.set_image(img);
    //     }
    //     label.set_text("Image Top");
    //     label.display = LabelDisplay::ImageAndText;
    //     label.layout = LabelLayout::ImageTop;
    //     label.layer.font_style = FontStyle::new(16.0, Color::RED);

    //     let mut button = Button::new(frame);
    //     button.set_label(label);
    //     button.layer.corner_radius = 3.0;
    //     scene.add_control(Box::new(button));

    //     // Image Bottom
    //     xpos += frame.width() + 5.0;

    //     let frame = scene.sub_frame((xpos, ypos), (100.0, 100.0));
    //     let mut label = Label::new(frame.clone());
    //     if let Ok(img) = image::load_from_memory(bytes.as_slice()) {
    //         label.set_image(img);
    //     }
    //     label.set_text("Image Bottom");
    //     label.display = LabelDisplay::ImageAndText;
    //     label.layout = LabelLayout::ImageBottom;
    //     label.layer.font_style = FontStyle::new(16.0, Color::WHITE);

    //     let mut button = Button::new(frame);
    //     button.set_label(label);
    //     button.layer.corner_radius = 3.0;
    //     scene.add_control(Box::new(button));

    //     // Previous button
    //     xpos = 100.0;
    //     ypos += frame.height() + 10.0;

    //     let bytes = DrawImage::load_file_bytes("icons/png/chevron-left.png");

    //     let frame = scene.sub_frame((xpos, ypos), (150.0, 50.0));
    //     let mut label = Label::new(frame.clone());
    //     if let Ok(img) = image::load_from_memory(bytes.as_slice()) {
    //         label.set_image(img);
    //     }
    //     label.set_text("Image Left");
    //     label.display = LabelDisplay::ImageAndText;
    //     label.layout = LabelLayout::ImageLeft;
    //     label.layer.font_style = FontStyle::new(16.0, Color::WHITE);

    //     let mut button = Button::new(frame);
    //     button.set_label(label);
    //     button.layer.corner_radius = 3.0;
    //     scene.add_control(Box::new(button));

    //     // Image Right button
    //     xpos += frame.width() + 10.0;
    //     let bytes = DrawImage::load_file_bytes("icons/png/chevron-right.png");

    //     let frame = scene.sub_frame((xpos, ypos), (150.0, 50.0));
    //     let mut label = Label::new(frame.clone());
    //     if let Ok(img) = image::load_from_memory(bytes.as_slice()) {
    //         label.set_image(img);
    //     }
    //     label.set_text("Image Right");
    //     label.display = LabelDisplay::ImageAndText;
    //     label.layout = LabelLayout::ImageRight;
    //     label.layer.font_style = FontStyle::new(16.0, Color::WHITE);

    //     let mut button = Button::new(frame);
    //     button.set_label(label);
    //     button.layer.corner_radius = 3.0;
    //     scene.add_control(Box::new(button));

    //     stage.add_scene(scene);
    //     stage
    // }

    fn text_editor_demo(screen: Vector) -> Stage {
        let frame = Rectangle::new_sized(screen);
        let mut stage = Stage::new(frame.clone());
        stage.title = "Text Editor Demo".to_string();

        let mut scene = Scene::new(frame);

        let text: String = include_str!("../../static/lipsum.txt").into();
        let mut xpos = 200.0;
        let mut ypos = 200.0;

        let frame = Rectangle::new((xpos, ypos), (200.0, 40.0));
        let mut textfield = TextField::new(frame, true);
        // textfield.set_text("ABCDEFGHIJK");
        textfield.set_placeholder("Enter email address");
        // textfield.set_text("čćdđe ёєжзѕиіїйјк");  // FIXME: Special chars not accepted
        scene.add_control(Box::new(textfield));

        // xpos += 170.0;
        // let frame = Rectangle::new((xpos, ypos), (150.0, 40.0));
        // let dot = char::from_u32(0x26AB).unwrap();

        // let mut textfield = TextField::new(frame, true).with_type(TextFieldType::Secure(dot));
        // textfield.set_placeholder("Enter password");
        // scene.add_control(Box::new(textfield));

        ypos += 50.0;

        let frame = Rectangle::new((xpos, ypos), (320.0, 200.0));
        let mut textarea = TextArea::new(frame, true);
        // textarea.set_color(&Color::from_hex(HexColors::White));
        textarea.set_text(&text);
        scene.add_control(Box::new(textarea));
        stage.add_scene(scene);
        stage
    }

    fn checkboxes_demo(screen: Vector) -> Stage {
        let frame = Rectangle::new_sized(screen);

        let mut stage = Stage::new(frame.clone());
        stage.title = "Checkboxes and Radio button groups".to_string();
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

        // Radio buttons style with single-select
        ypos += 80.0;
        let frame = Rectangle::new((xpos, ypos), (300.0, 160.0));
        let mut group = OptionGroup::new(frame);
        group.multi_select = false;
        group.check_style = CheckStyle::Radio;
        group.set_options(options.clone());
        scene.add_control(Box::new(group));

        // Regular checkboxes with multi-select
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

        stage.add_scene(scene);
        stage
    }

    fn shapes_demo(screen: Vector) -> Stage {
        let frame = Rectangle::new_sized(screen);

        let mut stage = Stage::new(frame.clone());
        stage.title = "Shapes".to_string();
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
        scene.add_view(Box::new(shape));

        xpos += 250.0;
        ypos = 100.0;
        let frame = Rectangle::new((xpos, ypos), (200.0, 150.0));
        let fill_color = Color::from_hex("#FFD700");
        let mut ellipse = DrawShape::ellipse(&frame, Some(fill_color), None, 0.0, 30.0);
        let mut shape = ShapeView::new(frame, ShapeDef::Ellipse).with_mesh(&mut ellipse);
        scene.add_view(Box::new(shape));

        xpos = 200.0;
        ypos = 200.0;

        let frame = Rectangle::new((xpos, ypos), (100.0, 100.0));
        let pts: [&Vector; 2] =
            [&Vector::new(frame.x(), frame.y()), &Vector::new(frame.x() - frame.width(), frame.y() + frame.height())];
        let line_color = Color::from_hex("#46F02F");
        let mut line = DrawShape::line(&pts, line_color, 4.0);
        let mut shape = ShapeView::new(frame, ShapeDef::Line).with_mesh(&mut line);
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

        // FIXME: why are coords off?
        // let frame = Rectangle::new((xpos, ypos), (100.0, 100.0));

        // let image = Image::from_bytes(include_bytes!("../../static/icons/png/ios-heart.png")).unwrap();
        // let frame = Rectangle::new((xpos, ypos), (100.0, 100.0));
        // let mut image_view = ImageView::new(frame, image);
        // scene.add_view(Box::new(image_view));

        xpos = 600.0;
        ypos = 100.0;
        let path = "tile.png";

        let asset = Asset::new(load_file(path));
        let frame = Rectangle::new((xpos, ypos), (100.0, 100.0));
        let mut image_view = ImageView::new(frame, asset);
        scene.add_view(Box::new(image_view));

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
