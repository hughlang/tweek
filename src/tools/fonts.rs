/// A Font manager to load and vend fonts and font bytes.
///
use super::*;

use glyph_brush::{self, GlyphBrush, GlyphBrushBuilder};
use glyph_brush::rusttype::{Font as RTFont};

use std::collections::HashMap;

/// A holder of loaded Fonts
pub struct Fonts {
    /// A HashMap where font bytes are stored
    font_data: HashMap<String, Vec<u8>>,
    /// Unused. TODO: keep/remove
    glyph_brushes: HashMap<String, GlyphBrush<'static, GLVertex>>,
}

impl Fonts {
    /// Constructor
    pub fn new() -> Self {
        Fonts {
            // font_map: HashMap::new(),
            font_data: HashMap::new(),
            glyph_brushes: HashMap::new(),
        }
    }

    /// Purge all data if loading new fonts (when loading new Theme)
    pub fn reset(&mut self) {
        self.font_data.clear();
        self.glyph_brushes.clear();
    }

    /// Check to see if font already registered before trying to add it again.
    pub fn has_font(&self, name: &str) -> bool {
        self.font_data.contains_key(name)
    }

    /// Method for adding font bytes
    pub fn add_font(&mut self, name: &str, bytes: Vec<u8>) {
        self.font_data.insert(name.to_string(), bytes.clone());
        let raw_font = RTFont::from_bytes(bytes).unwrap();
        let glyph_brush = GlyphBrushBuilder::using_font(raw_font.clone()).build();
        self.glyph_brushes.insert(name.to_string(), glyph_brush);
    }

    /// Get the Vec<u8> data for a font
    pub fn get_font_data(&self, name: &str) -> Option<Vec<u8>> {
        if let Some(data) = self.font_data.get(name) {
            return Some(data.clone());
        }
        None
    }
}

