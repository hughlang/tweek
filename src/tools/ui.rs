/// Misc tools for common operations for Quicksilver UI
///
use super::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Scalar, Shape, Transform, Vector},
    graphics::{Background::Col, Color, MeshTask, Font, GpuTriangle, Mesh, Vertex},
    input::Key,
};

use std::any::TypeId;
use std::f32;

/// Simple utility for manipulated UI data
pub struct UITools {}

impl UITools {
    /// Default width of scrollbar
    /// TODO: Make this configurable
    pub const SCROLLBAR_WIDTH: f32 = 10.0;
    /// Default scrollbar color
    pub const SCROLLBAR_COLOR: &'static str = "#BBBBBB";

    // pub const TEXT_KEY_COMMANDS: &[Key] = &[
    //     Key::Back,
    //     Key::Tab,
    //     Key::Left,
    //     Key::Right,
    //     Key::Return,
    //     Key::Escape,
    // ];

    /// FIXME: Unused
    pub fn scrollable_types() -> Vec<TypeId> {
        vec![
            // TypeId::of::<ListBox>(),
            // TypeId::of::<TextArea>(),
            // TypeId::of::<TextField>(),
        ]
    }

    /// FIXME: Unused
    pub fn is_scrollable(type_id: &TypeId) -> bool {
        UITools::scrollable_types().contains(type_id)
    }

    /// Create a rectangle frame as the union of two Rectangles
    pub fn combine_frames(r1: &Rectangle, r2: &Rectangle) -> Rectangle {
        let x = f32::min(r1.x(), r2.x());
        let y = f32::min(r1.y(), r2.y());
        let w = f32::max(r1.width(), r2.width());
        let h = f32::max(r1.height(), r2.height());
        Rectangle::new((x, y), (w, h))
    }

    /// Create a larger rectange with horizontal padding x and vertical padding y
    pub fn padded_rect(rect: &Rectangle, x: f32, y: f32) -> Rectangle {
        UITools::inset_rect(rect, -x, -y, -x, -y)
    }

    /// Create a Rectangle given the left, top, right, bottom inset values.
    /// Notably, negative values will give you an outset frame.
    pub fn inset_rect(rect: &Rectangle, left: f32, top: f32, right: f32, bottom: f32) -> Rectangle {
        Rectangle::new((rect.x() + left, rect.y() + top), (rect.width() - left - right, rect.height() - top - bottom))
    }

    /// Create an inset Rectangle given an outer Rectangle and an inner one
    pub fn inset_left_middle(rect: &Rectangle, object: &Rectangle, margin: f32) -> Rectangle {
        let y = rect.y() + (rect.height() - object.height()) / 2.0;
        Rectangle::new((rect.x() + margin, y), (rect.width() - margin * 2.0, object.height()))
    }

    /// Create an inset Rectangle that is aligned left inside the outer rectangle.
    /// A way of creating left-aligned content inside a rect with a specified margin
    pub fn position_left_middle(rect: &Rectangle, object: &Rectangle, margin: f32) -> Rectangle {
        let y = rect.y() + (rect.height() - object.height()) / 2.0;
        Rectangle::new((rect.x() + margin, y), (object.width(), object.height()))
    }

    /// Calculate an area for a scrollbar inside another Rectangle
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

    /// Given a rectangle, generate an array of Lines for all 4 sides
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
        if (outside.pos.x - frame.pos.x).abs() < FLOAT_TOLERANCE
            && (outside.pos.y - frame.pos.y).abs() < FLOAT_TOLERANCE
            && (outside.size.x - frame.size.x).abs() < FLOAT_TOLERANCE
            && (outside.size.y - frame.size.y).abs() < FLOAT_TOLERANCE
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

    /// Helper function for creating a MeshTask from a Rectangle primitive outside of the base
    /// Drawable implementations done within Quicksilver. This is only necessary for painting over
    /// other objects on screen. This is not ideal, since it's just a hacky way of covering up UI problems.
    /// Later, we hope to have a way of cropping content in the GPU to prevent overflow.
    pub fn draw_rectangles(rectangles: Vec<Rectangle>, color: Color) -> MeshTask {
        let mut task = MeshTask::new(0);
        for rect in rectangles {
            let offset = task.vertices.len() as u32;
            // top left
            task.vertices.push(Vertex::new(Vector::new(rect.x(), rect.y()), None, Col(color)));
            // top right
            task.vertices.push(Vertex::new(Vector::new(rect.x() + rect.width(), rect.y()), None, Col(color)));
            // bottom right
            task.vertices.push(Vertex::new(
                Vector::new(rect.x() + rect.width(), rect.y() + rect.height()),
                None,
                Col(color),
            ));
            // bottom left
            task.vertices.push(Vertex::new(Vector::new(rect.x(), rect.y() + rect.height()), None, Col(color)));

            task.triangles.push(GpuTriangle::new(offset, [0, 1, 2], 9, Col(color)));
            task.triangles.push(GpuTriangle::new(offset, [2, 3, 0], 9, Col(color)));
        }
        task
    }
}

