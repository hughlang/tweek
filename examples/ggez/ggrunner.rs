/// Placeholder file
mod helper1;
use helper1::*;

extern crate ggez;
extern crate tween;

use ggez::conf;
use ggez::event::{self};
use ggez::graphics::{self};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::mint;

use std::env;
use std::path;
use tween::*;

const SQUARE_ITEM_ID: usize = 100;
const ROUND_ITEM_ID: usize = 101;

struct MainState {
    tweek: Tweek,
    context: TKContext,
    items: Vec<ItemState>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {

        // Add a rectangle
        let rect = graphics::Rect::new(0.0, 0.0, 50.0, 50.0);
        let mut item1 = ItemState::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.layer.graphics.color = graphics::Color::from_rgb_u32(0x333333);

        let tween1 = Tween::with(SQUARE_ITEM_ID, &vec![&item1.layer.frame, &item1.layer.graphics])
            .to(vec![position(400.0, 300.0), size(100.0, 100.0), alpha(0.2)])
            .duration(2.0);

        // Add a circle
        let mut item2 = ItemState::new(ROUND_ITEM_ID, Shape::Circle(mint::Point2{x: 500.0, y: 200.0}, 40.0))?;
        item2.layer.graphics.color = graphics::Color::from_rgb_u32(0xCD09AA);

        let tween2 = Tween::with(ROUND_ITEM_ID, &vec![&item2.layer.frame, &item2.layer.graphics])
            .to(vec![position(40.0, 200.0), alpha(0.2)])
            .duration(3.0);

        let mut tweek = Tweek::new();
        let context = TKContext::new();
        let timeline = Timeline::create(vec![tween1, tween2], TweenAlign::Normal);
        tweek.add_timeline(timeline);
        &tweek.play();

        let mut items: Vec<ItemState> = Vec::new();
        items.push(item1);
        items.push(item2);

        let s = MainState {
            tweek: tweek,
            context: context,
            items: items,
        };

        Ok(s)
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        self.tweek.tick(); // This is called to check on completion status of each tween

        for item in &mut self.items {
            item.update()?;
        }

        // let item = &mut self.square_item;
        // if let Some(update) = self.tweek.get_update(&item.get_id()) {
        //     item.bounds.render_update(&update.props);
        //     item.graphics.color.render_update(&update.props);
        // }

        // let item = &mut self.round_item;
        // if let Some(update) = self.tweek.get_update(&item.get_id()) {
        //     item.bounds.render_update(&update.props);
        //     item.graphics.color.render_update(&update.props);
        // }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);

        for item in &mut self.items {
            item.render(ctx)?;
        }

        graphics::present(ctx)?;

        // timer::yield_now();
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

    let setup = conf::WindowSetup::default().title("Tween Runner test");

    let cb = ContextBuilder::new("tween0", "tweenkit")
        .window_setup(setup)
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
