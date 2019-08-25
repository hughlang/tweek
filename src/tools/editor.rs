/// This provides services for view components that allow text editing.
/// https://docs.rs/glyph_brush/0.4.1/glyph_brush/
// #[allow(unused_imports)]
use super::*;
use crate::core::*;
use crate::gui::Theme;

use glyph_brush::rusttype::{self, Font as RTFont, GlyphId, Scale};

#[allow(unused_imports)]
use glyph_brush::{
    self, BrushAction, BrushError, Color, DefaultSectionHasher, FontId, GlyphBrush, GlyphBrushBuilder, GlyphCalculator,
    GlyphCalculatorBuilder, GlyphCruncher, GlyphPositioner, HorizontalAlign as HAlign, Layout, Section,
    SectionGeometry, SectionText, VariedSection,
};
use image::{imageops, DynamicImage, ImageBuffer, Rgba};
use std::{collections::HashMap, f32, ops::Range};

use quicksilver::{
    graphics::MeshTask,
};

const SPACE: char = ' ';
const ALPHANUMERICS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ";

// TODO: Please use this
// pub trait Editor {}

/// Base struct for TextEditor that holds much of the state information
pub struct EditorContext {
    /// The raw Rusttype font
    raw_font: RTFont<'static>,
    /// Instance of the GlyphCalculator which calculates text size and position information
    glyph_calc: GlyphCalculator<'static>,
    /// Owned DrawFont
    pub draw_font: DrawFont,
    /// Cached MeshTask
    pub cached_mesh: Option<MeshTask>,
    /// Is it multi-line text?
    is_multiline: bool,
    /// The actual string content
    string: String,
    /// The font size
    pub font_size: f32,
    /// The font color as a hex u32 value
    pub font_color: u32,
    /// The cursor position as an index value in the entire char array
    pub cursor_pos: usize,
    /// The baseline coordinates of where the cursor belongs
    pub cursor_origin: (f32, f32),
    /// The origin coordinates of where the text should render in the Window
    pub text_origin: (f32, f32),
    /// The size of the text content
    pub text_size: (u32, u32),
    /// Has the content changed since the last run loop?
    pub has_changed: bool,
    /// The Rusttype frame
    pub frame: rusttype::Rect<f32>,
    /// A Hashmap storing the mapping of GlyphId to char. Used mainly for debugging?
    pub glyph_db: HashMap<GlyphId, char>,
    /// A Hashmap storing the mapping of a char to its width
    pub char_db: HashMap<char, f32>,
    /// Temporary value for debugging
    pub debug: bool,
    metrics: Vec<(f32, f32, f32, char)>,
    visible_range: Range<usize>,
}

impl EditorContext {

    fn with_theme(theme: &mut Theme) -> Self {
        let data = theme.data_for_font(Theme::DEFAULT_FONT).clone();
        let font = RTFont::from_bytes(data.clone()).unwrap();
        let glyph_calc = GlyphCalculatorBuilder::using_font(font.clone()).build();
        let draw_font = DrawFont::from_bytes(data, None);
        let rect = rusttype::Rect { min: rusttype::point(0.0, 0.0), max: rusttype::point(1.0, 1.0) };
        let mut ctx = EditorContext {
            raw_font: font,
            glyph_calc,
            draw_font,
            cached_mesh: None,
            is_multiline: false,
            string: String::default(),
            font_size: 14.0,
            font_color: 0x000000,
            cursor_pos: 0,
            cursor_origin: (0.0, 0.0),
            text_origin: (0.0, 0.0),
            text_size: (0, 0),
            has_changed: true,
            frame: rect,
            glyph_db: HashMap::new(),
            char_db: HashMap::new(),
            debug: false,
            metrics: Vec::new(),
            visible_range: 0..1,
        };
        ctx.measure_glyphs();
        ctx
    }
    /// Builder method to define the editor as a multiline, word-wrapped editor
    pub fn multiline(mut self, multiline: bool) -> Self {
        self.is_multiline = multiline;
        // self.gpu_text.set_multiline(multiline);
        self
    }

