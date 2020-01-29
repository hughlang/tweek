/// TextArea – A simple scrollable text box with editor functionality
///
use super::*;
use crate::core::*;
use crate::events::*;
use crate::tools::*;

use image_rs::{imageops, DynamicImage};

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Color, Image, MeshTask},
    input::{ButtonState, Key, MouseButton, MouseCursor},
    lifecycle::Window,
};

use std::any::TypeId;
use std::ops::Range;

//-- TextArea -----------------------------------------------------------------------

/// UI component that resembles HTML textarea where word-wrapped content is displayed in a scrollable frame.
/// It can either be as read-only or read/write editing. In read mode, the text is just a block of rendered
/// image text. In edit mode, the text is displayed as live font glyphs rendered by the GPU
#[allow(dead_code)]
pub struct TextArea {
    /// The base layer
    pub layer: Layer,
    /// Currently unused.
    /// TODO: Implement placeholder like TextField
    pub placeholder: Option<String>,
    /// Optional cursor to display when in edit mode.
    pub cursor: Option<Cursor>,
    /// The internal frame for text display (inset from the outer frame)
    input_frame: Rectangle,
    /// The editor utility that manages all text state while user edits text.
    editor: TextAreaEditor,
    // draw_font: DrawFont,
    is_editing: bool,
    is_hovering: bool,
    can_edit: bool,
    can_scroll: bool,
    scroll_offset: Vector,
}

impl TextArea {
    /// Constructor
    pub fn new(frame: Rectangle, can_edit: bool) -> Self {
        // FIXME: The default() does not load a font and therefore requires a font to be set in set_theme()
        let layer = Layer::new(frame);

        let input_frame = layer.inset_by(5.0, 0.0, 5.0, 0.0);
        // log::debug!("outer frame={:?} input frame={:?}", frame, input_frame);

        let mut editor = TextAreaEditor::default()
            .with_frame((input_frame.x(), input_frame.y()), (input_frame.width(), input_frame.height()));

        // temporary hack to test scroll bar
        editor.ctx.frame.max.x = editor.ctx.frame.max.x - 10.0;
        editor.ctx.debug = true;

        TextArea {
            layer: Layer::new(frame),
            placeholder: None,
            cursor: None,
            input_frame,
            editor,
            is_editing: false,
            is_hovering: false,
            can_edit,
            can_scroll: true,
            scroll_offset: Vector::new(0.0, 0.0),
        }
    }

    /// Set the text in the editor
    pub fn set_text(&mut self, text: &str) {
        self.editor.ctx.set_text(text);
    }

    /// Get the text content of the editor
    pub fn get_text(&self) -> &str {
        return self.editor.ctx.get_text();
    }

    /// Calculate the range of lines that are visible in the editor
    /// FIXME: Unused?
    fn _get_visible_rows(&self) -> Range<usize> {
        let row_height = self.editor.ctx.font_size;
        let shift = self.scroll_offset.y / row_height;
        let start = shift.floor() as usize;
        let row_count = (self.layer.frame.height() / row_height + shift.fract()).ceil() as usize;
        return start..(start + row_count);
    }

    /// Tells the editor to switch to the editing state.
    fn start_editing(&mut self, position: Option<usize>) {
        log::debug!("TextArea start_editing");
        if !self.is_editing {
            self.layer.meshes.clear();
        }

        self.layer.mouse_state = MouseState::Focus;
        self.is_editing = true;
        self.editor.ctx.start_editing(position);

        let rect = self.input_frame;
        let pt1 = Vector::new(rect.x(), rect.y());
        let pt2 = Vector::new(rect.x(), rect.y() + rect.height());
        let mut cursor = Cursor::new(pt1, pt2, 2.0).default_animation();
        cursor.set_id(self.get_id());
        self.cursor = Some(cursor);
    }

    /// Switches to read-only state
    fn stop_editing(&mut self) {
        self.is_editing = false;
        self.cursor = None;
        self.update_rendered_text();
    }

