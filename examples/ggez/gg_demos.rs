/// All the demos belong here
///
mod shape_helper;
use shape_helper::*;

extern crate ggez;
extern crate tweek;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::input::{mouse};
use ggez::graphics::{self, Rect, DrawParam, Color};
use ggez::mint::{self, Point2};
use ggez::nalgebra as na;
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;
use tweek::prelude::*;


const NEXT_COMMAND: u32 = 1;
const PREV_COMMAND: u32 = 2;

struct DemoHelper {}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
impl DemoHelper {

    /// This creates the Next and Previous buttons that make it easy to load and view animations.
    /// The set_onclick method appends a u32 value that is evaluated in the run loop update() method.
    fn make_buttons(ctx: &mut Context) -> GameResult<Vec<ButtonView>> {
        const BUTTON_WIDTH: f32 = 90.0;
        const BUTTON_HEIGHT: f32 = 40.0;
        const BUTTON_GAP: f32 = 20.0;
        let screen_w = ctx.conf.window_mode.width;

        let font = graphics::Font::new(ctx, "/Roboto-Bold.ttf")?;

        let mut buttons: Vec<ButtonView> = Vec::new();
        let xpos = 30.0;
        let ypos = 30.0;

        // ---- Previous ---------------------
        let frame = Rect::new(xpos, ypos, BUTTON_WIDTH, BUTTON_HEIGHT);
        let mut button = ButtonView::new(frame).with_title("Previous");
        button.set_font(&font, &18.0, &Color::from_rgb_u32(0xFFFFFF));
        button.set_color(&Color::from_rgb_u32(HexColors::Tan));
        button.set_hover_animation(vec![color(HexColors::Chocolate)], 0.1);
        button.set_onclick(move |_action, tk| {
            tk.commands.push(PREV_COMMAND);
        });
        buttons.push(button);

        // ---- Next ---------------------
        let frame = Rect::new(screen_w - BUTTON_WIDTH - 30.0, ypos, BUTTON_WIDTH, BUTTON_HEIGHT);
        let mut button = ButtonView::new(frame).with_title("Next");
        button.set_font(&font, &18.0, &Color::from_rgb_u32(0xFFFFFF));
        button.set_color(&Color::from_rgb_u32(HexColors::Tan));
        button.set_hover_animation(vec![color(HexColors::Chocolate)], 0.1);
        button.set_onclick(move |_action, state| {
            state.commands.push(NEXT_COMMAND);
        });
        buttons.push(button);

        Ok(buttons)
    }

    /// Create a unit vector representing the
    /// given angle (in radians)
    fn vec_from_angle(angle: f32) -> mint::Vector2<f32> {
        let vx = angle.sin();
        let vy = angle.cos();
        mint::Vector2{ x: vx, y: vy }
    }

    // nalgebra version
    // fn vec_from_angle(angle: f32) -> na::Vector2<f32> {
    //     let vx = angle.sin();
    //     let vy = angle.cos();
    //     na::Vector2::new(vx, vy)
    // }

    /// This demo shows a collection of dots rotating around in a circle
    fn build_dots_demo(ctx: &mut Context) -> GameResult<(Timeline, Vec<ItemState>)> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;
        let center_pt = mint::Point2{ x: screen_w / 2.0, y: screen_h / 2.0 };
        let mut items: Vec<ItemState> = Vec::new();
        let mut tweens: Vec<Tween> = Vec::new();

        let dot_count = 8;
        let dot_radius = 10.0;
        let scene_radius = 96.0;

        for i in 0..dot_count {
            let item_id = i + 10 as usize;

            let mut item1 = ItemState::new(item_id, Shape::Circle(mint::Point2{x: center_pt.x, y: center_pt.y - scene_radius}, dot_radius))?;
            item1.layer.graphics.color = Color::from_rgb_u32(HexColors::Red);
            let alpha = 1.0 - (i as f32 / dot_count as f32)/2.0;
            item1.layer.graphics.color.a = alpha;
            item1.layer.graphics.offset = na::Point2::new(center_pt.x, center_pt.y);

            let tween1 = Tween::with(item_id, &item1.layer)
                .to(vec![rotate(360.0)])
                .duration(1.8)
                .ease(Ease::SineInOut)
                .repeat(-1, 0.8)
                ;
            items.push(item1);
            tweens.push(tween1)
        }

