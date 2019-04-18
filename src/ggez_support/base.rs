/// Base UI stuff
///
extern crate ggez;
use crate::core::*;

use std::any::TypeId;
use std::f32;
use std::fs::File;
use std::io::Read;

use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color, Font, Rect};
use ggez::{Context, GameResult};

use glyph_brush::rusttype::Font as RTFont;

pub enum DisplayEvent {
    Activate,
    Deactivate,
    Ready,
}

/// This trait lives in ggez_support because it is heavily tied into ggez.
/// It defines the necessary methods for operating within a ggez run loop to provide info, prepare objects for
/// display, and render them.
pub trait TKDisplayable {
    fn get_type_id(&self) -> TypeId;

    fn get_frame(&self) -> Rect;

    /// This trait method should set the specified theme to the TweenLayer and
    /// to apply the colors and fonts for each Displayable object.
    /// This is the default action. Uncomment this line or apply the them to a child Displayable property
    /// self.layer.theme = theme.clone();
    fn set_theme(&mut self, _theme: &Theme);

    fn notify(&mut self, _event: &DisplayEvent) {}

    /// Purpose: apply default props
    fn reset(&mut self) {}

    /// This method is essential if you are animating display characteristics and
    /// expect the object to return to its original state.
    fn load_defaults(&mut self) {}

    fn update(&mut self) -> GameResult {
        Ok(())
    }
    fn render(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn render_inside(&mut self, _rect: &Rect, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn get_perimeter_frame(&self) -> Option<Rect> {
        None
    }

    fn set_hover_animation(&mut self, _props: &[Prop], _seconds: f64) {}
}

impl TKDisplayable {}

/// This trait is implemented by ButtonView and other controls to conveniently handle mouse
/// events in a game/animation runloop. The mutable TKState parameter allows the developer
/// to arbitrarily add u32 values to specify that a specific action should be handled in
/// another part of the code.
pub trait TKResponder: TKDisplayable {
    fn has_focus(&self) -> bool {
        false
    }

    /// The mouse was moved; it provides both absolute x and y coordinates in the window,
    /// and relative x and y coordinates compared to its last position.
    fn handle_mouse_at(&mut self, _x: f32, _y: f32) -> bool {
        false
    }

    /// A mouse button was pressed
    fn handle_mouse_down(&mut self, _x: f32, _y: f32, _state: &mut TKState) -> bool {
        false
    }

    /// A mouse button was released
    fn handle_mouse_up(&mut self, _x: f32, _y: f32, _state: &mut TKState) -> bool {
        false
    }

    /// The mousewheel was scrolled, vertically (y, positive away from and negative toward the user)
    /// or horizontally (x, positive to the right and negative to the left).
    fn handle_mouse_scroll(&mut self, _x: f32, _y: f32, _state: &mut TKState) {}

    /// A keyboard button was pressed.
    fn handle_key_press(&mut self, _c: char, _ctx: &mut Context) {}

    /// TODO: Handle all kinds of command keys: backspace, enter, etc.
    /// A true response means the parent Scene or other entity should evaluate the response.
    fn handle_key_command(
        &mut self,
        _code: KeyCode,
        _keymods: KeyMods,
        _ctx: &mut Context,
    ) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub font: Font,
    pub font_size: f32,
    pub title_font: Font,
    pub title_font_size: f32,
    pub raw_font: Option<RTFont<'static>>,
    pub raw_title_font: Option<RTFont<'static>>,
    pub bg_color: Color,
    pub fg_color: Color,
    pub button_bg_color: Color,
    pub button_fg_color: Color,
    pub border_color: Color,
    pub focus_color: Color,
    pub unfocus_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            font: Font::default(),
            font_size: 14.0,
            title_font: Font::default(),
            title_font_size: 16.0,
            raw_font: None,
            raw_title_font: None,
            bg_color: graphics::WHITE,
            fg_color: graphics::BLACK,
            button_bg_color: Color::from_rgb_u32(0x999999),
            button_fg_color: Color::from_rgb_u32(0xFFFFFF),
            border_color: Color::from_rgb_u32(0xAAAAAA),
            focus_color: Color::from_rgb_u32(0x000000),
            unfocus_color: Color::from_rgb_u32(0x777777),
        }
    }
}

impl Theme {
    /// Helper method for setting self.font and loading the corresponding Rusttype font.
    /// The path parameter should only be the subpath within the environment and is only used for
    /// loading the Rusttype font.
    /// The filename awkwardly requires a forward slash at the start.
    pub fn load_normal_font(&mut self, path: &str, filename: &str, ctx: &mut Context) {
        let font = graphics::Font::new(ctx, filename);
        if font.is_ok() {
            self.font = font.unwrap();

            // load RT font
            let font_path = format!("{}{}{}", env!("CARGO_MANIFEST_DIR"), path, filename);
            self.raw_font = Theme::load_raw_font(&font_path);
        }
    }

    pub fn load_title_font(&mut self, path: &str, filename: &str, ctx: &mut Context) {
        let font = graphics::Font::new(ctx, filename);
        if font.is_ok() {
            self.title_font = font.unwrap();

            // load RT font
            let font_path = format!("{}{}{}", env!("CARGO_MANIFEST_DIR"), path, filename);
            self.raw_title_font = Theme::load_raw_font(&font_path);
        }
    }

    pub fn load_raw_font(path: &str) -> Option<RTFont<'static>> {
        let f = File::open(path);
        if f.is_ok() {
            let mut buffer = Vec::new();
            let _ = f.unwrap().read_to_end(&mut buffer);
            let font: RTFont<'static> = RTFont::from_bytes(buffer).unwrap();
            return Some(font);
        }
        None
    }
}

pub enum TextAlign {
    Left,
    Center,
    Right,
}
