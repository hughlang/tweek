/// A helper for drawing glyph text using the GPU via glyph_brush
/// – Currently an experiment
///
use super::*;
use crate::gui::FontStyle;

use glyph_brush::rusttype::{Font as RTFont, GlyphId, Scale};
use glyph_brush::{
    self, BrushAction, BrushError, GlyphBrush, GlyphBrushBuilder, GlyphCalculator, GlyphCalculatorBuilder,
    GlyphCruncher, HorizontalAlign as HAlign, Layout, Section, VerticalAlign as VAlign,
};
use image_rs::{imageops, DynamicImage, Rgba, RgbaImage};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color, GpuTriangle, Image, MeshTask, PixelFormat, Texture, Vertex},
};
use std::collections::HashMap;
use std::f32;

/// Utility for Theme struct for drawing live text using glyph_brush crate
pub struct DrawFont {
    /// Instance of GlyphBrush using the GLVertex struct as the wrapper for glyph vertices
    glyph_brush: GlyphBrush<'static, GLVertex>,
    /// Instance of the GlyphCalculator which calculates text size and position information
    glyph_calc: GlyphCalculator<'static>,
    /// The raw font
    raw_font: RTFont<'static>,
    /// The texture index in Quicksilver GL3 and WebGL backends
    index: usize,
    /// The cached MeshTask
    pub cached_mesh: Option<MeshTask>,
    /// A Hashmap storing the mapping of GlyphId to char. Used mainly for debugging?
    pub glyph_db: HashMap<GlyphId, char>,
    /// A Hashmap storing the mapping of a char to its width
    pub char_db: HashMap<char, f32>,
}

impl DrawFont {
    /// Constructor using bytes from Truetype font
    pub fn from_bytes(data: Vec<u8>, tex_id: Option<&usize>) -> Self {
        let raw_font = RTFont::from_bytes(data).unwrap();
        let glyph_calc = GlyphCalculatorBuilder::using_font(raw_font.clone()).build();
        let glyph_brush = GlyphBrushBuilder::using_font(raw_font.clone()).build();

        let (width, height) = glyph_brush.texture_dimensions();
        let mut draw = DrawFont {
            glyph_brush,
            glyph_calc,
            raw_font,
            index: 0,
            cached_mesh: None,
            glyph_db: HashMap::new(),
            char_db: HashMap::new(),
        };
        if let Some(index) = tex_id {
            log::error!("Re-using tex: {:?}", index);
            draw.index = index.clone();
        } else {
            draw.setup_gpu(width, height);
        }
        draw
    }

    /// Get the texture index
    pub fn get_index(&self) -> usize {
        self.index
    }

    /// Initialize GPU
    pub fn setup_gpu(&mut self, width: u32, height: u32) {
        let mut texture = Texture::new("DrawFont").with_shaders(VERTEX_SHADER, FRAGMENT_SHADER).with_fields(
            TEX_FIELDS,
            self::serialize_vertex,
            "outColor",
            "font_tex",
        );
        // wth is this?
        let result = texture.activate();
        if result.is_err() {
            log::error!("activate: {:?}", result);
        } else {
            let idx = result.unwrap();

            let result = texture.upload(idx, &[], width, height, PixelFormat::RGBA);
            if result.is_err() {
                log::error!("activate, upload: {:?}", result);
            }
            self.index = idx;
        }
    }

