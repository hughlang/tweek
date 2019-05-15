use std::f32;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color, Font},
    lifecycle::{Event, Window},
};

use glyph_brush::rusttype::SharedBytes;

pub struct Theme {
    pub font: Font,
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
}

impl Theme {
    pub fn new(bytes: &'static [u8]) -> Self {
        let font = Font::from_slice(bytes).unwrap();

        Theme {
            font: font,
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
        }
    }
}
