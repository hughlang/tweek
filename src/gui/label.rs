/// The Label view displays text content rendered as image text
///
use super::*;
use crate::core::*;
use crate::events::*;
use crate::tools::TextParams;

// use image_rs::{imageops, DynamicImage, GenericImageView};
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::Img, GpuTriangle, MeshTask},
    lifecycle::Window,
};

use std::any::TypeId;

//-- Label -----------------------------------------------------------------------

/// Tweenable text label
pub struct Label {
    /// The base layer
    pub layer: Layer,
    pub multiline: bool,
    text: Option<String>,
    size: Vector,
}

impl Label {
    /// Constructor with the text string
    pub fn new(frame: Rectangle) -> Self {
        let size = frame.size;
        let layer = Layer::new(frame);
        Label { layer, multiline: false, text: None, size }
    }

    /// Set the text string if changed from initial value
    pub fn set_text(&mut self, text: &str) {
        self.text = Some(text.to_owned());
        self.layer.meshes.clear();
    }

    /// Method to render contents of this Label as an Image
    fn draw_content(&mut self, theme: &mut Theme) -> Option<MeshTask> {
        // Calculate the relative frames for the image and text content
        let mut frame = self.layer.frame;
        if let Some(text) = &self.text {
            let params =
                TextParams::new(self.layer.font_style).text(text).frame(frame.clone()).multiline(self.multiline);

            if let Some(image) = theme.default_font.render_image(params) {
                frame.pos.x = frame.pos.x + (frame.width() - image.area().width()) / 2.0;
                frame.pos.y = frame.pos.y + (frame.height() - image.area().height()) / 2.0;
                frame.size = image.area().size;
                self.size = frame.size;

                let mut mesh = MeshTask::new(0);
                let bkg = Img(&image);

                let trans = Transform::translate(frame.top_left() + frame.size() / 2)
                    * Transform::translate(-frame.size() / 2)
                    * Transform::scale(frame.size());
                let tex_trans = bkg.image().map(|img| img.projection(Rectangle::new_sized((1, 1))));
                let offset = mesh.add_positioned_vertices(
                    [Vector::ZERO, Vector::X, Vector::ONE, Vector::Y].iter().cloned(),
                    trans,
                    tex_trans,
                    bkg,
                );

                mesh.triangles.push(GpuTriangle::new(offset, [0, 1, 2], 9, bkg));
                mesh.triangles.push(GpuTriangle::new(offset, [2, 3, 0], 9, bkg));

                return Some(mesh);
            }
        }
        None
    }
}

impl Displayable for Label {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Label>()
    }

    fn get_layer(&self) -> &Layer {
        &self.layer
    }

    fn get_layer_mut(&mut self) -> &mut Layer {
        &mut self.layer
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn get_content_size(&self) -> Vector {
        self.size
    }

    fn move_to(&mut self, pos: (f32, f32)) {
        self.layer.frame.pos.x = pos.0;
        self.layer.frame.pos.y = pos.1;
    }

    fn set_theme(&mut self, theme: &mut Theme) {
        let ok = self.layer.apply_theme(theme);
        if !ok {
            return;
        }
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Ready => {
                self.layer.on_ready();
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
            }
            _ => {}
        }
    }

    fn update(&mut self, _window: &mut Window, state: &mut AppState) {
        self.layer.tween_update(state);
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        let buffers = self.layer.prepare_render(window);
        if self.layer.debug {
            // log::trace!("{:?} meshes={:?}", self.debug_id(), meshes.len());
        }
        if buffers.len() > 0 {
            for task in buffers.into_iter() {
                window.add_task(task);
            }
        } else {
            if let Some(task) = self.draw_content(theme) {
                self.layer.meshes.clear();
                window.add_task(task.clone());
                self.layer.meshes.push(task);
            }
        }
    }
}
