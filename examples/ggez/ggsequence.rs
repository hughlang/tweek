//! The simplest possible example that does something.
mod helper1;
use helper1::*;

extern crate ggez;
extern crate tween;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::mint;

use std::env;
use std::path;
use tween::*;

const SQUARE_ITEM_ID: usize = 100;
const ROUND_ITEM_ID: usize = 101;

struct MainState {
    square_item: ItemState,
    round_item: ItemState,
}

impl MainState {
    fn new(_: &mut Context) -> GameResult<MainState> {

        // Add a rectangle
        let rect = graphics::Rect::new(0.0, 0.0, 50.0, 50.0);
        let mut item1 = ItemState::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.fill_color = graphics::Color::from_rgb_u32(0x333333);

        let mut tween1 = Tween::with(SQUARE_ITEM_ID, &vec![&item1.frame, &item1.fill_color])
            .to(vec![position(400.0, 100.0), size(100.0, 100.0), alpha(0.2)])
            .duration(1.0).repeat(-1, 0.25).yoyo();

        &tween1.play();
        item1.tween = Some(tween1);

        // Add a circle
        let mut item2 = ItemState::new(ROUND_ITEM_ID, Shape::Circle(mint::Point2{x: 500.0, y: 200.0}, 40.0))?;
        item2.fill_color = graphics::Color::from_rgb_u32(0xCD09AA);

        let mut tween2 = Tween::with(ROUND_ITEM_ID, &vec![&item2.frame, &item2.fill_color])
            .to(vec![position(40.0, 400.0), alpha(0.2)]).duration(1.0)
            .to(vec![position(40.0, 40.0), alpha(1.0)]).duration(0.5)
            .to(vec![position(300.0, 40.0), alpha(1.0)]).duration(0.5)
            .to(vec![size(200.0, 200.0)]).duration(1.0)
            .repeat(-1, 0.25);

        &tween2.play();
        item2.tween = Some(tween2);


        // let mut item3 = ItemState
        let s = MainState {
            square_item: item1,
            round_item: item2,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        self.square_item.update()?;
        self.round_item.update()?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        self.square_item.render(ctx)?;
        self.round_item.render(ctx)?;

        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
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
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0).hidpi(true))
        .add_resource_path(resource_dir);


    let (ctx, events_loop) = &mut cb.build()?;

    println!("HIDPI: {}", graphics::os_hidpi_factor(ctx));

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
