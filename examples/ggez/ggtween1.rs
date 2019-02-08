//! The simplest possible example that does something.

extern crate ggez;
extern crate tween;

use ggez::conf;
use ggez::event;
use ggez::graphics::{self};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::nalgebra as na;

use std::env;
use std::path;
use tween::*;

const SQUARE_ITEM_ID: usize = 100;
const ROUND_ITEM_ID: usize = 101;

struct Assets {
    // square_rect: graphics::Rect,
    square_item: ItemState,
    round_item: ItemState,
}

impl Assets {
    fn new(_ctx: &mut Context) -> GameResult<Assets> {
        let square_item = ItemState::new(0.0, 0.0, 50.0, 50.0)?;
        let mut round_item = ItemState::new(500.0, 200.0, 80.0, 80.0)?;
        round_item.fill_color = graphics::Color::from_rgb_u32(0xCD09AA);
        Ok(Assets {
            square_item,
            round_item,
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
    square_tween: Option<Tween>,
    round_tween: Option<Tween>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);

        let assets = Assets::new(ctx)?;
        let item1 = &assets.square_item;

        let mut tween1 = Tween::with(&vec![&item1.bounds, &item1.fill_color])
            .to(vec![position(400.0, 300.0), size(100.0, 100.0), alpha(0.1)])
            .duration(2.0).with_id(SQUARE_ITEM_ID);
        tween1.add_events_hook(Logger);

        &tween1.play();

        let item2 = &assets.round_item;
        let mut tween2 = Tween::with(&vec![&item2.bounds, &item2.fill_color])
            .to(vec![position(40.0, 400.0), alpha(0.2)])
            .duration(2.0).ease(Easing::SineIn).with_id(ROUND_ITEM_ID);
        &tween2.play();

        let s = MainState {
            assets: assets,
            square_tween: Some(tween1),
            round_tween: Some(tween2),
        };
        Ok(s)
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Here is where you tell which objects to update in each run loop.
        // MainState will have one or more Tween objects that need to be updated.
        if let Some(tween) = &self.square_tween {
            if let Some(update) = tween.update_item(&SQUARE_ITEM_ID) {
                self.assets.square_item.bounds.render_update(&update.props);
                self.assets.square_item.fill_color.render_update(&update.props);
            }
        }
        if let Some(tween) = &self.round_tween {
            if let Some(update) = tween.update_item(&ROUND_ITEM_ID) {
                self.assets.round_item.bounds.render_update(&update.props);
                self.assets.round_item.fill_color.render_update(&update.props);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.assets.square_item.bounds, self.assets.square_item.fill_color)?;
        let drawparams = graphics::DrawParam::new();
        //     .dest(Point2::new(r1.buffer, y));
            // .rotation(actor.facing as f32)
            // .offset(Point2::new(0.5, 0.5));
        let _result = graphics::draw(ctx, &r1, drawparams);


        let p2 = na::Point2::new(self.assets.round_item.bounds.x, self.assets.round_item.bounds.y);
        let r2 = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), p2, self.assets.round_item.bounds.h / 2.0, 1.0, self.assets.round_item.fill_color)?;
        // let drawparams = graphics::DrawParam::new();
        let _result = graphics::draw(ctx, &r2, drawparams);

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
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
