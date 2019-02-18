/// This file will contain various helpers that will make it easier to use Tweek
/// in conjunction with ggez. Some ideas:
/// * A progress/timeline widget that can display timeline status information
/// * Buttons for play/pause/restart
///
///
extern crate ggez;

use ggez::graphics::{self, DrawParam};
use ggez::mint;
use ggez::{Context, GameResult};
// use glyph_brush::{rusttype::Scale, FontId, HorizontalAlign as Align};

use super::property::*;
use super::tween::*;
use super::tweek::*;

//-- Base -----------------------------------------------------------------------

pub trait GGDisplayable {
    fn update(&mut self) -> GameResult;
    fn render(&mut self, ctx: &mut Context) -> GameResult;
    fn render_inside(&mut self, _rect: &graphics::Rect, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

pub enum GGShape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
    Image(graphics::Rect),
    Text(graphics::Rect),
    Line(mint::Point2<f32>, mint::Point2<f32>),
}

/// This also implements Tweenable
pub struct GGLayer {
    pub frame: graphics::Rect,
    pub graphics: DrawParam,
    pub animation: Option<Tween>,
    pub redraw: bool,
}

impl GGLayer {
    pub fn new(frame: graphics::Rect, graphics: DrawParam) -> Self {
        GGLayer {
            frame: frame,
            graphics: graphics,
            animation: None,
            redraw: false,
        }
    }
}

//-- Main -----------------------------------------------------------------------

pub struct GGLabel {
    pub layer: GGLayer,
    pub title: String,
    pub text: graphics::Text,
}

impl GGLabel {
    pub fn new(frame: &graphics::Rect, text: &str) -> Self {
        let layer = GGLayer::new(
            frame.clone(),
            DrawParam::new().color(graphics::Color::from_rgb_u32(0x333333)),
        );

        GGLabel {
            layer: layer,
            title: text.to_string(),
            text: graphics::Text::new(text.to_string()),
        }
    }

    pub fn with_font(mut self, font: graphics::Font, size: f32) -> Self {
        self.text = graphics::Text::new((self.title.clone(), font, size));
        self
    }

    pub fn set_font(&mut self, font: &graphics::Font, size: &f32) {
        self.text = graphics::Text::new((self.title.clone(), font.clone(), size.clone()));
    }

    pub fn set_color(&mut self, color: &graphics::Color) {
        self.layer.graphics.color = color.clone();
    }
}

impl GGDisplayable for GGLabel {

    fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.frame.render_update(&update.props);
                self.layer.graphics.render_update(&update.props);
                self.layer.redraw = true;
            }
        }
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context) -> GameResult {
        // let pt = mint::Point2{x: self.layer.frame.x, y: self.layer.frame.y};
        let _result = graphics::draw(ctx, &self.text, self.layer.graphics);
        Ok(())
    }

    fn render_inside(&mut self, rect: &graphics::Rect, ctx: &mut Context) -> GameResult {
        let pt = mint::Point2{x: rect.x + self.layer.frame.x , y: rect.y + self.layer.frame.y};
        // println!("inside={:?} // dest={:?}", rect,  pt);
        let _result = graphics::draw(ctx, &self.text, self.layer.graphics.dest(pt));
        Ok(())
    }
}

pub struct GGButton {
    pub layer: GGLayer,
    pub label: Option<GGLabel>,
    pub props: Vec<Prop>,
    pub on_hover: Vec<Prop>,
}

impl GGButton {
    pub fn new(frame: graphics::Rect) -> Self {
        let layer = GGLayer::new(frame, DrawParam::new());
        GGButton {
            layer: layer,
            label: None,
            props: Vec::new(),
            on_hover: Vec::new(),
        }
    }

    pub fn with_title(mut self, text: &str) -> Self {
        let frame = graphics::Rect::new(0.0, 0.0, self.layer.frame.w, self.layer.frame.h);
        let label = GGLabel::new(&frame, text);
        self.label = Some(label);
        self
    }

    pub fn set_font(&mut self, font: &graphics::Font, size: &f32, color: &graphics::Color) {
        if let Some(label) = &mut self.label {
            label.set_font(font, size);
            label.set_color(color);
        }
    }

    pub fn set_color(&mut self, color: &graphics::Color) {
        self.layer.graphics.color = color.clone();
    }

}

impl GGDisplayable for GGButton {
    fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.frame.render_update(&update.props);
                self.layer.graphics.render_update(&update.props);
                self.layer.redraw = true;
            }
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
        let _result = graphics::draw(ctx, &mesh, self.layer.graphics);
        if let Some(label) = &mut self.label {
            label.render_inside(&self.layer.frame, ctx)?;
        }

        Ok(())
    }
}

//-- Support -----------------------------------------------------------------------

/// This is a wrapper for the ggez properties that are tweenable. It is used as a convenient substitute
/// for having to manage multiple tweenables per displayed asset.
impl Tweenable for GGLayer {
    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Alpha(val) => self.graphics.color.a = val[0] as f32,
            Prop::Rotate(val) => self.graphics.rotation = val[0] as f32,
            Prop::Position(pos) => {
                self.frame.x = pos[0] as f32;
                self.frame.y = pos[1] as f32
            }
            Prop::Size(v) => {
                self.frame.w = v[0] as f32;
                self.frame.h = v[1] as f32
            }
            _ => (),
        }
    }
    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Alpha(_) => Prop::Alpha(FloatProp::new(self.graphics.color.a as f64)),
            Prop::Rotate(_) => Prop::Rotate(FloatProp::new(self.graphics.rotation as f64)),
            Prop::Position(_) => {
                Prop::Position(Point2D::new(self.frame.x as f64, self.frame.y as f64))
            }
            Prop::Size(_) => Prop::Size(Frame2D::new(self.frame.w as f64, self.frame.h as f64)),
            _ => Prop::None,
        }
    }
}

impl Tweenable for ggez::graphics::Rect {
    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Position(pos) => {
                self.x = pos[0] as f32;
                self.y = pos[1] as f32
            }
            Prop::Size(v) => {
                self.w = v[0] as f32;
                self.h = v[1] as f32
            }
            _ => (),
        }
    }
    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Position(_) => Prop::Position(Point2D::new(self.x as f64, self.y as f64)),
            Prop::Size(_) => Prop::Size(Frame2D::new(self.w as f64, self.h as f64)),
            _ => Prop::None,
        }
    }
}

impl Tweenable for ggez::graphics::DrawParam {
    fn apply(&mut self, prop: &Prop) {
        match prop {
            Prop::Alpha(val) => self.color.a = val[0] as f32,
            Prop::Rotate(val) => self.rotation = val[0] as f32,
            _ => (),
        }
    }
    fn get_prop(&self, prop: &Prop) -> Prop {
        match prop {
            Prop::Alpha(_) => Prop::Alpha(FloatProp::new(self.color.a as f64)),
            Prop::Rotate(_) => Prop::Rotate(FloatProp::new(self.rotation as f64)),
            _ => Prop::None,
        }
    }
}
