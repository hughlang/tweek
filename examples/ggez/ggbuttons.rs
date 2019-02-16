/// Experiments with buttons
///
///

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
const IMAGE_ITEM_ID: usize = 102;
const TEXT_ITEM_ID: usize = 103;

struct MainState {
    items: Vec<ItemState>,
    controls: Vec<TKButton>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {

        // Add a rectangle
        let rect = graphics::Rect::new(0.0, 0.0, 50.0, 50.0);
        let mut item1 = ItemState::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.layer.graphics.color = graphics::Color::from_rgb_u32(0x333333);

        let mut tween1 = Tween::with(SQUARE_ITEM_ID, &vec![&item1.layer.frame, &item1.layer.graphics])
            .to(vec![position(400.0, 100.0), size(100.0, 100.0), alpha(0.2)])
            .duration(1.0).repeat(7, 0.25).yoyo();

        &tween1.play();
        item1.tween = Some(tween1);

        let mut controls: Vec<TKButton> = Vec::new();

        let frame = graphics::Rect::new(50.0, 300.0, 120.0, 50.0);
        let label = graphics::Text::new("Play");
        let button = TKButton::new(frame, label);
        controls.push(button);

        let mut items: Vec<ItemState> = Vec::new();
        items.push(item1);

        let s = MainState {
            items: items,
            controls: controls,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for item in &mut self.items {
            item.update()?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for item in &mut self.items {
            item.render(ctx)?;
        }

        for control in &self.controls {
            control.render(ctx)?;
        }

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
