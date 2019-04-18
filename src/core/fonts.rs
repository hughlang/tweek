
use std::collections::HashMap;
// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};
use std::io;
use std::io::prelude::*;
use std::fs::File;

use glyph_brush::rusttype::{Font, FontCollection, PositionedGlyph, Scale, point};
use glyph_brush::rusttype::Error as FontError;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_mut)]
struct FontHelper {
    font_db: HashMap<u64, Font<'static>>,
}

impl FontHelper {
    pub fn new() -> Self {
        FontHelper{
            font_db: HashMap::new(),
        }
    }

    // pub(crate) fn default_font_bytes() -> &'static [u8] {
    //     include_bytes!(concat!(
    //         env!("CARGO_MANIFEST_DIR"),
    //         "/resources/DejaVuSerif.ttf"
    //     ))
    // }
    pub fn register_font(id: u64, path: &str) -> io::Result<()> {

        // let id = hash(path);
        // path.to
        let mut f = File::open(path)?;

        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        let font: Font<'static> = Font::from_bytes(buffer)?;

        Ok(())
    }

    pub fn read_string(string: &str) {

    }
}