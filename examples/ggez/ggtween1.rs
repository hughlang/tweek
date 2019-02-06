//! The simplest possible example that does something.

extern crate ggez;
extern crate tween;

use ggez::conf;
use ggez::event;
use ggez::graphics::{self, Drawable, DrawParam};
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;
use tween::*;

struct Assets {
    square_rect: graphics::Rect,
}

impl Assets {
    fn new(_ctx: &mut Context) -> GameResult<Assets> {
        let square_rect = graphics::Rect::new(100.0, 200.0, 50.0, 50.0);
        Ok(Assets {
            square_rect,
        })
    }
}

struct MainState {
    assets: Assets,
    tweens: Vec<Tween>,
    screen_width: f32,
    screen_height: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);

        let assets = Assets::new(ctx)?;
        let mut tweens: Vec<Tween> = Vec::new();
        let mut tween = Tween::animate(&assets.square_rect, vec![position(640.0, 480.0)]).duration(4.0);
        &tween.play();
        tweens.push(tween);

        let s = MainState {
            assets,
            tweens,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for tween in &self.tweens {
            let updates = tween.get_updates();
            for update in updates {
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
