/// The Text view displays live text (as opposed to Label, which displays rendered text as image)
///
use crate::core::*;

use glyph_brush::HorizontalAlign as HAlign;

use quicksilver::{
    geom::Rectangle,
    graphics::{Color, DrawTask},
    lifecycle::Window,
};

use std::any::TypeId;

use super::*;

//-- Text -----------------------------------------------------------------------

pub struct Text {
    pub layer: TweenLayer,
    text: String,
    h_align: HAlign,
    draw_task: Option<DrawTask>,
}

impl Text {
    pub fn new(frame: &Rectangle, text: &str) -> Self {
        let layer = TweenLayer::new(frame.clone());
        Text { layer: layer, text: text.to_string(), h_align: HAlign::Left, draw_task: None }
    }

    pub fn margin(mut self, x: f32, y: f32) -> Self {
        self.layer.frame = UITools::inset_rect(&self.layer.frame, x, y, x, y);
        self
    }

    pub fn set_text(&mut self, text: String) {
        if self.text != text {
            self.draw_task = None;
            self.text = text;
        }
    }

    pub fn align_h(&mut self, align: HAlign) {
        self.h_align = align;
    }

    pub fn set_color(&mut self, color: &Color) {
        self.layer.color = color.clone();
    }

    // pub fn get_content_size(&self) -> Vector {
    //     if let Some(task) = self.draw_task {
    //         return Vector::new(task.content_size.0, task.content_size.0);
    //     }
    //     (0.0, 0.0)
    // }
}

impl TKDisplayable for Text {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Text>()
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
        // window.draw(&self.layer.frame, Col(Color::WHITE));

        if let Some(task) = &self.draw_task {
            window.add_task(task.clone());
        } else {
            let task = theme.draw_text.draw(
                &self.text,
                &self.layer.font_style,
                self.h_align,
                &self.layer.frame,
                window.screen_size(),
            );
            match task {
                Ok(task) => {
                    self.draw_task = Some(task.clone());
                    window.add_task(task);
                }
                Err(e) => eprintln!("ERROR={:?}", e),
            }
        }
        Ok(())
    }
}
