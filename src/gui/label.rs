/// The Label view displays text content rendered as image text
///
use super::*;
use crate::core::*;
use crate::events::*;
use crate::tools::{DrawImage, TextParams};

use image_rs::{imageops, DynamicImage, GenericImageView};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{MeshTask},
    lifecycle::Window,
};

use std::any::TypeId;

/// Defines if the label consists of image, text, or both
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LabelDisplay {
    Text,
    Image,
    ImageAndText,
}

/// Defines position of image relative to text where LabelType is ImageAndText
/// The image will be sized to fit the frame of the overall label +/- any inset parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LabelLayout {
    ImageLeft,
    ImageRight,
    ImageTop,
    ImageBottom,
}

const PAD: f32 = 0.0;

//-- Label -----------------------------------------------------------------------

/// Tweenable text label
pub struct Label {
    /// The base layer
    pub layer: Layer,
    /// Is it Text, Image, or both?
    pub display: LabelDisplay,
    /// How is it laid out?
    pub layout: LabelLayout,
    /// The string value
    text: Option<String>,
    /// The optional image
    raw_image: Option<DynamicImage>,
    /// Horizontal and vertical margins for icon
    pub icon_margins: (f32, f32),
    /// Horizontal and vertical margins for icon
    pub text_margins: (f32, f32),
    outer_padding: f32,
    inner_padding: f32,
}

impl Label {