    fn update_rendered_text(&mut self) {
        self.editor.tex_info = None;
        if self.get_text().len() == 0 {
            return;
        }
        // let style = FontStyle::new(self.editor.ctx.font_size, self.ctx.font_color);
        let frame = self.input_frame;
        let params =
            TextParams::new(self.layer.font_style.clone()).frame(frame.clone()).text(self.get_text()).multiline(true);

        let (imgbuf, text_w, text_h) = self.editor.ctx.draw_font.render_pixels(params);
        log::trace!("update_rendered_text w={:?} h={:?}", text_w, text_h);
        self.editor.full_render = Some(imgbuf.clone());

        // Create a new image to overlay the rendered text onto. Otherwise, problems with rendering can happen.
        // It seems that the output from render_pixels is not right.
        let mut canvas = DynamicImage::new_rgba8(text_w, text_h);
        imageops::overlay(&mut canvas, &imgbuf, 0, 0);
        let raw = canvas.to_rgba().into_raw();

        if let Some(idx) = DrawImage::upload_image(&self.node_key(), raw.as_slice(), text_w, text_h) {
            let tex = GPUTexture::new(idx, text_w, text_h);
            self.editor.tex_info = Some(tex);
            self.editor.ctx.text_size = (text_w, text_h);
        }
    }
}

impl Displayable for TextArea {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<TextArea>()
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

        // self.editor.ctx.update_metrics();
        // self.editor.update_textarea();
        self.layer.meshes.clear();
        self.update_rendered_text();

        let data = theme.data_for_font(Theme::DEFAULT_FONT).clone();
        self.editor.ctx.set_font_data(data, theme.font_size);
        self.layer.border_style = BorderStyle::SolidLine(theme.border_color, theme.border_width);
        self.layer.bg_style = BackgroundStyle::None;
    }

    fn get_perimeter_frame(&self) -> Option<Rectangle> {
        let perimeter = self.layer.offset_by(10.0, 0.0, 10.0, 0.0);
        Some(perimeter)
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Activate => {
                // self.start_editing(None);
            }
            DisplayEvent::Deactivate => {
                self.stop_editing();
            }
            DisplayEvent::Ready => {
                self.layer.on_ready();
                if self.get_text().len() > 0 {
                    self.editor.ctx.update_metrics();
                    self.editor.update_textarea();
                    self.update_rendered_text();
                }
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
            }
            _ => {}
        }
    }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        self.layer.tween_update(state);
        if let Some(cursor) = &mut self.cursor {
            cursor.update(window, state);
        }
    }

    #[allow(unused_mut)]
    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        self.layer.draw_background(window);

        if self.get_text().len() == 0 {
            return;
        }

        if self.is_editing {
            self.editor.update_textarea();
            if self.get_text().len() > 0 {
                if let Some(mesh_task) = &self.editor.ctx.draw_font.cached_mesh {
                    window.add_task(mesh_task.clone());
                } else {
                    if let Some(text) = self.editor.get_visible_text(self.scroll_offset.y) {
                        let params = TextParams::new(self.layer.font_style)
                            .frame(self.input_frame.clone())
                            .text(&text)
                            .multiline(true);
                        if let Some(task) = self.editor.ctx.draw_font.draw(params) {
                            window.add_task(task);
                        }
                    }
                }
            }
            if let Some(cursor) = &mut self.cursor {
                let mut pt = self.editor.ctx.cursor_origin;

                let cursor_height = theme.font_size;
                let y2 = pt.1 + cursor_height * 0.2;
                let y1 = y2 - cursor_height;

                let pt1 = Vector::new(pt.0, y1);
                let pt2 = Vector::new(pt.0, y2);
                let mut line = cursor.render_line(&pt1, &pt2, &theme);

                let mut mesh = MeshTask::new(0);
                mesh.append(&mut line);
                window.add_task(mesh.clone());

                // log::debug!("frame={:?} cursor={:?}", self.input_frame, cursor_pt);
            }
        } else {
            if self.layer.meshes.len() > 0 {
                for task in &self.layer.meshes {
                    window.add_task(task.clone());
                }
            } else {
                if let Some(tex) = &mut self.editor.tex_info {
                    let full_size = Vector::new(self.input_frame.width(), tex.height as f32);
                    let region = Rectangle::new((0.0, self.scroll_offset.y), self.input_frame.size);
                    // log::trace!("Render full_size={:?} region={:?}", full_size, region);

                    let tex_quad = DrawImage::normalize_tex_quad(full_size, region);
                    let color = self.layer.font_style.get_color();
                    if let Some(mesh) = DrawImage::sub_texture(tex.idx, self.input_frame.clone(), Some(tex_quad), color)
                    {
                        window.add_task(mesh.clone());
                        self.layer.meshes.push(mesh);
                    } else {
                        log::error!("DrawImage failed ");
                    }
                } else {
                    log::error!("Full render is missing");
                }
            }
        }

        // Render scrollbar
        let content_height = self.editor.ctx.text_size.1 as f32;
        if let Some(rect) = UITools::get_scrollbar_frame(content_height, &self.layer.frame, self.scroll_offset.y) {
            // log::debug!("scrollbar={:?}", rect);
            // FIXME: use theme for scrollbar color
            window.draw(&rect, Col(Color::from_hex(UITools::SCROLLBAR_COLOR)));
        }

        // Draw border
        self.layer.draw_border(window);
    }

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        self.is_hovering = self.layer.handle_mouse_over(pt);
        if self.is_hovering {
            if pt.overlaps_rectangle(&self.input_frame) {
                window.set_cursor(MouseCursor::Text);
            // let local_pt = *pt - self.layer.frame.pos;
            // self.editor.find_cursor_position(local_pt.x, local_pt.y, self.scroll_offset.y);
            } else {
                window.set_cursor(MouseCursor::Hand);
            }
        } else {
            window.set_cursor(MouseCursor::Default);
        }
        return self.is_hovering;
    }
}

