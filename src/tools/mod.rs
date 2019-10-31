//! The Shared module contains miscellaneous utilities for GUI components. Currently, it is
//! Quicksilver only, so this module/files may get renamed or moved in the future.

pub use self::app_delegate::*;
pub use self::command::*;
pub use self::draw_font::*;
pub use self::draw_image::*;
pub use self::draw_shape::*;
pub use self::editor::*;
pub use self::ui::*;

mod app_delegate;
mod command;
mod draw_font;
mod draw_image;
mod draw_shape;
mod editor;
mod ui;

/// Used for checking equality between floats
/// TODO: Use EPSILON? https://doc.rust-lang.org/beta/std/f32/constant.EPSILON.html
pub const FLOAT_TOLERANCE: f32 = 0.001;

use glyph_brush::rusttype::Rect as RTRect;

use quicksilver::{geom::Vector, graphics::Vertex};

// ************************************************************************************
// GPU stuff for OpenGL and WebGL
// ************************************************************************************

/// Wrapper struct to hold the output of to_vertex for every GlyphVertex input.
/// GLVertex also is used as the generic when creating a GlyphBrush instance.
#[derive(Clone, Debug)]
pub(super) struct GLVertex {
    frame: RTRect<i32>,
    tex_frame: RTRect<f32>,
    color: [f32; 4],
}

/// This is the function that converts the GlyphVertex output from glyph_brush
/// and transforms it to a struct that is processed in
pub(super) fn to_vertex(v: glyph_brush::GlyphVertex) -> GLVertex {
    GLVertex { frame: v.pixel_coords, tex_frame: v.tex_coords, color: v.color }
}

pub(super) fn serialize_vertex(vertex: Vertex) -> Vec<f32> {
    let mut result: Vec<f32> = Vec::new();
    result.push(vertex.pos.x);
    result.push(vertex.pos.y);
    let tex_pos = vertex.tex_pos.unwrap_or(Vector::ZERO);
    result.push(tex_pos.x);
    result.push(tex_pos.y);
    result.push(vertex.col.r);
    result.push(vertex.col.g);
    result.push(vertex.col.b);
    result.push(vertex.col.a);
    result.push(if vertex.tex_pos.is_some() { 1f32 } else { 0f32 });
    result
}
