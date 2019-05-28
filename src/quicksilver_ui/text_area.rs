/// TextArea – A simple scrollable text box with editor functionality
///
use super::*;
use crate::core::*;
use crate::shared::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Background::Img, Color, FontStyle, Image, PixelFormat},
    input::{Key, MouseCursor},
    lifecycle::Window,
};

pub use glyph_brush::HorizontalAlign as HAlign;
use std::any::TypeId;
use std::ops::Range;

//-- TextArea -----------------------------------------------------------------------

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_mut)]
pub struct TextArea {
    pub layer: TweenLayer,
    pub placeholder: Option<String>,
    pub cursor: Option<Cursor>,
    input_frame: Rectangle,
    image_text: Option<Image>,
    text_size: (u32, u32),
    editor: TextAreaEditor,
    is_editing: bool,
    is_hovering: bool,
    can_edit: bool,
    can_scroll: bool,
    scroll_offset: Vector,
}

impl TextArea {
    pub fn new(frame: Rectangle, can_edit: bool) -> Self {
        // FIXME: The default() does not load a font and therefore requires a font to be set in set_theme()
        let layer = TweenLayer::new(frame);

        let input_frame = layer.inset_by(10.0, 10.0, 10.0, 10.0);
        // log::debug!("outer frame={:?} input frame={:?}", frame, input_frame);

        let mut editor = TextAreaEditor::default()
            .with_frame((input_frame.x(), input_frame.y()), (input_frame.width(), input_frame.height()));

        // temporary hack to test scroll bar
        editor.ctx.frame.max.x = editor.ctx.frame.max.x - 10.0;
        editor.ctx.debug = true;

        TextArea {
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

    pub fn set_color(&mut self, color: &Color) {
        self.layer.color = color.clone();
    }

    pub fn set_text(&mut self, text: &str) {
        self.editor.ctx.set_text(text);
    }

    pub fn get_text(&self) -> &str {
        return self.editor.ctx.get_text();
    }

    pub fn get_visible_rows(&self) -> Range<usize> {
        let row_height = self.editor.ctx.font_size;
        let shift = self.scroll_offset.y / row_height;
        let start = shift.floor() as usize;

        let row_count = (self.layer.frame.height() / row_height + shift.fract()).ceil() as usize;
        return start..(start + row_count);
    }

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

    fn stop_editing(&mut self) {
        self.is_editing = false;
        self.cursor = None;
        self.image_text = None;
        self.editor.update_rendered_text();
    }
}

impl TKDisplayable for TextArea {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<TextArea>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn set_theme(&mut self, theme: &Theme) {
        self.layer.color = theme.bg_color;
        self.editor.ctx.font_size = theme.font_size;
        if theme.border_width > 0.0 {
            self.layer.border_width = theme.border_width;
            self.layer.border_color = Some(theme.border_color);
        }
        // self.layer.font = theme.font;
        // self.layer.font_size = theme.font_size;
        // self.editor.ctx.set_font_bytes(theme.font_bytes.into();
        // if let Some(raw_font) = &theme.raw_font {
        //     self.editor.ctx.set_font(raw_font.clone());
        // }
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
                    self.editor.update_rendered_text();
                }
                // self.start_editing();
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

    #[allow(unused_mut)]
    fn render(&mut self, theme: &mut Theme, window: &mut Window) -> TKResult {
        window.draw(&self.layer.frame, Col(self.layer.color));

        if self.get_text().len() == 0 {
            return Ok(());
        }

        if self.is_editing {
            self.editor.update_textarea();
            let cursor_space = 0.0;
            if self.get_text().len() > 0 {
                if let Some(text) = self.editor.get_visible_text(self.scroll_offset.y) {
                    let style = FontStyle::new(theme.font_size, Color::BLUE);
                    let _ = self.editor.ctx.gpu_text.draw_text(&text, &style, &self.input_frame, window);
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
        if let Some(color) = self.layer.border_color {
            for line in self.layer.get_border_lines(self.layer.border_width) {
                window.draw_ex(&line.with_thickness(line.t), Col(color), Transform::IDENTITY, 0);
            }
        }

        Ok(())
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        self.is_hovering = self.layer.handle_mouse_over(pt);
        return self.is_hovering;
    }
}

impl TKResponder for TextArea {
    fn get_field_value(&self) -> FieldValue {
        FieldValue::Text(self.get_text().to_owned())
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

    fn handle_mouse_scroll(&mut self, pt: &Vector, _state: &mut TKState) {
        // Only scroll if hovering and not editing
        if self.is_hovering && !self.is_editing {
            let upper_limit = self.editor.ctx.text_size.1 as f32 - self.layer.frame.height();
            let eval_y = (self.scroll_offset.y + pt.y).max(0.0).min(upper_limit);
            self.scroll_offset.y = eval_y;
            self.image_text = None;
        }
    }
}