    /// Builder method to set the initial text
    pub fn with_text(mut self, text: String, font_size: f32) -> Self {
        self.string = text;
        self.font_size = font_size;
        self
    }

    /// Builder method to set the position and size of the content
    pub fn with_frame(mut self, origin: (f32, f32), size: (f32, f32)) -> Self {
        let frame = rusttype::Rect {
            min: rusttype::point(origin.0, origin.1),
            max: rusttype::point(origin.0 + size.0, origin.1 + size.1),
        };
        self.frame = frame;
        self
    }

    /// Method to get the text content as a string
    /// TODO: Replace with get_field_value
    pub fn get_text(&self) -> &str {
        return &self.string;
    }

    /// Method to set the text content of the editor
    pub fn set_text(&mut self, text: &str) {
        self.string = text.to_owned();
    }

    /// This is called whenever self.metrics needs to be updated.
    /// – When initializing a text input view
    /// – When the user has inserted or deleted characters (edit mode)
    pub fn update_metrics(&mut self) {
        log::debug!("============= update_metrics =============");
        self.cached_mesh = None;
        if self.string.len() == 0 {
            return;
        };
        let bounds = {
            if self.is_multiline {
                (self.frame.width(), f32::INFINITY)
            } else {
                (f32::INFINITY, f32::INFINITY)
            }
        };

        let mut glyph_calc = self.glyph_calc.cache_scope();
        let layout = Layout::default();
        let scale = Scale::uniform(self.font_size);
        let varied = VariedSection {
            layout,
            bounds,
            text: vec![SectionText {
                text: &self.string,
                scale: Scale::uniform(self.font_size),
                ..SectionText::default()
            }],
            ..VariedSection::default()
        };
        let glyphs = glyph_calc.glyphs(&varied);

        let xy_coords: Vec<(f32, f32)> = glyphs.map(|v| (v.position().x, v.position().y)).collect();
        let mut pointer = 0 as usize;
        let mut metrics: Vec<(f32, f32, f32, char)> = Vec::new();
        let mut last_data: (f32, f32, f32) = (0.0, 0.0, 0.0);
        let space_w = self.raw_font.glyph(SPACE).scaled(scale).h_metrics().advance_width;

        for c in self.string.chars() {
            let width = {
                if let Some(w) = self.char_db.get(&c) {
                    *w
                } else {
                    let w = self.raw_font.glyph(c).scaled(scale).h_metrics().advance_width;
                    log::debug!("Insert in char_db [{:?}] width={:?}", c, w);
                    self.char_db.insert(c, w);
                    w
                }
            };
            if c != SPACE {
                if pointer < xy_coords.len() {
                    last_data = (xy_coords[pointer].0, xy_coords[pointer].1, width);
                    metrics.push((last_data.0, last_data.1, width, c));
                    pointer += 1;
                }
            } else {
                metrics.push(((last_data.0 + last_data.2), last_data.1, space_w, c));
            }
        }

        self.metrics = metrics;
        log::debug!("metrics.len={:?} string.len={:?}", self.metrics.len(), self.string.len());
        // log::debug!("metrics={:?}", self.metrics);

        if let Some(size) = glyph_calc.pixel_bounds(&varied) {
            self.text_size = (size.width() as u32, size.height() as u32);
        }
        // TBD: remove this?
        self.cursor_origin = (self.frame.min.x, self.frame.min.y + (self.frame.height() + self.font_size as f32) / 2.0);
    }

    /// Possibly useful but currently unused function to create a lookup table for all alphanumeric characters and
    /// store in self.glyph_db as HashMap<GlyphId, char>
    pub fn measure_glyphs(&mut self) {
        let start = current_time();
        log::debug!(">>> start={:?}", start);

        let mut glyph_calc = self.glyph_calc.cache_scope();
        let layout = Layout::default();
        let scale = Scale::uniform(self.font_size);
        let section = Section { layout, scale, text: ALPHANUMERICS, ..Section::default() };

        let glyphs = glyph_calc.glyphs(&section);
        let glyph_count = glyphs.len();
        for (i, glyph) in glyphs.enumerate() {
            if i < glyph_count {
                let c = ALPHANUMERICS[i..].chars().next().unwrap();
                let width = glyph.unpositioned().h_metrics().advance_width;
                // let height = glyph.unpositioned().v_metrics().advance_height;
                self.char_db.insert(c, width);
            }
        }
        let elapsed = elapsed_time(start);
        log::debug!(">>> time elapsed={:?}", elapsed);
    }