    /// Draw word-wrapped text in the given rect using glyph_brush
    pub fn draw(&mut self, params: TextParams) -> Option<MeshTask> {
        let rect = params.frame;
        let mut origin: (f32, f32) = (params.frame.x(), params.frame.y());
        let h_align = params.text_align.to_glyph_align();
        let v_align = params.vert_align.to_glyph_align();
        let layout = {
            if params.multiline {
                Layout::default_wrap().h_align(h_align)
            } else {
                origin.0 = match h_align {
                    HAlign::Left => rect.x(),
                    HAlign::Center => rect.x() + rect.width() / 2.0,
                    HAlign::Right => rect.x() + rect.width(),
                };
                origin.1 = match v_align {
                    VAlign::Top => rect.y(),
                    VAlign::Center => rect.y() + rect.height() / 2.0,
                    VAlign::Bottom => rect.y() + rect.height(),
                };
                Layout::default_single_line().v_align(v_align).h_align(h_align)
            }
        };
        let color = params.style.get_color();
        let section = Section {
            layout,
            bounds: (params.frame.width(), params.frame.height()),
            screen_position: origin,
            scale: Scale::uniform(params.style.get_size()),
            text: &params.text,
            color: [color.r, color.g, color.b, color.a],
            ..Section::default()
        };
        let index = self.index;

        self.glyph_brush.queue(&section);

        let mut brush_action;
        loop {
            brush_action = self.glyph_brush.process_queued(
                |rect, data| {
                    // Update part of gpu texture with new glyph alpha values.
                    let sub_rect = Rectangle::new((rect.min.x, rect.min.y), (rect.width(), rect.height()));
                    let rgba_data: Vec<[u8; 4]> = data.iter().map(|c| [255, 255, 255, *c]).collect();
                    let flattened: Vec<u8> = rgba_data.iter().flat_map(|s| s.to_vec()).collect();
                    let _ = Texture::update(index, &flattened.as_slice(), &sub_rect, PixelFormat::RGBA);
                    // window.update_texture(index, &flattened.as_slice(), &sub_rect, PixelFormat::RGBA);
                },
                to_vertex, // See function defined below
            );

            match brush_action {
                Ok(_) => {
                    break;
                }
                Err(BrushError::TextureTooSmall { suggested, .. }) => {
                    let (new_width, new_height) = suggested;
                    log::warn!("Resizing glyph texture -> {}x{}", new_width, new_height);
                    // FIXME: This needs to work

                    // let _ = window.create_texture(&[], new_width as u32, new_height as u32, PixelFormat::Alpha).unwrap();
                    // self.glyph_brush.resize_texture(new_width, new_height);
                }
            }
        }
        match brush_action.unwrap() {
            // The Draw(vertices) enum contains the aggregate output of the to_vertex function.
            BrushAction::Draw(vertices) => {
                // if params.debug {
                //     log::debug!("vertices count={:?} y={:?}", vertices.len(), 0);
                // }
                let mut task = MeshTask::new(index);

                for (i, ref mut glv) in vertices.into_iter().enumerate() {
                    if params.debug == true {
                        log::trace!("{} frame={:?} tex_frame={:?}", params.text, glv.frame, glv.tex_frame);
                    }

                    let color = Color { r: glv.color[0], g: glv.color[1], b: glv.color[2], a: glv.color[3] };
                    let offset = i as u32 * 4;

                    if let Some(subframe) = params.subframe {
                        self.clip_vertex(glv, &subframe, GlyphSide::Left);
                        self.clip_vertex(glv, &subframe, GlyphSide::Top);
                        self.clip_vertex(glv, &subframe, GlyphSide::Right);
                        self.clip_vertex(glv, &subframe, GlyphSide::Bottom);
                    }

                    let v = self.make_vertex(VertexPoint::TopLeft, &glv, color);
                    if params.subframe.is_some() {
                        log::trace!("**TopLeft={:?}", v);
                    }
                    task.vertices.push(v);

                    // top right
                    let v = self.make_vertex(VertexPoint::TopRight, &glv, color);
                    if params.subframe.is_some() {
                        log::trace!("**TopRight={:?}", v);
                    }
                    task.vertices.push(v);

                    // bottom right
                    let v = self.make_vertex(VertexPoint::BottomRight, &glv, color);
                    if params.subframe.is_some() {
                        log::trace!("**BottomRight={:?}", v);
                    }
                    task.vertices.push(v);

                    // bottom left
                    let v = self.make_vertex(VertexPoint::BottomLeft, &glv, color);
                    if params.subframe.is_some() {
                        log::trace!("**BottomLeft={:?}", v);
                    }
                    task.vertices.push(v);

                    // Add triangles based on clockwise insertion of vertices from top-left
                    task.triangles.push(GpuTriangle::new(offset, [0, 1, 2], 9, Col(Color::WHITE)));
                    task.triangles.push(GpuTriangle::new(offset, [2, 3, 0], 9, Col(Color::WHITE)));
                }
                self.cached_mesh = Some(task.clone());
                Some(task)
            }
            BrushAction::ReDraw => self.cached_mesh.clone(),
        }
    }

