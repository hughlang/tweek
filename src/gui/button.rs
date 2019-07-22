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
    pub text: Option<Text>,
    pub icon: Option<Image>,
    onclick: Option<Box<FnMut(TKAction, &mut TKState) + 'static>>,
}

impl Button {
    pub fn new(frame: Rectangle) -> Self {
        let mut layer = TweenLayer::new(frame);
        layer.color.a = 0.0;  // Button background color default is transparent
        Button { layer: layer, label: None, text: None, icon: None, onclick: None }
    }

    pub fn id(mut self, id: u32) -> Self {
        self.layer.id = id;
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        // let rect = self.layer.inset_by(8.0, 4.0, 8.0, 4.0);
        let label = Label::new(&self.layer.frame, text);
        self.label = Some(label);
        self
    }

    pub fn set_text(&mut self, text: &str) {
        let label = Label::new(&self.layer.frame, text);
        self.label = Some(label);
    }

    pub fn set_image(&mut self, image: Image) {
        self.icon = Some(image);
    }

    // fn update_layout(&mut self) {

    // }

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

    pub fn onclick_action(&mut self) {

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

    fn get_content_size(&self) -> Vector {
        self.layer.frame.size
    }

    fn set_origin(&mut self, origin: &Vector) {
        self.layer.frame.pos.x = origin.x;
        self.layer.frame.pos.y = origin.y;

        if let Some(label) = &mut self.label {
            label.layer.frame.pos.x = origin.x;
            label.layer.frame.pos.y = origin.y;
            eprintln!("button={:?} label={:?}", self.layer.frame, label.layer.frame);
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

    fn update(&mut self, _window: &mut Window) -> TKResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.apply_updates(&update.props);
            }
        }
        Ok(())
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) -> TKResult {
        if self.layer.color.a > 0.0 {
            window.draw(&self.layer.frame, Col(self.layer.color));
        }
        // eprintln!("render button at {:?}", self.layer.frame);
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
        let hover = self.layer.handle_mouse_over(pt);
        // if hover { eprintln!("Button hover at pt={:?}", pt) }
        hover
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
