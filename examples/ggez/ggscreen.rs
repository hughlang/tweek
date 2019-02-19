use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics::{self, DrawParam};
use ggez::nalgebra as na;
use ggez::{Context, ContextBuilder, GameResult};

struct MainState {
    mesh1: graphics::Mesh,
    // mesh2: graphics::Mesh,
    // mesh3: graphics::Mesh,
    offset: f32,
}

const WINDOW_WIDTH: f32 = 1024.0;

impl MainState {
    fn new(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
        let width = ctx.conf.window_mode.width;
        let height = ctx.conf.window_mode.height;

        let color = graphics::BLACK;
        let mesh1 = graphics::MeshBuilder::new()
            .line(&[na::Point2::new(0.0, 0.0), na::Point2::new(0.0, 500.0)], 1.0, color,)?
            .line(
                &[na::Point2::new(50.0, 0.0), na::Point2::new(50.0, 500.0)],
                0.707, color,
            )?
            .line(
                &[na::Point2::new(100.0, 0.0), na::Point2::new(100.0, 500.0)],
                1.333, color,
            )?
            .line(
                &[na::Point2::new(150.0, 0.0), na::Point2::new(150.0, 500.0)],
                2.25, color,
            )?
            .line(
                &[na::Point2::new(200.0, 0.0), na::Point2::new(200.0, 500.0)],
                3.0, color,
            )?
            .build(ctx)?;
        Ok(Self { mesh1, offset: 0.0 })
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        while ggez::timer::check_update_time(ctx, 60) {
            self.offset += 0.1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        let pt = na::Point2::new(0.0, 0.0);
        graphics::draw(ctx, &self.mesh1, DrawParam::default())?;
        // graphics::draw(ctx, &self.mesh1, na::Point2::new(0.0, 0.0), self.offset / 100.0)?;

        // let pixel_perfect_offset = f32::floor(self.offset) + 25.0;
        // graphics::draw(ctx, &self.mesh1, na::Point2::new(pixel_perfect_offset, 300.0))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("tween0", "tweenkit")
        .window_setup(conf::WindowSetup::default().title("Tween test"))
        .window_mode(conf::WindowMode::default().dimensions(1024.0, 768.0).hidpi(true));


    let (ctx, events_loop) = &mut cb.build()?;

    println!("HIDPI: {}", graphics::os_hidpi_factor(ctx));

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)

    // let cb = ggez::ContextBuilder::new("heckin_dpi", "me")
    //     .window_setup(ggez::conf::WindowSetup::default()
    //         // .samples(8).unwrap()
    //         .hidpi(true))
    //     .window_mode(ggez::conf::WindowMode::default().dimensions(1024.0, 768.0));
    // let (ctx, events_loop) = &mut cb.build()?;

    // println!("HIDPI: {}", graphics::os_hidpi_factor(ctx));

    // let game = &mut MainState::new(ctx)?;
    // event::run(ctx, events_loop, game)
}