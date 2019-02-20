/// Convenient tools for building stuff
///

extern crate ggez;

#[allow(unused_imports)]
use crate::core::*;

use ggez::graphics::{self, Color};
use ggez::mint::Point2;
use ggez::{Context, GameResult, GameError};
// use std::{collections::HashMap};

#[allow(unused_imports)]
use super::ggez_helper::*;
use super::controls::*;

pub struct GGTools {

}

impl GGTools {
    pub fn build_grid(ctx: &mut Context, width: f32, height: f32, interval: f32, color: Color) -> GameResult<graphics::Mesh> {
        let mut builder = graphics::MeshBuilder::new();

        let mut xpos = 0.0;
        while xpos < width {
            builder.line(&[Point2{x: xpos, y: 0.0}, Point2{x: xpos, y: height}], 1.0, color,)?;
            xpos += interval;
        }
        let mut ypos = 0.0;
        while ypos < height {
            builder.line(&[Point2{x: 0.0, y: ypos}, Point2{x: width, y: ypos}], 1.0, color,)?;
            ypos += interval;
        }

        let gridmesh = builder.build(ctx)?;
        Ok(gridmesh)
    }

    pub fn build_progress_bar(ctx: &mut Context, tk_state: &mut TKContext) -> GameResult {

        Ok(())
    }

    pub fn build_play_button(ctx: &mut Context, frame: graphics::Rect) -> GameResult<GGButton> {
        let mut button = GGButton::new(frame).with_title("Play")
            .with_props(&vec![color(0xCD09AA)]);
        let font = graphics::Font::new(ctx, "/Roboto-Regular.ttf")?;

        button.set_font(&font, &24.0, &graphics::Color::from_rgb_u32(0xFFFFFF));
        button.set_color(&graphics::Color::from_rgb_u32(0x999999));
        button.set_on_hover(vec![color(0xFF8920)], 0.1);

        Ok(button)
    }
}