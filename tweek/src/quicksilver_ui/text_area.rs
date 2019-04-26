/// TextArea – A simple scrollable text box with editor functionality
///
extern crate quicksilver;

use super::*;
use crate::core::*;
use crate::shared::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Background::Img, Color, FontStyle, Image, ImmiRender, ImmiStatus, PixelFormat},
    lifecycle::{run, Settings, State, Window},
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

    pub fn set_text(&mut self, text: &str) {
        self.editor.ctx.string = text.to_string();
    }

    pub fn set_color(&mut self, color: &Color) {
        self.layer.color = color.clone();
    }

    pub fn get_text(&self) -> &str {
        return &self.editor.ctx.string;
    }

    pub fn get_visible_rows(&self) -> Range<usize> {
        let row_height = self.editor.ctx.font_size;
        let shift = self.scroll_offset.y / row_height;
        let start = shift.floor() as usize;

        let row_count = (self.layer.frame.height() / row_height + shift.fract()).ceil() as usize;
        return start..(start + row_count);
    }

    fn start_editing(&mut self) {
        self.layer.mouse_state = MouseState::Focus;
        self.is_editing = true;
        self.editor.start_editing();

        let rect = self.input_frame;
        let pt1 = Vector::new(rect.x(), rect.y());
        let pt2 = Vector::new(rect.x(), rect.y() + rect.height());
        let cursor = Cursor::new(pt1, pt2, 2.0).default_animation();
        self.cursor = Some(cursor);
    }

    fn stop_editing(&mut self) {
        self.is_editing = false;
        self.cursor = None;
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
        // self.layer.font = theme.font;
        // self.layer.font_size = theme.font_size;
        self.editor.ctx.font_size = theme.font_size;
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
                if self.get_text().len() > 0 {
                    self.editor.update_rendered_text();
                    self.start_editing();
                }
            }
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
    fn render(&mut self, theme: &Theme, window: &mut Window) -> TKResult {
        window.draw(&self.layer.frame, Col(self.layer.color));

        if self.get_text().len() == 0 {
            return Ok(());
        }

        // let origin = self.editor.get_text_origin();
        // let text_origin = Vector::new(origin.0, origin.1);

        // log::debug!("origin={:?} text_size={:?}", origin, self.editor.text_size);
        self.editor.update_display();

        if self.is_editing {
            let cursor_space = 1.0;
            if let Some(img) = &self.image_text {
                let frame = Rectangle::new(self.input_frame.pos, img.area().size);
                window.draw(&img.area().constrain(&frame), Img(&img));
            } else {
                // if let Some(text) = self.editor.get_wrapped_text(self.scroll_offset.y) {
                //     let style = FontStyle::new(theme.font_size, Color::BLACK);
                //     let img = theme.font.render(&text, &style).unwrap();
                //     // eprintln!("image_text={:?} frame={:?}", &image_text.area(), &self.input_frame);
                //     // window.draw(&image_text.area().constrain(&self.input_frame), Img(&image_text));
                //     let frame = Rectangle::new(self.input_frame.pos, img.area().size);
                //     window.draw(&img.area().constrain(&frame), Img(&img));
                //     self.image_text = Some(img);
                // }
                if let Some(imgbuf) = self.editor.render_visible_text(self.scroll_offset.y) {
                    let (text_w, text_h) = imgbuf.dimensions();
                    // eprintln!("w={:?} h={:?}", text_w, text_h);
                    let img: Image =
                        Image::from_raw(imgbuf.into_raw().as_slice(), text_w, text_h, PixelFormat::RGBA).unwrap();
                    let frame = Rectangle::new(self.input_frame.pos, img.area().size);
                    window.draw(&img.area().constrain(&frame), Img(&img));
                    self.image_text = Some(img);
                }

                // let vectors = self.editor.vertices_for_text(self.scroll_offset.y);
                // let trans = Transform::IDENTITY;
                // // let tex_trans = bkg.image().map(|img| img.projection(Rectangle::new_sized((1, 1))));
                // let bkg = Col(Color::BLACK);
                // let offset = window.mesh().add_positioned_vertices(vectors.iter().cloned(), trans, None, bkg);
            }

            if let Some(cursor) = &mut self.cursor {
                let cursor_pt =
                    Vector::new(self.editor.ctx.cursor_origin.0 + cursor_space, self.editor.ctx.cursor_origin.1);
                cursor.render_at_point(&cursor_pt, &theme, window);
                // eprintln!("frame={:?} cursor={:?}", self.input_frame, cursor_pt);
            }
        } else {
            if let Some(imgbuf) = self.editor.crop_cached_render(
                0,
                self.scroll_offset.y as u32,
                self.editor.ctx.frame.width() as u32,
                self.editor.ctx.frame.height() as u32,
            ) {
                let (text_w, text_h) = imgbuf.dimensions();

                let img: Image =
                    Image::from_raw(imgbuf.into_raw().as_slice(), text_w, text_h, PixelFormat::RGBA).unwrap();
                window.draw(&img.area().constrain(&self.input_frame), Img(&img));
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
}

impl TKResponder for TextArea {
    // TODO: How to change mouse_state to None after another Responder gets focus.
    fn has_focus(&self) -> bool {
        self.layer.mouse_state == MouseState::Focus
    }

    fn handle_key_press(&mut self, c: char, _window: &mut Window) {
        if c.is_ascii_control() {
            return;
        }
        if self.can_edit && c.is_ascii() {
            self.editor.insert_char(c);
            self.image_text = None;
        } else {
            log::debug!("### non ascii={}", c);
        }
    }

    // fn handle_key_command(&mut self, code: KeyCode, _keymods: KeyMods, _ctx: &mut Context) -> bool {
    //     match code {
    //         KeyCode::Back => {
    //             self.editor.delete_char();
    //         }
    //         KeyCode::Left => {
    //             self.editor.move_cursor(-1);
    //         }
    //         KeyCode::Right => {
    //             self.editor.move_cursor(1);
    //         }
    //         KeyCode::Tab => {
    //             // Optionally call stop_editing here or let a parent Scene take care of that.
    //             self.stop_editing();
    //             return true;
    //         }
    //         KeyCode::Return => {}
    //         _ => (),
    //     }
    //     false
    // }

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