    fn make_vertex(&self, position: VertexPoint, glvertex: &GLVertex, color: Color) -> Vertex {
        let vector = match position {
            VertexPoint::TopLeft => Vector::new(glvertex.frame.min.x as f32, glvertex.frame.max.y as f32),
            VertexPoint::TopRight => Vector::new(glvertex.frame.max.x as f32, glvertex.frame.max.y as f32),
            VertexPoint::BottomRight => Vector::new(glvertex.frame.max.x as f32, glvertex.frame.min.y as f32),
            VertexPoint::BottomLeft => Vector::new(glvertex.frame.min.x as f32, glvertex.frame.min.y as f32),
        };
        let tex_vector = match position {
            VertexPoint::TopLeft => Vector::new(glvertex.tex_frame.min.x, glvertex.tex_frame.max.y),
            VertexPoint::TopRight => Vector::new(glvertex.tex_frame.max.x, glvertex.tex_frame.max.y),
            VertexPoint::BottomRight => Vector::new(glvertex.tex_frame.max.x, glvertex.tex_frame.min.y),
            VertexPoint::BottomLeft => Vector::new(glvertex.tex_frame.min.x, glvertex.tex_frame.min.y),
        };
        Vertex::new(vector, Some(tex_vector), Col(color))
    }

    fn clip_vertex(&self, glv: &mut GLVertex, subframe: &Rectangle, side: GlyphSide) {
        match side {
            GlyphSide::Left => {
                let delta = subframe.pos.x - glv.frame.min.x as f32;
                if delta > 0.0 {
                    let ratio = delta / (glv.frame.max.x as f32 - glv.frame.min.x as f32);
                    log::trace!("{:?} delta={:?} ratio={:?}", side, delta, ratio);
                    glv.frame.min.x = subframe.pos.x as i32;
                    glv.tex_frame.min.x = glv.tex_frame.min.x + glv.tex_frame.width() * ratio;
                }
            }
            GlyphSide::Top => {
                let delta = subframe.pos.y - glv.frame.min.y as f32;
                if delta > 0.0 {
                    let ratio = delta / (glv.frame.max.y as f32 - glv.frame.min.y as f32);
                    log::trace!("{:?} delta={:?} ratio={:?}", side, delta, ratio);
                    glv.frame.min.y = subframe.pos.y as i32;
                    glv.tex_frame.min.y = glv.tex_frame.min.y + glv.tex_frame.height() * ratio;
                }
            }
            GlyphSide::Right => {
                let delta = glv.frame.max.x as f32 - (subframe.pos.x + subframe.width());
                if delta > 0.0 {
                    let ratio = delta / (glv.frame.max.x as f32 - glv.frame.min.x as f32);
                    log::trace!("{:?} delta={:?} ratio={:?}", side, delta, ratio);
                    glv.frame.max.x = subframe.pos.x as i32 + subframe.width() as i32;
                    glv.tex_frame.max.x = glv.tex_frame.max.x - glv.tex_frame.width() * ratio;
                }
            }
            GlyphSide::Bottom => {
                let delta = glv.frame.max.y as f32 - (subframe.pos.y + subframe.height());

                if delta > 0.0 {
                    let ratio = delta / (glv.frame.max.y as f32 - glv.frame.min.y as f32);
                    log::trace!("{:?} delta={:?} ratio={:?}", side, delta, ratio);
                    glv.frame.max.y = subframe.pos.y as i32 + subframe.height() as i32;
                    glv.tex_frame.max.y = glv.tex_frame.max.y - glv.tex_frame.height() * ratio;
                }
            }
        }
    }

