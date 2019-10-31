/// ImageView is a Tweenable object that displays an image
///
use crate::core::*;
use crate::events::*;

use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Img, Image},
    lifecycle::Window,
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
    /// The scaling factor as a range 0.0 to 1.0
    pub scale: f32,
    /// The image object itself
    pub image: Image,
}

impl ImageView {
    /// Constructor
    pub fn new(frame: Rectangle, image: Image) -> Self {
        let layer = Layer::new(frame);
        ImageView { layer, scale: 1.0, image }
    }
}

impl Displayable for ImageView {
    fn get_id(&self) -> u32 {
        self.layer.get_id()
    }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

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
        self.layer.frame.pos = self.layer.initial.pos + Vector::new(state.offset.0, state.offset.1);
        self.layer.tween_update();
    }

    fn render(&mut self, _theme: &mut Theme, window: &mut Window) {
        let scale_w = self.layer.frame.size.x / self.image.area().width();
        let scale_h = self.layer.frame.size.y / self.image.area().height();
        let scale = Vector { x: scale_w, y: scale_h };
        // window.draw(
        //     &self.image.area().constrain(&self.layer.frame),
        //     Img(&self.image)
        // );
        window.draw_ex(
            &self.image.area().constrain(&self.layer.frame),
            Img(&self.image),
            Transform::rotate(self.layer.transition.rotation) * Transform::scale(scale),
            1,
        );
    }
}
