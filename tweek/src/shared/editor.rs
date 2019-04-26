/// This provides services for view components that allow text editing.
/// https://docs.rs/glyph_brush/0.4.1/glyph_brush/
// #[allow(unused_imports)]
use glyph_brush::rusttype::{self, point, Font as RTFont, GlyphId, PositionedGlyph, Scale};

#[allow(unused_imports)]
use glyph_brush::{
    self, BrushAction, BrushError, Color, DefaultSectionHasher, FontId, GlyphBrush, GlyphBrushBuilder, GlyphCalculator,
    GlyphCalculatorBuilder, GlyphCruncher, GlyphPositioner, HorizontalAlign as HAlign, Layout, Section,
    SectionGeometry, SectionText, VariedSection,
};
use image::{imageops, DynamicImage, ImageBuffer, Rgba};
use std::collections::HashSet;
use std::f32;
use std::ops::Range;

#[allow(unused_imports)]
use quicksilver::geom::{Line, Rectangle, Vector};

const SPACE: char = ' ';
static ROBOTO_REGULAR: &[u8] = include_bytes!("../../static/Roboto-Regular.ttf");

pub trait Editor {}

pub struct EditorContext {
    raw_font: RTFont<'static>,
    glyph_calc: GlyphCalculator<'static>,
    pub string: String,
    pub font_size: f32,
    pub font_color: u32,
    pub cursor_pos: usize,
    pub cursor_origin: (f32, f32),
    pub text_origin: (f32, f32),
    pub text_size: (u32, u32),
    pub has_changed: bool,
    pub frame: rusttype::Rect<f32>,
    metrics: Vec<(f32, f32, char)>,
    visible_range: Range<usize>,
}

impl Default for EditorContext {
    fn default() -> Self {
        let font = RTFont::from_bytes(ROBOTO_REGULAR).unwrap();
        let glyph_calc = GlyphCalculatorBuilder::using_font(font.clone()).build();

        let rect = rusttype::Rect { min: rusttype::point(0.0, 0.0), max: rusttype::point(1.0, 1.0) };

        EditorContext {
            raw_font: font,
            glyph_calc: glyph_calc,
            string: String::default(),
            font_size: 14.0,
            font_color: 0x000000,
            cursor_pos: 0,
            cursor_origin: (0.0, 0.0),
            text_origin: (0.0, 0.0),
            text_size: (0, 0),
            has_changed: true,
            frame: rect,
            metrics: Vec::new(),
            visible_range: 0..1,
        }
    }
}

impl EditorContext {
    pub fn new(raw_font: RTFont<'static>) -> Self {
        let mut ctx = EditorContext::default();
        ctx.raw_font = raw_font;
        ctx
    }

    pub fn load(bytes: &'static [u8]) -> Self {
        let raw_font: RTFont<'static> = RTFont::from_bytes(bytes).unwrap();
        let mut ctx = EditorContext::default();
        ctx.raw_font = raw_font;
        ctx
    }

    pub fn with_text(mut self, text: String, font_size: f32) -> Self {
        self.string = text;
        self.font_size = font_size;
        self
    }

    pub fn with_frame(mut self, origin: (f32, f32), size: (f32, f32)) -> Self {
        let frame = rusttype::Rect {
            min: rusttype::point(origin.0, origin.1),
            max: rusttype::point(origin.0 + size.0, origin.1 + size.1),
        };
        self.frame = frame;
        self
    }

    pub fn set_font_bytes(mut self, bytes: &'static [u8]) {
        let raw_font: RTFont<'static> = RTFont::from_bytes(bytes).unwrap();
        self.raw_font = raw_font;
    }

    pub fn set_font(&mut self, raw_font: RTFont<'static>) {
        self.glyph_calc = GlyphCalculatorBuilder::using_font(raw_font.clone()).build();
        self.raw_font = raw_font;
    }

    // *****************************************************************************************************
    // Editor life cycle functions
    // *****************************************************************************************************

    pub fn start_editing(&mut self) {
        self.has_changed = true;
        self.cursor_pos = self.string.len();
    }

    pub fn stop_editing(&mut self) {
        self.has_changed = false;
    }

