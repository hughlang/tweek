/// Misc tools for common operations for Quicksilver UI
///

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Scalar, Shape, Transform, Vector},
    graphics::{Background::Col, Color, DrawTask, Font, GpuTriangle, Mesh, Vertex},
    input::Key,
};

use std::any::TypeId;
use std::f32;

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

    pub fn inset_rect(rect: &Rectangle, left: f32, top: f32, right: f32, bottom: f32) -> Rectangle {
        Rectangle::new((rect.x() + left, rect.y() + top), (rect.width() - left - right, rect.height() - top - bottom))
    }

    pub fn inset_left_middle(rect: &Rectangle, object: &Rectangle, margin: f32) -> Rectangle {
        let y = rect.y() + (rect.height() - object.height()) / 2.0;
        Rectangle::new((rect.x() + margin, y), (rect.width() - margin * 2.0, object.height()))
    }

    pub fn position_left_middle(rect: &Rectangle, object: &Rectangle, margin: f32) -> Rectangle {
        let y = rect.y() + (rect.height() - object.height()) / 2.0;
        Rectangle::new((rect.x() + margin, y), (object.width(), object.height()))
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

    /// Helper function for creating a DrawTask from a Rectangle primitive outside of the base
    /// Drawable implementations done within Quicksilver. This is only necessary for painting over
    /// other objects on screen. This is not ideal, since it's just a hacky way of covering up UI problems.
    /// Later, we hope to have a way of cropping content in the GPU to prevent overflow.
    pub fn draw_rectangles(rectangles: Vec<Rectangle>, color: Color) -> DrawTask {
        let mut task = DrawTask::new(0);
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

const CIRCLE_POINTS: [Vector; 64] = [
    Vector { x: 1.0, y: 0.0 },
    Vector { x: 0.9950307753654014, y: 0.09956784659581666 },
    Vector { x: 0.9801724878485438, y: 0.19814614319939758 },
    Vector { x: 0.9555728057861407, y: 0.2947551744109042 },
    Vector { x: 0.9214762118704076, y: 0.38843479627469474 },
    Vector { x: 0.8782215733702285, y: 0.47825397862131824 },
    Vector { x: 0.8262387743159949, y: 0.5633200580636221 },
    Vector { x: 0.766044443118978, y: 0.6427876096865394 },
    Vector { x: 0.6982368180860729, y: 0.7158668492597184 },
    Vector { x: 0.6234898018587336, y: 0.7818314824680298 },
    Vector { x: 0.5425462638657594, y: 0.8400259231507714 },
    Vector { x: 0.4562106573531629, y: 0.8898718088114687 },
    Vector { x: 0.365341024366395, y: 0.9308737486442042 },
    Vector { x: 0.27084046814300516, y: 0.962624246950012 },
    Vector { x: 0.17364817766693022, y: 0.9848077530122081 },
    Vector { x: 0.07473009358642417, y: 0.9972037971811801 },
    Vector { x: -0.024930691738072913, y: 0.9996891820008162 },
    Vector { x: -0.12434370464748516, y: 0.9922392066001721 },
    Vector { x: -0.22252093395631434, y: 0.9749279121818236 },
    Vector { x: -0.31848665025168454, y: 0.9479273461671317 },
    Vector { x: -0.41128710313061156, y: 0.9115058523116731 },
    Vector { x: -0.5000000000000002, y: 0.8660254037844385 },
    Vector { x: -0.58374367223479, y: 0.8119380057158564 },
    Vector { x: -0.6616858375968595, y: 0.7497812029677341 },
    Vector { x: -0.7330518718298263, y: 0.6801727377709194 },
    Vector { x: -0.7971325072229225, y: 0.6038044103254774 },
    Vector { x: -0.8532908816321556, y: 0.5214352033794981 },
    Vector { x: -0.900968867902419, y: 0.43388373911755823 },
    Vector { x: -0.9396926207859084, y: 0.3420201433256685 },
    Vector { x: -0.969077286229078, y: 0.24675739769029342 },
    Vector { x: -0.9888308262251285, y: 0.14904226617617428 },
    Vector { x: -0.9987569212189223, y: 0.04984588566069704 },
    Vector { x: -0.9987569212189223, y: -0.04984588566069723 },
    Vector { x: -0.9888308262251285, y: -0.14904226617617447 },
    Vector { x: -0.969077286229078, y: -0.24675739769029362 },
    Vector { x: -0.9396926207859084, y: -0.34202014332566866 },
    Vector { x: -0.9009688679024191, y: -0.433883739117558 },
    Vector { x: -0.8532908816321555, y: -0.5214352033794983 },
    Vector { x: -0.7971325072229224, y: -0.6038044103254775 },
    Vector { x: -0.7330518718298262, y: -0.6801727377709195 },
    Vector { x: -0.6616858375968594, y: -0.7497812029677342 },
    Vector { x: -0.5837436722347898, y: -0.8119380057158565 },
    Vector { x: -0.4999999999999996, y: -0.8660254037844388 },
    Vector { x: -0.4112871031306116, y: -0.9115058523116731 },
    Vector { x: -0.3184866502516841, y: -0.9479273461671318 },
    Vector { x: -0.2225209339563146, y: -0.9749279121818236 },
    Vector { x: -0.12434370464748495, y: -0.9922392066001721 },
    Vector { x: -0.024930691738073156, y: -0.9996891820008162 },
    Vector { x: 0.07473009358642436, y: -0.9972037971811801 },
    Vector { x: 0.17364817766693083, y: -0.984807753012208 },
    Vector { x: 0.2708404681430051, y: -0.962624246950012 },
    Vector { x: 0.3653410243663954, y: -0.9308737486442041 },
    Vector { x: 0.45621065735316285, y: -0.8898718088114687 },
    Vector { x: 0.5425462638657597, y: -0.8400259231507713 },
    Vector { x: 0.6234898018587334, y: -0.7818314824680299 },
    Vector { x: 0.698236818086073, y: -0.7158668492597183 },
    Vector { x: 0.7660444431189785, y: -0.6427876096865389 },
    Vector { x: 0.8262387743159949, y: -0.563320058063622 },
    Vector { x: 0.8782215733702288, y: -0.4782539786213178 },
    Vector { x: 0.9214762118704076, y: -0.38843479627469474 },
    Vector { x: 0.9555728057861408, y: -0.2947551744109039 },
    Vector { x: 0.9801724878485438, y: -0.19814614319939772 },
    Vector { x: 0.9950307753654014, y: -0.09956784659581641 },
    Vector { x: 1.0, y: 0.0 },
];
