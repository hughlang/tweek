/// This provides services for view components that allow text editing.
///
extern crate ggez;

#[allow(unused_imports)]
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba, imageops};
use glyph_brush::rusttype::{Font as RTFont, Scale, point};


#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
pub struct TextTool {
    raw_font: Option<RTFont<'static>>,
    pub string: String,
    pub font_size: f32,
    pub font_color: u32,
    pub cursor_pos: usize,
    pub text_size: (u32, u32),
    pub is_editing: bool,
    pub x_coords: Vec<u32>,
    // pub glyph_cache: Vec<rusttype::PositionedGlyph<'_>>,
}

impl Default for TextTool {
    fn default() -> Self {
        TextTool {
            raw_font: None,
            string: String::default(),
            font_size: 16.0,
            font_color: 0x000000,
            cursor_pos: 0,
            text_size: (0, 0),
            x_coords: Vec::new(),
            is_editing: false,
            // glyph_cache: Vec::new(),
        }
    }
}

impl TextTool {
    pub fn new(raw_font: RTFont<'static>) -> Self {
        TextTool {
            raw_font: Some(raw_font),
            string: String::default(),
            font_size: 16.0,
            font_color: 0x000000,
            cursor_pos: 0,
            text_size: (0, 0),
            x_coords: Vec::new(),
            is_editing: false,
        }
    }

    pub fn load(bytes: &'static [u8]) -> Self {
        let raw_font: RTFont<'static> = RTFont::from_bytes(bytes).unwrap();
        TextTool {
            raw_font: Some(raw_font),
            string: String::default(),
            font_size: 16.0,
            font_color: 0x000000,
            cursor_pos: 0,
            text_size: (0, 0),
            x_coords: Vec::new(),
            is_editing: false,
        }
    }

    pub fn with_font(mut self, raw_font: RTFont<'static>) -> Self {
        self.raw_font = Some(raw_font);
        self
    }

    pub fn start_editing(&mut self) {
        self.cursor_pos = self.string.len();
    }

    pub fn evaluate_string(&mut self, text: String, font_size: f32) {
        self.string = text;
        self.font_size = font_size;
        if let Some(raw_font) = &self.raw_font {

            let scale = Scale::uniform(self.font_size);
            let v_metrics = raw_font.v_metrics(scale);

            let glyphs: Vec<_> = raw_font
                .layout(self.string.trim(), scale, point(0.0, 0.0))
                .collect();

            // for (i, glyph) in glyphs.iter().enumerate() {
            //     let c = text[i..].chars().next().unwrap();
            //     log::debug!("[{}:{}] {:?} // bb={:?}", i, c, glyph.position(), glyph.pixel_bounding_box());
            // }
            let text_h = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
            let text_w = {
                let min_x = glyphs
                    .first()
                    .map(|g| g.pixel_bounding_box().unwrap().min.x)
                    .unwrap();
                let max_x = glyphs
                    .last()
                    .map(|g| g.pixel_bounding_box().unwrap().max.x)
                    .unwrap();
                (max_x - min_x) as u32
            };

            self.x_coords = glyphs.iter().map(|x| x.position().x.round() as u32).collect();
            self.text_size = (text_w, text_h);
        }
    }

    pub fn update_metrics(&mut self) {
        &self.evaluate_string(self.string.clone(), self.font_size.clone());
    }

    pub fn insert_char(&mut self, c: char) {

        if self.cursor_pos == self.string.len() {
            // cursor is at the end. add there.
            self.string.push(c);
            self.cursor_pos = self.string.len();
            self.update_metrics();
        } else if self.cursor_pos < self.string.len() {
            // insert at cursor
            self.string.insert(self.cursor_pos, c);
            self.cursor_pos += 1;
            self.update_metrics();
        }
    }

    pub fn delete_char(&mut self) {
        if self.string.len() == 0 {
            return;
        }
        if self.cursor_pos == self.string.len() {
            // cursor is at the end. add there.
            self.string.pop();
            self.cursor_pos = self.string.len();
            self.update_metrics();
        } else {
            // insert at cursor
            self.string.remove(self.cursor_pos);
            self.cursor_pos -= 1;
            self.update_metrics();
        }
        log::debug!("Backspace: string='{}' len={}", self.string, self.string.len());
    }

    pub fn move_cursor(&mut self, shift: i32) {
        if shift > 0 {
            if self.string.len() > 0 && self.cursor_pos < self.string.len() {
                self.cursor_pos += 1;
            }
        } else {
            if self.string.len() > 0 && self.cursor_pos > 0 {
                self.cursor_pos -= 1;
            }
        }
    }

    /// This is mainly used for printing out character size metrics.
    pub fn evaluate_text(&self, text: &str, size: f32) {
        if let Some(raw_font) = &self.raw_font {

            let scale = Scale::uniform(size);

            let glyphs: Vec<_> = raw_font
                .layout(text, scale, point(0.0, 0.0))
                .collect();
            for (i, glyph) in glyphs.iter().enumerate() {
                let c = text[i..].chars().next().unwrap();
                log::debug!("[{}:{}] {:?} // bb={:?}", i, c, glyph.position(), glyph.pixel_bounding_box());
            }
        }
    }

    pub fn get_x_coordinates(&self, text: &str, size: f32, origin_x: f32) -> Vec<i32> {
        if let Some(raw_font) = &self.raw_font {
            let scale = Scale::uniform(size);
            let glyphs: Vec<_> = raw_font
                .layout(text, scale, point(origin_x, 0.0))
                .collect();
            let x_coords: Vec<i32> = glyphs.iter().map(|x| x.position().x.round() as i32).collect();
            x_coords
        } else {
            Vec::new()
        }
    }

    pub fn render_to_image(&self, text: &str, size: f32) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        if let Some(raw_font) = &self.raw_font {
            let scale = Scale::uniform(size);
            let v_metrics = raw_font.v_metrics(scale);

            // FIXME: ypos is hard coded. It will panic/crash if glyph y is negative
            let glyphs: Vec<_> = raw_font
                .layout(text, scale, point(0.0, 30.0))
                .collect();

            let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
            let glyphs_width = {
                let min_x = glyphs
                    .first()
                    .map(|g| g.pixel_bounding_box().unwrap().min.x)
                    .unwrap();
                let max_x = glyphs
                    .last()
                    .map(|g| g.pixel_bounding_box().unwrap().max.x)
                    .unwrap();
                (max_x - min_x) as u32
            };

           // Create a new rgba image with some padding
            // https://docs.rs/image/0.21.0/image/
            // https://github.com/PistonDevelopers/image
            let mut imgbuf = DynamicImage::new_rgba8(glyphs_width, glyphs_height).to_rgba();

            // Loop through the glyphs in the text, positing each one on a line
            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    // Draw the glyph into the image per-pixel by using the draw closure
                    glyph.draw(|x, y, v| {
                        imgbuf.put_pixel(
                            // Offset the position by the glyph bounding box
                            x + bounding_box.min.x as u32,
                            y + bounding_box.min.y as u32,
                            // TODO: Make color customizable
                            Rgba {
                                data: [0, 0, 0, (v * 255.0) as u8],
                            },
                        )
                    });
                }
            }
            return Some(imgbuf);
        } else {
            return None;
        }
    }

    pub fn render_text_cropped(&self, text: &str, size: f32, x: u32, y: u32, w: u32, h: u32) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>>  {
        if let Some(imgbuf) = &mut self.render_to_image(text, size) {

            let subimg = imageops::crop(imgbuf, x, y, w, h);

            return Some(subimg.to_image());
        }
        None
    }
}