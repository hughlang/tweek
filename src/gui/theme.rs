use crate::shared::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color, Font},
    lifecycle::{Event, Window},
};

use glyph_brush::rusttype::{point, Font as RTFont, PositionedGlyph, Scale, SharedBytes};
use std::f32;

pub struct Theme {
    pub font: Font,
    pub raw_font: RTFont<'static>,
    pub font_bytes: SharedBytes<'static>,
    pub font_size: f32,
    pub title_font: Option<Font>,
    pub title_font_size: f32,
    pub bg_color: Color,
    pub fg_color: Color,
    pub button_bg_color: Color,
    pub button_fg_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub focus_color: Color,
    pub unfocus_color: Color,
    pub draw_text: DrawText,
}

impl Theme {
    pub fn new(bytes: &'static [u8]) -> Self {
        let font = Font::from_slice(bytes).unwrap();
        let raw_font = RTFont::from_bytes(bytes).unwrap();
        let draw_text = DrawText::from_bytes(bytes);
        Theme {
            font: font,
            raw_font: raw_font,
            font_bytes: bytes.into(),
            font_size: 16.0,
            title_font: None,
            title_font_size: 18.0,
            bg_color: Color::WHITE,
            fg_color: Color::BLACK,
            button_bg_color: Color::WHITE,
            button_fg_color: Color::BLACK,
            border_color: Color::from_hex("#AAAAAA"),
            border_width: 1.0,
            focus_color: Color::YELLOW,
            unfocus_color: Color::from_hex("#CCCCCC"),
            draw_text,
        }
    }

    /// A utility function for getting the size of specified text and calculate the cursor position.
    /// This is only useful for single-line text. Since glyph_brush does not count trailing spaces in
    /// pixel_bounds, we have to inspect the number of trailing spaces and pad the result.
    /// Another use case is calculating the width of a password mask in TextField. In this case, trailing
    /// spaces will not exist.
    pub fn measure_text(&self, text: &str, font_size: f32) -> (f32, f32) {
        let scale = Scale::uniform(font_size);
        let v_metrics = self.raw_font.v_metrics(scale);

        let height = (v_metrics.ascent - v_metrics.descent).ceil();
        let glyphs: Vec<PositionedGlyph<'_>> = self.raw_font.layout(text, scale, point(0.0, 0.0)).collect();
        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0);

        (width, height)
    }
}
