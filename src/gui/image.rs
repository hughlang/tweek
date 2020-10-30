/// ImageView is a Tweenable object that displays an image
///
use crate::core::*;
use crate::events::*;

use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Img, GpuTriangle, Image, MeshTask},
    lifecycle::{Asset, Window},
};

use std::any::TypeId;

use super::*;

//-- Support -----------------------------------------------------------------------

/// Defines how to fit the image in the given space. Roughly follows:
/// https://developer.apple.com/documentation/uikit/uiview/contentmode
#[derive(Debug, Clone, Copy)]
pub enum ImageScaleMode {
    /// Default. Scale the content to fit the size of itself by changing the aspect ratio of the content if necessary
    ScaleToFill,
    /// Scale the content to fit the size of the view by maintaining the aspect ratio. Any remaining area of the view’s
    /// bounds is transparent
    AspectFit,
    /// Scale the content to fill the size of the view. Some portion of the content may be clipped to fill the view’s
    /// bounds
    AspectFill,
    /// Leave original size. If larger than the given frame size, you may need to adjust
    /// the frame origin.
    Original,
}

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
    /// How to scale the image to fit in the Layer frame
    scale_mode: ImageScaleMode,
}

impl ImageView {
    /// Constructor
    pub fn new(frame: Rectangle, asset: Option<Asset<Vec<u8>>>) -> Self {
        let layer = Layer::new(frame);
        ImageView { layer, loader: asset, image_size: Vector::ZERO, scale_mode: ImageScaleMode::ScaleToFill }
    }

    pub fn with_scale_mode(mut self, scale_mode: ImageScaleMode) -> Self {
        self.scale_mode = scale_mode;
        self
    }

    pub fn set_asset(&mut self, asset: Option<Asset<Vec<u8>>>) {
        self.loader = asset;
    }

    fn draw_content(&mut self) -> Option<MeshTask> {
        let mut mesh = MeshTask::new(0);
        let scale_mode = self.scale_mode;
        let frame = self.layer.frame;
        if let Some(loader) = &mut self.loader {
            loader
                .execute(|bytes| {
                    if let Ok(image) = Image::from_bytes(bytes.as_slice()) {
                        let trans = ImageView::scale_image(image.area().size, scale_mode, &frame);
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

    /// Create Transform for image scaling
    fn scale_image(img_size: Vector, scale_mode: ImageScaleMode, frame: &Rectangle) -> Transform {
        let img_aspect = img_size.x / img_size.y;
        let frame_aspect = frame.size.x / frame.size.y;
        match scale_mode {
            ImageScaleMode::AspectFit => {
                let img_frame: Rectangle = {
                    if img_aspect > frame_aspect {
                        // Image is too wide. Fill the width and letterbox the top/bottom.
                        let h = frame.size.y / img_aspect;
                        let pos = Vector::new(frame.x(), frame.center().y - h / 2.0);
                        Rectangle::new(pos, Vector::new(frame.width(), h))
                    } else {
                        // Image is too tall. Fill the height and letterbox the left/right.
                        let w = frame.size.x * img_aspect;
                        let pos = Vector::new(frame.center().x - w / 2.0, frame.y());
                        Rectangle::new(pos, Vector::new(w, frame.height()))
                    }
                };
                let trans = Transform::translate(img_frame.top_left() + img_frame.size() / 2)
                    * Transform::translate(-img_frame.size / 2)
                    * Transform::scale(img_frame.size);
                trans
            }
            ImageScaleMode::AspectFill => {
                let img_frame: Rectangle = {
                    if img_aspect > frame_aspect {
                        // Image is too wide. Fill the height and set position to outside left
                        let w = frame.size.x * img_aspect;
                        let pos = Vector::new(frame.center().x - w / 2.0, frame.y());
                        Rectangle::new(pos, Vector::new(w, frame.height()))
                    } else {
                        // Image is too tall. Fill the width and set position to outside top
                        let h = frame.size.y / img_aspect;
                        let pos = Vector::new(frame.x(), frame.center().y - h / 2.0);
                        Rectangle::new(pos, Vector::new(frame.width(), h))
                    }
                };
                let trans = Transform::translate(img_frame.top_left() + img_frame.size() / 2)
                    * Transform::translate(-img_frame.size / 2)
                    * Transform::scale(img_frame.size);
                trans
            }
            _ => {
                // The default. Image aspect ratio should match frame, or else it gets distorted
                let trans = Transform::translate(frame.top_left() + frame.size() / 2)
                    * Transform::translate(-frame.size / 2)
                    * Transform::scale(frame.size);
                trans
            }
        }
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
