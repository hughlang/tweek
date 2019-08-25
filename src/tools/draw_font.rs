/// A helper for drawing glyph text using the GPU via glyph_brush
/// – Currently an experiment
///

use glyph_brush::{
    self, BrushAction, BrushError, GlyphBrush, GlyphBrushBuilder, GlyphCalculator,
    GlyphCalculatorBuilder, GlyphCruncher, HorizontalAlign as HAlign, Layout, Section,
    SectionText, VariedSection, VerticalAlign as VAlign,
};
use glyph_brush::rusttype::{Rect, Font as RTFont, PositionedGlyph, Scale, point};
use image::{imageops, DynamicImage, ImageBuffer, Rgba};

// #[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{
        Background::Col, Color, MeshTask, FontStyle, GpuTriangle, Image, PixelFormat, Texture,
        Vertex,
    },
    lifecycle::{Window},
};
use std::f32;

/// Utility for Theme struct for drawing live text using glyph_brush crate
// #[derive(Clone, Debug)]
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
    cached_mesh: Option<MeshTask>,
}

impl DrawFont {

    /// Constructor using bytes from Truetype font
    pub fn from_bytes(data: Vec<u8>, tex_id: Option<&usize>) -> Self {
        let raw_font = RTFont::from_bytes(data).unwrap();
        let glyph_calc = GlyphCalculatorBuilder::using_font(raw_font.clone()).build();
        let glyph_brush = GlyphBrushBuilder::using_font(raw_font.clone()).build();

        let (width, height) = glyph_brush.texture_dimensions();
        let mut draw = DrawFont { glyph_brush, glyph_calc, raw_font, index: 0, cached_mesh: None };
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
        log::debug!(">>> setup_gpu =================================");

        let mut texture = Texture::new("DrawFont").with_shaders(VERTEX_SHADER, FRAGMENT_SHADER).with_fields(
            TEX_FIELDS,
            self::serialize_vertex,
            "outColor",
            "font_tex",
        );
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
    pub fn draw(
        &mut self,
        text: &str,
        style: &FontStyle,
        h_align: HAlign,
        rect: &Rectangle,
        window: &mut Window,
        multiline: bool
    ) -> Option<MeshTask> {
        // let screen = window.screen_size();
        let origin: (f32, f32);
        let layout = {
            if multiline {
                origin = (rect.x(), rect.y());
                Layout::default_wrap().h_align(h_align)
            } else {
                origin = match h_align {
                    HAlign::Left => (rect.x(), rect.y() + rect.height() / 2.0),
                    HAlign::Center => (rect.x() + rect.width() / 2.0, rect.y() + rect.height() / 2.0),
                    HAlign::Right => (rect.x() + rect.width(), rect.y() + rect.height() / 2.0),
                };
                Layout::default_single_line().v_align(VAlign::Center).h_align(h_align)
            }
        };
        let color = style.get_color();
        let section = Section {
            layout,
            bounds: (rect.width(), rect.height()),
            screen_position: origin,
            scale: Scale::uniform(style.get_size()),
            text: &text,
            color: [color.r, color.g, color.b, color.a],
            ..Section::default()
        };
        let index = self.index;
        let mut task = MeshTask::new(index);

        self.glyph_brush.queue(&section);
        // let tex_index = self.index;
        let text_size = self.glyph_brush.texture_dimensions();
        task.content_size = (text_size.0 as f32, text_size.1 as f32);

        let mut brush_action;
        loop {
            brush_action = self.glyph_brush.process_queued(
                |rect, data| {
                    // log::debug!("{}/ Update texture={:?}", tex_index, rect);
                    // Update part of gpu texture with new glyph alpha values.
                    let sub_rect = Rectangle::new((rect.min.x, rect.min.y), (rect.width(), rect.height()));
                    let rgba_data: Vec<[u8; 4]> = data.iter().map(|c| [255, 255, 255, *c]).collect();
                    let flattened: Vec<u8> = rgba_data.iter().flat_map(|s| s.to_vec()).collect();
                    // let _ = Texture::update(index, &flattened.as_slice(), &sub_rect, PixelFormat::RGBA);
                    window.update_texture(index, &flattened.as_slice(), &sub_rect, PixelFormat::RGBA);
                },
                to_vertex, // See function defined below
            );

            match brush_action {
                Ok(_) => {
                    break;
                },
                Err(BrushError::TextureTooSmall { suggested, .. }) => {
                    let (new_width, new_height) = suggested;
                    log::debug!("Resizing glyph texture -> {}x{}", new_width, new_height);
                    // FIXME: This needs to work

                    // let _ = window.create_texture(&[], new_width as u32, new_height as u32, PixelFormat::Alpha).unwrap();
                    // self.glyph_brush.resize_texture(new_width, new_height);
                }
            }
        }
        match brush_action.unwrap() {
            // The Draw(vertices) enum contains the aggregate output of the to_vertex function.
            BrushAction::Draw(vertices) => {
                // log::debug!("vertices count={:?} y={:?}", vertices.len(), 0);

                for (i, glv) in vertices.iter().enumerate() {
                    // log::debug!("pix={:?} tex={:?}", glv.frame, glv.tex_frame);

                    let color = Color { r: glv.color[0], g: glv.color[1], b: glv.color[2], a: glv.color[3] };

                    let offset = i as u32 * 4;
                    // log::debug!("color={:?} offset={:?}", color, offset);
                    // top left
                    let v = Vertex::new(
                        Vector::new(glv.frame.min.x as f32, glv.frame.max.y as f32),
                        Some(Vector::new(glv.tex_frame.min.x, glv.tex_frame.max.y)),
                        Col(color),
                    );
                    task.vertices.push(v);

                    // top right
                    let v = Vertex::new(
                        Vector::new(glv.frame.max.x as f32, glv.frame.max.y as f32),
                        Some(Vector::new(glv.tex_frame.max.x, glv.tex_frame.max.y)),
                        Col(color),
                    );
                    task.vertices.push(v);

                    // bottom right
                    let v = Vertex::new(
                        Vector::new(glv.frame.max.x as f32, glv.frame.min.y as f32),
                        Some(Vector::new(glv.tex_frame.max.x, glv.tex_frame.min.y)),
                        Col(color),
                    );
                    task.vertices.push(v);

                    // bottom left
                    let v = Vertex::new(
                        Vector::new(glv.frame.min.x as f32, glv.frame.min.y as f32),
                        Some(Vector::new(glv.tex_frame.min.x, glv.tex_frame.min.y)),
                        Col(color),
                    );
                    task.vertices.push(v);

                    // Add triangles based on clockwise insertion of vertices from top-left
                    task.triangles.push(GpuTriangle::new(offset, [0, 1, 2], 9, Col(Color::YELLOW)));
                    task.triangles.push(GpuTriangle::new(offset, [2, 3, 0], 9, Col(Color::YELLOW)));
                    self.cached_mesh = Some(task.clone());
                }
                Some(task)
            }
            BrushAction::ReDraw => {
                None
            }
        }
    }

    /// Render text to image buffer
    pub fn render_pixels(
        &mut self,
        text: &str,
        style: &FontStyle,
        rect: &Rectangle,
        multiline: bool
    ) -> (ImageBuffer<Rgba<u8>, Vec<u8>>, u32, u32) {

        let layout = {
            if multiline {
                Layout::default_wrap()
            } else {
                Layout::default_single_line()
            }
        };
        let varied = VariedSection {
            layout,
            bounds: (rect.width(), f32::INFINITY),
            text: vec![SectionText {
                text: &text,
                scale: Scale::uniform(style.get_size()),
                ..SectionText::default()
            }],
            ..VariedSection::default()
        };

        let mut glyph_calc = self.glyph_calc.cache_scope();
        let text_size: (u32, u32) = {
            if let Some(rect) = glyph_calc.glyph_bounds_custom_layout(&varied, &layout) {
                log::debug!(">>> New glyph_bounds_custom: {:?}", rect);
                let buffer = style.get_size() as u32;
                (rect.width().round() as u32 + buffer, rect.height().round() as u32 + buffer)
            } else {
                // This is the old calculation method that was too small and had to be buffered
                let pixel_bounds = glyph_calc.pixel_bounds(&varied).expect("None bounds");
                let buffer = style.get_size() as u32;
                (pixel_bounds.width() as u32 + buffer, pixel_bounds.height() as u32 + buffer)
            }
        };

        let glyphs = glyph_calc.glyphs(&varied);
        let mut imgbuf = DynamicImage::new_rgba8( text_size.0, text_size.1 ).to_rgba();
        let color = style.get_color();
        let red = (255.0 * color.r) as u8;
        let green = (255.0 * color.g) as u8;
        let blue = (255.0 * color.b) as u8;

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounds) = glyph.pixel_bounding_box() {
                log::trace!("id={:?} bounds={:?}", glyph.id(), bounds);
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    let alpha = (255.0 * v) as u8;
                    // Offset the position by the glyph bounding box
                    let x = x + bounds.min.x as u32;
                    let y = y + bounds.min.y as u32;
                    imgbuf.put_pixel(x, y, Rgba { data: [red, green, blue, alpha] })
                });
            }
        }

        (imgbuf, text_size.0, text_size.1)
    }

    /// Render the text as an image with no cropping
    pub fn render(&mut self,
        text: &str,
        style: &FontStyle,
        rect: &Rectangle,
        multiline: bool
   ) -> Option<Image> {

        let (mut imgbuf, width, height) = self.render_pixels(text, style, rect, multiline);
        log::debug!(">>> render_pixels size w={:?} h={:?}", width, height);
        let height = style.get_size() as u32;
        let subimg = imageops::crop(&mut imgbuf, 0, 0, width, height);
        let img: Image = Image::from_raw(subimg.to_image().into_raw().as_slice(), width, height, PixelFormat::RGBA).unwrap();

        return Some(img);
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

/// Wrapper struct to hold the output of to_vertex for every GlyphVertex input.
/// GLVertex also is used as the generic when creating a GlyphBrush instance.
#[derive(Clone, Debug)]
pub struct GLVertex {
    frame: Rect<i32>,
    tex_frame: Rect<f32>,
    color: [f32; 4],
}

/// This is the function that converts the GlyphVertex output from glyph_brush
/// and transforms it to a struct that is processed in
fn to_vertex(v: glyph_brush::GlyphVertex) -> GLVertex {
    GLVertex { frame: v.pixel_coords, tex_frame: v.tex_coords, color: v.color }
}

fn serialize_vertex(vertex: Vertex) -> Vec<f32> {
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
