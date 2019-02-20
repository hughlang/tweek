/// GUI controls
///
///
extern crate ggez;

use crate::core::*;

use ggez::graphics::{self, DrawParam};
use ggez::mint;
use ggez::{Context, GameResult};
use std::{collections::HashMap};

use super::ggez_helper::*;


pub enum MouseState {
    None,
    Hover,
    Drag,
    Click,
}

pub trait GGDisplayable {
    fn update(&mut self) -> GameResult;
    fn render(&mut self, ctx: &mut Context) -> GameResult;
    fn render_inside(&mut self, _rect: &graphics::Rect, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn handle_mouse_at(&mut self, _x: f32, _y: f32) -> bool {
        false
    }
}

//-- GGLabel -----------------------------------------------------------------------

pub struct GGLabel {
    pub layer: GGLayer,
    pub title: String,
    pub text: graphics::Text,
}

impl GGLabel {
    pub fn new(frame: &graphics::Rect, text: &str) -> Self {
        let layer = GGLayer::new(
            frame.clone(),
            DrawParam::new().color(graphics::WHITE),
        );

        GGLabel {
            layer: layer,
            title: text.to_string(),
            text: graphics::Text::new(text.to_string()),
        }
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
                self.layer.render_update(&update.props);
                self.layer.render_update(&update.props);
                self.layer.redraw = true;
            }
        }
        Ok(())
    }

    fn render(&mut self, ctx: &mut Context) -> GameResult {
        let _result = graphics::draw(ctx, &self.text, self.layer.graphics);
        Ok(())
    }

    fn render_inside(&mut self, rect: &graphics::Rect, ctx: &mut Context) -> GameResult {
        let (width, height) = self.text.dimensions(ctx);
        let pt = mint::Point2{x: rect.x + (rect.w - width as f32)/2.0 , y: rect.y + (rect.h - height as f32)/2.0 };
        // println!("inside={:?} // dest={:?}", rect,  pt);
        let _result = graphics::draw(ctx, &self.text, self.layer.graphics.dest(pt));
        Ok(())
    }
}

//-- GGButton -----------------------------------------------------------------------

pub struct GGButton {
    pub layer: GGLayer,
    pub label: Option<GGLabel>,
    pub defaults: HashMap<u32, Prop>,
    pub on_hover: Option<Transition>,
    pub mouse_state: MouseState,
}

impl GGButton {
    pub fn new(frame: graphics::Rect) -> Self {
        let layer = GGLayer::new(frame, DrawParam::new());
        GGButton {
            layer: layer,
            label: None,
            defaults: HashMap::new(),
            on_hover: None,
            mouse_state: MouseState::None,
        }
    }

    pub fn with_props(mut self, props: &Vec<Prop>) -> Self {
        for prop in props {
            self.defaults.insert(prop.prop_id(), prop.clone());
        }
        self
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

    pub fn get_defaults(&self) -> Vec<Prop> {
        let mut props:Vec<Prop> = Vec::new();
        for (_, v) in &self.defaults {
            props.push(v.clone());
        }
        props
    }

    pub fn set_on_hover(&mut self, props: Vec<Prop>, seconds: f64) {
        let transition = Transition::new(props, seconds);
        self.on_hover = Some(transition);
    }

}

impl GGDisplayable for GGButton {
    fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.render_update(&update.props);
                self.layer.render_update(&update.props);
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

    fn handle_mouse_at(&mut self, x: f32, y: f32) -> bool {
        if self.layer.frame.contains(mint::Point2{ x, y }) {
            match self.mouse_state {
                MouseState::None => {
                    // change state to hover and start animations
                    // if self.on_hover.len() > 0 {
                    self.mouse_state = MouseState::Hover;
                    println!("Mouse hover at: x={} y={}", x, y);
                    println!("Layer frame = {:?}", self.layer.frame);

                    if let Some(transition) = &self.on_hover {
                        if transition.seconds > 0.0 {
                            let mut tween = Tween::with(0, &self.layer)
                                .to(transition.props.clone())
                                .duration(transition.seconds);
                            &tween.play();
                            self.layer.animation = Some(tween);
                        } else {
                            self.layer.render_update(&transition.props.clone());
                        }
                    }
                },
                _ => (),
            }
            return true;
        } else {
            match self.mouse_state {
                MouseState::Hover => {
                    println!("Mouse out at: x={} y={}", x, y);
                    self.layer.render_update(&self.get_defaults());
                    self.mouse_state = MouseState::None;
                    self.layer.animation = None;
                },
                _ => (),
            }
        }
        false
    }

}