        let timeline = Timeline::add(tweens)
            .stagger(0.12)
            ;
        Ok((timeline, items))
    }

    /// Draw lines and rotate from center
    fn build_lines_demo(ctx: &mut Context) -> GameResult<(Timeline, Vec<ItemState>)> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;
        let center_pt = mint::Point2{ x: screen_w / 2.0, y: screen_h / 2.0 };

        let mut items: Vec<ItemState> = Vec::new();
        let mut tweens: Vec<Tween> = Vec::new();

        const LINE_WIDTH: f32 = 9.0;
        let line_count = 3;
        let line_length = 300.0;

        for i in 0..line_count {
            let item_id = i as usize;
            let xpos = center_pt.x - line_length / 2.0;
            let ypos = center_pt.y;

            let mut item = ItemState::new(item_id,
                Shape::Line(
                    Point2{x: xpos, y: ypos},
                    Point2{x: xpos + line_length, y: ypos},
                    LINE_WIDTH)
                )?;

            item.layer.graphics.color = Color::from_rgb_u32(0xCD5C5C);
            item.layer.graphics.offset = na::Point2::new(center_pt.x, center_pt.y);

            // The plan was to make angle evenly distributed, but there seems to be a bug
            let delta = 360.0 / line_count as f32;
            let angle = i as f32 * (delta / 2.0);
            item.layer.graphics.rotation = angle.to_radians();
            let target = (120.0 + angle) as f64;
            let time = 2.0 as f64;
            let mut tween = Tween::with(item_id, &item.layer)
                .to(vec![rotate(target), color(0x556B2F)]).duration(time)
                .to(vec![rotate(target * 2.0), color(0x7FFFD4)]).duration(time)
                .to(vec![rotate(target * 3.0), color(0xCD5C5C)]).duration(time)
                .repeat(-1, 0.0)
                .yoyo()
                ;
            tween.debug = true;
            items.push(item);
            tweens.push(tween)
        }

        let timeline = Timeline::add(tweens);
        Ok((timeline, items))
    }

    /// Rockets!!! Many rockets of different sizes arcing across the screen endlessly.
    /// Random sizes and speed.
    fn build_rocket_demo(ctx: &mut Context) -> GameResult<(Timeline, Vec<ItemState>)> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let mut items: Vec<ItemState> = Vec::new();
        let mut tweens: Vec<Tween> = Vec::new();

        let rocket_count = 8;

        let image = graphics::Image::new(ctx, "/rocket.png")?;
        let base_h = *&image.height() as f32;
        let base_w = *&image.width() as f32;

        for i in 0..rocket_count {
            let item_id = i + 10 as usize;
            let scale = rand::random::<f32>();

            let x = base_w * scale * -1.0;
            let y = rand::random::<f32>() * screen_h * 0.8 + 50.0;

            let w = base_w * scale;
            let h = base_h * scale;
            let angle = 0.0 as f32;
            let rect = Rect::new(x, y, w, h);
            println!("new rocket={:?}", rect);
            let mut item = ItemState::new(item_id, Shape::Image(rect))?;
            item.image = Some(image.clone());
            // item.layer.graphics.offset = na::Point2::new(0.5, 0.5);
            item.layer.graphics.rotation = angle.to_radians();

            let tween = Tween::with(item_id, &item.layer)
                .to(vec![shift_x((screen_w + w).into())])
                .duration(1.8)
                .ease(Ease::SineInOut)
                .repeat(-1, 0.8)
                // .debug()
                ;
            items.push(item);
            tweens.push(tween)
        }

        let timeline = Timeline::add(tweens)
            // .stagger(0.12)
            ;
        Ok((timeline, items))
    }


    fn build_bars_demo(ctx: &mut Context) -> GameResult<(Timeline, Vec<ItemState>)> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let mut items: Vec<ItemState> = Vec::new();
        let mut tweens: Vec<Tween> = Vec::new();

        const STAGE_WIDTH: f32 = 600.0;
        const STAGE_HEIGHT: f32 = 400.0;
        const BAR_HEIGHT: f32 = 20.0;
        let draw_area = Rect::new((screen_w - STAGE_WIDTH) / 2.0, 100.0, STAGE_WIDTH, STAGE_HEIGHT);
        let line_count = 12;

        for i in 0..line_count {
            let item_id = i as usize;
            let ypos = i as f32 * (BAR_HEIGHT + 10.0) + draw_area.top();

            let rect = Rect::new(draw_area.left(), ypos, 0.0, BAR_HEIGHT);

            let mut item = ItemState::new(item_id, Shape::Rectangle(rect))?;
            item.layer.graphics.color = Color::from_rgb_u32(HexColors::Orange);

            let mut tween = Tween::with(item_id, &item.layer)
                .to(vec![size(draw_area.w as f64, BAR_HEIGHT as f64)])
                .duration(1.0)
                .ease(Ease::SineOut)
                .repeat(8, 0.2).yoyo()
                ;
            tween.debug = true;
            items.push(item);
            tweens.push(tween)
        }


        let timeline = Timeline::add(tweens)
            .stagger(0.1)
            ;
        Ok((timeline, items))
    }

    /// This demo scrolls lines of text from the bottom of the screen to the top, kind of like movie credits.
    /// It performs terribly in debug mode (11 fps), but achieves over 120 fps in release mode.
    /// A future optimization would be to use ggez graphics::draw_queued_text in the draw() function.
    /// Also consider combining all of the text into a mesh somehow.
    fn build_text_demo(ctx: &mut Context) -> GameResult<(Timeline, Vec<ItemState>)> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let mut items: Vec<ItemState> = Vec::new();
        let mut tweens: Vec<Tween> = Vec::new();

        let med_size = 20.0;
        let lines: Vec<(String, f32)> = vec![
            (String::from("Tweek: An animation kit for Rust"), 40.0),
            (String::from(" "), 40.0),
            (String::from("The name 'Tweek' originates from the word 'Tween', a term sometimes used in digital animation."), med_size),
            (String::from("Tweening can be more generally described as an interpolation system that calculates"), med_size),
            (String::from("changes in visual characteristics of an object over a specified duration."), med_size),
            (String::from("For example, the target position or size of an object can be specified and Tweek will provide"), med_size),
            (String::from("the numeric changes that can be applied in the graphics engine of your choice."), med_size),
        ];
        let font = graphics::Font::new(ctx, "/Roboto-Bold.ttf")?;
        let line_height = 50.0;
        let content_height = lines.len() as f64 * line_height as f64;

        for (i, line) in lines.iter().enumerate() {
            let item_id = (i + 1000) as usize;
            let text = graphics::Text::new((line.0.clone(), font, line.1.clone()));

            // Use text dimensions to center text horizontally
            let (width, height) = text.dimensions(ctx);
            let xpos = (screen_w - width as f32)/2.0;
            // Start the text offscreen below the bottom
            let ypos = screen_h + (i as f32 * line_height);

            let rect = graphics::Rect::new(xpos, ypos, width as f32, height as f32);
            let mut item = ItemState::new(item_id, Shape::Text(rect))?;
            item.text = Some(text);
            item.layer.graphics.color = graphics::BLACK;

            let move_y = (screen_h as f64 + content_height) * -1.0;
            let mut tween = Tween::with(item_id, &item.layer)
                .to(vec![shift_y(move_y)]).duration(12.0)
                .repeat(5, 0.5)
                ;

            items.push(item);
            tweens.push(tween);

        }

        let timeline = Timeline::add(tweens)
            // Add timeline configs here.
            ;
        Ok((timeline, items))
    }

    /// ********************************************************************************
    /// This is a template for creating a new animation.
    /// Copy it and try out different animation techniques.
    /// Add an entry to the Demo enum below to make it part of the Next/Previous cycle.
    fn empty_template(ctx: &mut Context) -> GameResult<(Timeline, Vec<ItemState>)> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let mut items: Vec<ItemState> = Vec::new();
        let mut tweens: Vec<Tween> = Vec::new();

        // =====================================================
        // Create items and tweens here and append results
        // =====================================================

        let timeline = Timeline::add(tweens)
            // Add timeline configs here.
            ;
        Ok((timeline, items))
    }

}

