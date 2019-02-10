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

struct ItemState {
    bounds: graphics::Rect,
    fill_color: graphics::Color,
    tween: Option<Tween>,
}

impl ItemState {
    fn new(x: f32, y: f32, w: f32, h: f32) -> GameResult<ItemState> {
        let rect = graphics::Rect::new(x, y, w, h);
        Ok(ItemState {
            bounds: rect,
            fill_color: graphics::WHITE,
            tween: None,
        })
    }
}


struct MainState {
    square_item: ItemState,
    round_item: ItemState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);
        let mut item1 = ItemState::new(0.0, 0.0, 50.0, 50.0)?;
        item1.fill_color = graphics::Color::from_rgb_u32(0x333333);

        let mut tween1 = Tween::with(&vec![&item1.bounds, &item1.fill_color]).with_id(SQUARE_ITEM_ID)
            .to(vec![position(400.0, 300.0), size(100.0, 100.0), alpha(0.2)])
            .duration(2.0);
        &tween1.play();
        item1.tween = Some(tween1);

        let mut item2 = ItemState::new(500.0, 200.0, 80.0, 80.0)?;
        item2.fill_color = graphics::Color::from_rgb_u32(0xCD09AA);
        let mut tween2 = Tween::with(&vec![&item2.bounds, &item2.fill_color]).with_id(ROUND_ITEM_ID)
            .to(vec![position(40.0, 400.0), alpha(0.2)])
            .duration(2.0).ease(Easing::SineIn);

        &tween2.play();
        item2.tween = Some(tween2);

        let s = MainState {
            square_item: item1,
            round_item: item2,
        };
        Ok(s)
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Here is where you tell which objects to update in each run loop.
        // MainState will have one or more Tween objects that need to be updated.
        if let Some(tween) = &self.square_item.tween {
            if let Some(update) = tween.update_item(&SQUARE_ITEM_ID) {
                self.square_item.bounds.render_update(&update.props);
                self.square_item.fill_color.render_update(&update.props);
            }
        }
        if let Some(tween) = &self.round_item.tween {
            if let Some(update) = tween.update_item(&ROUND_ITEM_ID) {
                self.round_item.bounds.render_update(&update.props);
                self.round_item.fill_color.render_update(&update.props);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);

        let item = &self.square_item;
        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), item.bounds, item.fill_color)?;
        let drawparams = graphics::DrawParam::new();
        let _result = graphics::draw(ctx, &r1, drawparams);

        let item = &self.round_item;
        let p2 = na::Point2::new(item.bounds.x, item.bounds.y);
        let r2 = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), p2, item.bounds.h / 2.0, 1.0, item.fill_color)?;
        let drawparams = graphics::DrawParam::new();
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
