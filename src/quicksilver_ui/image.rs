/// The Label view displays an image representation of text content.
///
use crate::core::*;

use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Img, Color, Image},
    lifecycle::Window,
};

use std::any::TypeId;

use super::*;

//-- Image -----------------------------------------------------------------------

pub struct ImageView {
    pub layer: TweenLayer,
    pub scale: f32,
    pub image: Image,
}

impl ImageView {
    pub fn new(frame: Rectangle, image: Image) -> Self {
        let layer = TweenLayer::new(frame);
        ImageView { layer: layer, scale: 1.0, image: image }
    }
}

impl TKDisplayable for ImageView {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ImageView>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn set_theme(&mut self, _theme: &Theme) {
        // if let Some(label) = &mut self.label {
        //     label.layer.graphics.color = theme.fg_color;
        // }
    }

    fn update(&mut self) -> TKResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.apply_updates(&update.props);
            }
        }
        Ok(())
    }

    fn render(&mut self, theme: &Theme, window: &mut Window) -> TKResult {
        let scale_w = self.layer.frame.size.x / self.image.area().width();
        let scale_h = self.layer.frame.size.y / self.image.area().height();
        let scale = Vector { x: scale_w, y: scale_h };
        window.draw_ex(
            &self.image.area().constrain(&self.layer.frame),
            Img(&self.image),
            Transform::rotate(self.layer.rotation) * Transform::scale(scale),
            1,
        );
        Ok(())
    }
}