    pub fn insert_char(&mut self, c: char) {
        self.has_changed = true;
        if self.cursor_pos == self.string.len() {
            // cursor is at the end. add there.
            self.string.push(c);
            self.cursor_pos = self.string.len();
        } else if self.cursor_pos < self.string.len() {
            // insert at cursor
            self.string.insert(self.cursor_pos, c);
            self.cursor_pos += 1;
        }
        // log::debug!(
        //     "Updated: string='{}' len={}",
        //     self.string,
        //     self.string.len()
        // );
    }

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
        log::debug!("Backspace: string='{}' len={}", self.string, self.string.len());
    }

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
        log::debug!("cursor at={:?} string.len={:?}", self.cursor_pos, self.string.len());
    }
}

// *****************************************************************************************************
// TextFieldEditor
// *****************************************************************************************************

pub struct TextFieldEditor {
    pub ctx: EditorContext,
}

impl Default for TextFieldEditor {
    fn default() -> Self {
        let ctx = EditorContext::default();
        TextFieldEditor { ctx: ctx }
    }
}

impl TextFieldEditor {
    pub fn with_frame(mut self, origin: (f32, f32), size: (f32, f32)) -> Self {
        let frame = rusttype::Rect {
            min: rusttype::point(origin.0, origin.1),
            max: rusttype::point(origin.0 + size.0, origin.1 + size.1),
        };
        self.ctx.frame = frame;
        self
    }

    pub fn get_line_height(&self) -> f32 {
        // TODO: handle hidpi factor
        return self.ctx.font_size;
    }

    // *****************************************************************************************************
    // Editor metrics
    // *****************************************************************************************************

    /// This is called whenever self.metrics needs to be updated.
    /// – When initializing a text input view
    /// – When the user has inserted or deleted characters (edit mode)
    /// – When scrolling?
    ///
    pub fn update_metrics(&mut self) {
        if self.ctx.string.len() == 0 {
            return;
        };
        let mut glyph_calc = self.ctx.glyph_calc.cache_scope();
        let layout = Layout::default();
        let scale = Scale::uniform(self.ctx.font_size);
        let varied = VariedSection {
            layout: layout,
            text: vec![SectionText {
                text: &self.ctx.string,
                scale: Scale::uniform(self.ctx.font_size),
                ..SectionText::default()
            }],
            ..VariedSection::default()
        };
        let glyphs = glyph_calc.glyphs(&varied);

        let xy_coords: Vec<(f32, f32)> = glyphs.map(|v| (v.position().x, v.position().y)).collect();
        let mut pointer = 0 as usize;
        let mut metrics: Vec<(f32, f32, char)> = Vec::new();
        let mut last_data: (f32, f32) = (0.0, 0.0);
        let space_w = self.ctx.raw_font.glyph(SPACE).scaled(scale).h_metrics().advance_width;

        for c in self.ctx.string.chars() {
            if c != SPACE {
                last_data = (xy_coords[pointer].0, xy_coords[pointer].1);
                metrics.push((last_data.0, last_data.1, c));
                pointer += 1;
            } else {
                metrics.push(((last_data.0 + space_w), last_data.1, c));
            }
        }

        self.ctx.metrics = metrics;
        let pixel_bounds = glyph_calc.pixel_bounds(&varied).expect("None bounds");
        self.ctx.text_size = (pixel_bounds.width() as u32, pixel_bounds.height() as u32);
        // log::debug!("metrics.len={:?} string.len={:?}", self.ctx.metrics.len(), self.ctx.string.len());
        // log::debug!("metrics={:?}", self.metrics);

        // let y_data: Vec<f32> = xy_coords.iter().map(|xy| xy.1).collect();
        // self.baselines = y_data;
        // self.baselines.dedup();
        // log::debug!("baselines={:?}", self.baselines);
    }

