/// Placeholder file

extern crate ggez;
extern crate tween;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self};
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;
use tween::*;

const SQUARE_ITEM_ID: usize = 100;

struct Assets {
    square_item: ItemState,
}

impl Assets {
    fn new(_ctx: &mut Context) -> GameResult<Assets> {
        let mut square_item = ItemState::new(0.0, 0.0, 50.0, 50.0)?;
        square_item.fill_color = graphics::Color::from_rgb_u32(0xCD09AA);
        Ok(Assets {
            square_item,
        })
    }
}

struct ItemState {
    bounds: graphics::Rect,
    fill_color: graphics::Color,
}

impl ItemState {
    fn new(x: f32, y: f32, w: f32, h: f32) -> GameResult<ItemState> {
        let rect = graphics::Rect::new(x, y, w, h);
        Ok(ItemState {
            bounds: rect,
            fill_color: graphics::WHITE,
        })
    }
}

struct MainState {
    assets: Assets,
    timeline: Timeline,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {

        let assets = Assets::new(ctx)?;
        let item1 = &assets.square_item;

        let tween1 = Tween::with(&vec![&item1.bounds, &item1.fill_color])
            .to(vec![position(400.0, 400.0)])
            .duration(2.0);

        let timeline = Timeline::create(vec![tween1], TweenAlign::Normal);
        let s = MainState {
            assets: assets,
            timeline: timeline,
        };
        Ok(s)
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Here is where you tell which objects to update in each run loop.
        // MainState will have one or more Tween objects that need to be updated.
        if let Some(update) = self.square_tween.update_item(&SQUARE_ITEM_ID) {
            self.assets.square_item.bounds.render_update(&update.props);
            self.assets.square_item.fill_color.render_update(&update.props);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(),
            self.assets.square_item.bounds, self.assets.square_item.fill_color)?;
        let drawparams = graphics::DrawParam::new();
        let _result = graphics::draw(ctx, &r1, drawparams);

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
        .window_setup(conf::WindowSetup::default().title("Timeline test"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;
    // println!("Game resource path: {:?}", ctx.filesystem);

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
