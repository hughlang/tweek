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
use super::*;
use crate::core::*;
use crate::shared::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Background::Img, Color, FontStyle, Image},
    input::{Key, MouseCursor},
    lifecycle::Window,
};

use std::any::TypeId;

//-- TextField -----------------------------------------------------------------------

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_mut)]
pub struct TextField {
    pub layer: TweenLayer,
    pub placeholder: Option<String>,
    pub cursor: Option<Cursor>,
    input_frame: Rectangle,
    image_text: Option<Image>,
    text_size: (u32, u32),
    editor: TextFieldEditor,
    is_editing: bool,
    is_hovering: bool,
    can_edit: bool,
    can_scroll: bool,
    scroll_offset: Vector,
}

impl TextField {
    pub fn new(frame: Rectangle, can_edit: bool) -> Self {
        // FIXME: The default() does not load a font and therefore requires a font to be set in set_theme()
        let layer = TweenLayer::new(frame);

        let input_frame = layer.inset_by(10.0, 10.0, 10.0, 10.0);
        // log::debug!("outer frame={:?} input frame={:?}", frame, input_frame);

        let mut editor = TextFieldEditor::default()
            .with_frame((input_frame.x(), input_frame.y()), (input_frame.width(), input_frame.height()));
        editor.ctx.debug = true;

        TextField {
            layer: TweenLayer::new(frame),
            placeholder: None,
            cursor: None,
            input_frame: input_frame,
            image_text: None,
            text_size: (0, 0),
            editor: editor,
            is_editing: false,
            is_hovering: false,
            can_edit: can_edit,
            can_scroll: true,
            scroll_offset: Vector::new(0.0, 0.0),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.editor.ctx.string = text.to_string();
    }

    pub fn set_placeholder(&mut self, text: &str) {
        self.placeholder = Some(text.to_string());
    }

    pub fn set_color(&mut self, color: &Color) {
        self.layer.color = color.clone();
    }

    pub fn get_text(&self) -> &str {
        return &self.editor.ctx.string;
    }

    fn start_editing(&mut self) {
        log::debug!("TextField start_editing");
        self.layer.mouse_state = MouseState::Focus;
        self.is_editing = true;
        self.editor.ctx.start_editing();

        let rect = self.input_frame;
        let pt1 = Vector::new(rect.x(), rect.y());
        let pt2 = Vector::new(rect.x(), rect.y() + rect.height());
        let cursor = Cursor::new(pt1, pt2, 2.0).default_animation();
        self.cursor = Some(cursor);
    }

    fn stop_editing(&mut self) {
        log::debug!("TextField stop_editing");
        self.is_editing = false;
        self.image_text = None;
        self.cursor = None;
    }
}

impl TKDisplayable for TextField {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<TextField>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn set_theme(&mut self, theme: &Theme) {
        self.layer.color = theme.bg_color;
        self.editor.ctx.font_size = theme.font_size;

        // let font = FontCollection::from_bytes(&theme.font_bytes)
        //     .unwrap()
        //     .into_font()
        //     .unwrap();

        // let font = RTFont::from_bytes(&*theme.font_bytes).unwrap();

        // if
        // self.editor.ctx.set_font_bytes(theme.font_bytes.clone_into());
        // let bytes: &[u8] = &*theme.font_bytes;
        // let font = RTFont::from_bytes(bytes);
        // if font.is_ok() {
        //     self.editor.ctx.set_font(font.unwrap());
        // }
        if theme.border_width > 0.0 {
            self.layer.border_width = theme.border_width;
            self.layer.border_color = Some(theme.border_color);
        }
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
                self.editor.ctx.gpu_text.setup_gpu();
                if self.get_text().len() > 0 {
                    self.editor.ctx.update_metrics();
                }
            }
            DisplayEvent::Resize(_screen_size) => {}
        }
    }

    fn update(&mut self) -> TKResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.apply_updates(&update.props);
            }
        }
        if let Some(cursor) = &mut self.cursor {
            cursor.update()?;
        }
        Ok(())
    }

    // #[allow(unused_mut)]
    fn render(&mut self, theme: &Theme, window: &mut Window) -> TKResult {
        window.draw(&self.layer.frame, Col(self.layer.color));

        if self.is_editing {
            self.editor.update_textfield();
            let style = FontStyle::new(theme.font_size, Color::BLUE);
            if self.get_text().len() > 0 {
                if let Some(text) = self.editor.get_visible_text(0.0) {
                    let _ = self.editor.ctx.gpu_text.draw_text(&text, &style, &self.input_frame, window);
                }
            }
            if let Some(cursor) = &mut self.cursor {
                let x = self.editor.ctx.cursor_origin.0;
                let y1 = self.input_frame.y() + (self.input_frame.height() - self.editor.ctx.font_size) / 2.0;
                let y2 = self.input_frame.y() + (self.input_frame.height() + self.editor.ctx.font_size) / 2.0;
                cursor.render_line(&Vector::new(x, y1), &Vector::new(x, y2), &theme, window);
                // log::debug!("frame={:?} cursor={:?}", self.input_frame, cursor_pt);
            }
        } else {
            if self.get_text().len() > 0 {
                if let Some(img) = &self.image_text {
                    window.draw(&img.area().constrain(&self.input_frame), Img(&img));
                } else {
                    if let Some(text) = self.editor.get_visible_text(0.0) {
                        let style = FontStyle::new(theme.font_size, Color::BLACK);
                        let img = theme.font.render(&text, &style).unwrap();
                        window.draw(&img.area().constrain(&self.input_frame), Img(&img));
                        self.image_text = Some(img);
                    }
                }
            } else {
                if let Some(img) = &self.image_text {
                    window.draw(&img.area().constrain(&self.input_frame), Img(&img));
                } else if let Some(text) = &self.placeholder {
                    let style = FontStyle::new(theme.font_size, Color::from_hex("#AAAAAA"));
                    let img = theme.font.render(&text, &style).unwrap();
                    window.draw(&img.area().constrain(&self.input_frame), Img(&img));
                    self.image_text = Some(img);
                } else {
                    log::debug!("No cached image and no placeholder text");
                }
            }
        }

        // Draw border
        if let Some(color) = self.layer.border_color {
            for line in self.layer.get_border_lines(self.layer.border_width) {
                window.draw_ex(&line.with_thickness(line.t), Col(color), Transform::IDENTITY, 0);
            }
        }

        Ok(())
    }
}

impl TKResponder for TextField {
    fn get_text_content(&self) -> Option<String> {
        Some(self.get_text().to_owned())
    }

    // TODO: How to change mouse_state to None after another Responder gets focus.
    fn has_focus(&self) -> bool {
        self.layer.mouse_state == MouseState::Focus
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

    fn handle_mouse_down(&mut self, pt: &Vector, _state: &mut TKState) -> bool {
        if pt.overlaps_rectangle(&self.layer.frame) {
            if self.is_editing {
                return true;
            }
            self.start_editing();
            return true;
        }
        false
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        self.is_hovering = self.layer.handle_mouse_over(pt);
        return self.is_hovering;
    }

    fn handle_mouse_scroll(&mut self, pt: &Vector, _state: &mut TKState) {
        // Only scroll if hovering and not editing
        if self.is_hovering && !self.is_editing {
            let upper_limit = self.editor.ctx.text_size.1 as f32 - self.layer.frame.height();
            let eval_y = (self.scroll_offset.y - pt.y).max(0.0).min(upper_limit);
            self.scroll_offset.y = eval_y;
        }
    }
}
