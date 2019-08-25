/// Checkbox
///
use crate::core::*;
use crate::tools::*;
use crate::events::*;

use quicksilver::{
    geom::{Line, Rectangle, Shape, Vector},
    graphics::{
        Background::{Col, Img},
        Color, MeshTask, Image,
    },
    lifecycle::Window,
};
use std::any::TypeId;

use super::*;

/// Enum to define the style of the checkbox/radio
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CheckStyle {
    /// Simple box with X
    X,
    /// A circular radio button
    Radio,
    /// A checkmark. Currently unused
    Check,
    /// A box that is filled with a solid box when checked
    /// The f32 param is the inset margin for the fill
    Fill(f32),
}

//-- Checkbox -----------------------------------------------------------------------

/// A Checkbox
#[allow(dead_code)]
pub struct Checkbox {
    /// Base layer
    pub layer: Layer,
    /// Text to display next to checkbox
    pub text: String,
    /// Is it checked?
    pub is_checked: bool,
    /// The style of the checkbox: X, radio, check mark, filled
    pub check_style: CheckStyle,
    /// Previously rendered text
    image_text: Option<Image>,
}

impl Checkbox {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);

        Checkbox {
            layer,
            text: String::default(),
            is_checked: false,
            check_style: CheckStyle::X,
            image_text: None,
        }
    }

    /// Builder method to set the text
    pub fn with_text(mut self, text: &str, is_checked: bool) -> Self {
        self.text = text.to_owned();
        self.is_checked = is_checked;
        self
    }

    /// Method used by OptionGroup to adjust position of checkbox options based on the OptionGroupLayout
    pub fn update_frame(&mut self, frame: Rectangle) {
        self.layer.frame = frame;
    }

    /// Public render method that can be called by OptionGroup. Currently the default method for the Displayable render() method
    fn render_at(&mut self, frame: &Rectangle, theme: &mut Theme, window: &mut Window) {
        self.layer.draw_background(window);

        let stroke = 1.0;
        let stroke_color = Color::BLACK;

        let box_frame = Rectangle::new((0.0, 0.0), (20.0, 20.0));
        let box_frame = UITools::position_left_middle(&frame, &box_frame, 5.0);
        match self.check_style {
            CheckStyle::X => {
                let mut lines = UITools::make_border_lines(&box_frame, stroke);
                if self.is_checked {
                    let rect = &box_frame;
                    let line = Line::new((rect.x(), rect.y()), (rect.x() + rect.width(), rect.y() + rect.height()))
                        .with_thickness(stroke);
                    lines.push(line);
                    let line = Line::new((rect.x() + rect.width(), rect.y()), (rect.x(), rect.y() + rect.height()))
                        .with_thickness(stroke);
                    lines.push(line);
                }

                for line in lines {
                    window.draw(&line.with_thickness(line.t), Col(stroke_color));
                }
            }
            CheckStyle::Radio => {
                let mut mesh =
                    DrawShape::circle(&box_frame.center(), &box_frame.width() / 2.0, None, Some(Color::BLACK), 1.0);
                let mut task = MeshTask::new(0);
                task.vertices.append(&mut mesh.vertices);
                task.triangles.append(&mut mesh.triangles);
                window.add_task(task);

                if self.is_checked {
                    let mut mesh = DrawShape::circle(
                        &box_frame.center(),
                        &box_frame.width() / 2.0 - 4.0,
                        Some(Color::BLACK),
                        None,
                        0.0,
                    );
                    let mut task = MeshTask::new(0);
                    task.vertices.append(&mut mesh.vertices);
                    task.triangles.append(&mut mesh.triangles);
                    window.add_task(task);
                }
            }
            _ => {}
        }


        if let Some(img) = &self.image_text {
            let y = frame.y() + (frame.height() - img.area().height()) / 2.0;
            let text_frame = Rectangle::new((frame.x() + 30.0, y), (frame.width() - 30.0, img.area().height()));
            window.draw(&img.area().constrain(&text_frame), Img(&img));
        } else {
            if let Some(img) = theme.default_font.render(
                &self.text,
                &self.layer.font_style,
                &self.layer.frame,
                false
            ) {
                let y = frame.y() + (frame.height() - img.area().height()) / 2.0;
                let text_frame = Rectangle::new((frame.x() + 30.0, y), (frame.width() - 30.0, img.area().height()));
                window.draw(&img.area().constrain(&text_frame), Img(&img));
                self.image_text = Some(img);
            }

        }
    }
}

// *****************************************************************************************************
// Checkbox :: Displayable
// *****************************************************************************************************

impl Displayable for Checkbox {

    fn get_id(&self) -> u32 { self.layer.get_id() }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Checkbox>()
    }

    fn get_layer_mut(&mut self) -> &mut Layer {
        &mut self.layer
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn get_content_size(&self) -> Vector {
        if let Some(img) = &self.image_text {
            Vector::new(40.0 + img.area().width(), self.layer.frame.height())
        } else {
            self.layer.frame.size
        }
    }

    fn move_to(&mut self, pos: (f32, f32)) {
        self.layer.frame.pos.x = pos.0;
        self.layer.frame.pos.y = pos.1;
    }

    /// Change the font, color, and size
    fn set_theme(&mut self, theme: &mut Theme) {
        if self.layer.lock_style { return }
        self.layer.apply_theme(theme);
        // let style = FontStyle::new(theme.font_size, Color::BLACK);
        // let img = theme.font.render(&self.text, &style).unwrap();
        // let hit_area = UITools::combine_frames(&self.box_frame, &img.area());
        // log::debug!(">>> hit_area={:?} y={:?}", hit_area, 0);
        // self.hit_area = UITools::padded_rect(&hit_area, 4.0, 4.0);
        self.image_text = None;
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
        self.render_at(&self.layer.frame.clone(), theme, window)
    }

    fn set_hover_animation(&mut self, props: PropSet) {
        self.layer.hover_effect = Some(props);
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        return self.layer.handle_mouse_over(pt);
    }
}

// *****************************************************************************************************
// Checkbox :: Responder
// *****************************************************************************************************

impl Responder for Checkbox {
    fn get_field_value(&self) -> FieldValue {
        FieldValue::Checkbox(self.is_checked)
    }

    fn handle_mouse_down(&mut self, pt: &Vector, _state: &mut AppState) -> bool {
        if pt.overlaps_rectangle(&self.layer.frame) {
            // log::debug!("Click at: x={} y={}", pt.x, pt.y);
            self.is_checked = !self.is_checked;

            return true;
        }
        false
    }
}
