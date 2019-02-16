/// Experiments with transforms
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
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {

        // Add a rectangle
        let rect = graphics::Rect::new(0.0, 0.0, 50.0, 50.0);
        let mut item1 = ItemState::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.layer.graphics.color = graphics::Color::from_rgb_u32(0x333333);

        let mut tween1 = Tween::with(SQUARE_ITEM_ID, &item1.layer)
            .to(vec![position(400.0, 100.0), size(100.0, 100.0), alpha(0.2)])
            .duration(1.0).repeat(7, 0.25).yoyo();

        &tween1.play();
        item1.tween = Some(tween1);

        // Add a circle
        let mut item2 = ItemState::new(ROUND_ITEM_ID, Shape::Circle(mint::Point2{x: 500.0, y: 200.0}, 40.0))?;
        item2.layer.graphics.color = graphics::Color::from_rgb_u32(0xCD09AA);

        let mut tween2 = Tween::with(ROUND_ITEM_ID, &item2.layer)
            .to(vec![position(40.0, 400.0), alpha(0.2)])
            .duration(2.0);

        &tween2.play();
        item2.tween = Some(tween2);

        let tile = graphics::Image::new(ctx, "/tile.png")?;
        let rect = graphics::Rect::new(10.0, 300.0, 80.0, 80.0);
        let mut item3 = ItemState::new(IMAGE_ITEM_ID, Shape::Image(rect))?;
        item3.image = Some(tile);

        let mut tween3 = Tween::with(IMAGE_ITEM_ID, &item3.layer)
            .to(vec![position(400.0, 300.0), alpha(0.2)]).duration(1.0)
            .to(vec![rotate(45.0)]);
        &tween3.play();
        item3.tween = Some(tween3);

        let text = graphics::Text::new(("Tweek everything", graphics::Font::default(), 48.0));
        let rect = graphics::Rect::new(20.0, 20.0, 200.0, 40.0);
        let mut item4 = ItemState::new(TEXT_ITEM_ID, Shape::Text(rect))?;
        item4.text = Some(text);

        let mut tween4 = Tween::with(TEXT_ITEM_ID, &item4.layer)
            .to(vec![position(400.0, 20.0), alpha(0.2)])
            .duration(3.0);
        &tween4.play();
        item4.tween = Some(tween4);

        let mut items: Vec<ItemState> = Vec::new();
        items.push(item1);
        items.push(item2);
        items.push(item3);
        // items.push(item4);

        let s = MainState {
            items: items,
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