    /// This function is called when a TextField is executing the render() function and
    /// is_editing=true.
    /// Calculate the following:
    /// – cursor x y position where x is the insertion point and y is the baseline position
    ///   for the current line.
    /// – range of visible text. For single-line, this is limited by the width of dimensions.
    pub fn update_display(&mut self) {
        if !self.ctx.has_changed {
            return;
        }
        self.ctx.has_changed = false;

        let cursor_space = 0.0;
        if self.ctx.cursor_pos > self.ctx.metrics.len() {
            log::debug!("PANIC! cursor_pos={:?} OOB metrics={:?}", self.ctx.cursor_pos, self.ctx.metrics.len());
            // TODO: return false or error so that text field can stop rendering
            return;
        }

        // Set default for easiest case
        self.ctx.cursor_origin =
            (self.ctx.frame.min.x + cursor_space, self.ctx.frame.min.y + self.ctx.frame.height() / 2.0);
        self.ctx.text_origin = (
            self.ctx.frame.min.x,
            self.ctx.frame.min.y + (self.ctx.frame.height() - self.ctx.text_size.1 as f32) / 2.0,
        );

        if self.ctx.text_size.0 as f32 <= self.ctx.frame.width() {
            self.ctx.visible_range = 0..self.ctx.string.len();
            if self.ctx.cursor_pos == self.ctx.string.len() {
                self.ctx.cursor_origin.0 = self.ctx.frame.min.x + self.ctx.text_size.0 as f32 + cursor_space;
            } else {
                self.ctx.cursor_origin.0 = self.ctx.frame.min.x + self.ctx.metrics[self.ctx.cursor_pos].0;
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
                    .position(|x| x.2 != SPACE && x.0 < self.ctx.text_size.0 as f32 - self.ctx.frame.width())
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
        let chunk = &self.ctx.string[self.ctx.visible_range.clone()];
        log::debug!("chunk={:?} range={:?}", chunk, self.ctx.visible_range);
    }

    pub fn get_visible_text(&self, _scroll_x: f32) -> Option<String> {
        let chunk = &self.ctx.string[self.ctx.visible_range.clone()];
        Some(chunk.to_string())
    }

    pub fn get_text_origin(&self) -> (f32, f32) {
        self.ctx.text_origin
    }

    // *****************************************************************************************************
    // Editor life cycle functions
    // *****************************************************************************************************

    pub fn start_editing(&mut self) {
        self.ctx.start_editing();
        self.update_metrics();
    }

    pub fn stop_editing(&mut self) {
        self.ctx.stop_editing();
    }

    pub fn insert_char(&mut self, c: char) {
        self.ctx.insert_char(c);
        self.update_metrics();
    }

    pub fn delete_char(&mut self) {
        self.ctx.delete_char();
        self.update_metrics();
    }

    pub fn move_cursor(&mut self, shift: i32) {
        self.ctx.move_cursor(shift);
    }

    // *****************************************************************************************************
    // Render as image functions
    // *****************************************************************************************************

    pub fn render_visible_text(&mut self) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let mut glyph_calc = self.ctx.glyph_calc.cache_scope();
        let layout = Layout::default_wrap().h_align(HAlign::Left);

        let varied = VariedSection {
            layout: layout,
            text: vec![SectionText {
                text: &self.ctx.string,
                scale: Scale::uniform(self.ctx.font_size),
                ..SectionText::default()
            }],
            ..Default::default()
        };
        let glyphs = glyph_calc.glyphs(&varied);

        let buffer = self.ctx.font_size as u32;
        let mut imgbuf =
            DynamicImage::new_rgba8(self.ctx.frame.width() as u32 + buffer, self.ctx.frame.height() as u32 + buffer)
                .to_rgba();

        // Loop through the glyphs in the text, positing each one on a line
        let mut last_y = 0 as u32;
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    if y != last_y {
                        last_y = y;
                    }
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

        Some(imgbuf)
    }

    pub fn render_to_image(&self, text: &str, size: f32) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let scale = Scale::uniform(size);
        let v_metrics = self.ctx.raw_font.v_metrics(scale);

        // FIXME: ypos is hard coded. It will panic/crash if glyph y is negative
        let glyphs: Vec<_> = self.ctx.raw_font.layout(text, scale, point(0.0, 10.0)).collect();

        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
            let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
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
                        Rgba { data: [0, 0, 0, (v * 255.0) as u8] },
                    )
                });
            }
        }
        return Some(imgbuf);
    }

    pub fn render_text_cropped(
        &self,
        text: &str,
        size: f32,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    ) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        if let Some(imgbuf) = &mut self.render_to_image(text, size) {
            let subimg = imageops::crop(imgbuf, x, y, w, h);

            return Some(subimg.to_image());
        }
        None
    }
}

// *****************************************************************************************************
// TextAreaEditor
// *****************************************************************************************************

#[allow(dead_code)]
pub struct TextAreaEditor {
    pub ctx: EditorContext,
    full_render: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    line_count: usize,
    baselines: Vec<f32>,
}

impl Default for TextAreaEditor {
    fn default() -> Self {
        let ctx = EditorContext::default();
        TextAreaEditor { ctx: ctx, full_render: None, line_count: 1, baselines: Vec::new() }
    }
}

