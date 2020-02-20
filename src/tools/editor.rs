/// This provides services for view components that allow text editing.
/// https://docs.rs/glyph_brush/0.4.1/glyph_brush/
// #[allow(unused_imports)]
use super::*;

use glyph_brush::rusttype::{self, GlyphId, Scale};
use glyph_brush::{self, GlyphCruncher, Layout, Section, SectionText, VariedSection};

use image_rs::RgbaImage;
use std::{collections::HashMap, f32, ops::Range};

//-- Support -----------------------------------------------------------------------

const SPACE: char = ' ';
const ALPHANUMERICS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ";

static ROBOTO_REGULAR: &[u8] = include_bytes!("../../static/Roboto-Regular.ttf");

/// Model for storing character metrics from GlyphBrush, including coordinates, row number, char width
#[derive(Debug, Clone)]
pub struct CharData {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub letter: char,
    pub row: usize,
}

impl CharData {
    pub fn new(x: f32, y: f32, width: f32, letter: char, row: usize) -> Self {
        CharData { x, y, width, letter, row }
    }
}

/// Describes where the cursor is inserting text with a bool flag where true means the
/// text size is overflowing the field size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InsertMode {
    /// At position 0
    Start(bool),
    /// At the end of the string
    End(bool),
    /// Intra-text
    Intra(bool),
    /// Special case for multiline where the cursor should append at end of previous line.
    AppendPreviousLine,
}

//-- Main -----------------------------------------------------------------------

/// Base struct for TextEditor that holds much of the state information
pub struct EditorContext {
    /// Owned DrawFont
    pub draw_font: DrawFont,
    /// Is it multi-line text?
    is_multiline: bool,
    /// The actual string content
    string: String,
    /// The font size
    pub font_size: f32,
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
    /// Enum to store where the cursor is inserting text
    pub insert_mode: InsertMode,
    /// Temporary value for debugging
    pub debug: bool,
    /// Data that stores position and width data for every character in the text.
    metrics: Vec<CharData>,
    /// The baseline y float values for every row of text.
    baselines: Vec<f32>,
    /// Holds the currently visible range of chars
    visible_range: Range<usize>,
}

impl Default for EditorContext {
    fn default() -> Self {
        let draw_font = DrawFont::from_bytes(ROBOTO_REGULAR.clone().to_vec(), None);
        let rect = rusttype::Rect { min: rusttype::point(0.0, 0.0), max: rusttype::point(1.0, 1.0) };
        let ctx = EditorContext {
            draw_font,
            is_multiline: false,
            string: String::default(),
            font_size: 14.0,
            cursor_pos: 0,
            cursor_origin: (0.0, 0.0),
            text_origin: (0.0, 0.0),
            text_size: (0, 0),
            has_changed: true,
            frame: rect,
            glyph_db: HashMap::new(),
            char_db: HashMap::new(),
            insert_mode: InsertMode::Start(false),
            debug: false,
            metrics: Vec::new(),
            baselines: Vec::new(),
            visible_range: 0..1,
        };
        ctx
    }
}

impl EditorContext {
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

    /// Set the font bytes and font size. This is meant to be called when the Theme
    /// has been selected on startup
    pub fn set_font_data(&mut self, data: Vec<u8>, font_size: f32) {
        self.draw_font = DrawFont::from_bytes(data, None);
        self.font_size = font_size;
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
        // log::debug!("============= update_metrics =============");
        self.draw_font.cached_mesh = None;
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

        let mut glyph_calc = self.draw_font.glyph_calc().cache_scope();
        let layout = Layout::default();
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
        let mut row_index = 0 as usize;
        let mut metrics: Vec<CharData> = Vec::new();
        let mut last_data: (f32, f32, f32) = (0.0, 0.0, 0.0);
        let (space_w, _) = self.draw_font.char_size(SPACE, self.font_size);

        for c in self.string.chars() {
            let width = {
                if let Some(w) = self.char_db.get(&c) {
                    *w
                } else {
                    let (w, _) = self.draw_font.char_size(c, self.font_size);
                    // log::trace!("Insert in char_db [{:?}] width={:?}", c, w);
                    self.char_db.insert(c, w);
                    w
                }
            };

            if c != SPACE {
                if pointer < xy_coords.len() {
                    if !approx_eq!(f32, xy_coords[pointer].1, last_data.1, ulps = 2) && last_data.1 > 0.0 {
                        row_index += 1;
                        self.baselines.push(last_data.1);
                    }
                    last_data = (xy_coords[pointer].0, xy_coords[pointer].1, width);
                    let m = CharData::new(last_data.0, last_data.1, width, c, row_index);
                    metrics.push(m);
                    pointer += 1;
                }
            } else {
                let m = CharData::new(last_data.0 + last_data.2, last_data.1, space_w, c, row_index);
                metrics.push(m);
            }
        }

        self.metrics = metrics;
        // log::debug!("metrics={:?}", self.metrics);
        // log::debug!("baselines={:?}", self.baselines);

        if let Some(size) = glyph_calc.pixel_bounds(&varied) {
            self.text_size = (size.width() as u32, size.height() as u32);
        }
    }

