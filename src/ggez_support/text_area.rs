/// TextArea
/// This is yet another experimental UI component that needs to solve the problem of overflow
/// of content inside a scrollable area. First, we will try drawing to a Canvas and cropping that
/// back into the screen.
/// Also, we can try painting over components that need it.
///
extern crate ggez;

use crate::core::*;
use crate::shared::*;

use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Align, Color, DrawParam, Rect, Text};
use ggez::mint::{self, Point2};
use ggez::{Context, GameResult};
pub use glyph_brush::HorizontalAlign as HAlign;
use std::any::TypeId;
use std::ops::Range;

use super::*;

//-- TextArea -----------------------------------------------------------------------

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_mut)]
pub struct TextArea {
    pub layer: TweenLayer,
    pub placeholder: Option<String>,
    pub align: TextAlign,
    pub cursor: Option<Cursor>,
    last_text: Option<Text>,
    image_text: Option<graphics::Image>,
    text_size: (u32, u32),
    editor: TextAreaEditor,
    is_editing: bool,
    is_hovering: bool,
    can_edit: bool,
    can_scroll: bool,
    scroll_offset: mint::Point2<f32>,
}

impl TextArea {
    pub fn new(frame: Rect, can_edit: bool) -> Self {
        // FIXME: The default() does not load a font and therefore requires a font to be set in set_theme()
        let layer = TweenLayer::new(frame, DrawParam::new().color(graphics::BLACK));

        let input_frame = layer.inset_by(10.0, 10.0, 10.0, 10.0);
        // log::debug!("outer frame={:?} input frame={:?}", frame, input_frame);

        let mut editor =
            TextAreaEditor::default().with_frame((input_frame.x, input_frame.y), (input_frame.w, input_frame.h));

        // temporary hack to test scroll bar
        editor.ctx.frame.max.x = editor.ctx.frame.max.x - 10.0;

        TextArea {
            layer: TweenLayer::new(frame, DrawParam::new().color(graphics::BLACK)),
            placeholder: None,
            align: TextAlign::Left,
            cursor: None,
            last_text: None,
            image_text: None,
            text_size: (0, 0),
            editor: editor,
            is_editing: false,
            is_hovering: false,
            can_edit: can_edit,
            can_scroll: true,
            scroll_offset: Point2 { x: 0.0, y: 0.0 },
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.editor.ctx.string = text.to_string();
    }

    pub fn set_color(&mut self, color: &graphics::Color) {
        self.layer.graphics.color = color.clone();
    }

    pub fn get_text(&self) -> &str {
        return &self.editor.ctx.string;
    }

    pub fn get_visible_rows(&self) -> Range<usize> {
        let row_height = self.editor.ctx.font_size;
        let shift = self.scroll_offset.y / row_height;
        let start = shift.floor() as usize;

        let row_count = (self.layer.frame.h / row_height + shift.fract()).ceil() as usize;
        return start..(start + row_count);
    }

    fn start_editing(&mut self) {
        self.layer.mouse_state = MouseState::Focus;
        self.is_editing = true;
        self.editor.start_editing();

        let rect = &self.layer.offset_by(10.0, 10.0, 10.0, 10.0);
        let pt1 = mint::Point2 { x: rect.x, y: rect.y };
        let pt2 = mint::Point2 { x: rect.x, y: rect.y + rect.h };

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

    fn get_frame(&self) -> Rect {
        return self.layer.frame;
    }

    fn set_theme(&mut self, theme: &Theme) {
        self.layer.graphics.color = theme.bg_color;
        self.layer.font = theme.font;
        self.layer.font_size = theme.font_size;
        self.editor.ctx.font_size = theme.font_size;
        if let Some(raw_font) = &theme.raw_font {
            self.editor.ctx.set_font(raw_font.clone());
        }
    }

    fn get_perimeter_frame(&self) -> Option<Rect> {
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
                }
            }
        }
    }

    fn update(&mut self) -> GameResult {
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
    fn render(&mut self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.layer.frame,
            self.layer.graphics.color,
        )?;

        let _ = graphics::draw(ctx, &mesh, self.layer.graphics);

        if self.get_text().len() == 0 {
            return Ok(());
        }

        let origin = self.editor.get_text_origin();
        let text_origin = mint::Point2 { x: origin.0, y: origin.1 };
        // log::debug!("origin={:?} text_size={:?}", origin, self.editor.text_size);
        self.editor.update_display();

        if self.is_editing {
            let cursor_space = 1.0;

            if let Some(text) = self.editor.get_visible_text(self.scroll_offset.y) {
                let mut render_text = Text::new((text, self.layer.font, self.layer.font_size));
                let bounds = mint::Point2 { x: self.editor.ctx.frame.width(), y: self.editor.ctx.frame.height() };
                render_text.set_bounds(bounds, Align::Left);

                // let (text_w, text_h) = render_text.dimensions(ctx);
                // log::debug!("text_w={:?} text_h={:?}", text_w, text_h);
                graphics::draw(ctx, &render_text, self.layer.graphics.dest(text_origin).color(graphics::BLACK))?;
            }

            if let Some(cursor) = &mut self.cursor {
                let cursor_pt = self.editor.ctx.cursor_origin;
                let line_height = self.editor.ctx.font_size;

                let c_frame = Rect::new(cursor_pt.0 + cursor_space, cursor_pt.1 - line_height / 2.0, 10.0, line_height);
                // log::debug!("c_frame={:?} line_height={:?}", c_frame, line_height);
                cursor.render_inside(&c_frame, ctx)?;
            }
        } else {
            if let Some(imgbuf) = self.editor.crop_cached_render(
                0,
                self.scroll_offset.y as u32,
                self.editor.ctx.frame.width() as u32,
                self.editor.ctx.frame.height() as u32,
            ) {
                let (text_w, text_h) = imgbuf.dimensions();
                // log::debug!("w={:?} h={:?}", text_w, text_h);
                let img = graphics::Image::from_rgba8(ctx, text_w as u16, text_h as u16, imgbuf.into_raw().as_slice())?;
                // text_origin.y = input_frame.y + (input_frame.h - img.height() as f32) / 2.0;
                // log::debug!("text_origin={:?} input_frame={:?}", text_origin, input_frame);
                let drawparams = graphics::DrawParam::new().dest(text_origin);
                let _result = graphics::draw(ctx, &img, drawparams);
            }
        }

        // Render scrollbar
        // Calculate height of scrollbar as ratio between viewport height and total text height
        let rect = self.layer.frame;
        let bar_h = (rect.h / self.editor.ctx.text_size.1 as f32).min(0.2) * rect.h;
        let ypos =
            rect.y + (self.scroll_offset.y / self.editor.ctx.text_size.1 as f32 * rect.h).min(rect.y + rect.h - bar_h);

        let frame = Rect::new(self.layer.frame.right() - 10.0, ypos, 10.0, bar_h);
        let scrollbar =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), frame, Color::from_rgb_u32(0xBBBBBB))?;
        graphics::draw(ctx, &scrollbar, self.layer.graphics)?;

        Ok(())
    }
}

