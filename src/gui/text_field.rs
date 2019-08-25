/// TextField – A simple scrollable text box with editor functionality
/// This is a single-line text input field that can handle most of the standard use cases:
/// * On click, display the cursor
/// * On ascii input, insert character at current cursor position
/// * On backspace, delete character left of cursor position
/// * On left arrow, move cursor before previous character
/// * On right arrow, move cursor to right if not at end position
///
/// View mode vs. Edit mode:
/// * The initial state of the textfield is view mode and is_editing=false. There may be pre-existing
/// text or if blank, there may be placeholder text. All content is left aligned and text that is
/// wider than the textfield must be truncated.
/// * In edit mode, the textfield will either start with no content or previously entered content.
/// If placeholder text exists for an empty field, it will be hidden. If previous text exists, it will
/// be displayed left-aligned and the cursor will be at the beginning. Later, this may change with all
/// text selected. (like mobile browser url field)
///
/// The challenge of cursor positioning:
/// * When cursor is at end, get width of rendered text and place cursor there.
///   – If size of rendered text is larger than width of input_frame, anchor text input to right side.
///     Also, calculate the approximate number of characters to display left of the cursor, since there
///     is no easy means of masking rendered text. Also shorter text means better performance.
/// * If cursor is inserting within the string, a temporary buffer is
/// TODO:
/// * Hyperlink fieldtype
/// * Password fieldtype
///
/// Known Issues:
/// * Special chars cause crash. Try: textfield.set_text("čćdđe ёєжзѕиіїйјк");
///
use super::*;
use crate::core::*;
use crate::events::*;
use crate::tools::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Background::Img, Color, FontStyle, Image},
    input::{Key, MouseCursor},
    lifecycle::Window,
};
use glyph_brush::HorizontalAlign as HAlign;

use std::any::TypeId;

/// A type specifier that allows the textfield to have different behaviors, such as password input,
/// URL rendering, etc.
/// Future: date input
pub enum TextFieldType {
    /// Basic text
    Normal,
    /// Passwords and other text that are masked with the specified char
    Secure(char),
    /// It should display as hyperlink with click to open browser
    /// TODO: Implement
    Url,
}

/// Dot char for secure text masking
pub const MASK_TEXT_DOT: char = '•';
/// Asterisk char for secure text masking
pub const MASK_TEXT_ASTERISK: char = '*';

//-- TextField -----------------------------------------------------------------------

/// UI component to display an editable text field
pub struct TextField {
    /// The base layer
    layer: Layer,
    /// Placeholder text to display if the field is empty
    placeholder: Option<String>,
    /// Cursor to display while editing
    cursor: Option<Cursor>,
    /// Field type enum
    field_type: TextFieldType,
    /// The internal bounds of the text
    input_frame: Rectangle,
    /// Cached copy of the rendered text for read mode
    image_text: Option<Image>,
    /// The Editor which handles all text editing state
    editor: TextFieldEditor,
    // draw_font: DrawFont<'a>,
    is_editing: bool,
    is_hovering: bool,
    can_edit: bool,
    scroll_offset: Vector,
}

impl TextField {
    /// Constructor
    pub fn new(frame: Rectangle, theme: &mut Theme, can_edit: bool) -> Self {
        // FIXME: The default() does not load a font and therefore requires a font to be set in set_theme()
        let layer = Layer::new(frame);

        let input_frame = layer.inset_by(10.0, 10.0, 10.0, 10.0);
        // log::debug!("outer frame={:?} input frame={:?}", frame, input_frame);

        let mut editor = TextFieldEditor::create(theme)
            .with_frame((input_frame.x(), input_frame.y()), (input_frame.width(), input_frame.height()));
        editor.ctx.debug = true;

        TextField {
            layer: Layer::new(frame),
            placeholder: None,
            cursor: None,
            field_type: TextFieldType::Normal,
            input_frame,
            image_text: None,
            editor,
            is_editing: false,
            is_hovering: false,
            can_edit,
            scroll_offset: Vector::new(0.0, 0.0),
        }
    }

    /// Builder method to define the TextFieldType
    pub fn with_type(mut self, field_type: TextFieldType) -> Self {
        self.field_type = field_type;
        self
    }

    /// Set the text in the field
    pub fn set_text(&mut self, text: &str) {
        self.editor.ctx.set_text(text);
    }

    /// Get the text in the field
    pub fn get_text(&self) -> &str {
        return &self.editor.ctx.get_text();
    }

    /// Set the placeholder text
    pub fn set_placeholder(&mut self, text: &str) {
        self.placeholder = Some(text.to_string());
    }

    /// Switch to editing mode
    fn start_editing(&mut self) {
        self.layer.mouse_state = MouseState::Focus;
        self.is_editing = true;
        self.editor.ctx.start_editing();

        let rect = self.input_frame;
        let pt1 = Vector::new(rect.x(), rect.y());
        let pt2 = Vector::new(rect.x(), rect.y() + rect.height());
        let cursor = Cursor::new(pt1, pt2, 2.0).default_animation();
        self.cursor = Some(cursor);
    }