impl TextAreaEditor {
    pub fn with_frame(mut self, origin: (f32, f32), size: (f32, f32)) -> Self {
        let frame = rusttype::Rect {
            min: rusttype::point(origin.0, origin.1),
            max: rusttype::point(origin.0 + size.0, origin.1 + size.1),
        };
        self.ctx.frame = frame;
        self
    }

    // *****************************************************************************************************
    // Editor metrics
    // *****************************************************************************************************

    pub fn update_metrics(&mut self) {
        if self.ctx.string.len() == 0 {
            return;
        };
        let mut glyph_calc = self.ctx.glyph_calc.cache_scope();
        let layout = Layout::default();
        let varied = VariedSection {
            layout: layout,
            bounds: (self.ctx.frame.width(), f32::INFINITY),
            text: vec![SectionText {
                text: &self.ctx.string,
                scale: Scale::uniform(self.ctx.font_size),
                ..SectionText::default()
            }],
            ..VariedSection::default()
        };

        let pixel_bounds = glyph_calc.pixel_bounds(&varied).expect("None bounds");
        self.ctx.text_size = (pixel_bounds.width() as u32, pixel_bounds.height() as u32);

        let glyphs = glyph_calc.glyphs(&varied);
        let xy_coords: Vec<(f32, f32)> = glyphs.map(|v| (v.position().x, v.position().y)).collect();

        let mut pointer = 0 as usize;
        let mut metrics: Vec<(f32, f32, char)> = Vec::new();
        let mut last_data: (f32, f32) = (0.0, 0.0);

        let scale = Scale::uniform(self.ctx.font_size);
        let space_w = self.ctx.raw_font.glyph(SPACE).scaled(scale).h_metrics().advance_width;

        for c in self.ctx.string.chars() {
            if c != SPACE {
                last_data = (xy_coords[pointer].0, xy_coords[pointer].1);
                metrics.push((last_data.0, last_data.1, c));
                pointer += 1;
            } else {
                metrics.push(((last_data.0 + space_w), last_data.1, c));
            }
            if pointer == xy_coords.len() {
                break;
            }
        }
        // log:trace!("metrics={:?}", metrics);
        self.ctx.metrics = metrics;
        // let y_data: Vec<f32> = xy_coords.iter().map(|xy| xy.1).collect();
        // self.ctx.baselines = y_data;
        // self.ctx.baselines.dedup();
        self.ctx.cursor_origin = (
            self.ctx.frame.min.x + self.ctx.metrics[self.ctx.cursor_pos].0,
            self.ctx.frame.min.y + self.ctx.metrics[self.ctx.cursor_pos].1 - self.ctx.font_size / 2.0,
        );
    }

    pub fn update_rendered_text(&mut self) {
        if self.ctx.string.len() == 0 {
            return;
        }
        let mut glyph_calc = self.ctx.glyph_calc.cache_scope();
        let layout = Layout::default();
        let varied = VariedSection {
            layout: layout,
            bounds: (self.ctx.frame.width(), f32::INFINITY),
            text: vec![SectionText {
                text: &self.ctx.string,
                scale: Scale::uniform(self.ctx.font_size),
                ..SectionText::default()
            }],
            ..VariedSection::default()
        };

        let pixel_bounds = glyph_calc.pixel_bounds(&varied).expect("None bounds");
        self.ctx.text_size = (pixel_bounds.width() as u32, pixel_bounds.height() as u32);

        let mut xy_coords: Vec<(f32, f32)> = Vec::new();
        let glyphs = glyph_calc.glyphs(&varied);

        let mut imgbuf = DynamicImage::new_rgba8(self.ctx.text_size.0 + 100, self.ctx.text_size.1 + 100).to_rgba();

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            xy_coords.push((glyph.position().x, glyph.position().y));
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
        self.full_render = Some(imgbuf);

        let mut pointer = 0 as usize;
        let mut metrics: Vec<(f32, f32, char)> = Vec::new();
        let mut last_data: (f32, f32) = (0.0, 0.0);

        let scale = Scale::uniform(self.ctx.font_size);
        let space_w = self.ctx.raw_font.glyph(SPACE).scaled(scale).h_metrics().advance_width;

        for c in self.ctx.string.chars() {
            if c != SPACE {
                last_data = (xy_coords[pointer].0, xy_coords[pointer].1);
                metrics.push((last_data.0, last_data.1, c));
                pointer += 1;
            } else {
                metrics.push(((last_data.0 + space_w), last_data.1, c));
            }
            if pointer == xy_coords.len() {
                break;
            }
        }

        self.ctx.metrics = metrics;

        // let y_data: Vec<f32> = xy_coords.iter().map(|xy| xy.1).collect();
        // self.baselines = y_data;
        // self.baselines.dedup();
        // self.cursor_origin = (self.frame.min.x + self.metrics[self.cursor_pos].0, self.frame.min.y + self.metrics[self.cursor_pos].1 - self.font_size/2.0);
    }

