
use tweek::prelude::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Circle, Line, Rectangle, Scalar, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Result,
};
use glyph_brush::HorizontalAlign as HAlign;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub mod constants {
    pub const NAV_SCENE: u32 = 100;
    pub const BG_SCENE: u32 = 200;
    pub const MAIN_SCENE: u32 = 300;

    pub const FPS_TAG: u32 = 901;
    pub const TITLE_TAG: u32 = 902;

    // Misc
    pub const FPS_FRAMES_INTERVAL: usize = 40;
}

pub struct Grid {
    pub lines: Vec<Line>,
    pub color: Color,
}

use constants::*;
pub struct DemoHelper {}

#[allow(dead_code)]
#[allow(unused_variables)]
impl DemoHelper {

    pub fn get_draw_area(screen: Vector) -> Rectangle {
        let margin = 80.0;
        let stage_width = screen.x - margin * 2.0;
        let stage_height = screen.y - margin * 2.0;
        let draw_area = Rectangle::new(((screen.x - stage_width) / 2.0, 120.0), (stage_width, stage_height));
        draw_area
    }

    pub fn build_grid(width: f32, height: f32, interval: f32, color: Color) -> Grid {
        let mut lines: Vec<Line> = Vec::new();
        let mut xpos = 0.0;
        while xpos < width {
            let line = Line::new((xpos, 0.0), (xpos, height)).with_thickness(0.5);
            lines.push(line);
            xpos += interval;
        }
        let mut ypos = 0.0;
        while ypos < height {
            let line = Line::new((0.0, ypos), (width, ypos)).with_thickness(0.5);
            lines.push(line);
            ypos += interval;
        }
        Grid { lines, color }
    }

    pub fn make_next_prev_buttons(screen: &Vector) -> Vec<Button> {
        const BUTTON_WIDTH: f32 = 90.0;
        const BUTTON_HEIGHT: f32 = 40.0;

        let mut buttons: Vec<Button> = Vec::with_capacity(2);
        let ypos = 30.0;

        let style = FontStyle::new(18.0, Color::BLACK);
        let click_props = PropSet::new(vec![shift(2.0, 2.0)], 0.1).for_type(TweenType::Click);
        let hover_props = PropSet::new(vec![color(HexColors::Chocolate), tint(HexColors::White)], 0.2)
            .for_type(TweenType::Hover);
        // ---- Previous ---------------------
        let frame = Rectangle::new((30.0, ypos), (BUTTON_WIDTH, BUTTON_HEIGHT));
        // let image = theme.font.render("Previous", &style).unwrap();
        let mut button = Button::new(frame)
            .background(BackgroundStyle::Solid(Color::from_hex(HexColors::Tan)))
            .with_text("Previous");
        button.layer.corner_radius = 5.0;
        button.layer.font_style = style;
        button.set_hover_animation(hover_props.clone());
        button.set_click_animation(click_props.clone());
        button.set_onclick(move |state| {
            state.event_bus.register_event(NavEvent::Back);
        });

        buttons.push(button);

        // ---- Next ---------------------
        let frame = Rectangle::new((screen.x - BUTTON_WIDTH - 30.0, ypos), (BUTTON_WIDTH, BUTTON_HEIGHT));
        // let image = theme.font.render("Next", &style).unwrap();
        let mut button = Button::new(frame)
            .background(BackgroundStyle::Solid(Color::from_hex(HexColors::Tan)))
            .with_text("Next");
        button.layer.font_style = style;
        button.layer.corner_radius = 5.0;

        button.set_hover_animation(hover_props.clone());
        button.set_click_animation(click_props.clone());
        button.set_onclick(move |state| {
            state.event_bus.register_event(NavEvent::Next);
        });
        buttons.push(button);

        buttons
    }

    pub fn build_nav_scene(screen: Vector) -> Scene {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame).with_id(NAV_SCENE, "Nav Scene");

        let mut buttons = DemoHelper::make_next_prev_buttons(&screen);
        buttons.drain(..).for_each(|x| scene.add_control(Box::new(x)) );