    /// Render text to image buffer
    pub fn render_pixels(&mut self, params: TextParams) -> (RgbaImage, u32, u32) {
        let h_align = params.text_align.to_glyph_align();
        let _v_align = params.vert_align.to_glyph_align();

        let layout = {
            if params.multiline {
                Layout::default_wrap().h_align(h_align)
            } else {
                Layout::default_single_line()
            }
        };
        let color = params.style.get_color();
        let section = Section {
            layout,
            bounds: (params.frame.width(), f32::INFINITY),
            scale: Scale::uniform(params.style.get_size()),
            text: &params.text,
            color: [color.r, color.g, color.b, color.a],
            ..Section::default()
        };

        let mut glyph_calc = self.glyph_calc.cache_scope();
        let text_size: (f32, f32) = {
            if let Some(rect) = glyph_calc.glyph_bounds_custom_layout(&section, &layout) {
                (rect.width().round(), rect.height().round())
            } else {
                // This is the old calculation method that was too small and had to be buffered
                let pixel_bounds = glyph_calc.pixel_bounds(&section).expect("None bounds");
                (pixel_bounds.width() as f32, pixel_bounds.height() as f32)
            }
        };
        let buf_size = (text_size.0 as u32, text_size.1 as u32);
        let glyphs = glyph_calc.glyphs(&section);
        let mut imgbuf = DynamicImage::new_rgba8(buf_size.0, buf_size.1).to_rgba();
        let color = params.style.get_color();
        let red = (255.0 * color.r) as u8;
        let green = (255.0 * color.g) as u8;
        let blue = (255.0 * color.b) as u8;

        // Loop through the glyphs in the text, positioning each one on a line
        for glyph in glyphs {
            // TODO: try exact_bounding_box
            if let Some(bounds) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                if bounds.min.x < 0
                    || bounds.min.y < 0
                    || bounds.max.x > text_size.0 as i32
                    || bounds.max.y > text_size.1 as i32
                {
                    log::error!("Glyph out of bounds {:?}", bounds);
                    continue;
                }
                // log::trace!("render_pixel for glyph id={:?} bounds={:?}", glyph.id(), bounds);
                glyph.draw(|x, y, v| {
                    let alpha = (255.0 * v) as u8;
                    // Offset the position by the glyph bounding box
                    let x = x + bounds.min.x as u32;
                    let y = y + bounds.min.y as u32;
                    imgbuf.put_pixel(x, y, Rgba { 0: [red, green, blue, alpha] })
                });
            }
        }

        (imgbuf, text_size.0 as u32, text_size.1 as u32)
    }

    /// Given the provided TextParams, call render_pixels and convert to QS Image
    pub fn render_image(&mut self, params: TextParams) -> Option<Image> {
        // let height = params.style.get_size() as u32;
        let (mut imgbuf, width, height) = self.render_pixels(params);
        log::debug!(">>> render_pixels size w={:?} h={:?}", width, height);
        let subimg = imageops::crop(&mut imgbuf, 0, 0, width, height);
        let img: Image =
            Image::from_raw(subimg.to_image().into_raw().as_slice(), width, height, PixelFormat::RGBA).unwrap();

        return Some(img);
    }

    /// Render the text as an image with no cropping
    pub fn render(&mut self, text: &str, style: &FontStyle, rect: &Rectangle, multiline: bool) -> Option<Image> {
        let params = TextParams::new(style.clone()).frame(rect.clone()).text(text).multiline(multiline);

        let (mut imgbuf, width, height) = self.render_pixels(params);
        // log::debug!(">>> render_pixels size w={:?} h={:?}", width, height);
        let subimg = imageops::crop(&mut imgbuf, 0, 0, width, height);
        let img: Image =
            Image::from_raw(subimg.to_image().into_raw().as_slice(), width, height, PixelFormat::RGBA).unwrap();

        return Some(img);
    }

    // ************************************************************************************
    // Helper methods used by EditorContext and various others
    // ************************************************************************************

    pub fn get_raw_font(&self) -> &RTFont<'static> {
        &self.raw_font
    }

    pub fn glyph_calc(&self) -> &GlyphCalculator<'static> {
        &self.glyph_calc
    }

    pub fn char_size(&self, c: char, font_size: f32) -> (f32, f32) {
        let scale = Scale::uniform(font_size);
        let w = self.raw_font.glyph(c).scaled(scale).h_metrics().advance_width;
        let v_metrics = self.raw_font.v_metrics(scale);
        let h = (v_metrics.ascent - v_metrics.descent).ceil();
        (w, h)
    }

    pub fn measure_text(&self, text: &str, font_size: f32) -> (f32, f32) {
        let mut glyph_calc = self.glyph_calc.cache_scope();
        let layout = Layout::default();
        let scale = Scale::uniform(font_size);
        let section = Section { layout, scale, text, ..Section::default() };

        let text_size: (f32, f32) = {
            if let Some(rect) = glyph_calc.glyph_bounds_custom_layout(&section, &layout) {
                (rect.width() as f32, rect.height() as f32)
            } else {
                (0.0, 0.0)
            }
        };
        text_size
    }
}

// impl NotifyDispatcher for DrawFont {
//     type Update = MeshTask;
//     type Params = TextParams;      // Not in use yet, but possibly use this to request PropSet for specific time in seconds

