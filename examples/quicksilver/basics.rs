/// Tweek Basics demo with Quicksilver
use tweek::prelude::*;

mod demo_helper;
use demo_helper::*;

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
    fn demo_square_1(screen: Vector) -> Vec<Item> {
        let item_id = 1;
        let draw_area = StageHelper::get_draw_area(screen);
        let rect = Rectangle::new((draw_area.pos.x, 200.0), (80.0, 80.0));
        let mut item1 = Item::new(item_id, ShapeType::Rectangle(rect));
        item1.layer.color = Color::RED;

        let target_x = draw_area.pos.x + draw_area.size.x - 120.0;
        let mut tween1 = Tween::with(item_id, &item1.layer)
            .to(&[position(target_x as f64, 400.0), size(120.0, 120.0), color(HexColors::Gold)])
            .duration(1.0)
            .ease(Ease::SineInOut)
            .repeat(9, 0.2)
            .yoyo();

        &tween1.play();
        item1.tween = Some(tween1);
        vec![item1]
    }

    fn demo_circle_1(screen: Vector) -> Vec<Item> {
        let draw_area = StageHelper::get_draw_area(screen);
        let center_pt = Vector { x: screen.x / 2.0 - 100.0, y: screen.y / 2.0 };
        eprintln!("center_pt={:?} y={:?}", center_pt, 0);
        let item_id = 2;
        // Add a circle
        let mut item2 = Item::new(item_id, ShapeType::Circle(center_pt, 40.0));
        item2.layer.color = Color::from_hex("#CD09AA");

        let mut tween2 = Tween::with(item_id, &item2.layer)
            .to(&[size(200.0, 200.0), alpha(0.9)])
            .duration(4.0)
            .to(&[position(700.0, draw_area.pos.y as f64 + 40.0), size(100.0, 100.0), alpha(0.8)])
            .duration(0.2)
            .ease(Ease::SineIn)
            .to(&[position((draw_area.pos.x + draw_area.size.x) as f64, 200.0), size(50.0, 50.0), alpha(0.7)])
            .duration(0.2)
            .to(&[position(650.0, (draw_area.pos.y + draw_area.size.y) as f64), size(20.0, 20.0), alpha(0.6)])
            .duration(0.2)
            .to(&[position(400.0, 300.0), size(5.0, 5.0), alpha(0.2)])
            .duration(0.5)
            .repeat(-1, 2.0);

        &tween2.play();
        item2.tween = Some(tween2);
        vec![item2]
    }

    fn demo_image_1(screen: Vector) -> Vec<Item> {
        let draw_area = StageHelper::get_draw_area(screen);

        let item_id = 3;
        let image = Image::from_bytes(include_bytes!("../../static/tile.png")).unwrap();
        let rect = Rectangle::new((draw_area.pos.x, 400.0), (100.0, 100.0));
        let mut item3 = Item::new(item_id, ShapeType::Image(rect));
        item3.image = Some(image);
        // item3.layer.graphics.offset = Point2 { x: 0.5, y: 0.5 };

        let mut tween3 = Tween::with(item_id, &item3.layer)
            .to(&[shift_x(draw_area.width() as f64), rotate(360.0)])
            .duration(3.0)
            .ease(Ease::BounceOut)
            .repeat(5, 0.5);

        &tween3.play();
        item3.tween = Some(tween3);

        vec![item3]
    }

    fn demo_rectangle_1(screen: Vector) -> Vec<Item> {
        let draw_area = StageHelper::get_draw_area(screen);
        let rect = Rectangle::new((draw_area.pos.x + 80.0, draw_area.pos.y), (20.0, 20.0));

        let w = 640.0 as f64;
        let h = 400.0 as f64;

        let item_id = 1;
        let mut item1 = Item::new(item_id, ShapeType::Rectangle(rect));
        let (r, g, b) = hex_to_rgb(HexColors::HotPink);
        item1.layer.color = Color::from_rgba(r, g, b, 1.0);

        let mut tween1 = Tween::with(item_id, &item1.layer)
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

        &tween1.play();
        item1.tween = Some(tween1);
        vec![item1]
    }
    /// ********************************************************************************
    /// This is a template for creating a new animation.
    /// Copy it and try out different animation techniques.
    /// Add an entry to the Demo enum below to make it part of the Next/Previous cycle.
    fn empty_template(screen: Vector) -> (Vec<Item>) {
        let draw_area = StageHelper::get_draw_area(screen);

        // =====================================================
        // Create item and tween here
        // =====================================================

        vec![]
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
enum Demo {
    Square1,
    Circle1,
    Image1,
    Rectangle1,
}

#[allow(dead_code)]
#[allow(unused_variables)]
struct MainState {
    grid: Grid,
    screen: Vector,
    theme: Theme,
    items: Vec<Item>,
    buttons: Vec<Button>,
    tk_state: TKState,
    demo_index: usize,
    demo_list: Vec<Demo>,
    show_fps: bool,
    // buttons: Vec<Button>,
}

impl MainState {
    fn new(screen: Vector) -> Result<MainState> {
        let theme = StageHelper::load_theme();
        let buttons = StageHelper::make_next_prev_buttons(&screen, &theme);

        let grid = StageHelper::build_grid(screen.x, screen.y, 32.0, Color::from_hex("#CCCCCC"));
        let mut s = MainState {
            grid,
            screen,
            theme,
            items: Vec::new(),
            buttons: buttons,
            tk_state: TKState::new(),
            demo_index: 0,
            demo_list: Vec::new(),
            show_fps: false,
        };

        s.demo_list.push(Demo::Square1);
        s.demo_list.push(Demo::Circle1);
        s.demo_list.push(Demo::Image1);
        s.demo_list.push(Demo::Rectangle1);

        s.demo_index = 2;
        let demo = s.demo_list[s.demo_index].clone();
        s.load_demo(screen, &demo);
        Ok(s)
    }

    fn load_demo(&mut self, screen: Vector, demo: &Demo) {
        self.tk_state.commands.clear();
        let items = match demo {
            Demo::Square1 => DemoBuilder::demo_square_1(screen),
            Demo::Circle1 => DemoBuilder::demo_circle_1(screen),
            Demo::Image1 => DemoBuilder::demo_image_1(screen),
            Demo::Rectangle1 => DemoBuilder::demo_rectangle_1(screen),
            // _ => DemoBuilder::empty_template(screen),
        };
        self.items = items;
    }
}

impl State for MainState {
    fn new() -> Result<MainState> {
        Err(Error::ContextError("Use run_with to execute custom new method".to_string()))
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
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
        for item in &mut self.items {
            item.update();
        }
        for button in &mut self.buttons {
            let _ = button.update();
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
                    if button.handle_mouse_up(&window.mouse().pos(), &mut self.tk_state) {
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

        for item in &mut self.items {
            item.render(window);
        }
        for button in &mut self.buttons {
            let _ = button.render(&mut self.theme, window);
        }

        Ok(())
    }
}

// The main isn't that important in Quicksilver: it just serves as an entrypoint into the event
// loop
fn main() {
    std::env::set_var("RUST_LOG", "main=trace,tweek=debug");

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::builder().default_format_timestamp(false).default_format_module_path(false).init();

    let screen = Vector::new(1024, 768);
    run_with("Tweek Basics", screen, Settings::default(), || MainState::new(screen));
}
