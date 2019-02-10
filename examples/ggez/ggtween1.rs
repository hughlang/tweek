//! The simplest possible example that does something.

extern crate ggez;
extern crate tween;

use ggez::conf;
use ggez::event;
use ggez::graphics::{self};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
// use ggez::nalgebra as na;
use ggez::mint;

use std::env;
use std::path;
use tween::*;

const SQUARE_ITEM_ID: usize = 100;
const ROUND_ITEM_ID: usize = 101;

enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}

struct ItemState {
    id: usize,
    shape: Shape,
    bounds: graphics::Rect,
    fill_color: graphics::Color,
    tween: Option<Tween>,
}

impl ItemState {
    fn new(id: usize, shape: Shape) -> GameResult<ItemState> {
        let rect = match shape {
            Shape::Rectangle(rect) => rect,
            Shape::Circle(pt, r) => {
                graphics::Rect::new(pt.x - r, pt.y - r, r * 2.0, r * 2.0)
            },
        };

        Ok(ItemState {
            id: id,
            shape: shape,
            bounds: rect,
            fill_color: graphics::BLACK,
            tween: None,
        })
    }

    pub fn update(&mut self) -> GameResult {
        if let Some(tween) = &self.tween {
            if let Some(update) = tween.update_item(&self.id) {
                self.bounds.render_update(&update.props);
                self.fill_color.render_update(&update.props);
            }
        }
        Ok(())
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        match self.shape {
            Shape::Rectangle(_) => {
                let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.bounds, self.fill_color)?;
                let drawparams = graphics::DrawParam::new();
                let _result = graphics::draw(ctx, &mesh, drawparams);
            },
            Shape::Circle(_, r) => {
                let pt = mint::Point2{x: self.bounds.x + r, y: self.bounds.y + r};
                let mesh = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), pt, r, 1.0, self.fill_color)?;
                let drawparams = graphics::DrawParam::new();
                let _result = graphics::draw(ctx, &mesh, drawparams);
            },
        }
        Ok(())
    }
}


struct MainState {
    square_item: ItemState,
    round_item: ItemState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);

        let rect = graphics::Rect::new(0.0, 0.0, 50.0, 50.0);
        let mut item1 = ItemState::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.fill_color = graphics::Color::from_rgb_u32(0x333333);

        let mut tween1 = Tween::with(&vec![&item1.bounds, &item1.fill_color]).with_id(SQUARE_ITEM_ID)
            .to(vec![position(400.0, 300.0), size(100.0, 100.0), alpha(0.2)])
            .duration(2.0);
        &tween1.play();
        item1.tween = Some(tween1);

        let pt = mint::Point2{x: 500.0, y: 200.0};
        let mut item2 = ItemState::new(ROUND_ITEM_ID, Shape::Circle(pt, 40.0))?;
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

        self.square_item.update()?;
        self.round_item.update()?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);

        self.square_item.render(ctx)?;
        self.round_item.render(ctx)?;

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
