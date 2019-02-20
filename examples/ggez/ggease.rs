//! The simplest possible example that does something.
mod helper1;
use helper1::*;

extern crate ggez;
extern crate tweek;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self, DrawParam};
use ggez::{Context, ContextBuilder, GameResult};
// use ggez::mint::Point2;

use std::env;
use std::path;
use tweek::prelude::*;


const SQUARE_ITEM_ID: usize = 100;
// const ROUND_ITEM_ID: usize = 101;
// const IMAGE_ITEM_ID: usize = 102;
// const TEXT_ITEM_ID: usize = 103;

struct MainState {
    gridmesh: graphics::Mesh,
    items: Vec<ItemState>,
    frames: usize,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let width = ctx.conf.window_mode.width;
        let height = ctx.conf.window_mode.height;

        let gridmesh = GGTools::build_grid(ctx, width, height, 50.0, graphics::BLACK)?;

        // Add a rectangle
        let rect = graphics::Rect::new(0.0, 0.0, 50.0, 50.0);
        let mut item1 = ItemState::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.layer.graphics.color = graphics::Color::from_rgb_u32(0x333333);

        let mut tween1 = Tween::with(SQUARE_ITEM_ID, &item1.layer)
            .to(vec![position(400.0, 100.0), size(100.0, 100.0), alpha(0.2)])
            .duration(1.0).ease(Ease::ElasticIn)
            .repeat(7, 0.2)
            ;

        &tween1.play();
        item1.tween = Some(tween1);


        let mut items: Vec<ItemState> = Vec::new();
        items.push(item1);
        // items.push(item2);
        // items.push(item3);
        // items.push(item4);
        let frames = 0 as usize;
        let s = MainState {
            gridmesh,
            items,
            frames
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for item in &mut self.items {
            item.update()?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(ctx, &self.gridmesh, DrawParam::default())?;

        for item in &mut self.items {
            item.render(ctx)?;
        }

        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        // timer::yield_now();
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
        // let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        // graphics::set_screen_coordinates(ctx, new_rect).unwrap();
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
        .window_mode(conf::WindowMode::default().dimensions(1024.0, 768.0).hidpi(false))
        .add_resource_path(resource_dir);


    let (ctx, events_loop) = &mut cb.build()?;

    println!("HIDPI: {}", graphics::os_hidpi_factor(ctx));

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
