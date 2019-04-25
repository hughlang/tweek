use super::*;
use crate::core::*;

use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::f32;
use std::rc::Rc;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color, Font},
    lifecycle::{Event, Window},
};

// pub const TEXT_KEY_COMMANDS: &[KeyCode] = &[
//     KeyCode::Back,
//     KeyCode::Tab,
//     KeyCode::Left,
//     KeyCode::Right,
//     KeyCode::Return,
//     KeyCode::Escape,
// ];

pub struct Scene {
    pub layer: TweenLayer,
    pub timeline: Option<Timeline>,
    pub views: Vec<Rc<RefCell<TKDisplayable>>>,
    pub controls: Vec<Rc<RefCell<TKResponder>>>,
    /// index in controls vec of currently selected control (ie, textfield)
    active_control_idx: Option<usize>,
    next_control_idx: Option<usize>,
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
            types_map: HashMap::new(),
        }
    }

    // This is an experimental means of differentiating different types. Each Displayable should
    // provide it's type_id() and therefore allow the system to know how to handle different
    // object types.
    // pub fn prepare(mut self) -> Self {
    //     self.types_map.insert(TypeId::of::<ButtonView>(), "ButtonView".to_string());
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
    fn render(&mut self, theme: &Theme, window: &mut Window) -> TKResult {
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

        // if mask_areas.len() > 0 {
        //     let mut builder = graphics::MeshBuilder::new();
        //     for rect in mask_areas {
        //         // log::debug!("rect={:?} color={:?}", rect, self.layer.graphics.color);
        //         builder.rectangle(DrawMode::fill(), rect, self.layer.graphics.color);
        //     }
        //     let mesh = builder.build(ctx)?;
        //     graphics::draw(ctx, &mesh, DrawParam::default())?;
        // }
        Ok(())
    }
}

impl TKResponder for Scene {
    fn has_focus(&self) -> bool {
        false
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        for cell in &mut self.controls {
            let mut control = cell.borrow_mut();
            let hover = (&mut *control).handle_mouse_at(pt);
            if hover {
                return true;
            }
        }
        false
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        for (i, cell) in &mut self.controls.iter().enumerate() {
            let mut control = cell.borrow_mut();
            let focus = (&mut *control).handle_mouse_down(pt, state);
            if focus {
                self.active_control_idx = Some(i);
                return true;
            }
        }
        false
    }

    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        for (i, cell) in &mut self.controls.iter().enumerate() {
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
    // fn handle_key_command(&mut self, code: KeyCode, keymods: KeyMods, window: &mut Window) -> bool {
    //     if let Some(active_idx) = self.active_control_idx {
    //         let controls_count = self.controls.len();
    //         let cell = &mut self.controls[active_idx];
    //         let mut control = cell.borrow_mut();
    //         let handled = (&mut *control).handle_key_command(code, keymods, ctx);
    //         if handled {
    //             match code {
    //                 KeyCode::Tab => {
    //                     // if textfield, try to advance to next field.
    //                     let mut next_idx = usize::max_value();
    //                     if active_idx + 1 == controls_count {
    //                         next_idx = 0;
    //                     } else {
    //                         next_idx = active_idx + 1;
    //                     }
    //                     if next_idx != active_idx {
    //                         log::debug!("next_idx={:?} WAS={:?}", next_idx, active_idx);
    //                         self.next_control_idx = Some(next_idx);
    //                     }
    //                 }
    //                 KeyCode::Return => {}
    //                 _ => (),
    //             }
    //             return true;
    //         }
    //     } else {
    //         // TODO: Check other listeners
    //     }
    //     false
    // }
}
