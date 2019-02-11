/// Placeholder file
mod manager1;
use manager1::*;

extern crate ggez;
extern crate tween;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::mint;

use std::env;
use std::path;
use tween::*;

const SQUARE_ITEM_ID: usize = 100;
const ROUND_ITEM_ID: usize = 101;

struct MainState {
    timeline: Timeline,
    square_item: ItemState,
    round_item: ItemState,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {

        // Add a rectangle
        let rect = graphics::Rect::new(0.0, 0.0, 50.0, 50.0);
        let mut item1 = ItemState::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.fill_color = graphics::Color::from_rgb_u32(0x333333);

        let tween1 = Tween::with(&vec![&item1.bounds, &item1.fill_color]).with_id(SQUARE_ITEM_ID)
            .to(vec![position(400.0, 300.0), size(100.0, 100.0), alpha(0.2)])
            .duration(2.0);

        // Add a circle
        let mut item2 = ItemState::new(ROUND_ITEM_ID, Shape::Circle(mint::Point2{x: 500.0, y: 200.0}, 40.0))?;
        item2.fill_color = graphics::Color::from_rgb_u32(0xCD09AA);

        let tween2 = Tween::with(&vec![&item2.bounds, &item2.fill_color]).with_id(ROUND_ITEM_ID)
            .to(vec![position(40.0, 200.0), alpha(0.2)])
            .duration(2.0).ease(Easing::SineIn);


        let mut timeline = Timeline::create(vec![tween1, tween2], TweenAlign::Normal);
        &timeline.play();

        let s = MainState {
            timeline: timeline,
            square_item: item1,
            round_item: item2,
        };

        Ok(s)
    }

    // Note: this won't work. The compiler knows if you are making a self reference
    // mutable more than once, which is what happens when you try to call this from the
    // run loop. I think it's also a protection against re-entrancy bugs
    // fn render_update(&self, item: &mut ItemState) {
    //     if let Some(update) = self.timeline.get_update(&item.get_id()) {
    //         item.bounds.render_update(&update.props);
    //         item.fill_color.render_update(&update.props);
    //     }
    // }

}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let item = &mut self.square_item;
        self.timeline.tick();
        if let Some(update) = self.timeline.get_update(&item.get_id()) {
            item.bounds.render_update(&update.props);
            item.fill_color.render_update(&update.props);
        }

        let item = &mut self.round_item;
        if let Some(update) = self.timeline.get_update(&item.get_id()) {
            item.bounds.render_update(&update.props);
            item.fill_color.render_update(&update.props);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);

        self.square_item.render(ctx)?;
        self.round_item.render(ctx)?;

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

    let cb = ContextBuilder::new("tween0", "tweenkit")
        .window_setup(conf::WindowSetup::default().title("Tween Runner test"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
