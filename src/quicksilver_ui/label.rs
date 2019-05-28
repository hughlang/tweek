/// The Label view displays an image representation of text content.
///
use crate::core::*;

use quicksilver::{
    geom::{Rectangle, Shape},
    graphics::{Background::Img, Color, FontStyle, Image},
    lifecycle::Window,
};

use std::any::TypeId;

use super::*;

//-- Label -----------------------------------------------------------------------

pub struct Label {
    pub layer: TweenLayer,
    text: String,
    content: Option<Image>,
}

impl Label {
    pub fn new(frame: &Rectangle, text: &str) -> Self {
        let layer = TweenLayer::new(frame.clone());

        Label { layer: layer, text: text.to_string(), content: None }
    }

    pub fn set_color(&mut self, color: &Color) {
        self.layer.color = color.clone();
    }
}

impl TKDisplayable for Label {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Label>()
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

    fn render(&mut self, theme: &mut Theme, window: &mut Window) -> TKResult {
        if let Some(image) = &self.content {
            window.draw(&image.area().with_center(self.layer.frame.center()), Img(&image));
        } else {
            let style = FontStyle::new(theme.font_size, Color::BLACK);
            let image_text = theme.font.render(&self.text, &style).unwrap();
            window.draw(&image_text.area().with_center(self.layer.frame.center()), Img(&image_text));
            self.content = Some(image_text);
        }
        Ok(())
    }
}