    /// A utility function for getting the size of specified text and calculate the cursor position.
    /// This is only useful for single-line text. Since glyph_brush does not count trailing spaces in
    /// pixel_bounds, we have to inspect the number of trailing spaces and pad the result.
    /// Another use case is calculating the width of a password mask in TextField. In this case, trailing
    /// spaces will not exist.
    pub fn measure_text(&self, text: &str) -> (f32, f32) {
        let mut glyph_calc = self.glyph_calc.cache_scope();
        let layout = Layout::default();
        let scale = Scale::uniform(self.font_size);
        let section = Section { layout, scale, text, ..Section::default() };

        let mut text_size: (f32, f32) = {
            if let Some(size) = glyph_calc.pixel_bounds(&section) {
                (size.width() as f32, size.height() as f32)
            } else {
                (0.0, 0.0)
            }
        };

        let add_space: f32 = {
            let space_w = self.char_db.get(&SPACE).unwrap_or(&0.0);
            if let Some(end) = text.chars().rev().position(|c| c != SPACE) {
                // If there are trailing spaces, the first non-space position will be used to calculate width
                *space_w * end as f32
            } else {
                // All the chars are spaces.
                *space_w * text.len() as f32
            }
        };
        text_size.0 = text_size.0 + add_space;
        text_size
    }
    // *****************************************************************************************************
    // EditorContext life cycle functions
    // *****************************************************************************************************

    /// Switch to editing mode. Calculate cursor and other metrics.
    pub fn start_editing(&mut self) {
        if self.debug && self.glyph_db.len() == 0 {
            let glyphs = self.raw_font.glyphs_for(ALPHANUMERICS.chars());
            let pairs = glyphs.zip(ALPHANUMERICS.chars());
            self.glyph_db.extend(pairs.map(|p| (p.0.id(), p.1)));
        }

        self.has_changed = true;
        if self.is_multiline {
            self.cursor_pos = 0;
        } else {
            self.cursor_pos = self.string.len();
        }
        self.update_metrics();
        // self.gpu_text.activate();
    }

    /// Switch read mode
    pub fn stop_editing(&mut self) {
        self.has_changed = false;
    }

    /// Handle keyboard input by inserting char at current cursor_pos
    pub fn insert_char(&mut self, c: char) {
        self.has_changed = true;
        log::debug!("Insert char={:?}", c);
        if self.cursor_pos == self.string.len() {
            // cursor is at the end. add there.
            self.string.push(c);
            self.cursor_pos = self.string.len();
        } else if self.cursor_pos < self.string.len() {
            // insert at cursor
            self.string.insert(self.cursor_pos, c);
            self.cursor_pos += 1;
        }
        self.update_metrics();
    }

    /// Handle delete button
    pub fn delete_char(&mut self) {
        self.has_changed = true;
        if self.string.len() == 0 || self.cursor_pos == 0 {
            return;
        }
        if self.cursor_pos == self.string.len() {
            // cursor is at the end. add there.
            self.string.pop();
            self.cursor_pos = self.string.len();
        } else {
            // insert at cursor
            self.string.remove(self.cursor_pos);
            self.cursor_pos -= 1;
        }
        self.update_metrics();
        // log::debug!("Backspace: string='{}' len={}", self.string, self.string.len());
    }

    /// Move the cursor N places
    pub fn move_cursor(&mut self, shift: i32) {
        self.has_changed = true;
        if shift > 0 {
            if self.string.len() > 0 && self.cursor_pos < self.string.len() {
                self.cursor_pos += 1;
            }
        } else {
            if self.string.len() > 0 && self.cursor_pos > 0 {
                self.cursor_pos -= 1;
            }
        }
        // log::debug!("cursor at={:?} string.len={:?}", self.cursor_pos, self.string.len());
    }
}

