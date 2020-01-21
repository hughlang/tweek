/// Checkbox
///
use crate::core::*;
use crate::events::*;
use crate::tools::*;

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Color, MeshTask},
    input::MouseCursor,
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
    /// The calculated minimum size of the checkbox based on size of the text
    pub content_size: Vector,
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
            content_size: Vector::ZERO,
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

    /// Method to clear previous MeshTasks. This is also called by OptionGroup which sometimes needs to invalidate Checkboxes
    pub fn clear_draw_cache(&mut self) {
        self.layer.meshes.clear();
    }
}

// *****************************************************************************************************
// Checkbox :: Displayable
// *****************************************************************************************************

impl Displayable for Checkbox {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Checkbox>()
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
        if self.content_size != Vector::ZERO {
            self.content_size
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
        let ok = self.layer.apply_theme(theme);
        if !ok {
            return;
        }
        self.clear_draw_cache();
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
        // self.layer.draw_background(window);

        // Use previous mesh if exists
        if self.layer.meshes.len() > 0 {
            for task in &self.layer.meshes {
                window.add_task(task.clone());
            }
            return;
        }
        let border = self.layer.border_style.get_border();

        // TODO: make these themeable
        let stroke = border.1;
        let stroke_color = border.0;
        let frame = self.layer.frame;
        let box_frame = Rectangle::new((0.0, 0.0), (20.0, 20.0));
        let box_frame = UITools::position_left_middle(&frame, &box_frame, 5.0);
        match self.check_style {
            CheckStyle::X => {
                let mut task = MeshTask::new(0);
                let mut mesh = DrawShape::rectangle(&box_frame, None, Some(stroke_color), stroke, 0.0);
                task.append(&mut mesh);

                if self.is_checked {
                    let rect = &box_frame;
                    let pts: [&Vector; 2] = [
                        &Vector::new(rect.x(), rect.y()),
                        &Vector::new(rect.x() + rect.width(), rect.y() + rect.height()),
                    ];
                    let mut line = DrawShape::line(&pts, stroke_color, stroke);
                    task.append(&mut line);

                    task.vertices.append(&mut line.vertices);
                    task.triangles.append(&mut line.triangles);

                    let pts: [&Vector; 2] = [
                        &Vector::new(rect.x() + rect.width(), rect.y()),
                        &Vector::new(rect.x(), rect.y() + rect.height()),
                    ];
                    let mut line = DrawShape::line(&pts, stroke_color, stroke);
                    task.append(&mut line);
                }
                self.layer.meshes.push(task.clone());
                window.add_task(task);
            }

            CheckStyle::Radio => {
                let mut mesh =
                    DrawShape::circle(&box_frame.center(), &box_frame.width() / 2.0, None, Some(Color::BLACK), 1.0);
                let mut task = MeshTask::new(0);
                task.append(&mut mesh);

                if self.is_checked {
                    let mut mesh = DrawShape::circle(
                        &box_frame.center(),
                        &box_frame.width() / 2.0 - 4.0,
                        Some(Color::BLACK),
                        None,
                        0.0,
                    );
                    task.append(&mut mesh);
                }
                self.layer.meshes.push(task.clone());
                window.add_task(task);
            }
            _ => {}
        }

        let text_frame = Rectangle::new((frame.x() + 30.0, frame.y()), (frame.width() - 30.0, frame.height()));
        let params = TextParams::new(self.layer.font_style).frame(text_frame.clone()).text(&self.text).multiline(false);

        if let Some(task) = theme.default_font.draw(params) {
            self.content_size = Vector::new(task.content_size.0 + 30.0, task.content_size.1);
            // log::error!("Checkbox frame size={:?}", self.content_size);
            self.layer.meshes.push(task.clone());
            window.add_task(task);
        } else {
            log::debug!(">>> mesh_task is None!");
        }
    }

    fn set_hover_animation(&mut self, props: PropSet) {
        self.layer.hover_effect = Some(props);
    }

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        let hover = self.layer.handle_mouse_over(pt);
        if hover {
            window.set_cursor(MouseCursor::Hand);
        } else {
            window.set_cursor(MouseCursor::Default);
        }
        hover
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
            self.is_checked = !self.is_checked;
            self.clear_draw_cache();
            return true;
        }
        false
    }
}
