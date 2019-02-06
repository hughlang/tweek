//! The simplest possible example that does something.

extern crate ggez;

use ggez::event;
use ggez::graphics::{self, Drawable, DrawParam};
use ggez::{Context, GameResult};

struct Assets {
    square_rect: graphics::Rect,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let square_rect = graphics::Rect::new(100.0, 200.0, 50.0, 50.0);
        Ok(Assets {
            square_rect,
        })
    }
}

struct MainState {
    assets: Assets,
    screen_width: f32,
    screen_height: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);

        let assets = Assets::new(ctx)?;
        let s = MainState {
            assets,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.assets.square_rect.x = self.assets.square_rect.x % 800.0 + 1.0;
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
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