// *****************************************************************************************************
// TextFieldEditor
// *****************************************************************************************************

/// A TextFieldEditor is a simple wrapper around an EditorContext where multiline = false
pub struct TextFieldEditor {
    /// The EditorContext holds all of the state information while editing a TextField
    pub ctx: EditorContext,
}

impl TextFieldEditor {
    /// Create the TextFieldEditor using the provided theme. The theme will provide the DrawFont
    /// that will be used for font metrics and rendering.
    pub fn create(theme: &mut Theme) -> Self {
        let ctx = EditorContext::with_theme(theme).multiline(false);
        TextFieldEditor { ctx }
    }

    /// Constructor with specified position and size
    pub fn with_frame(mut self, origin: (f32, f32), size: (f32, f32)) -> Self {
        let frame = rusttype::Rect {
            min: rusttype::point(origin.0, origin.1),
            max: rusttype::point(origin.0 + size.0, origin.1 + size.1),
        };
        self.ctx.frame = frame;
        self
    }

    // *****************************************************************************************************
    // TextFieldEditor metrics
    // *****************************************************************************************************

    /// This function is called when a TextField is executing the render() function and
    /// is_editing=true.
    /// Calculate the following:
    /// – cursor x y position where x is the insertion point and y is the baseline position
    ///   for the current line.
    /// – range of visible text. For single-line, this is limited by the width of dimensions.
    pub fn update_textfield(&mut self) {
        // If editor state has not changed, then no need to update text and cursor display.
        if !self.ctx.has_changed {
            return;
        }
        self.ctx.has_changed = false;

        if self.ctx.cursor_pos > self.ctx.metrics.len() {
            log::debug!("PANIC! cursor_pos={:?} OOB metrics={:?}", self.ctx.cursor_pos, self.ctx.metrics.len());
            // TODO: return false or error so that text field can stop rendering
            self.ctx.cursor_pos = 0;
            return;
        }

        // Set default for easiest case
        let cursor_space = 0.0;
        self.ctx.cursor_origin = (
            self.ctx.frame.min.x + cursor_space,
            self.ctx.frame.min.y + (self.ctx.frame.height() + self.ctx.font_size as f32) / 2.0,
        );

        self.ctx.text_origin =
            (self.ctx.frame.min.x, self.ctx.frame.min.y + (self.ctx.frame.height() - self.ctx.font_size as f32) / 2.0);

        if self.ctx.text_size.0 as f32 <= self.ctx.frame.width() {
            if self.ctx.string.len() > 0 {
                self.ctx.visible_range = 0..self.ctx.string.len();
                if self.ctx.cursor_pos == self.ctx.string.len() {
                    // Since text_size does not account for trailing spaces, append a space.
                    let text_size = self.ctx.measure_text(&self.ctx.string);
                    self.ctx.cursor_origin.0 = self.ctx.frame.min.x + text_size.0;
                } else {
                    self.ctx.cursor_origin.0 = self.ctx.frame.min.x + self.ctx.metrics[self.ctx.cursor_pos].0;
                }
            }
        } else {
            // log::debug!("text_size={:?} frame.w={:?}", self.text_size, self.frame.width());
            if self.ctx.cursor_pos == 0 {
                // Either the field is not being edited or is at beginning.
                if let Some(end) = self.ctx.metrics.iter().position(|x| x.0 > self.ctx.frame.width()) {
                    log::debug!("Case 1: range={:?} cursor={:?}", self.ctx.visible_range, self.ctx.cursor_origin);
                    self.ctx.visible_range = 0..end;
                } else {
                    // Unsure what scenario this is.
                    log::debug!(
                        "### Unexpected result: cursor={:?} metrics={:?}",
                        self.ctx.cursor_pos,
                        self.ctx.metrics
                    );
                    self.ctx.visible_range = 0..self.ctx.string.len();
                }
            } else {
                // The field is being edited.
                // Starting the from the right end of self.metrics, find the starting point where the
                // x coordinate is less than the width of self.frame
                if let Some(rev_index) = self
                    .ctx
                    .metrics
                    .iter()
                    .rev()
                    .position(|m| m.3 != SPACE && m.0 < self.ctx.text_size.0 as f32 - self.ctx.frame.width())
                {
                    let start = self.ctx.string.len() - rev_index - 1;
                    self.ctx.visible_range = start..self.ctx.string.len();
                    // Determine offset width for range of text left of the view
                    let offset = self.ctx.metrics[start].0;
                    let string_w = self.ctx.text_size.0 as f32 - offset;
                    self.ctx.text_origin.0 = self.ctx.frame.max.x - string_w as f32;

                    if self.ctx.cursor_pos == self.ctx.string.len() {
                        self.ctx.cursor_origin.0 = self.ctx.frame.max.x;
                    } else {
                        self.ctx.cursor_origin.0 =
                            self.ctx.text_origin.0 + self.ctx.metrics[self.ctx.cursor_pos].0 - offset;
                    }
                    log::debug!(
                        "Case 2: range={:?} cursor_x={:?} offset={} string_w={}",
                        self.ctx.visible_range,
                        self.ctx.cursor_origin.0,
                        offset,
                        string_w
                    );
                } else {
                    // Unsure what scenario this is.
                    log::debug!(
                        ">>> Unexpected result: cursor={:?} metrics={:?}",
                        self.ctx.cursor_pos,
                        self.ctx.metrics
                    );
                    self.ctx.visible_range = 0..self.ctx.string.len();
                }
            }
        }
        // This is just for debugging. Comment out or delete later.
        // let chunk = &self.ctx.string[self.ctx.visible_range.clone()];
        // log::debug!("chunk={:?} range={:?}", chunk, self.ctx.visible_range);
    }

