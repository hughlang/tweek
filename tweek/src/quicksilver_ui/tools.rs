/// Misc tools for common operations for Quicksilver UI
///

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Vector},
    graphics::{Background::Col, Color, Font},
    input::Key,
};

use std::any::TypeId;

pub struct UITools {}

impl UITools {
    pub const SCROLLBAR_WIDTH: f32 = 10.0;
    pub const SCROLLBAR_COLOR: &'static str = "#BBBBBB";

    // pub const TEXT_KEY_COMMANDS: &[Key] = &[
    //     Key::Back,
    //     Key::Tab,
    //     Key::Left,
    //     Key::Right,
    //     Key::Return,
    //     Key::Escape,
    // ];

    pub fn scrollable_types() -> Vec<TypeId> {
        vec![
            // TypeId::of::<ListBox>(),
            // TypeId::of::<TextArea>(),
            // TypeId::of::<TextField>(),
        ]
    }

    pub fn is_scrollable(type_id: &TypeId) -> bool {
        UITools::scrollable_types().contains(type_id)
    }

    pub fn inset_rect(rect: &Rectangle, left: f32, top: f32, right: f32, bottom: f32) -> Rectangle {
        Rectangle::new((rect.x() + left, rect.y() + top), (rect.width() - left - right, rect.height() - top - bottom))
    }

    pub fn inset_left_middle(rect: &Rectangle, object: &Rectangle, margin: f32) -> Rectangle {
        let y = rect.y() + (rect.height() - object.height()) / 2.0;
        Rectangle::new((rect.x() + margin, y), (rect.width() - margin * 2.0, object.height()))
    }

    pub fn get_scrollbar_frame(content_height: f32, rect: &Rectangle, offset: f32) -> Option<Rectangle> {
        if content_height > rect.height() {
            let bar_h = (rect.height() / content_height).min(0.2) * rect.height();
            let upper_limit = content_height - rect.height();

            let ypos = (rect.y() + offset / upper_limit * rect.height()).min(rect.y() + rect.height() - bar_h);

            let rect = Rectangle::new(
                (rect.x() + rect.width() - UITools::SCROLLBAR_WIDTH, ypos),
                (UITools::SCROLLBAR_WIDTH, bar_h),
            );
            return Some(rect);
        }
        None
    }

    pub fn make_border_lines(rect: &Rectangle, width: f32) -> Vec<Line> {
        let mut lines: Vec<Line> = Vec::new();
        let line = Line::new((rect.x(), rect.y()), (rect.x() + rect.width(), rect.y())).with_thickness(width);
        lines.push(line);
        let line = Line::new((rect.x() + rect.width(), rect.y()), (rect.x() + rect.width(), rect.y() + rect.height()))
            .with_thickness(width);
        lines.push(line);
        let line = Line::new((rect.x() + rect.width(), rect.y() + rect.height()), (rect.x(), rect.y() + rect.height()))
            .with_thickness(width);
        lines.push(line);
        let line = Line::new((rect.x(), rect.y() + rect.height()), (rect.x(), rect.y())).with_thickness(width);
        lines.push(line);
        lines
    }

    /**
    * Method which returns the list of Rect areas outside of the object frame.
    * Up to four Rect objects are returned based on the outside Rect param provided.
    * The diagram below illustrates how the Rect areas are defined.
    * Top and Bottom, full width. Left and Right, in between.
    *
       +--------------+
       |              |
       +--------------+
       |  |********|  |
       |  |********|  |
       |  |********|  |
       +--------------+
       |              |
       +--------------+
    */
    pub fn get_perimeter_blocks(frame: &Rectangle, outside: &Rectangle) -> Vec<Rectangle> {
        if outside.pos.x == frame.pos.x
            && outside.pos.y == frame.pos.y
            && outside.size.x == frame.size.x
            && outside.size.y == frame.size.y
        {
            return Vec::new();
        }
        let outside_right = outside.pos.x + outside.size.x;
        let frame_right = frame.pos.x + frame.size.x;
        let outer_bottom = outside.pos.y + outside.size.y;
        let frame_bottom = frame.pos.y + frame.size.y;

        let mut blocks: Vec<Rectangle> = Vec::new();
        if frame.pos.y - outside.pos.y > 0.0 {
            blocks.push(Rectangle::new(
                (outside.pos.x, outside.pos.y),
                (outside.size.x, frame.pos.y - outside.pos.y - 1.0),
            ));
        }
        if outer_bottom - frame_bottom > 0.0 {
            blocks.push(Rectangle::new(
                (outside.pos.x, frame.pos.x + frame.size.x + 1.0),
                (outside.size.x, outer_bottom - frame_bottom),
            ));
        }
        if frame.pos.x - outside.pos.x > 0.0 {
            blocks.push(Rectangle::new(
                (outside.pos.x, frame.pos.x),
                (frame.pos.x - outside.pos.x - 1.0, frame_bottom - frame.pos.y),
            ));
        }
        if outside_right - frame_right > 0.0 {
            blocks.push(Rectangle::new(
                (frame_right + 1.0, frame.pos.y),
                (outside_right - frame_right + 1.0, frame_bottom - frame.pos.y),
            ));
        }
        blocks
    }
}
