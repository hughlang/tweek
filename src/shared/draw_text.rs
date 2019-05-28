/// A helper for drawing glyph text using the GPU via glyph_brush
/// – Currently an experiment
///
use glyph_brush::rusttype::{Rect, Scale};

#[allow(unused_imports)]
use glyph_brush::{
    self, BrushAction, BrushError, DefaultSectionHasher, FontId, GlyphBrush, GlyphBrushBuilder, GlyphCalculator,
    GlyphCalculatorBuilder, GlyphCruncher, GlyphPositioner, HorizontalAlign as HAlign, Layout, Section,
    SectionGeometry, SectionText, VariedSection, VerticalAlign as VAlign,
};

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{
        Background::Col, Background::Img, Color, DrawTask, FontStyle, GpuTriangle, Image, Mesh, PixelFormat, Texture,
        Vertex,
    },
    lifecycle::{Settings, State, Window},
    Result,
};

pub struct DrawText {
    glyph_brush: GlyphBrush<'static, GLVertex>,
    texture: Option<Texture>,
    texture_idx: usize,
    multiline: bool,
}

impl DrawText {
    pub fn from_bytes(data: &'static [u8]) -> Self {
        let glyph_brush: GlyphBrush<'static, GLVertex> = GlyphBrushBuilder::using_font_bytes(data).build();
        let mut gpu_text = DrawText { glyph_brush, texture: None, texture_idx: 0, multiline: false };
        gpu_text.setup_gpu();
        gpu_text.activate();
        gpu_text
    }

    pub fn setup_gpu(&mut self) {
        let texture = Texture::default().with_shaders(VERTEX_SHADER, FRAGMENT_SHADER).with_fields(
            TEX_FIELDS,
            self::serialize_vertex,
            "outColor",
            "font_tex",
        );

        self.texture = Some(texture);
    }

    pub fn set_multiline(&mut self, is_multiline: bool) {
        self.multiline = is_multiline;
    }

    pub fn activate(&mut self) {
        log::debug!("==Activate=======================================================");
        let (width, height) = self.glyph_brush.texture_dimensions();

        if let Some(texture) = &mut self.texture {
            let result = texture.activate();
            if result.is_err() {
                log::error!("activate: {:?}", result);
            } else {
                let idx = result.unwrap();

                let result = texture.upload(idx, &[], width, height, PixelFormat::RGBA);
                if result.is_err() {
                    log::error!("activate, upload: {:?}", result);
                }
                self.texture_idx = idx;
            }
        }
    }

    /// Draw word-wrapped text in the given rect using glyph_brush
    pub fn draw(
        &mut self,
        text: &str,
        style: &FontStyle,
        h_align: HAlign,
        rect: &Rectangle,
        screen: Vector,
    ) -> Result<DrawTask> {
        if self.texture.is_none() {
            self.setup_gpu();
            self.activate();
        }

        let origin: (f32, f32);
        let layout = {
            if self.multiline {
                origin = (rect.x(), rect.y());
                Layout::default().h_align(h_align)
            } else {
                origin = (rect.x(), rect.y() + rect.height() / 2.0);
                Layout::default_single_line().v_align(VAlign::Center).h_align(h_align)
            }
        };
        let color = style.get_color();
        let section = Section {
            layout: layout,
            bounds: (rect.width(), rect.height()),
            screen_position: origin,
            scale: Scale::uniform(style.get_size()),
            text: &text,
            color: [color.r, color.g, color.b, color.a],
            ..Section::default()
        };
        let texture_idx = self.texture_idx;
        let mut task = DrawTask::new(texture_idx);

        self.glyph_brush.queue(&section);
        let mut brush_action;
        loop {
            brush_action = self.glyph_brush.process_queued(
                (screen.x as _, screen.y as _),
                |rect, data| {
                    // eprintln!("Update texture={:?} len={}", rect, data.len());
                    // Update part of gpu texture with new glyph alpha values.
                    let sub_rect = Rectangle::new((rect.min.x, rect.min.y), (rect.width(), rect.height()));
                    let rgba_data: Vec<[u8; 4]> = data.iter().map(|c| [255, 255, 255, *c]).collect();
                    let flattened: Vec<u8> = rgba_data.iter().flat_map(|s| s.to_vec()).collect();
                    let _ = Texture::update(texture_idx, &flattened.as_slice(), &sub_rect, PixelFormat::RGBA);
                },
                to_vertex,
            );

            match brush_action {
                Ok(_) => break,
                Err(BrushError::TextureTooSmall { suggested, .. }) => {
                    let (new_width, new_height) = suggested;
                    log::debug!("Resizing glyph texture -> {}x{}", new_width, new_height);
                    // let _ = window.create_texture(&[], new_width as u32, new_height as u32, PixelFormat::Alpha).unwrap();
                    // self.glyph_brush.resize_texture(new_width, new_height);
                }
            }
        }
        match brush_action.unwrap() {
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
                }
            }
            BrushAction::ReDraw => {}
        }
        Ok(task)
    }
}

#[derive(Clone, Debug)]
struct GLVertex {
    frame: Rect<i32>,
    tex_frame: Rect<f32>,
    color: [f32; 4],
}

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
