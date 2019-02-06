//! The simplest possible example that does something.

extern crate ggez;
extern crate cgmath;
extern crate tween;

use tween::*;

use ggez::event;
use ggez::graphics::{self, Drawable, DrawParam};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let rect = graphics::Rect::new(self.pos_x, 200.0, 50.0, 50.0);
        let r1 =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::Fill, rect, graphics::WHITE)?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        // graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::Fill,
        //     na::Point2::new(self.pos_x, 380.0),
        //     100.0,
        //     2.0,
        //     graphics::WHITE,
        // )?
        // .draw(ctx, (na::Point2::new(0.0, 0.0),))?;

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
