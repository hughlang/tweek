use ggez;
use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self, DrawParam};
use ggez::nalgebra as na;
use ggez::{Context, ContextBuilder, GameResult};

use tweek::prelude::*;

struct MainState {
    gridmesh: graphics::Mesh,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
        let width = ctx.conf.window_mode.width;
        let height = ctx.conf.window_mode.height;

        let gridmesh = GGTools::build_grid(ctx, width, height, 50.0, graphics::BLACK)?;

        Ok(Self { gridmesh })
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(ctx, &self.gridmesh, DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        println!("Mouse down at: x={} y={}", x, y);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        println!("resize_event w={} h={}", width, height);
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("tween0", "tweenkit")
        .window_setup(conf::WindowSetup::default().title("Screen size test"))
        // .window_mode(conf::WindowMode::default().dimensions(1024.0, 768.0).hidpi(true));
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0).hidpi(true));

    let (ctx, events_loop) = &mut cb.build()?;

    println!("graphics::os_hidpi_factor: {}", graphics::os_hidpi_factor(ctx));

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}