    /// Constructor with the text string
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);
        Label { layer,
            text: None,
            raw_image: None,
            display: LabelDisplay::Text,
            layout: LabelLayout::ImageLeft,
            icon_margins: (5.0, 5.0),
            text_margins: (5.0, 10.0),
            outer_padding: 10.0,
            inner_padding: 10.0,
        }
    }

    /// Set the text string if changed from initial value
    pub fn set_text(&mut self, text: &str) {
        self.text = Some(text.to_owned());
        self.layer.meshes.clear();
    }

    pub fn set_image(&mut self, image: DynamicImage) {
        self.raw_image = Some(image);
        self.layer.meshes.clear();
    }

    /// Returns two Rectangles with the space allocation for the text and image within the Label's frame
    /// The origin is zero-based relative to the Label's frame so needs to be translated for rendering
    fn layout_content(&self) -> (Rectangle, Rectangle) {
        let mut txt_frame = Rectangle::new_sized(Vector::ZERO);
        let mut img_frame = Rectangle::new_sized(Vector::ZERO);
        let frame = self.layer.frame;
        match self.display {
            LabelDisplay::Text => {
                txt_frame = Rectangle::new_sized(frame.size);
            }
            LabelDisplay::Image => {
                img_frame = Rectangle::new_sized(frame.size);
            }
            LabelDisplay::ImageAndText => {
                /* Calculate how to best use the space between the text and image based on the
                   LabelLayout value */
                if let (Some(_text), Some(raw_image)) = (self.text.as_ref(), self.raw_image.as_ref()) {
                    let (source_w, source_h) = raw_image.dimensions();
                    let _aspect = source_w as f32 / source_h as f32;
                    match self.layout {
                        LabelLayout::ImageLeft => {
                            // Layout H:|-img-|-text-| with img aligned left edge and text aligned left to img
                            let scale = frame.height() / source_h as f32;
                            let img_h = frame.height();
                            let img_w = source_w as f32 * scale;
                            img_frame = Rectangle::new_sized((img_w, img_h));
                            let txt_x = img_w + PAD;
                            txt_frame = Rectangle::new((txt_x, 0.0), (frame.width() - txt_x, frame.height()));
                        }
                        LabelLayout::ImageRight => {
                            // Layout H:|-text-|-img-| with img aligned right edge and text aligned right to img
                            let scale = frame.height() / source_h as f32;
                            let img_h = frame.height();
                            let img_w = source_w as f32 * scale;
                            let img_x = frame.width() - img_w;
                            img_frame = Rectangle::new((img_x, 0.0), (img_w, img_h));
                            let txt_w = img_x - PAD;
                            txt_frame = Rectangle::new((0.0, 0.0), (txt_w, frame.height()));
                        }
                        LabelLayout::ImageTop => {
                            // Layout V:|-img-|-text-| with img aligned top with size to fit with space for text
                            // Assume that text fits in the provided width and does not wrap (yet)
                            let txt_h = self.layer.font_style.get_size() + self.outer_padding;
                            let txt_y = frame.height() - txt_h;
                            let img_h = txt_y - self.inner_padding;
                            let scale = img_h / source_h as f32;
                            let img_w = source_w as f32 * scale;
                            let img_x = (frame.width() - img_w) / 2.0;
                            let img_y = self.outer_padding;
                            img_frame = Rectangle::new((img_x, img_y), (img_w, img_h));
                            txt_frame = Rectangle::new((0.0, txt_y), (frame.width(), txt_h));
                        }
                        LabelLayout::ImageBottom => {
                            // Layout V:|-text-|-img-| with img aligned top with size to fit with space for text
                            // Assume that text fits in the provided width and does not wrap (yet)
                            let txt_h = self.layer.font_style.get_size() + self.outer_padding;
                            let _txt_y = self.outer_padding;
                            let img_h = frame.height() - txt_h - self.inner_padding;
                            let scale = img_h / source_h as f32;
                            let img_w = source_w as f32 * scale;
                            let img_x = (frame.width() - img_w) / 2.0;
                            let img_y = txt_h + self.inner_padding;
                            img_frame = Rectangle::new((img_x, img_y), (img_w, img_h));
                            txt_frame = Rectangle::new((0.0, 0.0), (frame.width(), txt_h));
                        }
                    }
                } else {
                    log::error!("{:?} is missing text or image", self.display);
                }
            }
        }
        (img_frame, txt_frame)
    }

    /// Method to render contents of this Label as an Image
    fn draw_content(&mut self, theme: &mut Theme) -> Option<MeshTask> {
        // Calculate the relative frames for the image and text content
        let (img_frame, mut txt_frame) = self.layout_content();
        log::debug!(">>> img_frame: {:?}", img_frame);
        log::debug!(">>> txt_frame: {:?}", txt_frame);

        let frame = self.layer.frame;

        let txt_render: Option<DynamicImage> = {
            if let Some(text) = &self.text {
                let params = TextParams::new(self.layer.font_style.clone())
                    .frame(txt_frame.clone())
                    .text(text)
                    .multiline(false);

                let (font_buf, text_w, text_h) = theme.default_font.render_pixels(params);
                // Center the txt_frame
                txt_frame.pos.x += (txt_frame.width() - text_w as f32) / 2.0;
                txt_frame.pos.y += (txt_frame.height() - text_h as f32) / 2.0;
                txt_frame.size.x = text_w as f32;
                log::trace!("Updated txt_frame={:?} ", txt_frame);
                let img = DynamicImage::ImageRgba8(font_buf);
                Some(img)
            } else {
                None
            }
        };
        let icon_render: Option<DynamicImage> = {
            if let Some(image) = &self.raw_image {
                let icon = image.thumbnail(img_frame.width() as u32, img_frame.height() as u32);
                Some(icon)
            } else {
                None
            }
        };

        // Create a blank canvas for the entire frame and overlay the icon and/or image
        let mut canvas = DynamicImage::new_rgba8(frame.width() as u32, frame.height() as u32);
        if let Some(icon_image) = icon_render {
            imageops::overlay(&mut canvas, &icon_image, img_frame.pos.x as u32, img_frame.pos.y as u32);
        }
        if let Some(txt_image) = txt_render {
            imageops::overlay(&mut canvas, &txt_image, txt_frame.pos.x as u32, txt_frame.pos.y as u32);
        }

        let raw = canvas.to_rgba().into_raw();
        if let Some(idx) = DrawImage::upload_image(&self.node_key(), raw.as_slice(),
                                                frame.width() as u32, frame.height() as u32) {
            return DrawImage::draw_texture(idx, &self.layer.frame, self.layer.font_style.get_color());
        }
        None
    }
}

impl Displayable for Label {
    fn get_id(&self) -> u32 {
        self.layer.get_id()
    }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Label>()
    }

    fn get_layer(&self) -> &Layer { &self.layer }

    fn get_layer_mut(&mut self) -> &mut Layer { &mut self.layer }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn get_content_size(&self) -> Vector {
        // if let Some(image) = &self.content {
        //     return image.area().size;
        // }
        Vector::new(0.0, 0.0)
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
                // let (img_frame, txt_frame) = self.layout_content();
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
            }
            _ => {}
        }
    }

    fn update(&mut self, _window: &mut Window, state: &mut AppState) {
        self.layer.frame.pos = self.layer.initial.pos + Vector::new(state.offset.0, state.offset.1);
        self.layer.tween_update();
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        if self.layer.meshes.len() > 0 {
            for task in &self.layer.meshes {
                window.add_task(task.clone());
            }
            return;
        }
        if let Some(task) = self.draw_content(theme) {
            window.add_task(task.clone());
            self.layer.meshes.push(task);
        }
    }
}
