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
    // context: TKContext,
    items: Vec<ItemState>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {

        // Add a rectangle

        let mut item_id = 100 as usize;
        let mut ypos = 50.0 as f32;
        let mut items: Vec<ItemState> = Vec::new();
        let mut tweens: Vec<Tween> = Vec::new();
        for i in 0..4 {
            item_id = SQUARE_ITEM_ID + i as usize;
            let rect = graphics::Rect::new(50.0, ypos, 50.0, 50.0);
            let mut item1 = ItemState::new(item_id, Shape::Rectangle(rect))?;
            item1.layer.graphics.color = graphics::Color::from_rgb_u32(0x333333);

            let tween1 = Tween::with(item_id, &item1.layer)
                .to(vec![position(400.0, ypos as f64), size(100.0, 100.0), alpha(0.2)])
                .duration(1.0).yoyo();
            ypos += 100.0;
            items.push(item1);
            tweens.push(tween1)
        }

        let mut tweek = Tweek::new();
        // let context = TKContext::new();

        let timeline = Timeline::create(tweens, TweenAlign::Sequence).stagger(0.2);

        tweek.add_timeline(timeline);
        &tweek.play();

        let s = MainState {
            tweek: tweek,
            // context: context,
            items: items,
        };

        Ok(s)
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        self.tweek.tick(); // This is called to check on completion status of each tween

        for item in &mut self.items {
            item.try_update(&mut self.tweek)?;
        }

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
        .window_mode(conf::WindowMode::default().dimensions(1024.0, 768.0))
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
