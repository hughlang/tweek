//! The Shared module contains miscellaneous utilities for GUI components. Currently, it is
//! Quicksilver only, so this module/files may get renamed or moved in the future.

pub use self::app_delegate::*;
pub use self::command::*;
pub use self::draw_font::*;
pub use self::draw_shape::*;
pub use self::editor::*;
pub use self::fonts::*;
pub use self::ui::*;

mod app_delegate;
mod command;
mod draw_font;
mod draw_shape;
mod editor;
mod fonts;
mod ui;

/// Used for checking equality between floats
/// TODO: Use EPSILON? https://doc.rust-lang.org/beta/std/f32/constant.EPSILON.html
pub const FLOAT_TOLERANCE: f32 = 0.001;

