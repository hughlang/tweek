/// Utils for drawing shapes as GPU vertices and indices using Lyon
///
///
use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Color, Mesh, ShapeRenderer},
    lyon::{
        geom::math::*,
        tessellation::{
            basic_shapes::*,
            {FillOptions, StrokeOptions},
        },
    },
};

/// A utility struct for creating Quicksilver Mesh objects for a variety of shapes using Lyon
pub struct DrawShape {}

impl DrawShape {
    pub fn line(points: &[&Vector; 2], color: Color, line_width: f32) -> Mesh {
        let mut mesh = Mesh::new();
        let pt1 = point(points[0].x, points[0].y);
        let pt2 = point(points[1].x, points[1].y);

        let mut renderer = ShapeRenderer::new(&mut mesh, color);
        let options = StrokeOptions::tolerance(0.01).with_line_width(line_width);
        stroke_polyline([pt1, pt2].iter().cloned(), false, &options, &mut renderer).unwrap();

        mesh
    }

    /// Draws a circle with optional fill and border
    pub fn circle(center: &Vector, radius: f32, fill: Option<Color>, line: Option<Color>, line_width: f32) -> Mesh {
        let mut mesh = Mesh::new();

        if let Some(color) = fill {
            let mut renderer = ShapeRenderer::new(&mut mesh, color);
            let options = FillOptions::tolerance(0.01);
            let _ = fill_circle(point(center.x, center.y), radius, &options, &mut renderer).unwrap();
        }
        if let Some(color) = line {
            let mut renderer = ShapeRenderer::new(&mut mesh, color);
            let options = StrokeOptions::tolerance(0.01).with_line_width(line_width);
            let _ = stroke_circle(point(center.x, center.y), radius, &options, &mut renderer).unwrap();
        }
        mesh
    }

    /// Draw an ellipse bounded by the specified Rectangle which defines the center point and 2 radii for the ellipse.
    /// Furthermore, specify the optional fill and line colors. And lastly, define the rotation in degrees.
    /// Technically, you can draw a circle with this method, but the options are different. Internally within the Lyon
    /// basic_shapes methods for creating an ellipse, it will draw a circle if the radii are equal.
    pub fn ellipse(
        rect: &Rectangle,
        fill: Option<Color>,
        line: Option<Color>,
        line_width: f32,
        rotate_degrees: f32,
    ) -> Mesh {
        let mut mesh = Mesh::new();

        if let Some(color) = fill {
            let mut renderer = ShapeRenderer::new(&mut mesh, color);
            let options = FillOptions::tolerance(0.01);
            let _ = fill_ellipse(
                point(rect.center().x, rect.center().y),
                vector(rect.width() / 2.0, rect.height() / 2.0),
                Angle::degrees(rotate_degrees),
                &options,
                &mut renderer,
            )
            .unwrap();
        }
        if let Some(color) = line {
            let mut renderer = ShapeRenderer::new(&mut mesh, color);
            let options = StrokeOptions::tolerance(0.01).with_line_width(line_width);
            let _ = stroke_ellipse(
                point(rect.center().x, rect.center().y),
                vector(rect.width() / 2.0, rect.height() / 2.0),
                Angle::degrees(rotate_degrees),
                &options,
                &mut renderer,
            )
            .unwrap();
        }
        mesh
    }

    /// Draw a triangle using the specified points.
    pub fn triangle(points: &[&Vector; 3], fill: Option<Color>, line: Option<Color>, line_width: f32) -> Mesh {
        let mut mesh = Mesh::new();
        let pt1 = point(points[0].x, points[0].y);
        let pt2 = point(points[1].x, points[1].y);
        let pt3 = point(points[2].x, points[2].y);

        if let Some(color) = fill {
            let mut renderer = ShapeRenderer::new(&mut mesh, color);
            let options = FillOptions::tolerance(0.01);
            fill_triangle(pt1, pt2, pt3, &options, &mut renderer).unwrap();
        }

        if let Some(color) = line {
            if line_width > 0.0 {
                let mut renderer = ShapeRenderer::new(&mut mesh, color);
                let options = StrokeOptions::tolerance(0.01).with_line_width(line_width);
                stroke_triangle(pt1, pt2, pt3, &options, &mut renderer).unwrap();
            }
        }
        mesh
    }

    /// Draws a Rectangle with optional line and fill colors. If the corner_radius <= 0, then just draw a rectangle
    pub fn rectangle(
        frame: &Rectangle,
        fill: Option<Color>,
        line: Option<Color>,
        line_width: f32,
        corner_radius: f32,
    ) -> Mesh {
        let mut mesh = Mesh::new();
        let rect = rect(frame.x(), frame.y(), frame.width(), frame.height());

        if corner_radius >= 0.0 {
            let radii = BorderRadii::new_all_same(corner_radius);
            if let Some(color) = fill {
                let mut renderer = ShapeRenderer::new(&mut mesh, color);
                let options = FillOptions::tolerance(0.01);
                fill_rounded_rectangle(&rect, &radii, &options, &mut renderer).unwrap();
            }

            if let Some(color) = line {
                if line_width > 0.0 {
                    let mut renderer = ShapeRenderer::new(&mut mesh, color);
                    let options = StrokeOptions::tolerance(0.01).with_line_width(line_width);
                    stroke_rounded_rectangle(&rect, &radii, &options, &mut renderer).unwrap();
                }
            }
        } else {
            if let Some(color) = fill {
                let mut renderer = ShapeRenderer::new(&mut mesh, color);
                let options = FillOptions::tolerance(0.01);
                fill_rectangle(&rect, &options, &mut renderer).unwrap();
            }

            if let Some(color) = line {
                if line_width > 0.0 {
                    let mut renderer = ShapeRenderer::new(&mut mesh, color);
                    let options = StrokeOptions::tolerance(0.01).with_line_width(line_width);
                    stroke_rectangle(&rect, &options, &mut renderer).unwrap();
                }
            }
        }
        mesh
    }

    /// Draw a four-sided polygon using the specified points.
    pub fn quad(points: &[&Vector; 4], fill: Option<Color>, line: Option<Color>, line_width: f32) -> Mesh {
        let mut mesh = Mesh::new();
        let pt1 = point(points[0].x, points[0].y);
        let pt2 = point(points[1].x, points[1].y);
        let pt3 = point(points[2].x, points[2].y);
        let pt4 = point(points[3].x, points[3].y);

        if let Some(color) = fill {
            let mut renderer = ShapeRenderer::new(&mut mesh, color);
            let options = FillOptions::tolerance(0.01);
            fill_quad(pt1, pt2, pt3, pt4, &options, &mut renderer).unwrap();
        }

        if let Some(color) = line {
            if line_width > 0.0 {
                let mut renderer = ShapeRenderer::new(&mut mesh, color);
                let options = StrokeOptions::tolerance(0.01).with_line_width(line_width);
                stroke_quad(pt1, pt2, pt3, pt4, &options, &mut renderer).unwrap();
            }
        }
        mesh
    }
}