impl Responder for TextArea {
    fn get_field_value(&self) -> FieldValue {
        FieldValue::Text(self.get_text().to_owned())
    }

    fn handle_key_press(&mut self, c: char, _window: &mut Window) {
        if c.is_ascii_control() {
            return;
        }
        if self.can_edit && c.is_ascii() {
            self.editor.ctx.insert_char(c);
        } else {
            // log::debug!("### non ascii={}", c);
        }
    }

    fn handle_key_command(&mut self, key: &Key, _window: &mut Window) -> bool {
        match key {
            Key::Back => {
                self.editor.ctx.delete_char();
            }
            Key::Left => {
                self.editor.ctx.move_cursor(-1);
            }
            Key::Right => {
                self.editor.ctx.move_cursor(1);
            }
            Key::Tab => {
                // Optionally call stop_editing here or let a parent Scene take care of that.
                self.stop_editing();
                return true;
            }
            Key::Return => {
                if self.is_editing && self.can_edit {
                    self.editor.ctx.insert_char('\n');
                }
            }
            _ => (),
        }
        false
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        if pt.overlaps_rectangle(&self.input_frame) {
            state.event_bus.register_event(MouseEvent::Select(self.get_type_id(), self.get_id()));
            if self.can_edit {
                let local_pt = *pt - self.input_frame.pos;
                eprintln!("local_pt={:?}", local_pt);
                let pos = self.editor.find_cursor_position(local_pt.x, local_pt.y, self.scroll_offset.y);
                self.start_editing(pos);
                return true;
            }
        }
        false
    }

    fn handle_mouse_scroll(&mut self, pt: &Vector, _state: &mut AppState) {
        // Only scroll if hovering and not editing
        if self.is_hovering && !self.is_editing {
            let upper_limit = self.editor.ctx.text_size.1 as f32 - self.layer.frame.height();
            let eval_y = (self.scroll_offset.y + pt.y).max(0.0).min(upper_limit);
            self.scroll_offset.y = eval_y;
            // log::debug!(">>> self.scroll_offset.y {}", self.scroll_offset.y);
            self.layer.meshes.clear();
        }
    }
}

//-- Support -----------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EditState {
    /// User is idle
    Idle,
    /// User is typing and scrolling is disabled
    Typing,
    /// Scrolling underway and edits not allowed. Cursor is not visible.
    Scrolling,
}