impl TKResponder for TextArea {
    // TODO: How to change mouse_state to None after another Responder gets focus.
    fn has_focus(&self) -> bool {
        self.layer.mouse_state == MouseState::Focus
    }

    fn handle_key_press(&mut self, c: char, _ctx: &mut Context) {
        if c.is_ascii_control() {
            return;
        }
        if self.can_edit && c.is_ascii() {
            self.editor.insert_char(c);
        } else {
            log::debug!("### non ascii={}", c);
        }
    }

    fn handle_key_command(&mut self, code: KeyCode, _keymods: KeyMods, _ctx: &mut Context) -> bool {
        match code {
            KeyCode::Back => {
                self.editor.delete_char();
            }
            KeyCode::Left => {
                self.editor.move_cursor(-1);
            }
            KeyCode::Right => {
                self.editor.move_cursor(1);
            }
            KeyCode::Tab => {
                // Optionally call stop_editing here or let a parent Scene take care of that.
                self.stop_editing();
                return true;
            }
            KeyCode::Return => {}
            _ => (),
        }
        false
    }

    fn handle_mouse_down(&mut self, _x: f32, _y: f32, _state: &mut TKState) -> bool {
        false
    }

    fn handle_mouse_up(&mut self, x: f32, y: f32, _state: &mut TKState) -> bool {
        if self.layer.frame.contains(mint::Point2 { x, y }) {
            if self.is_editing {
                return true;
            }
            self.start_editing();
            return true;
        }
        false
    }

    fn handle_mouse_at(&mut self, x: f32, y: f32) -> bool {
        self.is_hovering = self.layer.handle_mouse_over(x, y);
        return self.is_hovering;
    }

    // TODO: only scroll if hovering
    fn handle_mouse_scroll(&mut self, _x: f32, y: f32, _state: &mut TKState) {
        if self.is_hovering {
            self.last_text = None;
            let upper_limit = self.editor.ctx.text_size.1 as f32 - self.layer.frame.h;
            let eval_y = (self.scroll_offset.y - y).max(0.0).min(upper_limit);
            // log::debug!("-------------- self.scroll_offset.y={:?} y={:?}", self.scroll_offset.y, y);
            self.scroll_offset.y = eval_y;
        }
    }
}
