extern crate tweek;

use tweek::prelude::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Circle, Line, Rectangle, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Result,
};

static ROBOTO_REGULAR: &[u8] = include_bytes!("../../static/Roboto-Regular.ttf");
static ROBOTO_BOLD: &[u8] = include_bytes!("../../static/Roboto-Bold.ttf");
const STAGE_WIDTH: f32 = 800.0;
const STAGE_HEIGHT: f32 = 500.0;

#[allow(dead_code)]
pub const NEXT_COMMAND: usize = 1;
#[allow(dead_code)]
pub const PREV_COMMAND: usize = 2;

#[allow(dead_code)]
pub enum ShapeType {
    Circle(Vector, f32),
    Image(Rectangle),
    /// Parameters are start point, end point, and line width
    Line(Vector, Vector, f32),
    Rectangle(Rectangle),
    Text(Rectangle),
}

pub struct Grid {
    pub lines: Vec<Line>,
    pub color: Color,
}

pub struct StageHelper {}

#[allow(dead_code)]
#[allow(unused_variables)]
impl StageHelper {
    pub fn get_draw_area(screen: Vector) -> Rectangle {
        let draw_area = Rectangle::new(((screen.x - STAGE_WIDTH) / 2.0, 120.0), (STAGE_WIDTH, STAGE_HEIGHT));
        draw_area
    }

    pub fn build_grid(width: f32, height: f32, interval: f32, color: Color) -> Grid {
        let mut lines: Vec<Line> = Vec::new();
        let mut xpos = 0.0;
        while xpos < width {
            let line = Line::new((xpos, 0.0), (xpos, height)).with_thickness(1.0);
            lines.push(line);
            xpos += interval;
        }
        let mut ypos = 0.0;
        while ypos < height {
            let line = Line::new((0.0, ypos), (width, ypos)).with_thickness(1.0);
            lines.push(line);
            ypos += interval;
        }
        Grid { lines, color }
    }

    pub fn make_next_prev_buttons(screen: &Vector, theme: &Theme) -> Vec<Button> {
        const BUTTON_WIDTH: f32 = 90.0;
        const BUTTON_HEIGHT: f32 = 40.0;

        let mut buttons: Vec<Button> = Vec::with_capacity(2);
        let xpos = 30.0;
        let ypos = 30.0;

        let style = FontStyle::new(16.0, Color::WHITE);
        // ---- Previous ---------------------
        let frame = Rectangle::new((30.0, ypos), (BUTTON_WIDTH, BUTTON_HEIGHT));
        // let image = theme.font.render("Previous", &style).unwrap();
        let mut button = Button::new(frame).with_text("Previous");

        let (r, g, b) = hex_to_rgb(HexColors::Tan);
        button.set_color(&Color::from_rgba(r, g, b, 1.0));
        button.set_hover_animation(&[color(HexColors::Chocolate)], 0.1);
        button.set_onclick(move |_action, tk| {
            tk.click_target = Some(PREV_COMMAND);
        });
        buttons.push(button);

        // ---- Next ---------------------
        let frame = Rectangle::new((screen.x - BUTTON_WIDTH - 30.0, ypos), (BUTTON_WIDTH, BUTTON_HEIGHT));
        // let image = theme.font.render("Next", &style).unwrap();
        let mut button = Button::new(frame).with_text("Next");

        let (r, g, b) = hex_to_rgb(HexColors::Tan);
        button.set_color(&Color::from_rgba(r, g, b, 1.0));
        button.set_hover_animation(&[color(HexColors::Chocolate)], 0.1);
        button.set_onclick(move |_action, tk| {
            tk.click_target = Some(NEXT_COMMAND);
        });
        buttons.push(button);

        buttons
    }

    pub fn load_theme() -> Theme {
        let mut theme = Theme::new(ROBOTO_REGULAR);
        theme.font_size = 18.0;
        theme.font_bytes = ROBOTO_REGULAR.into();
        theme.bg_color = Color::from_hex("#FFFFEE");

        let font = Font::from_slice(ROBOTO_BOLD).unwrap();
        theme.title_font = Some(font);

        theme
    }

