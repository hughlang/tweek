/// Button
///
use super::*;
use crate::core::*;

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Color, Image},
    lifecycle::Window,
};
use std::any::TypeId;

pub enum ButtonState {
    Normal,
    Highlighted,
    Selected,
    Disabled,
}

//-- Button -----------------------------------------------------------------------

#[allow(dead_code)]
pub struct Button {
    pub layer: TweenLayer,
    pub label: Option<Label>,
    pub image: Option<Image>,
    onclick: Option<Box<FnMut(TKAction, &mut TKState) + 'static>>,
}

impl Button {
    pub fn new(frame: Rectangle) -> Self {
        let layer = TweenLayer::new(frame);
        Button { layer: layer, label: None, image: None, onclick: None }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        let rect = self.layer.inset_by(8.0, 4.0, 8.0, 4.0);
        let label = Label::new(&rect, text);
        self.label = Some(label);
        self
    }

    // pub fn with_image(mut self, _image: Image, _margin: f32) -> Self {
    //     let rect = self.layer.inset_by(8.0, 4.0, 8.0, 4.0);
    //     // let rect = Rectangle::new((0.0, 0.0), (self.layer.frame.width() - margin, self.layer.frame.height() - margin));
    //     let label = Label::new(&rect, "");
    //     self.label = Some(label);
    //     // let fraction = rect.h / image.source_height() as f32;
    //     // let mut img = ImageView::new(rect, image);
    //     // img.scale = fraction;
    //     // self.image = Some(img);
    //     self
    // }

    pub fn set_color(&mut self, color: &Color) {
        self.layer.color = color.clone();
    }

    pub fn set_onclick<C>(&mut self, cb: C)
    where
        C: FnMut(TKAction, &mut TKState) + 'static,
    {
        self.onclick = Some(Box::new(cb));
    }
}

// *****************************************************************************************************
// Button :: Displayable
// *****************************************************************************************************

impl TKDisplayable for Button {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Button>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn set_origin(&mut self, origin: &Vector) {
        self.layer.frame.pos.x = origin.x;
        self.layer.frame.pos.y = origin.y;

        if let Some(label) = &mut self.label {
            label.layer.frame.pos.x = origin.x;
            label.layer.frame.pos.y = origin.y;
        }
    }

    /// Change the button font, color, and size
    fn set_theme(&mut self, theme: &Theme) {
        if let Some(label) = &mut self.label {
            label.layer.color = theme.button_fg_color;
            // label.layer.font = theme.title_font;
            // label.layer.font_size = theme.title_font_size;
        }
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Ready => {
                self.layer.defaults = Tween::load_props(&self.layer);
            }
            _ => {}
        }
    }

    fn update(&mut self) -> TKResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.apply_updates(&update.props);
            }
        }
        Ok(())
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) -> TKResult {
        window.draw(&self.layer.frame, Col(self.layer.color));

        if let Some(label) = &mut self.label {
            label.render(theme, window)?;
        }

        Ok(())
    }

    fn set_hover_animation(&mut self, props: &[Prop], seconds: f64) {
        self.layer.defaults = Tween::load_props(&self.layer);
        let transition = UITransition::new(props.to_vec(), seconds);
        self.layer.on_hover = Some(transition);
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        return self.layer.handle_mouse_over(pt);
    }
}

// *****************************************************************************************************
// Button :: TKResponder
// *****************************************************************************************************

impl TKResponder for Button {
    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        if pt.overlaps_rectangle(&self.layer.frame) {
            log::debug!("Click at: x={} y={}", pt.x, pt.y);
            if let Some(cb) = &mut self.onclick {
                (&mut *cb)(TKAction::Click, state);
            }

            return true;
        }
        false
    }
}