        let x = 20.0;
        let y = screen.y - 40.0;
        let frame = Rectangle::new((x, y), (80.0, 20.0));
        let mut text = Text::new(frame, "");
        text.layer.font_style = FontStyle::new(14.0, Color::RED);
        text.set_id(FPS_TAG);
        scene.add_control(Box::new(text));

        let width = 300.0;
        let height = 30.0;
        let ypos = 30.0;
        let frame = Rectangle::new(((screen.x - width)/2.0, ypos), (width, height));
        let mut text = Text::new(frame, "");
        text.layer.font_style = FontStyle::new(18.0, Color::BLACK);
        text.set_id(TITLE_TAG);
        text.align_h(HAlign::Center);
        // text.layer.apply_props(&[border(1.0, "#333333")]);

        scene.add_control(Box::new(text));

        scene
    }

    pub fn load_theme() -> Theme {
        let mut theme = Theme::default();
        theme.font_size = 18.0;
        theme.bg_color = Color::from_hex("#FFFFEE");


        theme
    }

    // pub fn render(&mut self, window: &mut Window) {
    //     match self.shape {
    //         ShapeType::Circle(_, _) => {
    //             /*
    //             TODO:
    //             If layer.offset is defined, calculate the Transform::translate using the offset point
    //             as the frame.pos
    //              */
    //             let r = self.layer.frame.size.x / 2.0;
    //             let pt = Vector { x: self.layer.frame.pos.x + r, y: self.layer.frame.pos.y + r };
    //             let circle = Circle::new(pt, r);
    //             // This is equivalent to: self.layer.frame.size / self.layer.initial.size
    //             let scale = self.layer.frame.size.times(self.layer.initial.size.recip());
    //             window.draw_ex(
    //                 &circle,
    //                 Col(self.layer.color),
    //                 Transform::rotate(self.layer.rotation) * Transform::scale(scale) * Transform::translate(self.layer.offset_pt),
    //                 1,
    //             );
    //         }
    //         ShapeType::Image(_) => match &self.image {
    //             Some(image) => {
    //                 let scale_w = self.layer.transition.frame.size.x / image.area().width();
    //                 let scale_h = self.layer.transition.frame.size.y / image.area().height();
    //                 let scale = Vector { x: scale_w, y: scale_h };
    //                 window.draw_ex(
    //                     &image.area().constrain(&self.layer.transition.frame),
    //                     Img(&image),
    //                     Transform::rotate(self.layer.transition.rotation) * Transform::scale(scale) * Transform::translate(self.layer.offset_pt),
    //                     1,
    //                 );
    //             }
    //             None => (),
    //         },
    //         // ShapeType::Line(_, _, _) => {
    //         //     let points = [
    //         //         mint::Point2 {
    //         //             x: self.layer.frame.x,
    //         //             y: self.layer.frame.y,
    //         //         },
    //         //         mint::Point2 {
    //         //             x: self.layer.frame.x + self.layer.frame.w,
    //         //             y: self.layer.frame.y,
    //         //         },
    //         //     ];
    //         //     log::trace!("pt1={:?} // pt2={:?}", points[0], points[1]);
    //         //     let mesh = graphics::Mesh::new_line(
    //         //         ctx,
    //         //         &points,
    //         //         self.layer.stroke,
    //         //         self.layer.graphics.color,
    //         //     )?;
    //         //     let _result = graphics::draw(ctx, &mesh, self.layer.graphics);
    //         // }
    //         ShapeType::Rectangle(_) => {
    //             // This is equivalent to: self.layer.frame.size / self.layer.initial.size
    //             // let scale = self.layer.frame.size.times(self.layer.initial.size.recip());
    //             // window.draw_ex(
    //             //     &self.layer.frame,
    //             //     Col(self.layer.color),
    //             //     Transform::rotate(self.layer.rotation) * Transform::scale(scale),
    //             //     1,
    //             // );

    //             window.draw(&self.layer.frame, Col(self.layer.color));
    //         }
    //         _ => (),
    //     }
    // }

}

