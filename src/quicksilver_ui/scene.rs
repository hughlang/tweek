use super::*;
use crate::core::*;

use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
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
    pub types_map: HashMap<TypeId, String>,
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
            types_map: HashMap::new(),
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

    // This is an experimental means of differentiating different types. Each Displayable should
    // provide it's type_id() and therefore allow the system to know how to handle different
    // object types.
    // pub fn prepare(mut self) -> Self {
    //     self.types_map.insert(TypeId::of::<Button>(), "Button".to_string());
    //     self.types_map.insert(TypeId::of::<ListBox>(), "ListBox".to_string());
    //     self
    // }
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
            let mut control = cell.borrow_mut();
            (&mut *control).set_theme(theme);
        }
        for cell in &mut self.views {
            let mut view = cell.borrow_mut();
            (&mut *view).set_theme(theme);
        }
    }

    fn notify(&mut self, event: &DisplayEvent) {
        for cell in &mut self.controls {
            let mut control = cell.borrow_mut();
            (&mut *control).notify(event);
        }
        for cell in &mut self.views {
            let mut view = cell.borrow_mut();
            (&mut *view).notify(event);
        }
    }

    fn update(&mut self) -> TKResult {
        // Awkwardly, check if another control will become active and first try to
        // deactivate the previous control. Then activate the next one
        if let Some(next_idx) = self.next_control_idx {
            if let Some(last_idx) = self.active_control_idx {
                if last_idx != next_idx {
                    let cell = &mut self.controls[last_idx];
                    let mut control = cell.borrow_mut();
                    (&mut *control).notify(&DisplayEvent::Deactivate);
                }
            }
            let cell = &mut self.controls[next_idx];
            let mut control = cell.borrow_mut();
            (&mut *control).notify(&DisplayEvent::Activate);
            self.active_control_idx = Some(next_idx);
            self.next_control_idx = None;
        }
        for cell in &mut self.controls {
            let mut control = cell.borrow_mut();
            (&mut *control).update()?;
        }
        for cell in &mut self.views {
            let mut view = cell.borrow_mut();
            (&mut *view).update()?;
        }
        Ok(())
    }

    /// The top-level objects in the scene should all use the scene's coordinate system and
    /// therefore, this render() method should only call render() for all child Displayable objects.
    /// That's the current plan. It may change.
    fn render(&mut self, theme: &mut Theme, window: &mut Window) -> TKResult {
        let mut mask_areas: Vec<Rectangle> = Vec::new();

        for cell in &mut self.views {
            let mut view = cell.borrow_mut();
            (&mut *view).render(theme, window)?;
        }
        for cell in &mut self.controls {
            let mut control = cell.borrow_mut();
            if let Some(perimeter) = (&control).get_perimeter_frame() {
                // log::debug!("perimeter={:?}", perimeter);
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
        // let out = format!("handle_mouse_at {:?}", pt);
        // debug_log(&out);
        for cell in &mut self.views {
            let mut control = cell.borrow_mut();
            let hover = (&mut *control).handle_mouse_at(pt);
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
            let mut control = cell.borrow_mut();
            let focus = (&mut *control).handle_mouse_down(pt, state);
            if focus {
                self.next_control_idx = Some(i);
                return true;
            }
        }
        false
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        for (_, cell) in &mut self.controls.iter().enumerate() {
            let mut control = cell.borrow_mut();
            let focus = (&mut *control).handle_mouse_up(pt, state);
            if focus {
                return true;
            }
        }
        false
    }

    fn handle_mouse_scroll(&mut self, pt: &Vector, state: &mut TKState) {
        for cell in &mut self.controls {
            let mut control = cell.borrow_mut();
            (&mut *control).handle_mouse_scroll(pt, state);
        }
    }

    fn handle_key_press(&mut self, c: char, window: &mut Window) {
        if let Some(active_idx) = self.active_control_idx {
            let cell = &mut self.controls[active_idx];
            let mut control = cell.borrow_mut();
            (&mut *control).handle_key_press(c, window);
        }
    }

    // #[allow(unused_assignments)]
    fn handle_key_command(&mut self, key: &Key, window: &mut Window) -> bool {
        if let Some(active_idx) = self.active_control_idx {
            let controls_count = self.controls.len();
            let cell = &mut self.controls[active_idx];
            let mut control = cell.borrow_mut();
            let handled = (&mut *control).handle_key_command(key, window);
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
                            log::debug!("next_idx={:?} WAS={:?}", next_idx, active_idx);
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
