use super::*;
use crate::core::*;
use crate::shared::*;

use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::Col, Color, Font, FontStyle},
    input::{Key, MouseButton, MouseCursor},
    lifecycle::{Event, Window},
};

pub struct Scene {
    pub layer: TweenLayer,
    pub timeline: Option<Timeline>,
    pub views: Vec<Rc<RefCell<TKDisplayable>>>,
    pub controls: Vec<Rc<RefCell<TKResponder>>>,
    /// index in controls vec of currently selected control (ie, textfield)
    active_control_idx: Option<usize>,
    next_control_idx: Option<usize>,
    frames: usize,
    fps_text: Option<Text>,
}

impl Scene {
    pub fn new(frame: &Rectangle) -> Self {
        let layer = TweenLayer::new(frame.clone());

        Scene {
            layer: layer,
            timeline: None,
            views: Vec::new(),
            controls: Vec::new(),
            active_control_idx: None,
            next_control_idx: None,
            frames: 0,
            fps_text: None,
        }
    }

    pub fn show_fps(&mut self, show: bool) {
        if show {
            let x = 20.0;
            let y = self.layer.frame.height() - 40.0;
            let frame = Rectangle::new((x, y), (80.0, 20.0));
            let mut text = Text::new(&frame, "");
            text.layer.font_style = FontStyle::new(14.0, Color::RED);
            self.fps_text = Some(text);
        } else {
            self.fps_text = None;
        }
    }

    fn animate(&mut self, window: &mut Window) -> TKResult {

        for cell in &mut self.controls {
            (cell.borrow_mut()).update(window)?;
        }
        for cell in &mut self.views {
            (cell.borrow_mut()).update(window)?;
        }
        Ok(())
    }

}

impl TKDisplayable for Scene {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Scene>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn set_theme(&mut self, theme: &Theme) {
        for cell in &mut self.controls {
            (cell.borrow_mut()).set_theme(theme);
        }
        for cell in &mut self.views {
            (cell.borrow_mut()).set_theme(theme);
        }
    }

    fn notify(&mut self, event: &DisplayEvent) {
        for cell in &mut self.controls {
            (cell.borrow_mut()).notify(event);
        }
        for cell in &mut self.views {
            (cell.borrow_mut()).notify(event);
        }
    }

    fn update(&mut self, window: &mut Window) -> TKResult {
        // Awkwardly, check if another control will become active and first try to
        // deactivate the previous control. Then activate the next one
        if let Some(next_idx) = self.next_control_idx {
            if let Some(last_idx) = self.active_control_idx {
                if last_idx != next_idx {
                    let cell = &mut self.controls[last_idx];
                    (cell.borrow_mut()).notify(&DisplayEvent::Deactivate);
                }
            }
            let cell = &mut self.controls[next_idx];
            (cell.borrow_mut()).notify(&DisplayEvent::Activate);
            self.active_control_idx = Some(next_idx);
            self.next_control_idx = None;
        }
        for cell in &mut self.controls {
            (cell.borrow_mut()).update(window)?;
        }
        for cell in &mut self.views {
            (cell.borrow_mut()).update(window)?;
        }
        Ok(())
    }

    /// The top-level objects in the scene should all use the scene's coordinate system and
    /// therefore, this render() method should only call render() for all child Displayable objects.
    /// That's the current plan. It may change.
    fn render(&mut self, theme: &mut Theme, window: &mut Window) -> TKResult {
        let mut mask_areas: Vec<Rectangle> = Vec::new();

        for cell in &self.views {
            (cell.borrow_mut()).render(theme, window)?;
        }
        for cell in &self.controls {
            let mut control = cell.borrow_mut();
            if let Some(perimeter) = (&control).get_perimeter_frame() {
                let mut blocks = UITools::get_perimeter_blocks(&(&control).get_frame(), &perimeter);
                mask_areas.append(&mut blocks);
            }
            (&mut *control).render(theme, window)?;
        }

        if mask_areas.len() > 0 {
            let draw_task = UITools::draw_rectangles(mask_areas, self.layer.color);
            window.add_task(draw_task);
        }

        if let Some(text) = &mut self.fps_text {
            self.frames += 1;
            if (self.frames % 20) == 0 {
                let out = format!("FPS: {:.2}", window.current_fps());
                text.set_text(out);
            }
            text.render(theme, window)?;
        }

        Ok(())
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        // TODO: Verify if hover is handled ok
        for cell in &mut self.controls {
            let hover = (cell.borrow_mut()).handle_mouse_at(pt);
            if hover {
                return true;
            }
        }
        for cell in &mut self.views {
            let hover = (cell.borrow_mut()).handle_mouse_at(pt);
            if hover {
                return true;
            }
        }
        false
    }
}

impl TKResponder for Scene {
    fn has_focus(&self) -> bool {
        false
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        for (i, cell) in &mut self.controls.iter().enumerate() {
            let focus = (cell.borrow_mut()).handle_mouse_down(pt, state);
            if focus {
                self.next_control_idx = Some(i);
                return true;
            }
        }
        false
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        for (_, cell) in &mut self.controls.iter().enumerate() {
            let focus = (cell.borrow_mut()).handle_mouse_up(pt, state);
            if focus {
                return true;
            }
        }
        false
    }

    fn handle_mouse_scroll(&mut self, pt: &Vector, state: &mut TKState) {
        for cell in &mut self.controls {
            (cell.borrow_mut()).handle_mouse_scroll(pt, state);
        }
    }

    fn handle_key_press(&mut self, c: char, window: &mut Window) {
        if let Some(active_idx) = self.active_control_idx {
            let cell = &mut self.controls[active_idx];
            (cell.borrow_mut()).handle_key_press(c, window);
        }
    }

    fn handle_key_command(&mut self, key: &Key, window: &mut Window) -> bool {
        if let Some(active_idx) = self.active_control_idx {
            let controls_count = self.controls.len();
            let cell = &mut self.controls[active_idx];
            let handled = (cell.borrow_mut()).handle_key_command(key, window);
            if handled {
                match key {
                    Key::Tab => {
                        let next_idx;
                        if active_idx + 1 == controls_count {
                            next_idx = 0;
                        } else {
                            next_idx = active_idx + 1;
                        }
                        if next_idx != active_idx {
                            // log::debug!("next_idx={:?} WAS={:?}", next_idx, active_idx);
                            self.next_control_idx = Some(next_idx);
                        }
                        return true;
                    }
                    Key::Return => {}
                    _ => (),
                }
                return true;
            }
        } else {
            // TODO: Check other listeners
        }
        false
    }
}