    pub fn update_display(&mut self) {
        if !self.ctx.has_changed {
            return;
        }
        // Text is being edited
        self.ctx.has_changed = false;
        self.ctx.cursor_origin = (
            self.ctx.frame.min.x + self.ctx.metrics[self.ctx.cursor_pos].0,
            self.ctx.frame.min.y + self.ctx.metrics[self.ctx.cursor_pos].1,
        );
        self.ctx.text_origin = (self.ctx.frame.min.x, self.ctx.frame.min.y);
    }

    pub fn get_visible_text(&self, scroll_y: f32) -> Option<String> {
        let y1 = scroll_y;
        // Some letters extend below the baseline, so add a little extra
        let y2 = scroll_y + self.ctx.frame.height() + self.ctx.font_size * 0.2;
        let filter = self.ctx.metrics.iter().filter(|m| m.1 > y1 && m.1 < y2);
        let results: String = filter.map(|m| m.2).collect();
        Some(results)
    }

    pub fn get_wrapped_text(&self, scroll_y: f32) -> Option<String> {
        let y1 = scroll_y;
        let y2 = scroll_y + self.ctx.frame.height();
        let filter = self.ctx.metrics.iter().filter(|m| m.1 > y1 && m.1 < y2);
        // eprintln!("text={:?} y={:?}", filter, 0);
        // eprintln!("y1={:?} y2={:?} count={}", y1, y2, 0);
        let mut buffer: Vec<char> = Vec::new();
        for m in filter {
            if m.0 == 0.0 && buffer.len() > 0 {
                buffer.push('\n');
            }
            buffer.push(m.2);
        }
        let result: String = buffer.into_iter().collect();
        // eprintln!("count={:?} result={:?}", result.len(), result);
        Some(result)
    }

    pub fn draw_visible_text(&mut self, scroll_y: f32) {
        if let Some(string) = self.get_visible_text(scroll_y) {
            // let glyphs = self.get_glyphs_for_text(&string);
            let mut glyph_brush: GlyphBrush<'_, DefaultSectionHasher> =
                GlyphBrushBuilder::using_font_bytes(ROBOTO_REGULAR).build();
            let layout = Layout::default();
            let varied = VariedSection {
                layout: layout,
                bounds: (self.ctx.frame.width(), self.ctx.frame.height()),
                text: vec![SectionText {
                    text: &string,
                    scale: Scale::uniform(self.ctx.font_size),
                    ..SectionText::default()
                }],
                ..VariedSection::default()
            };
            glyph_brush.queue(&varied);
            // match glyph_brush.process_queued(
            //     (1024, 768),
            //     |rect, tex_data| {

            //     },
            //     |vertex_data| {
            //         // into_vertex(vertex_data)
            //         },
            // ) {
            //     Ok(BrushAction::Draw(vertices)) => {
            //         // Draw new vertices.
            //     }
            //     Ok(BrushAction::ReDraw) => {
            //         // Re-draw last frame's vertices unmodified.
            //     }
            //     Err(BrushError::TextureTooSmall { suggested }) => {
            //         // Enlarge texture + glyph_brush texture cache and retry.
            //     }
            // }
        }
    }

    // fn to_vertex(v: glyph_brush::GlyphVertex) -> DrawParam {
    //     let src_rect = Rect {
    //         x: v.tex_coords.min.x,
    //         y: v.tex_coords.min.y,
    //         w: v.tex_coords.max.x - v.tex_coords.min.x,
    //         h: v.tex_coords.max.y - v.tex_coords.min.y,
    //     };
    //     // it LOOKS like pixel_coords are the output coordinates?
    //     // I'm not sure though...
    //     let dest_pt = Point2::new(v.pixel_coords.min.x as f32, v.pixel_coords.min.y as f32);
    //     DrawParam::default()
    //         .src(src_rect)
    //         .dest(dest_pt)
    //         .color(v.color.into())
    // }

