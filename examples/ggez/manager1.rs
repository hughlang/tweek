/// WTF

extern crate ggez;
extern crate tween;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self};
use ggez::{Context, ContextBuilder, GameResult};

use tween::*;

const SQUARE_ITEM_ID: usize = 100;

pub struct Assets {
    pub square_item: ItemState,
}

impl Assets {
    pub fn new(_ctx: &mut Context) -> GameResult<Assets> {
        let mut square_item = ItemState::new(0.0, 0.0, 50.0, 50.0)?;
        square_item.fill_color = graphics::Color::from_rgb_u32(0xCD09AA);
        Ok(Assets {
            square_item,
        })
    }
}

pub struct ItemState {
    pub bounds: graphics::Rect,
    pub fill_color: graphics::Color,
}

impl ItemState {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> GameResult<ItemState> {
        let rect = graphics::Rect::new(x, y, w, h);
        Ok(ItemState {
            bounds: rect,
            fill_color: graphics::WHITE,
        })
    }
}

pub struct GGManager {

}