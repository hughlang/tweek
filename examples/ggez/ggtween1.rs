//! The simplest possible example that does something.

extern crate ggez;
extern crate tween;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::mint;

use std::env;
use std::path;
use tween::*;

const SQUARE_ITEM_ID: usize = 100;
const ROUND_ITEM_ID: usize = 101;
const IMAGE_ITEM_ID: usize = 102;
const TEXT_ITEM_ID: usize = 103;

enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
    Image(graphics::Rect),
    Text(graphics::Rect),

}

struct ItemState {
    id: usize,
    shape: Shape,
    frame: graphics::Rect,
    fill_color: graphics::Color,
    tween: Option<Tween>,
    image: Option<graphics::Image>,
    text: Option<graphics::Text>,
}

impl ItemState {
    fn new(id: usize, shape: Shape) -> GameResult<ItemState> {
        let rect = match shape {
            Shape::Rectangle(rect) => rect,
            Shape::Circle(pt, r) => {
                graphics::Rect::new(pt.x - r, pt.y - r, r * 2.0, r * 2.0)
            },
            Shape::Image(rect) => rect,
            Shape::Text(rect) => rect,
        };


        Ok(ItemState {
            id: id,
            shape: shape,
            frame: rect,
            fill_color: graphics::BLACK,
            tween: None,
            image: None,
            text: None,
        })
    }

    pub fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.tween {
            if let Some(update) = tween.get_update(&self.id) {
                self.frame.render_update(&update.props);
                self.fill_color.render_update(&update.props);
            }
        }
        Ok(())
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        match self.shape {
            Shape::Rectangle(_) => {
                let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.frame, self.fill_color)?;
                let drawparams = graphics::DrawParam::new();
                let _result = graphics::draw(ctx, &mesh, drawparams);
            },
            Shape::Circle(_, r) => {
                let pt = mint::Point2{x: self.frame.x + r, y: self.frame.y + r};
                let mesh = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), pt, r, 1.0, self.fill_color)?;
                let drawparams = graphics::DrawParam::new();
                let _result = graphics::draw(ctx, &mesh, drawparams);
            },
            Shape::Image(_) => {
                match &self.image {
                    Some(img) => {
                        let pt = mint::Point2{x: self.frame.x, y: self.frame.y};
                        let _result = graphics::draw(ctx, img, (pt,));
                    },
                    None => (),
                }

            },
            Shape::Text(_) => {
        // graphics::draw(ctx, &level_display, (level_dest, 0.0, graphics::WHITE))?;
                match &self.text {
                    Some(txt) => {
                        let pt = mint::Point2{x: self.frame.x, y: self.frame.y};
                        let _result = graphics::draw(ctx, txt, (pt,));
                    },
                    None => (),
                }
            },
        }
        Ok(())
    }
}

struct MainState {
    square_item: ItemState,
    round_item: ItemState,
    image_item: ItemState,
    text_item: ItemState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);

        // Add a rectangle
        let rect = graphics::Rect::new(0.0, 0.0, 50.0, 50.0);
        let mut item1 = ItemState::new(SQUARE_ITEM_ID, Shape::Rectangle(rect))?;
        item1.fill_color = graphics::Color::from_rgb_u32(0x333333);

        let mut tween1 = Tween::with(&vec![&item1.frame, &item1.fill_color]).with_id(SQUARE_ITEM_ID)
            .to(vec![position(400.0, 300.0), size(100.0, 100.0), alpha(0.2)])
            .duration(2.0);
        &tween1.play();
        item1.tween = Some(tween1);

        // Add a circle
        let mut item2 = ItemState::new(ROUND_ITEM_ID, Shape::Circle(mint::Point2{x: 500.0, y: 200.0}, 40.0))?;
        item2.fill_color = graphics::Color::from_rgb_u32(0xCD09AA);

        let mut tween2 = Tween::with(&vec![&item2.frame, &item2.fill_color]).with_id(ROUND_ITEM_ID)
            .to(vec![position(40.0, 400.0), alpha(0.2)])
            .duration(2.0);

        &tween2.play();
        item2.tween = Some(tween2);

        let tile = graphics::Image::new(ctx, "/tile.png")?;
        let rect = graphics::Rect::new(200.0, 50.0, 80.0, 80.0);
        let mut item3 = ItemState::new(IMAGE_ITEM_ID, Shape::Image(rect))?;
        item3.image = Some(tile);

        let mut tween3 = Tween::with(&vec![&item2.frame, &item2.fill_color]).with_id(IMAGE_ITEM_ID)
            .to(vec![position(400.0, 400.0), alpha(0.2)])
            .duration(3.0);
        &tween3.play();
        item3.tween = Some(tween3);

        let text = graphics::Text::new(("Tweek everything", graphics::Font::default(), 48.0));
        let rect = graphics::Rect::new(20.0, 20.0, 200.0, 40.0);
        let mut item4 = ItemState::new(TEXT_ITEM_ID, Shape::Text(rect))?;
        item4.text = Some(text);

        // let mut item3 = ItemState
        let s = MainState {
            square_item: item1,
            round_item: item2,
            image_item: item3,
            text_item: item4,
        };
        Ok(s)
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        self.square_item.update()?;
        self.round_item.update()?;
        self.image_item.update()?;
        self.text_item.update()?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        self.square_item.render(ctx)?;
        self.round_item.render(ctx)?;
        self.image_item.render(ctx)?;
        self.text_item.render(ctx)?;

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
