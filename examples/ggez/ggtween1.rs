//! The simplest possible example that does something.

extern crate ggez;
extern crate tween;

use ggez::conf;
use ggez::event;
use ggez::graphics::{self, Drawable, DrawParam};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;
use tween::*;

const SQUARE_ITEM_ID: usize = 100;


// #[derive(Debug)]
// enum ActorType {
//     Shape,
//     Image,
// }


// #[derive(Debug)]
// struct Actor {
//     tag: ActorType,
//     bbox_size: f32,
// }

struct Assets {
    // square_rect: graphics::Rect,
    square_item: ItemState,
}

impl Assets {
    fn new(_ctx: &mut Context) -> GameResult<Assets> {
        let square_item = ItemState::new(SQUARE_ITEM_ID, 0.0, 0.0, 50.0, 50.0)?;
        Ok(Assets {
            square_item,
        })
    }
}

struct ItemState {
    id: usize,
    bounds: graphics::Rect,
    fill_color: graphics::Color,
}

impl ItemState {
    fn new(id: usize, x: f32, y: f32, w: f32, h: f32) -> GameResult<ItemState> {
        let rect = graphics::Rect::new(x, y, w, h);
        Ok(ItemState {
            id: id,
            bounds: rect,
            fill_color: graphics::WHITE,
        })
    }
}


struct MainState {
    assets: Assets,
    square_tween: Option<Tween>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);

        let assets = Assets::new(ctx)?;
        let mut tween = Tween::to(&assets.square_item.fill_color, vec![position(640.0, 480.0), alpha(0.1)]).duration(2.0).with_id(SQUARE_ITEM_ID);
        &tween.play();

        let s = MainState {
            assets: assets,
            square_tween: Some(tween),
        };
        Ok(s)
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if let Some(tween) = &self.square_tween {
            if let Some(update) = tween.update_item(&SQUARE_ITEM_ID) {
                self.assets.square_item.bounds.render_update(&update.props);
                self.assets.square_item.fill_color.render_update(&update.props);
                // self.square_state = update;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::Fill, self.assets.square_item.bounds, self.assets.square_item.fill_color)?;
        let x = r1.blend_mode();

        let drawparams = graphics::DrawParam::new();
        //     .dest(Point2::new(r1.buffer, y));
            // .rotation(actor.facing as f32)
            // .offset(Point2::new(0.5, 0.5));
        let _result = graphics::draw(ctx, &r1, drawparams);

        graphics::present(ctx)?;

        timer::yield_now();

        Ok(())
    }
}

fn _create_drawable(ctx: &mut Context, rect: &graphics::Rect, ui_state: &UIState) -> GameResult {
    let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::Fill, rect.clone(), graphics::WHITE)?;
    let drawparams = graphics::DrawParam::new();
        //     .dest(Point2::new(r1.buffer, y));
            // .rotation(actor.facing as f32)
            // .offset(Point2::new(0.5, 0.5));
    graphics::draw(ctx, &r1, drawparams)
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