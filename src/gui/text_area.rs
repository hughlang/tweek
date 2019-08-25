/// TextArea – A simple scrollable text box with editor functionality
///
use super::*;
use crate::core::*;
use crate::events::*;
use crate::tools::*;

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Background::Img, Color, FontStyle, Image, PixelFormat},
    input::{Key},
    lifecycle::Window,
};

use glyph_brush::HorizontalAlign as HAlign;
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
    /// Cached image text that is unchanged until the user scrolls or tries to start editing.
    image_text: Option<Image>,
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
    pub fn new(frame: Rectangle, theme: &mut Theme, can_edit: bool) -> Self {
        // FIXME: The default() does not load a font and therefore requires a font to be set in set_theme()
        let layer = Layer::new(frame);

        let input_frame = layer.inset_by(10.0, 10.0, 10.0, 10.0);
        // log::debug!("outer frame={:?} input frame={:?}", frame, input_frame);

        let mut editor = TextAreaEditor::create(theme)
            .with_frame((input_frame.x(), input_frame.y()), (input_frame.width(), input_frame.height()));


        // temporary hack to test scroll bar
        editor.ctx.frame.max.x = editor.ctx.frame.max.x - 10.0;
        editor.ctx.debug = true;

        TextArea {
            layer: Layer::new(frame),
            placeholder: None,
            cursor: None,
            input_frame,
            image_text: None,
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
    fn start_editing(&mut self) {
        log::debug!("TextArea start_editing");

        self.layer.mouse_state = MouseState::Focus;
        self.is_editing = true;
        self.editor.ctx.start_editing();

        let rect = self.input_frame;
        let pt1 = Vector::new(rect.x(), rect.y());
        let pt2 = Vector::new(rect.x(), rect.y() + rect.height());
        let cursor = Cursor::new(pt1, pt2, 2.0).default_animation();
        self.cursor = Some(cursor);
    }

    /// Switches to read-only state
    fn stop_editing(&mut self) {
        self.is_editing = false;
        self.cursor = None;
        self.image_text = None;
        self.editor.update_rendered_text();
    }
}

impl Displayable for TextArea {

    fn get_id(&self) -> u32 { self.layer.get_id() }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<TextArea>()
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
        if self.layer.lock_style { return }
        self.layer.apply_theme(theme);
        self.editor.ctx.font_size = theme.font_size;
        self.layer.border_style = BorderStyle::SolidLine(theme.border_color, theme.border_width);
    }

    fn get_perimeter_frame(&self) -> Option<Rectangle> {
        let perimeter = self.layer.offset_by(10.0, 0.0, 10.0, 0.0);
        Some(perimeter)
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Activate => {
                self.start_editing();
            }
            DisplayEvent::Deactivate => {
                self.stop_editing();
            }
            DisplayEvent::Ready => {
                self.layer.on_ready();
                if self.get_text().len() > 0 {
                    self.editor.update_rendered_text();
                }
                // self.start_editing();
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
            }
            _ => {}
        }
    }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        let offset = Vector::new(state.offset.0, state.offset.1);
        self.layer.frame.pos = self.layer.initial.pos + offset;
        // self.input_frame = self.layer.inset_by(10.0, 10.0, 10.0, 10.0);
        self.layer.tween_update();
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
            let cursor_space = 0.0;
            if self.get_text().len() > 0 {
                if let Some(mesh_task) = &self.editor.ctx.cached_mesh {
                    window.add_task(mesh_task.clone());
                } else {
                    if let Some(text) = self.editor.get_visible_text(self.scroll_offset.y) {
                        let style = FontStyle::new(theme.font_size, Color::BLUE);
                        if let Some(task) = self.editor.ctx.draw_font.draw(
                            &text,
                            &style,
                            HAlign::Left,
                            &self.input_frame,
                            window,
                            true
                        ) {
                            self.editor.ctx.cached_mesh = Some(task.clone());
                            window.add_task(task);
                        }
                    }
                }
            }
            if let Some(cursor) = &mut self.cursor {
                let cursor_pt =
                    Vector::new(self.editor.ctx.cursor_origin.0 + cursor_space, self.editor.ctx.cursor_origin.1);
                cursor.render_at_point(&cursor_pt, &theme, window);
                // log::debug!("frame={:?} cursor={:?}", self.input_frame, cursor_pt);
            }
        } else {
            if let Some(img) = &self.image_text {
                window.draw(&img.area().constrain(&self.input_frame), Img(&img));
            } else {
                if let Some(imgbuf) = self.editor.crop_cached_render(
                    0,
                    self.scroll_offset.y as u32,
                    self.editor.ctx.frame.width() as u32,
                    self.editor.ctx.frame.height() as u32,
                ) {
                    let (text_w, text_h) = imgbuf.dimensions();
                    // log::debug!("image text w={:?} h={:?}", text_w, text_h);
                    let img: Image =
                        Image::from_raw(imgbuf.into_raw().as_slice(), text_w, text_h, PixelFormat::RGBA).unwrap();
                    window.draw(&img.area().constrain(&self.input_frame), Img(&img));
                    self.image_text = Some(img);
                } else {
                    log::debug!("NO IMAGE x={:?} y={:?}", 0, 0);
                }
            }
        }

        // Render scrollbar
        let content_height = self.editor.ctx.text_size.1 as f32;
        if let Some(rect) = UITools::get_scrollbar_frame(content_height, &self.layer.frame, self.scroll_offset.y) {
            window.draw(&rect, Col(Color::from_hex(UITools::SCROLLBAR_COLOR)));
        }

        // Draw border
        self.layer.draw_border(window);
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        self.is_hovering = self.layer.handle_mouse_over(pt);
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
            self.image_text = None;
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
                    self.image_text = None;
                }
            }
            _ => (),
        }
        false
    }

    fn handle_mouse_down(&mut self, pt: &Vector, _state: &mut AppState) -> bool {
        if pt.overlaps_rectangle(&self.layer.frame) {
            if self.is_editing {
                return true;
            }
            self.start_editing();
            return true;
        }
        false
    }

    fn handle_mouse_scroll(&mut self, pt: &Vector, _state: &mut AppState) {
        // Only scroll if hovering and not editing
        if self.is_hovering && !self.is_editing {
            let upper_limit = self.editor.ctx.text_size.1 as f32 - self.layer.frame.height();
            let eval_y = (self.scroll_offset.y + pt.y).max(0.0).min(upper_limit);
            self.scroll_offset.y = eval_y;
            self.image_text = None;
        }
    }
}
