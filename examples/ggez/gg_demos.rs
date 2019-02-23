//! The simplest possible example that does something.
mod shape_helper;
use shape_helper::*;

extern crate ggez;
extern crate tweek;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self, DrawParam};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::mint::{self};
use ggez::nalgebra as na;

use std::env;
use std::path;
use tweek::prelude::*;


struct DemoHelper {}

#[allow(dead_code)]
#[allow(unused_mut)]
impl DemoHelper {

    fn build_arc_demo(ctx: &mut Context) -> GameResult<(Timeline, Vec<ItemState>)> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;
        let center_pt = mint::Point2{ x: screen_w/2.0, y: screen_h / 2.0 };
        let mut items: Vec<ItemState> = Vec::new();
        let mut tweens: Vec<Tween> = Vec::new();

        let dot_count = 6;
        let dot_radius = 10.0;
        let scene_radius = 96.0;
        const SQUARE_ITEM_ID: usize = 100;

        for i in 0..dot_count {
            let item_id = SQUARE_ITEM_ID + i as usize;
            let rect = graphics::Rect::new(center_pt.x, center_pt.y - scene_radius, dot_radius, dot_radius);
            let mut item1 = ItemState::new(item_id, Shape::Rectangle(rect))?;
            item1.layer.graphics.color = graphics::Color::from_rgb_u32(HexColors::Red);
            let alpha = i as f32 + 1.0 / dot_count as f32;
            println!("alpha={}", alpha);
            item1.layer.graphics.color.a = alpha;
            item1.layer.graphics.offset = na::Point2::new(center_pt.x, center_pt.y);

            let tween1 = Tween::with(item_id, &item1.layer)
                .to(vec![rotate(360.0)])
                .duration(1.5)
                .ease(Ease::SineIn)
                .repeat(-1, 0.5)
                ;
            items.push(item1);
            tweens.push(tween1)
        }

        let timeline = Timeline::add(tweens)
            .stagger(0.2)
            ;
        Ok((timeline, items))
    }

}

struct MainState {
    grid: graphics::Mesh,
    tweek: Tweek,
    tk_state: TKState,
    items: Vec<ItemState>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let screen_w = ctx.conf.window_mode.width;
        let screen_h = ctx.conf.window_mode.height;

        let gridmesh = GGTools::build_grid(ctx, screen_w, screen_h, 32.0, graphics::Color::from_rgb_u32(0xCCCCCC))?;

        let (timeline, items) = DemoHelper::build_arc_demo(ctx)?;

        let mut tweek = Tweek::new();
        tweek.add_timeline(timeline);
        &tweek.play();

        let tk_state = TKState::new();

        let s = MainState {
            grid: gridmesh,
            tweek: tweek,
            tk_state: tk_state,
            items: items,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        self.tweek.update(&mut self.tk_state);

        for item in &mut self.items {
            item.try_update(&mut self.tweek)?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(ctx, &self.grid, DrawParam::default())?;

        self.tweek.update(&mut self.tk_state);

        for item in &mut self.items {
            item.render(ctx)?;
        }

        graphics::present(ctx)?;

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
