/// Checkbox
///
use crate::core::*;
use crate::shared::*;

use quicksilver::{
    geom::{Line, Rectangle, Shape, Vector},
    graphics::{
        Background::{Col, Img},
        Color, DrawTask, FontStyle, Image,
    },
    lifecycle::Window,
};
use std::any::TypeId;

use super::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CheckStyle {
    X,
    Radio,
    Check,
    Fill(f32), // f32 is the inset margin for the fill
}

//-- Checkbox -----------------------------------------------------------------------

#[allow(dead_code)]
pub struct Checkbox {
    pub layer: TweenLayer,
    pub text: String,
    pub is_checked: bool,
    pub check_style: CheckStyle,
    image_text: Option<Image>,
    onclick: Option<Box<FnMut(TKAction, &mut TKState) + 'static>>,
}

impl Checkbox {
    pub fn new(frame: Rectangle) -> Self {
        let layer = TweenLayer::new(frame);
        // let box_frame = Rectangle::new((0.0, 0.0), (20.0, 20.0));
        // let box_frame = UITools::position_left_middle(&frame, &box_frame, 5.0);

        Checkbox {
            layer: layer,
            text: String::default(),
            is_checked: false,
            check_style: CheckStyle::X,
            image_text: None,
            onclick: None,
        }
    }

    pub fn with_text(mut self, text: &str, is_checked: bool) -> Self {
        self.text = text.to_owned();
        self.is_checked = is_checked;
        self
    }

    pub fn set_onclick<C>(&mut self, cb: C)
    where
        C: FnMut(TKAction, &mut TKState) + 'static,
    {
        self.onclick = Some(Box::new(cb));
    }

    pub fn update_frame(&mut self, frame: Rectangle) {
        self.layer.frame = frame;
    }

    fn render_at(&mut self, frame: &Rectangle, _theme: &Theme, window: &mut Window) -> TKResult {
        window.draw(frame, Col(self.layer.color));

        let stroke = 1.0;
        let stroke_color = Color::BLACK;

        let box_frame = Rectangle::new((0.0, 0.0), (20.0, 20.0));
        let box_frame = UITools::position_left_middle(&frame, &box_frame, 5.0);
        match self.check_style {
            CheckStyle::X => {
                let mut lines = UITools::make_border_lines(&box_frame, stroke);
                if self.is_checked {
                    let rect = &box_frame;
                    let line = Line::new((rect.x(), rect.y()), (rect.x() + rect.width(), rect.y() + rect.height()))
                        .with_thickness(stroke);
                    lines.push(line);
                    let line = Line::new((rect.x() + rect.width(), rect.y()), (rect.x(), rect.y() + rect.height()))
                        .with_thickness(stroke);
                    lines.push(line);
                }

                for line in lines {
                    window.draw(&line.with_thickness(line.t), Col(stroke_color));
                }
            }
            CheckStyle::Radio => {
                let mut mesh =
                    DrawShape::circle(&box_frame.center(), &box_frame.width() / 2.0, None, Some(Color::BLACK), 1.0);
                let mut task = DrawTask::new(0);
                task.vertices.append(&mut mesh.vertices);
                task.triangles.append(&mut mesh.triangles);
                window.add_task(task);

                if self.is_checked {
                    let mut mesh = DrawShape::circle(
                        &box_frame.center(),
                        &box_frame.width() / 2.0 - 4.0,
                        Some(Color::BLACK),
                        None,
                        0.0,
                    );
                    let mut task = DrawTask::new(0);
                    task.vertices.append(&mut mesh.vertices);
                    task.triangles.append(&mut mesh.triangles);
                    window.add_task(task);
                }
            }
            _ => {}
        }

        if let Some(img) = &self.image_text {
            let y = frame.y() + (frame.height() - img.area().height()) / 2.0;
            let text_frame = Rectangle::new((frame.x() + 30.0, y), (frame.width() - 30.0, img.area().height()));
            window.draw(&img.area().constrain(&text_frame), Img(&img));
        }

        Ok(())
    }
}

// *****************************************************************************************************
// Checkbox :: Displayable
// *****************************************************************************************************

impl TKDisplayable for Checkbox {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Checkbox>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn get_content_size(&self) -> Vector {
        if let Some(img) = &self.image_text {
            Vector::new(40.0 + img.area().width(), self.layer.frame.height())
        } else {
            self.layer.frame.size
        }
    }

    /// Change the font, color, and size
    fn set_theme(&mut self, theme: &Theme) {
        let style = FontStyle::new(theme.font_size, Color::BLACK);
        let img = theme.font.render(&self.text, &style).unwrap();

        // let hit_area = UITools::combine_frames(&self.box_frame, &img.area());
        // eprintln!(">>> hit_area={:?} y={:?}", hit_area, 0);
        // self.hit_area = UITools::padded_rect(&hit_area, 4.0, 4.0);
        self.image_text = Some(img);
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
        self.render_at(&self.layer.frame.clone(), theme, window)
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
// Checkbox :: TKResponder
// *****************************************************************************************************

impl TKResponder for Checkbox {
    fn get_field_value(&self) -> FieldValue {
        FieldValue::Checkbox(self.is_checked)
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        if pt.overlaps_rectangle(&self.layer.frame) {
            // log::debug!("Click at: x={} y={}", pt.x, pt.y);
            self.is_checked = !self.is_checked;
            if let Some(cb) = &mut self.onclick {
                (&mut *cb)(TKAction::Click, state);
            }

            return true;
        }
        false
    }
}
