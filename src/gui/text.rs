/// The Text view displays live text (as opposed to Label, which displays rendered text as image)
///
use crate::core::*;
use crate::events::*;
use crate::tools::*;

use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::MeshTask,
    lifecycle::Window,
};

use std::any::TypeId;

use super::*;

//-- Text -----------------------------------------------------------------------

/// UI component for live text rendered with glyph_brush
pub struct Text {
    /// The base layer
    pub layer: Layer,
    /// Is it multiline? Default is false
    pub multiline: bool,
    /// The text string
    text: String,
    /// The horizontal alignment
    pub text_align: TextAlign,
    /// The vertical alignment
    pub vert_align: VertAlign,
    /// Cached mesh data
    mesh_task: Option<MeshTask>,
    /// The offset is used when the parent Scene is moved and thus needs to inform child objects where to render
    offset: Vector,
    /// A subframe for clipping text
    pub subframe: Option<Rectangle>,
}

impl Text {
    /// Constructor
    pub fn new(frame: Rectangle, text: &str) -> Self {
        let layer = Layer::new(frame);
        Text {
            layer,
            multiline: false,
            text: text.to_string(),
            text_align: TextAlign::Left,
            vert_align: VertAlign::Middle,
            mesh_task: None,
            offset: Vector::ZERO,
            subframe: None,
        }
    }

    /// Builder method to define the horizontal and vertical margins for the text
    pub fn margin(mut self, x: f32, y: f32) -> Self {
        self.layer.frame = UITools::inset_rect(&self.layer.frame, x, y, x, y);
        self
    }

    /// Set the string text and nullify the existing MeshTask
    pub fn set_text(&mut self, text: String) {
        if self.text != text {
            self.mesh_task = None;
            self.text = text;
        }
    }

    /// Getter for the text string
    pub fn get_text(&self) -> &String {
        &self.text
    }

    /// Set the horizontal alignment
    pub fn text_align(&mut self, align: TextAlign) {
        self.text_align = align;
    }
}

impl Displayable for Text {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Text>()
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
        if let Some(task) = &self.mesh_task {
            return Vector::new(task.content_size.0, task.content_size.1);
        }
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
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
                self.mesh_task = None;
                if let Some(task) = &mut self.mesh_task {
                    for (_, vertex) in task.vertices.iter_mut().enumerate() {
                        vertex.pos = Transform::translate(self.offset) * vertex.pos;
                    }
                }
            }
            _ => {}
        }
    }

    fn update(&mut self, _window: &mut Window, state: &mut AppState) {
        if self.layer.is_animating() {
            self.offset = self.layer.get_movement_offset();
        } else {
            self.offset = state.offset;
        }
        self.layer.tween_update(state);
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        self.layer.draw_background(window);
        self.layer.draw_border(window);
        if let Some(task) = &self.mesh_task {
            if self.layer.is_animating() {
                // If tint animation, change the text color
                let mut task = task.clone();
                for (_, vertex) in task.vertices.iter_mut().enumerate() {
                    vertex.pos = Transform::translate(self.offset) * vertex.pos;
                }
                window.add_task(task);
            } else {
                let mut task = task.clone();
                if self.offset != Vector::ZERO {
                    for (_, vertex) in task.vertices.iter_mut().enumerate() {
                        vertex.pos = Transform::translate(self.offset) * vertex.pos;
                    }
                }
                window.add_task(task);
            }
        } else {
            let mut params = TextParams::new(self.layer.font_style.clone())
                .frame(self.layer.frame.clone())
                .text(&self.text)
                .align(self.text_align, self.vert_align)
                .multiline(self.multiline);

            params.subframe = self.subframe;
            params.debug = self.layer.debug;

            if let Some(task) = theme.default_font.draw(params) {
                self.mesh_task = Some(task.clone());
                window.add_task(task);
            } else {
                log::debug!(">>> mesh_task is None!");
            }
        }
    }
}

impl Responder for Text {
    fn set_field_value(&mut self, value: &FieldValue, type_id: TypeId, layer_id: u32) -> bool {
        if type_id == self.get_type_id() && layer_id == self.layer.get_id() {
            match value {
                FieldValue::Text(text) => {
                    self.set_text(text.to_owned());
                    return true;
                }
                _ => (),
            }
        }
        false
    }
}
