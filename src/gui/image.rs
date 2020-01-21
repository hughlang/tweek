/// ImageView is a Tweenable object that displays an image
///
use crate::core::*;
use crate::events::*;

use image_rs::{DynamicImage, GenericImageView};

use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::Img, GpuTriangle, Image, Mesh, MeshTask, PixelFormat},
    lifecycle::{Asset, Window},
};

use std::any::TypeId;

use super::*;

//-- Image -----------------------------------------------------------------------

/// A wrapper object for displaying an image.
/// Note: the name ImageView uses the "View" suffix to avoid confusion with other
/// Image structs.
pub struct ImageView {
    /// The base layer
    pub layer: Layer,
    /// Optional bytes container as alternative
    loader: Option<Asset<Vec<u8>>>,
    /// Store the calculated original image size
    image_size: Vector,
    /// flag to determine how image is scaled
    pub custom_size: Option<Vector>,
}

impl ImageView {
    /// Constructor
    pub fn new(frame: Rectangle, asset: Option<Asset<Vec<u8>>>) -> Self {
        let layer = Layer::new(frame);
        ImageView { layer, loader: asset, image_size: Vector::ZERO, custom_size: None }
    }

    pub fn set_asset(&mut self, asset: Option<Asset<Vec<u8>>>) {
        self.loader = asset;
    }

    fn draw_content(&mut self) -> Option<MeshTask> {
        let mut mesh = MeshTask::new(0);
        let trans = self.layer.build_transform();
        if let Some(loader) = &mut self.loader {
            loader
                .execute(|bytes| {
                    if let Ok(image) = Image::from_bytes(bytes.as_slice()) {
                        // FIXME: Need way for defining and preserving aspect ratio

                        let bkg = Img(&image);
                        let tex_trans = bkg.image().map(|img| img.projection(Rectangle::new_sized((1, 1))));
                        let offset = mesh.add_positioned_vertices(
                            [Vector::ZERO, Vector::X, Vector::ONE, Vector::Y].iter().cloned(),
                            trans,
                            tex_trans,
                            bkg,
                        );
                        mesh.triangles.push(GpuTriangle::new(offset, [0, 1, 2], 9, bkg));
                        mesh.triangles.push(GpuTriangle::new(offset, [2, 3, 0], 9, bkg));
                    }
                    Ok(())
                })
                .expect("Asset loading failed");
        }
        if mesh.vertices.len() > 0 {
            return Some(mesh);
        }

        None
    }
    /// Use custom size instead of transform scaling
    /// Unused for now
    #[allow(dead_code)]
    fn resize_content(&mut self, trans: Transform) -> Option<MeshTask> {
        let mut mesh = Mesh::new();
        let frame = self.layer.frame;
        let debug_id = self.debug_id();
        if let Some(loader) = &mut self.loader {
            loader
                .execute(|bytes| {
                    let out = format!("resize_content >>>>>>>> bytes={:?}", bytes.len());
                    log::debug!("{:?}", out);
                    // debug_log(&out);
                    let buf = image_rs::load_from_memory(bytes).unwrap().to_rgba();
                    let img = DynamicImage::ImageRgba8(buf);
                    let resize = img.thumbnail(frame.width() as u32, frame.height() as u32);
                    let dims = resize.dimensions();

                    let raw = resize.to_rgba().into_raw();

                    log::debug!("dimensions={:?} raw={:?}", resize.dimensions(), raw.len());
                    if let Ok(image) = Image::from_raw(raw.as_slice(), dims.0, dims.1, PixelFormat::RGBA) {
                        log::debug!("image={:?} raw={:?}", image.area().size, raw.len());
                        let bkg = Img(&image);
                        let trans = Transform::translate(frame.top_left() + frame.size() / 2)
                            * trans
                            * Transform::translate(-frame.size() / 2);
                        // * Transform::scale(frame.size());
                        let tex_trans = bkg.image().map(|img| img.projection(Rectangle::new_sized((1, 1))));
                        let offset = mesh.add_positioned_vertices(
                            [Vector::ZERO, Vector::X, Vector::ONE, Vector::Y].iter().cloned(),
                            trans,
                            tex_trans,
                            bkg,
                        );
                        mesh.triangles.push(GpuTriangle::new(offset, [0, 1, 2], 9, bkg));
                        mesh.triangles.push(GpuTriangle::new(offset, [2, 3, 0], 9, bkg));
                    } else {
                        let test = Image::from_bytes(raw.as_slice());
                        log::error!("Failed to create image from bytes len={:?}", test);
                    }
                    for v in &mesh.vertices {
                        log::trace!("{} {:?}", debug_id, v);
                    }

                    Ok(())
                })
                .expect("Asset loading failed");
            if mesh.vertices.len() > 0 {
                let mut task = MeshTask::new(0);
                task.append(&mut mesh);
                return Some(task);
            }
        }
        None
    }
}

impl Displayable for ImageView {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ImageView>()
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
        if self.image_size != Vector::ZERO {
            self.image_size
        } else {
            self.get_frame().size
        }
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
        if self.layer.debug && state.offset != Vector::ZERO {
            log::debug!("IMAGE_frame={:?}", self.layer.frame);
        }
    }

    fn render(&mut self, _theme: &mut Theme, window: &mut Window) {
        if self.layer.debug {
            self.layer.draw_border(window);
        }
        let meshes = self.layer.prepare_render(window);
        if meshes.len() > 0 {
            for task in meshes.into_iter() {
                window.add_task(task);
            }
        } else {
            if let Some(task) = self.draw_content() {
                self.layer.meshes.clear();
                window.add_task(task.clone());
                self.layer.meshes.push(task);
            }
        }
    }
}
