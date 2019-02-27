/// This module is used throughout the examples and serves as a basic and generic system for rendering
/// shapes on screen. However, it isn't comprehensive and certain compromises are made to reduce
/// complexity. For example, it does not support Line graphics as borders for shapes. Only the Line
/// shape uses DrawMode::Stroke. Someday, it may get migrated into the ggez_support module.
/// Also, there are still many bugs.

extern crate ggez;
extern crate tweek;

use ggez::graphics::{self, Rect, Color};
use ggez::{Context, GameResult};
use ggez::mint::{self, Point2};

use tweek::prelude::*;

#[allow(dead_code)]
pub enum Shape {
    Circle(mint::Point2<f32>, f32),
    Image(graphics::Rect),
    /// Parameters are start point, end point, and line width
    Line(mint::Point2<f32>, mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
    Text(graphics::Rect),
}

#[allow(dead_code)]
pub const NEXT_COMMAND: u32 = 1;
#[allow(dead_code)]
pub const PREV_COMMAND: u32 = 2;

pub struct ShapeHelper {
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl ShapeHelper {

    pub fn build_grid(ctx: &mut Context, width: f32, height: f32, interval: f32, color: Color) -> GameResult<graphics::Mesh> {
        let mut builder = graphics::MeshBuilder::new();

        let mut xpos = 0.0;
        while xpos < width {
            builder.line(&[Point2{x: xpos, y: 0.0}, Point2{x: xpos, y: height}], 1.0, color,)?;
            xpos += interval;
        }
        let mut ypos = 0.0;
        while ypos < height {
            builder.line(&[Point2{x: 0.0, y: ypos}, Point2{x: width, y: ypos}], 1.0, color,)?;
            ypos += interval;
        }

        let gridmesh = builder.build(ctx)?;
        Ok(gridmesh)
    }

    /// This creates the Next and Previous buttons that make it easy to load and view animations.
    /// The set_onclick method appends a u32 value that is evaluated in the run loop update() method.
    pub fn make_next_prev_buttons(ctx: &mut Context) -> GameResult<Vec<ButtonView>> {
        const BUTTON_WIDTH: f32 = 90.0;
        const BUTTON_HEIGHT: f32 = 40.0;
        let screen_w = ctx.conf.window_mode.width;

        let font = graphics::Font::new(ctx, "/Roboto-Bold.ttf")?;

        let mut buttons: Vec<ButtonView> = Vec::new();
        let xpos = 30.0;
        let ypos = 30.0;

        // ---- Previous ---------------------
        let frame = Rect::new(xpos, ypos, BUTTON_WIDTH, BUTTON_HEIGHT);
        let mut button = ButtonView::new(frame).with_title("Previous");
        button.set_font(&font, &18.0, &Color::from_rgb_u32(0xFFFFFF));
        button.set_color(&Color::from_rgb_u32(HexColors::Tan));
        button.set_hover_animation(vec![color(HexColors::Chocolate)], 0.1);
        button.set_onclick(move |_action, tk| {
            tk.commands.push(PREV_COMMAND);
        });
        buttons.push(button);

        // ---- Next ---------------------
        let frame = Rect::new(
            screen_w - BUTTON_WIDTH - 30.0,
            ypos,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
        );
        let mut button = ButtonView::new(frame).with_title("Next");
        button.set_font(&font, &18.0, &Color::from_rgb_u32(0xFFFFFF));
        button.set_color(&Color::from_rgb_u32(HexColors::Tan));
        button.set_hover_animation(vec![color(HexColors::Chocolate)], 0.1);
        button.set_onclick(move |_action, state| {
            state.commands.push(NEXT_COMMAND);
        });
        buttons.push(button);

        Ok(buttons)
    }

}
pub struct Item {
    pub id: usize,
    pub shape: Shape,
    pub layer: TweenLayer,
    pub tween: Option<Tween>,
    pub image: Option<graphics::Image>,
    pub text: Option<graphics::Text>,
}

impl Drop for Item {
    fn drop(&mut self) {
        self.tween = None;
    }
}

impl Item {

