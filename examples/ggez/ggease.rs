//! The simplest possible example that does something.

extern crate ggez;
extern crate tween;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self};
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;
use tween::*;

const SQUARE_ITEM_ID: usize = 100;

struct Assets {
    square_item: ItemState,
}

impl Assets {
    fn new(_ctx: &mut Context) -> GameResult<Assets> {
        let mut square_item = ItemState::new(SQUARE_ITEM_ID, 0.0, 0.0, 50.0, 50.0)?;
        square_item.fill_color = graphics::Color::from_rgb_u32(0xCD09AA);
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
    square_tween: Tween,
    ease_list: Vec<Easing>,
    ease_index: usize,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {

        let assets = Assets::new(ctx)?;
        let item1 = &assets.square_item;

        let tween1 = Tween::with(&vec![&item1.bounds, &item1.fill_color]).with_id(SQUARE_ITEM_ID);
        let s = MainState {
            assets: assets,
            square_tween: tween1,
            ease_list: Easing::get_list(),
            ease_index: 0,
        };
        Ok(s)
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Here is where you tell which objects to update in each run loop.
        // MainState will have one or more Tween objects that need to be updated.
        if let Some(update) = self.square_tween.update_item(&SQUARE_ITEM_ID) {
            self.assets.square_item.bounds.render_update(&update.props);
            self.assets.square_item.fill_color.render_update(&update.props);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(),
            self.assets.square_item.bounds, self.assets.square_item.fill_color)?;
        let drawparams = graphics::DrawParam::new();
        let _result = graphics::draw(ctx, &r1, drawparams);

        graphics::present(ctx)?;
        // timer::yield_now();
        Ok(())
    }

    /// Mouse event handling. On mouseup, start a new tween action.
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
        let item1 = &self.assets.square_item;
        let easing = &self.ease_list[self.ease_index];
        self.ease_index += 1;
        let mut tween1 = Tween::with(&vec![&item1.bounds, &item1.fill_color]).with_id(SQUARE_ITEM_ID)
            .to(vec![position(x as f64, y as f64)])
            .duration(2.0);
            // .ease(easing.clone())
        &tween1.play();
        self.square_tween = tween1;
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
        .window_setup(conf::WindowSetup::default().title("Ease test"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;
    // println!("Game resource path: {:?}", ctx.filesystem);

    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