    pub fn render_visible_text(&mut self, scroll_y: f32) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        if let Some(string) = self.get_visible_text(scroll_y) {
            // eprintln!("string={:?} y={:?}", &string, 0);
            let mut glyph_calc = self.ctx.glyph_calc.cache_scope();
            let layout = Layout::default();
            let varied = VariedSection {
                layout: layout,
                bounds: (self.ctx.frame.width(), self.ctx.frame.height()),
                text: vec![SectionText {
                    text: &string,
                    scale: Scale::uniform(self.ctx.font_size),
                    ..SectionText::default()
                }],
                ..VariedSection::default()
            };

            let glyphs = glyph_calc.glyphs(&varied);
            let mut imgbuf =
                DynamicImage::new_rgba8(self.ctx.frame.width() as u32 + 20, self.ctx.frame.height() as u32 + 20)
                    .to_rgba();

            // Loop through the glyphs in the text, positing each one on a line
            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    // if let Some(c) = self.glyph_id_to_char(&chars, glyph.id()) {
                    //     eprintln!("c={:?} bb={:?}", c, bounding_box);
                    // }
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
            return Some(imgbuf);
        }
        None
    }

    pub fn vertices_for_text(&mut self, scroll_y: f32) -> Vec<Vector> {
        let mut vectors: Vec<Vector> = Vec::new();
        if let Some(string) = self.get_visible_text(scroll_y) {
            // eprintln!("string={:?} y={:?}", &string, 0);
            let mut glyph_calc = self.ctx.glyph_calc.cache_scope();
            let layout = Layout::default();
            let varied = VariedSection {
                layout: layout,
                bounds: (self.ctx.frame.width(), self.ctx.frame.height()),
                text: vec![SectionText {
                    text: &string,
                    scale: Scale::uniform(self.ctx.font_size),
                    ..SectionText::default()
                }],
                ..VariedSection::default()
            };

            let glyphs = glyph_calc.glyphs(&varied);

            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|x, y, v| {
                        vectors.push(Vector { x: x as f32, y: y as f32 });
                    });
                }
            }
        }
        vectors
    }
    fn _glyph_id_to_char(&self, chars: &HashSet<char>, id: GlyphId) -> Option<char> {
        // let chars: HashSet<char> = self.ctx.string.chars().collect();
        chars.iter().find(|c| self.ctx.raw_font.glyph(**c).id() == id).cloned()
    }

    pub fn get_glyphs_for_text(&self, text: &str) -> Vec<(PositionedGlyph, Color, FontId)> {
        let font = RTFont::from_bytes(ROBOTO_REGULAR).unwrap();
        let fonts = vec![font];
        let glyphs = Layout::default().calculate_glyphs(
            &fonts,
            &SectionGeometry {
                bounds: (self.ctx.frame.width(), self.ctx.frame.height()),
                ..SectionGeometry::default()
            },
            &[SectionText { text: text, scale: Scale::uniform(self.ctx.font_size), ..SectionText::default() }],
        );
        glyphs
    }

    pub fn get_text_origin(&self) -> (f32, f32) {
        self.ctx.text_origin
    }

    pub fn get_line_height(&self) -> f32 {
        // TODO: handle hidpi factor
        return self.ctx.font_size;
    }

    // *****************************************************************************************************
    // Editor lifecycle functions
    // *****************************************************************************************************

    pub fn start_editing(&mut self) {
        self.ctx.start_editing();
        self.ctx.cursor_pos = 0;
        self.update_metrics();
    }

    pub fn stop_editing(&mut self) {
        self.ctx.stop_editing();
        self.update_rendered_text();
    }

    pub fn insert_char(&mut self, c: char) {
        self.ctx.insert_char(c);
        self.update_metrics();
    }

    pub fn delete_char(&mut self) {
        self.ctx.delete_char();
        self.update_metrics();
    }

    pub fn move_cursor(&mut self, shift: i32) {
        self.ctx.move_cursor(shift);
    }

    // *****************************************************************************************************
    // Render as image functions
    // *****************************************************************************************************

    pub fn crop_cached_render(&mut self, x: u32, y: u32, w: u32, h: u32) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        if let Some(imgbuf) = &mut self.full_render {
            let subimg = imageops::crop(imgbuf, x, y, w, h);
            return Some(subimg.to_image());
        }
        None
    }
}