    #[allow(dead_code)]
    pub fn new(id: usize, shape: Shape) -> GameResult<Item> {
        let layer = match shape {
            Shape::Rectangle(rect) => {
                TweenLayer::new(rect, graphics::DrawParam::new())
            },
            Shape::Circle(pt, r) => {
                let rect = graphics::Rect::new(pt.x - r, pt.y - r, r * 2.0, r * 2.0);
                TweenLayer::new(rect, graphics::DrawParam::new())
            },
            Shape::Image(rect) => {
                TweenLayer::new(rect, graphics::DrawParam::new())
            },
            Shape::Text(rect) => {
                TweenLayer::new(rect, graphics::DrawParam::new())
            },
            Shape::Line(pt1, pt2, line_width) => {
                let rect = graphics::Rect::new(pt1.x, pt1.y, (pt2.x - pt1.x).abs(), (pt2.y - pt1.y).abs());
                let mut layer = TweenLayer::new(rect, graphics::DrawParam::new());
                layer.stroke = line_width;
                layer
            },
        };

        Ok(Item {
            id: id,
            shape: shape,
            layer: layer,
            tween: None,
            image: None,
            text: None,
        })
    }

    #[allow(dead_code)]
    pub fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.tween {
            tween.tick();
            if let Some(update) = tween.get_update(&self.id) {
                // println!("update props={:?}", update.props);
                self.layer.apply_updates(&update.props);
            }
        }
        Ok(())
    }

    /// This is an experimental/temporary means of getting Tween updates.
    /// The publishing of UIState updates may be handled by TKState in the
    /// future.
    #[allow(dead_code)]
    pub fn timeline_update(&mut self, tweek: &mut Tweek) -> GameResult {
        if let Some(update) = tweek.get_update(&self.id) {
            self.layer.apply_updates(&update.props);
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        match self.shape {
            Shape::Circle(_, _) => {
                let r = self.layer.frame.w / 2.0;
                let pt = mint::Point2{x: self.layer.frame.x + r, y: self.layer.frame.y + r};
                let mesh = graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), pt, r, 0.2, self.layer.graphics.color)?;
                let _result = graphics::draw(ctx, &mesh, self.layer.graphics);
            },
            Shape::Image(_) => {
                match &self.image {
                    Some(img) => {
                        let scale_w = self.layer.frame.w / img.width() as f32;
                        let scale_h = self.layer.frame.h / img.height() as f32;
                        let scale = mint::Vector2{x: scale_w, y: scale_h};
                        let pt = mint::Point2{x: self.layer.frame.x, y: self.layer.frame.y};
                        let _result = graphics::draw(ctx, img, self.layer.graphics.dest(pt).scale(scale));
                    },
                    None => (),
                }
            },
            Shape::Line(_, _, _) => {
                let points = vec![
                    mint::Point2{x: self.layer.frame.x, y: self.layer.frame.y},
                    mint::Point2{x: self.layer.frame.x + self.layer.frame.w, y: self.layer.frame.y},
                ];
                // println!("pt1={:?} // pt2={:?}", points[0], points[1]);
                let mesh = graphics::Mesh::new_line(ctx, &points, self.layer.stroke, self.layer.graphics.color)?;
                let _result = graphics::draw(ctx, &mesh, self.layer.graphics);
            }
            Shape::Rectangle(_) => {
                let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.layer.frame, self.layer.graphics.color)?;
                let _result = graphics::draw(ctx, &mesh, self.layer.graphics);
            },
            Shape::Text(_) => {
                match &self.text {
                    Some(txt) => {
                        let pt = mint::Point2{x: self.layer.frame.x, y: self.layer.frame.y};
                        let _result = graphics::draw(ctx, txt, self.layer.graphics.dest(pt));
                    },
                    None => (),
                }
            },
        }
        Ok(())
    }
}

