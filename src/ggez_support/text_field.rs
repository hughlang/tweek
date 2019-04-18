/// TextField
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
///
///
///
extern crate ggez;

use crate::core::*;

use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawParam, Rect, Text};
use ggez::mint;
use ggez::{Context, GameResult};
use std::any::TypeId;

use super::*;

// *****************************************************************************************************
// TextField
// *****************************************************************************************************

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_mut)]
pub struct TextField {
    pub layer: TweenLayer,
    pub placeholder: Option<String>,
    pub image_text: Option<graphics::Image>,
    pub align: TextAlign,
    editor: TextFieldEditor,
    cursor: Option<Cursor>,
    text_size: (u32, u32),
    is_editing: bool,
    can_edit: bool,
    onclick: Option<Box<FnMut(TKAction, &mut TKState) + 'static>>,
}

impl TextField {
    pub fn new(frame: Rect) -> Self {
        let layer = TweenLayer::new(frame, DrawParam::new().color(graphics::BLACK));
        let input_frame = layer.inset_by(10.0, 10.0, 10.0, 10.0);
        let editor = TextFieldEditor::default().with_frame(
            (input_frame.x, input_frame.y),
            (input_frame.w, input_frame.h),
        );
        log::debug!("outer frame={:?} input frame={:?}", frame, input_frame);
        TextField {
            layer: TweenLayer::new(frame, DrawParam::new().color(graphics::BLACK)),
            placeholder: None,
            image_text: None,
            align: TextAlign::Left,
            editor: editor,
            cursor: None,
            text_size: (0, 0),
            is_editing: false,
            can_edit: true,
            onclick: None,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.editor.ctx.string = text.to_string();
    }

    pub fn get_text(&self) -> &str {
        return &self.editor.ctx.string;
    }

    pub fn set_placeholder(&mut self, text: &str) {
        self.placeholder = Some(text.to_string());
    }

    pub fn set_color(&mut self, hex_color: u32) {
        self.layer.graphics.color = Color::from_rgb_u32(hex_color);
    }

    pub fn set_onclick<C>(&mut self, cb: C)
    where
        C: FnMut(TKAction, &mut TKState) + 'static,
    {
        self.onclick = Some(Box::new(cb));
    }

    fn start_editing(&mut self) {
        self.layer.mouse_state = MouseState::Focus;
        self.is_editing = true;
        self.editor.start_editing();

        let rect = &self.layer.offset_by(10.0, 10.0, 10.0, 10.0);
        let pt1 = mint::Point2 {
            x: rect.x,
            y: rect.y,
        };
        let pt2 = mint::Point2 {
            x: rect.x,
            y: rect.y + rect.h,
        };

        let cursor = Cursor::new(pt1, pt2, 2.0).default_animation();
        self.cursor = Some(cursor);
    }

    fn stop_editing(&mut self) {
        self.is_editing = false;
        self.cursor = None;
    }
}

// *****************************************************************************************************
// TextField :: Displayable
// *****************************************************************************************************

impl TKDisplayable for TextField {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<TextField>()
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
                    self.editor.update_metrics();
                }
            }
        }
    }

    fn get_perimeter_frame(&self) -> Option<Rect> {
        // TODO: Calculate dynamically based on rendered_text width
        let perimeter = self.layer.offset_by(20.0, 0.0, 20.0, 00.0);
        Some(perimeter)
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

    fn render(&mut self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.layer.frame,
            self.layer.graphics.color,
        )?;

        let _ = graphics::draw(ctx, &mesh, self.layer.graphics);

        if self.get_text().len() == 0 {
            // Provide a frame for the cursor to render in. Width is ignored.
            // if self.is_editing {
            //     if let Some(cursor) = &mut self.cursor {
            //         let frame = Rect::new(input_frame.x, input_frame.y, 5.0, 10.0);
            //         cursor.render_inside(&frame, ctx)?;
            //     }
            // }
            // if let Some(text) = &self.placeholder {
            //     let (w, h) = (self.editor.ctx.frame.width() as u32, self.editor.ctx.frame.height() as u32);
            //     if let Some(imgbuf) =
            //         self.editor
            //             .render_text_cropped(&text, self.layer.font_size, 0, 0, w, h)
            //     {
            //         // let (text_w, text_h) = imgbuf.dimensions();
            //         // log::debug!("w={:?} h={:?}", text_w, text_h);
            //         // let img = graphics::Image::from_rgba8(
            //         //     ctx,
            //         //     text_w as u16,
            //         //     text_h as u16,
            //         //     imgbuf.into_raw().as_slice(),
            //         // )?;

            //         // text_origin.y = input_frame.y + (input_frame.h - img.height() as f32) / 2.0;
            //         // let drawparams = graphics::DrawParam::new()
            //         //     .dest(text_origin)
            //         //     .color(self.layer.theme.unfocus_color);
            //         // let _result = graphics::draw(ctx, &img, drawparams);
            //     }
            // }
            return Ok(());
        }

        let origin = self.editor.get_text_origin();
        let text_origin = mint::Point2 {
            x: origin.0,
            y: origin.1,
        };
        // log::debug!("origin={:?} text_size={:?}", origin, self.editor.text_size);
        self.editor.update_display();
        if self.is_editing {
            let cursor_space = 1.0;
            // This refreshes the current cursor position and displayable text range (todo)
            // log::debug!("text_origin={:?}", text_origin);
            if let Some(text) = self.editor.get_visible_text(0.0) {
                let render_text = Text::new((text, self.layer.font, self.layer.font_size));
                // let (text_w, text_h) = render_text.dimensions(ctx);
                // log::debug!("text_w={:?} text_h={:?}", text_w, text_h);
                graphics::draw(
                    ctx,
                    &render_text,
                    self.layer.graphics.dest(text_origin).color(graphics::BLACK),
                )?;
            }

            if let Some(cursor) = &mut self.cursor {
                let cursor_pt = self.editor.ctx.cursor_origin;
                let line_height = self.editor.get_line_height();

                let c_frame = Rect::new(
                    cursor_pt.0 + cursor_space,
                    cursor_pt.1 - line_height / 2.0,
                    10.0,
                    line_height,
                );
                // log::debug!("c_frame={:?} line_height={:?}", c_frame, line_height);
                cursor.render_inside(&c_frame, ctx)?;
            }
        } else {
            // if let Some(imgbuf) = self.editor.render_visible_text() {
            //     let (text_w, text_h) = imgbuf.dimensions();
            //     // log::debug!("w={:?} h={:?}", text_w, text_h);
            //     let img = graphics::Image::from_rgba8(
            //         ctx,
            //         text_w as u16,
            //         text_h as u16,
            //         imgbuf.into_raw().as_slice(),
            //     )?;
            //     text_origin.y = input_frame.y + (input_frame.h - img.height() as f32) / 2.0;
            //     // log::debug!("text_origin={:?} input_frame={:?}", text_origin, input_frame);
            //     let drawparams = graphics::DrawParam::new()
            //         .dest(text_origin)
            //         .color(self.layer.theme.fg_color);
            //     let _result = graphics::draw(ctx, &img, drawparams);
            // }

            if let Some(text) = self.editor.get_visible_text(0.0) {
                let render_text = Text::new((text, self.layer.font, self.layer.font_size));
                // render_text.set_bounds(bounds, Align::Left);
                graphics::draw(
                    ctx,
                    &render_text,
                    self.layer.graphics.dest(text_origin).color(graphics::BLACK),
                )?;
            }
        }

        Ok(())
    }

    fn set_hover_animation(&mut self, props: &[Prop], seconds: f64) {
        self.layer.defaults = Tween::load_props(&self.layer);
        let transition = UITransition::new(props.to_vec(), seconds);
        self.layer.on_hover = Some(transition);
    }
}

// *****************************************************************************************************
// TextField :: Responder
// *****************************************************************************************************

impl TKResponder for TextField {
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
            log::debug!(
                "string='{}' len={}",
                self.editor.ctx.string,
                self.editor.ctx.string.len()
            );
        } else {
            // log::debug!("### non ascii={}", c);
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
            log::debug!("HIT at x={:?} y={:?}", x, y);
            if self.is_editing {
                return false; // to indicate nothing changed
            }
            self.start_editing();
            // if let Some(cb) = &mut self.onclick {
            //     // TODO: modify state or pass new information
            //     (&mut *cb)(TKAction::Click, state);
            // }
            return true;
        }
        false
    }

    fn handle_mouse_at(&mut self, x: f32, y: f32) -> bool {
        return self.layer.handle_mouse_over(x, y);
    }
}