    /// Method to determine the text that is visible in the TextField
    pub fn get_visible_text(&self, _scroll_x: f32) -> Option<String> {
        let chunk = &self.ctx.string[self.ctx.visible_range.clone()];
        Some(chunk.to_string())
    }
}

// *****************************************************************************************************
// TextAreaEditor
// *****************************************************************************************************



#[allow(dead_code)]
/// A TextAreaEditor has an EditorContext that manages most of the state information.
/// In read mode, an ImageBuffer cache is used for drawing to the screen.
pub struct TextAreaEditor {
    /// The EditorContext holds all of the state information while editing a TextArea
    pub ctx: EditorContext,
    full_render: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    line_count: usize,
    baselines: Vec<f32>,
}

// impl Default for TextAreaEditor {
//     fn default() -> Self {
//         let ctx = EditorContext::default().multiline(true);
//         TextAreaEditor { ctx, full_render: None, line_count: 1, baselines: Vec::new() }
//     }
// }

impl TextAreaEditor {

    /// Create the TextFieldEditor using the provided theme. The theme will provide the DrawFont
    /// that will be used for font metrics and rendering.
    pub fn create(theme: &mut Theme) -> Self {
        let ctx = EditorContext::with_theme(theme).multiline(true);
        TextAreaEditor { ctx, full_render: None, line_count: 1, baselines: Vec::new() }
    }

    /// Constructor using specified position and size
    pub fn with_frame(mut self, origin: (f32, f32), size: (f32, f32)) -> Self {
        let frame = rusttype::Rect {
            min: rusttype::point(origin.0, origin.1),
            max: rusttype::point(origin.0 + size.0, origin.1 + size.1),
        };
        self.ctx.frame = frame;
        self
    }

