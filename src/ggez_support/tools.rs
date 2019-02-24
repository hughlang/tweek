/// Convenient tools for building stuff
///

extern crate ggez;

#[allow(unused_imports)]
use crate::core::*;

use ggez::graphics::{self, Color};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
// use std::{collections::HashMap};

#[allow(unused_imports)]
use super::base::*;
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

}