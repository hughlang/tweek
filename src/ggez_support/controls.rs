/// GUI controls
///
///
extern crate ggez;

use crate::core::*;

use ggez::graphics::{self, DrawParam, Color};
use ggez::mint;
use ggez::{Context, GameResult};
use std::{collections::HashMap};

use super::base::*;
use super::views::*;


//-- Button -----------------------------------------------------------------------

pub struct ButtonView {
    pub layer: GGLayer,
    pub label: Option<LabelView>,
    pub image: Option<ImageView>,
    pub defaults: HashMap<u32, Prop>,
    hover_animation: Option<UITransition>,
    mouse_state: MouseState,
    onclick: Option<Box<FnMut(TKAction, &mut TKState) + 'static>>,

}

impl ButtonView {
    pub fn new(frame: graphics::Rect) -> Self {
        let layer = GGLayer::new(frame, DrawParam::new());
        ButtonView {
            layer: layer,
            label: None,
            image: None,
            defaults: HashMap::new(),
            hover_animation: None,
            mouse_state: MouseState::None,
            onclick: None,
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
        let label = LabelView::new(&frame, text);
        self.label = Some(label);
        self
    }

    pub fn with_image(mut self, image: graphics::Image, margin: f32) -> Self {
        let rect = graphics::Rect::new(0.0, 0.0, self.layer.frame.h - margin, self.layer.frame.h - margin);
        let fraction = rect.h / image.height() as f32;
        let mut img = ImageView::new(rect, image);
        img.scale = fraction;
        self.image = Some(img);
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

    pub fn set_hover_animation(&mut self, props: Vec<Prop>, seconds: f64) {
        for prop in &props {
            let start_prop = self.layer.get_prop(&prop);
            match start_prop {
                Prop::None => {},
                _ => {
                    &self.defaults.insert(start_prop.prop_id(), start_prop);
                }
            }
        }
        let transition = UITransition::new(props, seconds);
        self.hover_animation = Some(transition);
    }

    pub fn set_onclick<C>(&mut self, cb: C) where C: FnMut(TKAction, &mut TKState) + 'static {
        self.onclick = Some(Box::new(cb));
    }

}

impl Displayable for ButtonView {
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
        if let Some(image) = &mut self.image {
            image.render_inside(&self.layer.frame, ctx)?;
        }

        Ok(())
    }

}

impl TKResponder for ButtonView {

    fn handle_mouse_down(&mut self, _x: f32, _y: f32, _state: &mut TKState) -> bool {
        false
    }
    fn handle_mouse_up(&mut self, x: f32, y: f32, state: &mut TKState) -> bool {
        if self.layer.frame.contains(mint::Point2{ x, y }) {
            println!("Click at: x={} y={}", x, y);
            if let Some(cb) = &mut self.onclick {
                // TODO: modify state or pass new information
                (&mut *cb)(TKAction::Click, state);
            }
            return true;
        }
        false
    }

    fn handle_mouse_at(&mut self, x: f32, y: f32) -> bool {
        if self.layer.frame.contains(mint::Point2{ x, y }) {
            match self.mouse_state {
                MouseState::None => {
                    // change state to hover and start animations
                    self.mouse_state = MouseState::Hover;

                    if let Some(transition) = &self.hover_animation {
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
                    // println!("Mouse out at: x={} y={}", x, y);
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


//-- GGProgressBar -----------------------------------------------------------------------

pub struct ProgressBarView {
    pub bg_layer: GGLayer,
    pub fg_layer: GGLayer,
    pub bg_image: Option<graphics::Mesh>,
    pub progress: f32,      // between 0.0 and 1.0
}

impl ProgressBarView {
    pub fn new(frame: graphics::Rect) -> Self {
        let layer1 = GGLayer::new(frame, DrawParam::new().color(graphics::BLACK));
        let layer2 = GGLayer::new(frame, DrawParam::new().color(graphics::WHITE));

        ProgressBarView {
            bg_layer: layer1,
            fg_layer: layer2,
            bg_image: None,
            progress: 0.30,   // set nice default
        }
    }

    /// This should be called in the update() part of the run loop with the latest
    /// time-elapsed percentage
    pub fn set_progress(&mut self, value: f32) {
        // Must be between 0.0 and 1.0
        self.progress = value;
        self.fg_layer.frame.w = self.bg_layer.frame.w * self.progress;
    }

    pub fn set_track_color(&mut self, color: Color) {
        self.bg_layer.graphics.color = color;
    }

    pub fn set_progress_color(&mut self, color: Color) {
        self.fg_layer.graphics.color = color;
    }
}

impl Displayable for ProgressBarView {
    fn update(&mut self) -> GameResult {
        Ok(())
    }
    fn render(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(bg) = &self.bg_image {
            graphics::draw(ctx, bg, self.bg_layer.graphics)?;
        } else {
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                self.bg_layer.frame,
                self.bg_layer.graphics.color,
            )?;
            graphics::draw(ctx, &mesh, self.bg_layer.graphics)?;
            self.bg_image = Some(mesh);
        }
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.fg_layer.frame,
            self.fg_layer.graphics.color,
        )?;
        graphics::draw(ctx, &mesh, self.fg_layer.graphics)?;
        Ok(())
    }
}


