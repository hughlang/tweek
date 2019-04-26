/// GUI controls
///
///
extern crate ggez;

use crate::core::*;

use ggez::graphics::{self, DrawParam, Rect};
use ggez::mint;
use ggez::{Context, GameResult};
use std::any::TypeId;

use super::*;

//-- Button -----------------------------------------------------------------------

pub struct ButtonView {
    pub layer: TweenLayer,
    pub label: Option<LabelView>,
    pub image: Option<ImageView>,
    // TODO: get rid of TKAction?
    onclick: Option<Box<FnMut(TKAction, &mut TKState) + 'static>>,
}

impl ButtonView {
    pub fn new(frame: Rect) -> Self {
        let layer = TweenLayer::new(frame, DrawParam::new());
        ButtonView { layer: layer, label: None, image: None, onclick: None }
    }

    pub fn with_title(mut self, text: &str) -> Self {
        let frame = Rect::new(0.0, 0.0, self.layer.frame.w, self.layer.frame.h);
        let label = LabelView::new(&frame, text);
        self.label = Some(label);
        self
    }

    pub fn with_image(mut self, image: graphics::Image, margin: f32) -> Self {
        let rect = Rect::new(0.0, 0.0, self.layer.frame.h - margin, self.layer.frame.h - margin);
        let fraction = rect.h / image.height() as f32;
        let mut img = ImageView::new(rect, image);
        img.scale = fraction;
        self.image = Some(img);
        self
    }

    pub fn set_color(&mut self, color: &graphics::Color) {
        self.layer.graphics.color = color.clone();
    }

    pub fn set_onclick<C>(&mut self, cb: C)
    where
        C: FnMut(TKAction, &mut TKState) + 'static,
    {
        self.onclick = Some(Box::new(cb));
    }
}

// *****************************************************************************************************
// ButtonView :: Displayable
// *****************************************************************************************************

impl TKDisplayable for ButtonView {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ButtonView>()
    }

    fn get_frame(&self) -> Rect {
        return self.layer.frame;
    }

    /// Change the button font, color, and size
    fn set_theme(&mut self, theme: &Theme) {
        if let Some(label) = &mut self.label {
            label.layer.graphics.color = theme.button_fg_color;
            label.layer.font = theme.title_font;
            label.layer.font_size = theme.title_font_size;
        }
    }

    fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.apply_updates(&update.props);
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
            graphics::draw_queued_text(ctx, graphics::DrawParam::default())?;
        }

        if let Some(image) = &mut self.image {
            image.render_inside(&self.layer.frame, ctx)?;
        }

        Ok(())
    }

    fn set_hover_animation(&mut self, props: &[Prop], seconds: f64) {
        self.layer.defaults = Tween::load_props(&self.layer);
        let transition = UITransition::new(props.to_vec(), seconds);
        self.layer.on_hover = Some(transition);
    }
}

// *****************************************************************************************************
// ButtonView :: TKResponder
// *****************************************************************************************************

impl TKResponder for ButtonView {
    fn handle_mouse_down(&mut self, _x: f32, _y: f32, _state: &mut TKState) -> bool {
        false
    }

    fn handle_mouse_up(&mut self, x: f32, y: f32, state: &mut TKState) -> bool {
        if self.layer.frame.contains(mint::Point2 { x, y }) {
            log::debug!("Click at: x={} y={}", x, y);
            if let Some(cb) = &mut self.onclick {
                // TODO: modify state or pass new information
                (&mut *cb)(TKAction::Click, state);
            }
            return true;
        }
        false
    }

    fn handle_mouse_at(&mut self, x: f32, y: f32) -> bool {
        return self.layer.handle_mouse_over(x, y);
    }
}