/// This enum is a list of all the loadable demo animations.
#[derive(Copy, Clone, Debug)]
enum Demo {
    Lines,
    Bars,
    TextScroller,
    DotCircle,
    Rocket,


}

/// ##########################################################################################
/// MainState is where the stage setup occurs and the creation of the Tweek objects that will
/// manage the animations. It also implements the EventHandler trait which is the run loop
/// in ggez.
/// ##########################################################################################

struct MainState {
    grid: graphics::Mesh,
    frames: usize,
    tweek: Tweek,
    tk_state: TKState,
    items: Vec<ItemState>,
    buttons: Vec<ButtonView>,
    demo_index: usize,
    demo_list: Vec<Demo>,
    debug: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let buttons = DemoHelper::make_buttons(ctx)?;
        let gridmesh = GGTools::build_grid(ctx, screen_w, screen_h, 32.0, Color::from_rgb_u32(0xCCCCCC))?;

        let mut s = MainState {
            grid: gridmesh,
            frames: 0,
            tweek: Tweek::new(),
            tk_state: TKState::new(),
            items: Vec::new(),
            buttons: buttons,
            demo_index: 0,
            demo_list: Vec::new(),
            debug: false,
        };

        // s.debug = true;


        // ===== If you are adding a new animation to try out, add it to the demo_list here. =====
        s.demo_list.push(Demo::Lines);
        s.demo_list.push(Demo::Bars);
        s.demo_list.push(Demo::DotCircle);
        s.demo_list.push(Demo::TextScroller);
        s.demo_list.push(Demo::Rocket);

