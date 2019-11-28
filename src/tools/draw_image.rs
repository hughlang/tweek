/// A helper for drawing images using bitmap data and/or text
///
///
use super::*;

use image_rs::DynamicImage;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{
        Background::{self, Col},
        Color, GpuTriangle, MeshTask, PixelFormat, Texture, Vertex,
    },
    lifecycle::Asset,
    load_file,
};

use std::path::Path;

pub struct GPUTexture {
    pub idx: usize,
    pub width: u32,
    pub height: u32,
}

impl GPUTexture {
    pub fn new(idx: usize, width: u32, height: u32) -> Self {
        GPUTexture { idx, width, height }
    }
}

//-- Main -----------------------------------------------------------------------

// #[derive(Debug, Default)]
pub struct DrawImage {}

impl DrawImage {
    /// Doesn't work in wasm
    pub fn load_file_bytes(path: &'static str) -> Vec<u8> {
        let mut asset = Asset::new(load_file(path));
        let mut bytes: Vec<u8> = Vec::new();
        let _ = asset.execute(|data| {
            bytes.append(data);
            Ok(())
        });
        bytes
    }

    /// Helper method to load an image from a relative file path.
    /// Unused
    pub fn load_image_file(path: &str) -> Option<DynamicImage> {
        let im = image::open(&Path::new(path));
        if im.is_err() {
            log::error!("FAILED TO LOAD FILE");
            return None;
        } else {
            Some(im.unwrap())
        }
    }

    /// Upload bytes to GPU, which returns a Texture index number from the backend
    /// If bytes array is empty, the Texture is created anyway with the expectation of writing to it later.
    pub fn upload_image(name: &str, data: &[u8], width: u32, height: u32) -> Option<usize> {
        let mut texture = Texture::new(name).with_shaders(VERTEX_SHADER, FRAGMENT_SHADER).with_fields(
            TEX_FIELDS,
            serialize_vertex,
            "outColor",
            "font_tex",
        );
        let result = texture.activate();
        if result.is_err() {
            log::error!("activate: {:?}", result);
        } else {
            let idx = result.unwrap();

            let result = texture.upload(idx, data, width, height, PixelFormat::RGBA);
            if result.is_err() {
                log::error!("activate, upload: {:?}", result);
                return None;
            }
            return Some(idx);
        }
        None
    }

    /// Calculate the 0..1 points that make up the four corners of a texture mapping
    pub fn normalize_tex_quad(texture_size: Vector, region: Rectangle) -> [Vector; 4] {
        let scale = texture_size.recip();
        let normalized_size = region.size().times(scale);
        let pt0 = region.pos.times(scale);
        let pt1 = pt0 + normalized_size.x_comp();
        let pt2 = pt0 + normalized_size;
        let pt3 = pt0 + normalized_size.y_comp();
        let points: [Vector; 4] = [pt0, pt1, pt2, pt3];
        points
    }

    /// Generate a Mesh from a GPU texture and the specified region to display
    pub fn sub_texture(idx: usize, frame: Rectangle, tex_quad: Option<[Vector; 4]>, color: Color) -> Option<MeshTask> {
        let mut task = MeshTask::new(idx);

        let offset = 0;

        // TODO: change this to unwrap_or
        let tex_quad = {
            if let Some(quad) = tex_quad {
                quad
            } else {
                [Vector::ZERO, Vector::X, Vector::ONE, Vector::Y]
            }
        };

        // top left
        let v = Vertex::new(frame.top_left(), Some(tex_quad[0]), Col(color));
        task.vertices.push(v);

        // top right
        let v = Vertex::new(Vector::new(frame.x() + frame.width(), frame.y()), Some(tex_quad[1]), Col(color));
        task.vertices.push(v);

        // bottom right
        let v = Vertex::new(
            Vector::new(frame.x() + frame.width(), frame.y() + frame.height()),
            Some(tex_quad[2]),
            Col(color),
        );
        task.vertices.push(v);

        // bottom left
        let v = Vertex::new(Vector::new(frame.x(), frame.y() + frame.height()), Some(tex_quad[3]), Col(color));
        task.vertices.push(v);

        // Add triangles based on clockwise insertion of vertices from top-left
        task.triangles.push(GpuTriangle::new(offset, [0, 1, 2], 9, Col(Color::WHITE)));
        task.triangles.push(GpuTriangle::new(offset, [2, 3, 0], 9, Col(Color::WHITE)));

        Some(task)
    }

    /// Create a MeshTask using the specified Texture index
    pub fn draw_texture(idx: usize, frame: &Rectangle, bkg: Background) -> Option<MeshTask> {
        let mut task = MeshTask::new(idx);

        let offset = 0;

        // top left
        let v = Vertex::new(frame.pos, Some(Vector::ZERO), bkg);
        task.vertices.push(v);

        // top right
        let v = Vertex::new(Vector::new(frame.x() + frame.width(), frame.y()), Some(Vector::X), bkg);
        task.vertices.push(v);

        // bottom right
        let v = Vertex::new(Vector::new(frame.x() + frame.width(), frame.y() + frame.height()), Some(Vector::ONE), bkg);
        task.vertices.push(v);

        // bottom left
        let v = Vertex::new(Vector::new(frame.x(), frame.y() + frame.height()), Some(Vector::Y), bkg);
        task.vertices.push(v);

        // Add triangles based on clockwise insertion of vertices from top-left
        task.triangles.push(GpuTriangle::new(offset, [0, 1, 2], 9, Col(Color::BLACK)));
        task.triangles.push(GpuTriangle::new(offset, [2, 3, 0], 9, Col(Color::BLACK)));

        Some(task)
    }
}

// ************************************************************************************
// GL scripts
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
    vec4 tex_color = (Uses_texture != 0) ? texture(font_tex, Tex_coord) : vec4(1, 1, 1, 1);
    outColor = Color * tex_color;
}"#;

// if (Uses_texture != 0) {
//     float alpha = texture(font_tex, Tex_coord).a;
//     outColor = Color * vec4(1.0, 1.0, 1.0, alpha);
// } else {
//     float alpha = texture(font_tex, Tex_coord).a;
//     outColor = Color;
// }

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