    /// Method for rendering the multiline text to an image and cached in self.full_render
    pub fn update_rendered_text(&mut self) {
        if self.ctx.string.len() == 0 {
            return;
        }
        let layout = Layout::default();
        let varied = VariedSection {
            layout,
            bounds: (self.ctx.frame.width(), f32::INFINITY),
            text: vec![SectionText {
                text: &self.ctx.string,
                scale: Scale::uniform(self.ctx.font_size),
                ..SectionText::default()
            }],
            ..VariedSection::default()
        };

        let mut glyph_calc = self.ctx.glyph_calc.cache_scope();
        let pixel_bounds = glyph_calc.pixel_bounds(&varied).expect("None bounds");
        self.ctx.text_size = (pixel_bounds.width() as u32, pixel_bounds.height() as u32);

        // let mut xy_coords: Vec<(f32, f32)> = Vec::new();
        let glyphs = glyph_calc.glyphs(&varied);

        let mut imgbuf = DynamicImage::new_rgba8(self.ctx.text_size.0 + 100, self.ctx.text_size.1 + 100).to_rgba();

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            // xy_coords.push((glyph.position().x, glyph.position().y));
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    imgbuf.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // TODO: Make color customizable
                        Rgba { data: [0, 0, 0, (v * 255.0) as u8] },
                    )
                });
            }
        }
        log::debug!("full_render complete: x={:?} y={:?}", 0, 0);
        self.full_render = Some(imgbuf);
    }

    /// Method to refresh the textarea metrics like cursor position
    pub fn update_textarea(&mut self) {
        if self.ctx.cursor_pos > self.ctx.metrics.len() {
            log::debug!("PANIC! cursor_pos={:?} OOB metrics={:?}", self.ctx.cursor_pos, self.ctx.metrics.len());
            // TODO: return false or error so that text field can stop rendering
            self.ctx.cursor_pos = 0;
            return;
        }
        self.ctx.cursor_origin = (
            self.ctx.frame.min.x + self.ctx.metrics[self.ctx.cursor_pos].0,
            self.ctx.frame.min.y + self.ctx.metrics[self.ctx.cursor_pos].1,
        );

        if !self.ctx.has_changed {
            return;
        }
        // Text is being edited
        self.ctx.has_changed = false;


        // log::debug!("=============================================================");
        // let scale = Scale::uniform(self.ctx.font_size);
        // let space_w = self.ctx.raw_font.glyph(SPACE).scaled(scale).h_metrics().advance_width;
        // log::debug!("origin_x={:?} pos={:?} space={}", self.ctx.metrics[self.ctx.cursor_pos].0, self.ctx.cursor_pos, space_w);
        // let metrics = self.current_line_metrics();
        // log::debug!("current line={:?}", metrics);

        self.ctx.text_origin = (self.ctx.frame.min.x, self.ctx.frame.min.y);
    }

    /// FIXME: Unused
    fn _current_line_metrics(&self) -> Vec<(f32, f32, char)> {
        let base_y = self.ctx.metrics[self.ctx.cursor_pos].1.round();
        let filter = self.ctx.metrics.iter().filter(|m| (m.1.round() - base_y).abs() < FLOAT_TOLERANCE);
        let results: Vec<(f32, f32, char)> = filter.map(|m| (m.0, m.1, m.3)).collect(); // Unfortunately only this worked.
        results
    }

    /// Method to calculate what text is visible in a TextArea given the current scroll offset
    pub fn get_visible_text(&self, scroll_y: f32) -> Option<String> {
        let y1 = scroll_y;
        // Some letters extend below the baseline, so add a little extra
        let y2 = scroll_y + self.ctx.frame.height() + self.ctx.font_size * 0.2;
        let filter = self.ctx.metrics.iter().filter(|m| m.1 > y1 && m.1 < y2);
        let results: String = filter.map(|m| m.3).collect();
        Some(results)
    }

    // *****************************************************************************************************
    // TextAreaEditor: Render as image functions
    // *****************************************************************************************************

    /// Method for cropping self.full_render to deliver only the visible chunk to the GPU
    pub fn crop_cached_render(&mut self, x: u32, y: u32, w: u32, h: u32) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        if let Some(imgbuf) = &mut self.full_render {
            let subimg = imageops::crop(imgbuf, x, y, w, h);
            return Some(subimg.to_image());
        }
        None
    }
}
