/// Toolbar
///
use crate::core::*;

// TODO: try out
// use cassowary::{
//     Solver, Variable, WeightedRelation::*, strength::*,
// };

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::Background::Col,
    lifecycle::Window,
};

use std::any::TypeId;
use std::ops::Range;

use super::*;

pub enum ToolbarStyle {
    Buttons,
    Tabs,
}

pub enum ToolbarLayout {
    FixedWidth,
    FullEqual,
    AlignCenter,
}

pub struct ButtonGroup {
    pub multiselect: bool,
    pub range: Range<usize>,
}

//-- Toolbar -----------------------------------------------------------------------

#[allow(dead_code)]
pub struct Toolbar {
    pub layer: TweenLayer,
    buttons: Vec<Button>,
    /// This will hold the groupings of buttons. Probably needs a special wrapper struct
    pub groups: Vec<ButtonGroup>,
    pub item_gap: f32,
    pub group_gap: f32,
}

impl Toolbar {
    pub fn new(frame: Rectangle) -> Self {
        let layer = TweenLayer::new(frame);
        Toolbar { layer: layer, buttons: Vec::new(), groups: Vec::new(), item_gap: 1.0, group_gap: 4.0 }
    }

    /// Insert a button at the specified index and consuming it.
    /// FIXME: When inserting a button into existing array of buttons, all of the frames need to get
    /// updated.
    pub fn insert_button(&mut self, mut btn: Button, index: usize) {
        let mut xpos: f32 = self.buttons.iter().take(index).map(|b| b.layer.frame.width()).sum();
        xpos += self.buttons.len() as f32 * self.item_gap;
        btn.set_origin(&Vector::new(self.layer.frame.x() + xpos, self.layer.frame.y()));
        self.buttons.insert(index, btn);
    }

    pub fn add_button(&mut self, btn: Button) {
        self.insert_button(btn, self.buttons.len());
    }
}

// *****************************************************************************************************
// Toolbar :: Displayable
// *****************************************************************************************************

impl TKDisplayable for Toolbar {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Toolbar>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    /// Change the button font, color, and size
    fn set_theme(&mut self, theme: &Theme) {
        for button in &mut self.buttons {
            button.set_theme(theme);
        }
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Ready => {
                for button in &mut self.buttons {
                    button.notify(event);
                }
            }
            _ => {}
        }
    }

    fn update(&mut self) -> TKResult {
        for btn in &mut self.buttons {
            btn.update()?;
        }

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

        for btn in &mut self.buttons {
            btn.render(theme, window)?;
        }

        Ok(())
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        for button in &mut self.buttons {
            let hover = button.handle_mouse_at(pt);
            if hover {
                return true;
            }
        }
        false
    }
}

// *****************************************************************************************************
// Toolbar :: TKResponder
// *****************************************************************************************************

impl TKResponder for Toolbar {
    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        if pt.overlaps_rectangle(&self.layer.frame) {
            for button in &mut self.buttons {
                let hover = button.handle_mouse_up(pt, state);
                if hover {
                    return true;
                }
            }
        }
        false
    }
}