    pub fn make_fps_counter(screen: &Vector, theme: &Theme) -> Label {
        let frame = Rectangle::new((30.0, screen.y - 60.0), (60.0, 30.0));
        let mut label = Label::new(&frame, "0 FPS");
        label.set_theme(theme);
        label
    }
}

pub struct Item {
    pub id: usize,
    pub shape: ShapeType,
    pub layer: TweenLayer,
    pub tween: Option<Tween>,
    pub image: Option<Image>,
}

impl Drop for Item {
    fn drop(&mut self) {
        self.tween = None;
    }
}

#[allow(dead_code)]
impl Item {
    pub fn new(id: usize, shape: ShapeType) -> Item {
        let layer = match shape {
            ShapeType::Rectangle(rect) => TweenLayer::new(rect),
            ShapeType::Circle(pt, r) => {
                let rect = Rectangle::new((pt.x - r, pt.y - r), (r * 2.0, r * 2.0));
                TweenLayer::new(rect)
            }
            ShapeType::Image(rect) => TweenLayer::new(rect),
            ShapeType::Text(rect) => TweenLayer::new(rect),
            ShapeType::Line(pt1, pt2, line_width) => {
                let rect = Rectangle::new((pt1.x, pt1.y), ((pt2.x - pt1.x).abs(), (pt2.y - pt1.y).abs()));
                let mut layer = TweenLayer::new(rect);
                layer.stroke = line_width;
                layer
            }
        };
        Item { id: id, shape: shape, layer: layer, tween: None, image: None }
    }

    /// This method is used for animation on a single Tweenable object and not a Timeline containing multiple
    /// Tween objects.
    pub fn update(&mut self) {
        if let Some(tween) = &mut self.tween {
            tween.tick();
            if let Some(update) = tween.get_update(&self.id) {
                self.layer.apply_updates(&update.props);
                if let Some(offset) = update.offset {
                    log::trace!("{:?}", offset);
                    // self.layer.graphics.offset = Point2 {
                    //     x: offset.x as f32,
                    //     y: offset.y as f32,
                    // };
                }
            }
        }
    }

    pub fn timeline_update(&mut self, tweek: &mut Tweek) {
        if let Some(update) = tweek.get_update(&self.id) {
            self.layer.apply_updates(&update.props);
        }
    }

    pub fn render(&mut self, window: &mut Window) {
        match self.shape {
            ShapeType::Circle(_, _) => {
                let r = self.layer.frame.size.x / 2.0;
                let pt = Vector { x: self.layer.frame.pos.x + r, y: self.layer.frame.pos.y + r };
                let circle = Circle::new(pt, r).with_center(self.layer.offset_pt);
                // window.draw(&circle, Col(self.layer.color));
                window.draw_ex(&circle, Col(self.layer.color), Transform::rotate(self.layer.rotation), 1);
            }
            ShapeType::Image(_) => match &self.image {
                Some(image) => {
                    let scale_w = self.layer.frame.size.x / image.area().width();
                    let scale_h = self.layer.frame.size.y / image.area().height();
                    let scale = Vector { x: scale_w, y: scale_h };
                    window.draw_ex(
                        &image.area().constrain(&self.layer.frame),
                        Img(&image),
                        Transform::rotate(self.layer.rotation) * Transform::scale(scale),
                        1,
                    );
                }
                None => (),
            },
            // ShapeType::Line(_, _, _) => {
            //     let points = [
            //         mint::Point2 {
            //             x: self.layer.frame.x,
            //             y: self.layer.frame.y,
            //         },
            //         mint::Point2 {
            //             x: self.layer.frame.x + self.layer.frame.w,
            //             y: self.layer.frame.y,
            //         },
            //     ];
            //     log::trace!("pt1={:?} // pt2={:?}", points[0], points[1]);
            //     let mesh = graphics::Mesh::new_line(
            //         ctx,
            //         &points,
            //         self.layer.stroke,
            //         self.layer.graphics.color,
            //     )?;
            //     let _result = graphics::draw(ctx, &mesh, self.layer.graphics);
            // }
            ShapeType::Rectangle(_) => {
                window.draw(&self.layer.frame, Col(self.layer.color));
            }
            _ => (),
        }
    }
}