        // Pick which demo to start with.
        s.demo_index = 0;
        let demo = s.demo_list[s.demo_index].clone();
        s.load_demo(ctx, &demo)?;

        Ok(s)
    }

    #[allow(unreachable_patterns)]
    /// This method takes a Demo enum as a parameter to identify which DemoHelper function
    /// to call and replace the current timeline animation.
    fn load_demo(&mut self, ctx: &mut Context, demo: &Demo) -> GameResult {
        let (timeline, items) = match demo {
            Demo::Bars => {
                DemoHelper::build_bars_demo(ctx)?
            },
            Demo::Lines => {
                DemoHelper::build_lines_demo(ctx)?
            },
            Demo::TextScroller => {
                DemoHelper::build_text_demo(ctx)?
            },
            Demo::DotCircle => {
                DemoHelper::build_dots_demo(ctx)?
            },
            Demo::Rocket => {
                DemoHelper::build_rocket_demo(ctx)?
            },
            _ => {
                DemoHelper::empty_template(ctx)?
            },
        };
        let mut tweek = Tweek::new();
        tweek.add_timeline(timeline);
        &tweek.play();

        let tk_state = TKState::new();
        self.tk_state = tk_state;
        self.tweek = tweek;
        self.items = items;
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
                    return Ok(())
                },
                PREV_COMMAND => {
                    if self.demo_index == 0 {
                        self.demo_index = self.demo_list.len() - 1;
                    } else {
                        self.demo_index -= 1;
                    }
                    let next = &self.demo_list[self.demo_index].clone();
                    &self.load_demo(ctx, next);
                    return Ok(())
                },
                _ => (),
            }
            self.tk_state.commands.clear();
        }

        self.tweek.update(&mut self.tk_state);

        for item in &mut self.items {
            item.try_update(&mut self.tweek)?;
        }
        for button in &mut self.buttons {
            button.update()?;
        }


        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);

        // To hide the grid, comment out this line.
        graphics::draw(ctx, &self.grid, DrawParam::default())?;

        if self.debug {
            self.frames += 1;
            if (self.frames % 20) == 0 {
                println!("FPS: {}", ggez::timer::fps(ctx));
            }
        }

        for button in &mut self.buttons {
            button.render(ctx)?;
        }

        self.tweek.update(&mut self.tk_state);

        for item in &mut self.items {
            // if let Some(text) = item.text {
            //     graphics::queue_text(ctx, &text, item.layer.graphics, None);
            // }
            item.render(ctx)?;
        }

        graphics::present(ctx)?;

        // timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        println!("Mouse down at: x={} y={}", x, y);
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

    let cb = ContextBuilder::new("tween0", "tweenkit")
        .window_setup(conf::WindowSetup::default().title("Tween test"))
        .window_mode(conf::WindowMode::default().dimensions(1024.0, 768.0).hidpi(false))
        .add_resource_path(resource_dir);


    let (ctx, events_loop) = &mut cb.build()?;

    println!("HIDPI: {}", graphics::os_hidpi_factor(ctx));

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