    /// Switch read mode
    fn stop_editing(&mut self) {
        self.is_editing = false;
        self.image_text = None;
        self.cursor = None;
    }
}

impl Displayable for TextField {

    fn get_id(&self) -> u32 { self.layer.get_id() }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<TextField>()
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

    // fn get_perimeter_frame(&self) -> Option<Rectangle> {
    //     let perimeter = self.layer.offset_by(10.0, 0.0, 10.0, 0.0);
    //     Some(perimeter)
    // }

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
                // self.editor.ctx.gpu_text.setup_gpu();
                if self.get_text().len() > 0 {
                    self.editor.ctx.update_metrics();
                }
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
        // FIXME: Make themeable
        // self.input_frame = self.layer.inset_by(10.0, 10.0, 10.0, 10.0);
        self.layer.tween_update();
        if let Some(cursor) = &mut self.cursor {
            cursor.update(window, state);
        }
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        self.layer.draw_background(window);
        if self.is_editing {
            self.editor.update_textfield();
            let mut cursor_x = self.editor.ctx.cursor_origin.0;

            if self.get_text().len() > 0 {
                if let Some(mesh_task) = &self.editor.ctx.cached_mesh {
                    window.add_task(mesh_task.clone());
                } else {
                    if let Some(text) = self.editor.get_visible_text(0.0) {
                        let text = {
                            match self.field_type {
                                TextFieldType::Secure(c) => {
                                    let mask = c.to_string().repeat(text.len());
                                    let size = self.editor.ctx.measure_text(&mask);
                                    cursor_x = self.input_frame.x() + size.0;
                                    mask
                                }
                                _ => text,
                            }
                        };
                        let style = FontStyle::new(theme.font_size, Color::BLUE);
                        if let Some(task) = self.editor.ctx.draw_font.draw(
                            &text,
                            &style,
                            HAlign::Left,
                            &self.input_frame,
                            window,
                            false
                        ) {
                            self.editor.ctx.cached_mesh = Some(task.clone());
                            window.add_task(task);
                        }
                    }
                }
            }
            if let Some(cursor) = &mut self.cursor {
                let y1 = self.input_frame.y() + (self.input_frame.height() - self.editor.ctx.font_size) / 2.0;
                let y2 = self.input_frame.y() + (self.input_frame.height() + self.editor.ctx.font_size) / 2.0;
                cursor.render_line(&Vector::new(cursor_x, y1), &Vector::new(cursor_x, y2), &theme, window);
                // log::debug!("frame={:?} cursor={:?}", self.input_frame, cursor_pt);
            }
        } else {
            if self.get_text().len() > 0 {
                if let Some(img) = &self.image_text {
                    window.draw(&img.area().constrain(&self.input_frame), Img(&img));
                } else {
                    if let Some(text) = self.editor.get_visible_text(0.0) {
                        let text = {
                            match self.field_type {
                                TextFieldType::Secure(c) => c.to_string().repeat(text.len()),
                                _ => text,
                            }
                        };
                        let style = FontStyle::new(theme.font_size, Color::BLACK);

                        if let Some(img) = self.editor.ctx.draw_font.render(
                            &text,
                            &style,
                            &self.input_frame,
                            false
                        ) {
                            window.draw(&img.area().constrain(&self.input_frame), Img(&img));
                            self.image_text = Some(img);
                        }

                        // let img = theme.font.render(&text, &style).unwrap();
                        // window.draw(&img.area().constrain(&self.input_frame), Img(&img));
                        // self.image_text = Some(img);
                    }
                }
            } else {
                if let Some(img) = &self.image_text {
                    window.draw(&img.area().constrain(&self.input_frame), Img(&img));
                } else if let Some(text) = &self.placeholder {
                    let style = FontStyle::new(theme.font_size, Color::from_hex("#AAAAAA"));
                    if let Some(img) = self.editor.ctx.draw_font.render(
                        &text,
                        &style,
                        &self.input_frame,
                        false
                    ) {
                        window.draw(&img.area().constrain(&self.input_frame), Img(&img));
                        self.image_text = Some(img);
                    }

                    // let img = theme.font.render(&text, &style).unwrap();
                    // window.draw(&img.area().constrain(&self.input_frame), Img(&img));
                    // self.image_text = Some(img);
                } else {
                    log::debug!("No cached image and no placeholder text");
                }
            }
        }

        // Draw border
        self.layer.draw_border(window);
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        self.is_hovering = self.layer.handle_mouse_over(pt);
        return self.is_hovering;
    }
}

impl Responder for TextField {
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
            log::debug!("### non ascii={}", c);
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
                self.stop_editing();
                return true;
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
            let eval_y = (self.scroll_offset.y - pt.y).max(0.0).min(upper_limit);
            self.scroll_offset.y = eval_y;
        }
    }
}