// }

// ************************************************************************************
// Support
// ************************************************************************************

/// Use this as a parameter for draw method
#[derive(Clone, Debug)]
pub struct TextParams {
    /// The font style to use
    pub style: FontStyle,
    /// The absolute-positioned Rectangle where text is drawn
    pub frame: Rectangle,
    /// An optional Rectangle that is used for clipping text
    pub subframe: Option<Rectangle>,
    /// The text to draw
    pub text: String,
    /// The horizontal alignment
    pub text_align: TextAlign,
    /// The vertical alignment
    pub vert_align: VertAlign,
    /// Whether the text should wrap
    pub multiline: bool,
    /// Debug flag which can be used for debugging a specific text object
    pub debug: bool,
}

impl TextParams {
    pub fn new(style: FontStyle) -> Self {
        TextParams {
            style,
            frame: Rectangle::new_sized(Vector::ONE),
            subframe: None,
            text: String::default(),
            text_align: TextAlign::Left,
            vert_align: VertAlign::Middle,
            multiline: false,
            debug: false,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn frame(mut self, frame: Rectangle) -> Self {
        self.frame = frame;
        self
    }

    pub fn align(mut self, horizontal: TextAlign, vertical: VertAlign) -> Self {
        self.text_align = horizontal;
        self.vert_align = vertical;
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }
}

/// Enum for Horizontal Alignment
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl TextAlign {
    pub fn to_glyph_align(&self) -> HAlign {
        match self {
            TextAlign::Left => HAlign::Left,
            TextAlign::Center => HAlign::Center,
            TextAlign::Right => HAlign::Right,
        }
    }
}

/// Enum for Vertical Alignment
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VertAlign {
    Top,
    Middle,
    Bottom,
}

impl VertAlign {
    pub fn to_glyph_align(&self) -> VAlign {
        match self {
            VertAlign::Top => VAlign::Top,
            VertAlign::Middle => VAlign::Center,
            VertAlign::Bottom => VAlign::Bottom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum VertexPoint {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GlyphSide {
    Left,
    Top,
    Right,
    Bottom,
}

// ************************************************************************************
// ************************************************************************************

#[cfg(not(target_arch = "wasm32"))]
const VERTEX_SHADER: &str = r#"#version 150
in vec2 position;
in vec2 tex_coord;
in vec4 color;
in float uses_texture;
out vec4 Color;
out vec2 Tex_coord;
out float Uses_texture;
void main() {
    Color = color;
    Tex_coord = tex_coord;
    Uses_texture = uses_texture;
    gl_Position = vec4(position, 0, 1);
}"#;
#[cfg(not(target_arch = "wasm32"))]
const FRAGMENT_SHADER: &str = r#"#version 150
uniform sampler2D font_tex;
in vec4 Color;
in vec2 Tex_coord;
in float Uses_texture;
out vec4 outColor;
void main() {
    if (Uses_texture != 0) {
        float alpha = texture(font_tex, Tex_coord).a;
        outColor = Color * vec4(1.0, 1.0, 1.0, alpha);
    } else {
        float alpha = texture(font_tex, Tex_coord).a;
        outColor = Color;
    }
}"#;

#[cfg(target_arch = "wasm32")]
const VERTEX_SHADER: &str = r#"attribute vec2 position;
attribute vec2 tex_coord;
attribute vec4 color;
attribute lowp float uses_texture;
varying vec2 Tex_coord;
varying vec4 Color;
varying lowp float Uses_texture;
void main() {
    gl_Position = vec4(position, 0, 1);
    Tex_coord = tex_coord;
    Color = color;
    Uses_texture = uses_texture;
}"#;
#[cfg(target_arch = "wasm32")]
const FRAGMENT_SHADER: &str = r#"varying highp vec4 Color;
varying highp vec2 Tex_coord;
varying lowp float Uses_texture;
uniform sampler2D font_tex;
void main() {
    highp vec4 tex_color = (int(Uses_texture) != 0) ? texture2D(font_tex, Tex_coord) : vec4(1, 1, 1, 1);
    gl_FragColor = Color * tex_color;
}"#;

const TEX_FIELDS: &[(&str, u32)] = &[("position", 2), ("tex_coord", 2), ("color", 4), ("uses_texture", 1)];
