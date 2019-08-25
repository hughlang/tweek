/// The Label view displays text content rendered as image text
///
use super::*;
use crate::core::*;
use crate::events::*;

// use glyph_brush::HorizontalAlign as HAlign;
use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Img, Image},
    lifecycle::Window,
};
use std::any::TypeId;

//-- Label -----------------------------------------------------------------------

/// Tweenable text label
pub struct Label {
    /// The base layer
    pub layer: Layer,
    /// The string value
    text: String,
    /// The rendered image text
    content: Option<Image>,
}

impl Label {
    /// Constructor with the text string
    pub fn new(frame: Rectangle, text: &str) -> Self {
        let layer = Layer::new(frame);
        Label { layer, text: text.to_owned(), content: None }
    }

    /// Set the text string if changed from initial value
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
        self.content = None;
    }
}

impl Displayable for Label {

    fn get_id(&self) -> u32 { self.layer.get_id() }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Label>()
    }

    fn get_layer_mut(&mut self) -> &mut Layer {
        &mut self.layer
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn get_content_size(&self) -> Vector {
        if let Some(image) = &self.content {
            return image.area().size;
        }
        Vector::new(0.0, 0.0)
    }

    fn move_to(&mut self, pos: (f32, f32)) {
        self.layer.frame.pos.x = pos.0;
        self.layer.frame.pos.y = pos.1;
    }

    fn set_theme(&mut self, theme: &mut Theme) {
        if self.layer.lock_style { return }
        self.layer.apply_theme(theme);

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

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        self.layer.draw_background(window);
        if let Some(image) = &self.content {
            window.draw(&image.area().with_center(self.layer.frame.center()), Img(&image));
        } else {
            log::debug!("Render <{:?}> frame={:?}", self.text, self.layer.frame);
            if let Some(image_text) = theme.default_font.render(&self.text, &self.layer.font_style, &self.layer.frame, false) {
                window.draw(&image_text.area().with_center(self.layer.frame.center()), Img(&image_text));
                self.content = Some(image_text);
            }
        }
    }
}
