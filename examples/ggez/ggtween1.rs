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
    square_rect: graphics::Rect,
}

impl Assets {
    fn new(_ctx: &mut Context) -> GameResult<Assets> {
        let square_rect = graphics::Rect::new(0.0, 0.0, 50.0, 50.0);
        Ok(Assets {
            square_rect,
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
        let mut tween = Tween::animate(&assets.square_rect, vec![position(640.0, 480.0)]).duration(4.0).with_id(SQUARE_ITEM_ID);
        &tween.play();

        let s = MainState {
            assets: assets,
            square_tween: Some(tween),
        };
        Ok(s)
    }
}

// fn draw_actor(
//     assets: &mut Assets,
//     ctx: &mut Context,
//     actor: &Actor,
//     world_coords: (f32, f32),
// ) -> GameResult {
//     let drawparams = graphics::DrawParam::new()
//         .dest(pos)
//         // .rotation(actor.facing as f32)
//         // .offset(Point2::new(0.5, 0.5));
//     graphics::draw(ctx, image, drawparams)
// }


impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if let Some(tween) = &self.square_tween {
            if let Some(update) = tween.update_item(&SQUARE_ITEM_ID) {
                self.assets.square_rect.render_update(&update.props);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let r1 =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::Fill, self.assets.square_rect, graphics::WHITE)?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

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
        .window_setup(conf::WindowSetup::default().title("Astroblasto!"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