    /// A utility function for getting the size of specified text and calculate the cursor position.
    /// This is only useful for single-line text. Since glyph_brush does not count trailing spaces in
    /// pixel_bounds, we have to inspect the number of trailing spaces and pad the result.
    /// Another use case is calculating the width of a password mask in TextField. In this case, trailing
    /// spaces will not exist.
    /// TODO: Migrate to DrawFont.measure_text
    pub fn measure_text(&self, text: &str) -> (f32, f32) {
        let mut glyph_calc = self.draw_font.glyph_calc().cache_scope();

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
    pub fn start_editing(&mut self, position: Option<usize>) {
        if self.debug && self.glyph_db.len() == 0 {
            let glyphs = self.draw_font.get_raw_font().glyphs_for(ALPHANUMERICS.chars());
            let pairs = glyphs.zip(ALPHANUMERICS.chars());
            self.glyph_db.extend(pairs.map(|p| (p.0.id(), p.1)));
        }

        if self.has_changed {
            self.update_metrics();
        }
        self.has_changed = true;
        if let Some(position) = position {
            log::debug!("position={:?} y={:?}", position, 0);
            self.cursor_pos = position;
        } else {
            self.cursor_pos = 0;
        }
        // if self.is_multiline {
        // } else {
        //     if let Some(position) = position {
        //         self.cursor_pos = position;
        //     } else {
        //         self.cursor_pos = 0;
        //     }
        // }
    }

    /// Switch read mode
    pub fn stop_editing(&mut self) {
        self.has_changed = false;
    }

    /// Handle keyboard input by inserting char at current cursor_pos
    pub fn insert_char(&mut self, c: char) {
        self.has_changed = true;
        log::trace!("Insert char={:?}", c);
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

impl Default for TextFieldEditor {
    fn default() -> Self {
        let ctx = EditorContext::default().multiline(false);
        TextFieldEditor { ctx }
    }
}

impl TextFieldEditor {
    /// Builder method with specified position and size. Avoid backend-specific parameters
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
            // Text size is less than frame width
            if self.ctx.string.len() > 0 {
                self.ctx.visible_range = 0..self.ctx.string.len();
                if self.ctx.cursor_pos == self.ctx.string.len() {
                    self.ctx.insert_mode = InsertMode::End(false);
                    // Since text_size does not account for trailing spaces, append a space.
                    let text_size = self.ctx.measure_text(&self.ctx.string);
                    self.ctx.cursor_origin.0 = self.ctx.frame.min.x + text_size.0;
                } else {
                    self.ctx.insert_mode = InsertMode::Intra(false);
                    self.ctx.cursor_origin.0 = self.ctx.frame.min.x + self.ctx.metrics[self.ctx.cursor_pos].x;
                }
            }
        } else {
            // Text size exceeds frame width
            // log::debug!("text_size={:?} frame.w={:?}", self.text_size, self.frame.width());
            if self.ctx.cursor_pos == 0 {
                // Either the field is not being edited or is at beginning.
                self.ctx.insert_mode = InsertMode::Start(true);
                if let Some(end) = self.ctx.metrics.iter().position(|m| m.x > self.ctx.frame.width()) {
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
                    .position(|m| m.letter != SPACE && m.x < self.ctx.text_size.0 as f32 - self.ctx.frame.width())
                {
                    let start = self.ctx.string.len() - rev_index - 1;
                    self.ctx.visible_range = start..self.ctx.string.len();
                    // Determine offset width for range of text left of the view
                    let offset = self.ctx.metrics[start].x;
                    let string_w = self.ctx.text_size.0 as f32 - offset;
                    self.ctx.text_origin.0 = self.ctx.frame.max.x - string_w as f32;

                    if self.ctx.cursor_pos == self.ctx.string.len() {
                        // Cursor is at the end
                        self.ctx.insert_mode = InsertMode::End(true);
                        self.ctx.cursor_origin.0 = self.ctx.frame.max.x;
                    } else {
                        // Cursor is intra-text
                        self.ctx.insert_mode = InsertMode::Intra(true);
                        self.ctx.cursor_origin.0 =
                            self.ctx.text_origin.0 + self.ctx.metrics[self.ctx.cursor_pos].x - offset;
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
                    log::error!(
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
        log::debug!("visible text={:?}", chunk);
        Some(chunk.to_string())
    }

    /// Locate the closest cursor insertion point based on the mouse position
    pub fn find_cursor_position(&self, mouse_x: f32) -> Option<usize> {
        if self.ctx.metrics.len() > 0 {
            let chars = &self.ctx.metrics[self.ctx.visible_range.clone()];
            log::debug!("mouse_x={:?} chars={:?}", mouse_x, chars);
            let mut xpos: usize = 0;

            let start_pos: usize = 0;
            for data in chars {
                if mouse_x < data.x + data.width / 2.0 {
                    // Found intra-text insertion point
                    break;
                } else {
                    log::debug!("char={:?}", data);
                }
                xpos += 1;
            }
            let cursor_pos = start_pos + xpos;
            log::debug!(
                "FOUND char mouse_x={} // start_pos={} + xpos={} = cursor_pos={}",
                mouse_x,
                start_pos,
                xpos,
                cursor_pos
            );
            Some(cursor_pos)
        } else {
            None
        }
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
    pub full_render: Option<RgbaImage>,
    /// Stores the index value of the texture
    pub tex_info: Option<GPUTexture>,
}

impl Default for TextAreaEditor {
    fn default() -> Self {
        let ctx = EditorContext::default().multiline(true);
        TextAreaEditor { ctx, full_render: None, tex_info: None }
    }
}

impl TextAreaEditor {
    /// Builder method with specified position and size. Avoid backend-specific parameters
    pub fn with_frame(mut self, origin: (f32, f32), size: (f32, f32)) -> Self {
        let frame = rusttype::Rect {
            min: rusttype::point(origin.0, origin.1),
            max: rusttype::point(origin.0 + size.0, origin.1 + size.1),
        };
        self.ctx.frame = frame;
        self
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
            self.ctx.frame.min.x + self.ctx.metrics[self.ctx.cursor_pos].x,
            self.ctx.frame.min.y + self.ctx.metrics[self.ctx.cursor_pos].y,
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
        let base_y = self.ctx.metrics[self.ctx.cursor_pos].y.round() as u32;
        let filter = self.ctx.metrics.iter().filter(|m| m.y.round() as u32 == base_y);
        let results: Vec<(f32, f32, char)> = filter.map(|m| (m.x, m.y, m.letter)).collect(); // Unfortunately only this worked.
        results
    }

    /// Method to calculate what text is visible in a TextArea given the current scroll offset
    pub fn get_visible_text(&self, scroll_y: f32) -> Option<String> {
        let y1 = scroll_y;
        // Some letters extend below the baseline, so add a little extra
        let y2 = scroll_y + self.ctx.frame.height() + self.ctx.font_size * 0.2;
        let filter = self.ctx.metrics.iter().filter(|m| m.y > y1 && m.y < y2);
        let results: String = filter.map(|m| m.letter).collect();
        Some(results)
    }

    /// Calculate the closest insertion point based on the x-y coordinates of the mouse and the scroll_y position.
    /// First, get the visible range of characters and use the position offset as the base value. Next, find the row
    /// that the mouse cursor is on. From there, use x coordinate to determine closest letter gap
    pub fn find_cursor_position(&mut self, mouse_x: f32, mouse_y: f32, scroll_y: f32) -> Option<usize> {
        let row_size = self.ctx.font_size;
        let real_y = mouse_y + scroll_y;
        // Find the row_index where the baseline is within the mouse position range (subtract row_size to get lower bound)
        if let Some(row_index) = self.ctx.baselines.iter().position(|y| real_y > *y - row_size && real_y < *y) {
            // Get the position of the first CharData in metrics for the specified row
            if let Some(start_pos) = self.ctx.metrics.iter().position(|m| m.row == row_index) {
                // Get the array of CharData with the same row_index
                let chars: Vec<CharData> = self.ctx.metrics.iter().filter(|m| m.row == row_index).cloned().collect();
                let mut xpos: usize = 0;
                // Find the closest insertion point, using char width to help find
                for data in chars {
                    if mouse_x < data.x + data.width / 2.0 {
                        // Found intra-text insertion point
                        break;
                    } else {
                        log::trace!("char={:?}", data);
                    }
                    xpos += 1;
                }
                let cursor_pos = start_pos + xpos;
                log::debug!(
                    "FOUND char mouse_x={} // start_pos={} + xpos={} = cursor_pos={}",
                    mouse_x,
                    start_pos,
                    xpos,
                    cursor_pos
                );
                return Some(cursor_pos);
            }
        }
        None
    }
}
