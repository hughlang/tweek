/// Button
///
use super::*;
use crate::core::*;
use crate::events::*;
use crate::tools::TextAlign;

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Color, FontStyle},
    input::{MouseCursor},
    lifecycle::Window,
};
use std::any::TypeId;

//-- Button -----------------------------------------------------------------------

/// A basic Button object
pub struct Button {
    /// The underlying Layer
    pub layer: Layer,
    /// Optional button text
    text: Option<Text>,
    /// Optional Label which can have static image and/or text
    label: Option<Label>,
}

impl Button {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let mut layer = Layer::new(frame);
        layer.bg_style = BackgroundStyle::Solid(Color::from_hex("#AAAAAA"));
        Button { layer, text: None, label: None }
    }

    /// Builder method to apply the specified background style
    pub fn background(mut self, style: BackgroundStyle) -> Self {
        self.layer.bg_style = style;
        self
    }

    /// Define border style. Default is none
    pub fn border(mut self, style: BorderStyle) -> Self {
        self.layer.border_style = style;
        self
    }

    /// Builder method to set a text label for the button
    pub fn with_text(mut self, string: &str) -> Self {
        let mut text = Text::new(self.layer.frame, string);
        text.text_align(TextAlign::Center);
        self.text = Some(text);
        self
    }

    /// Method to set a text label for the button.
    pub fn set_text(&mut self, string: &str) {
        let mut text = Text::new(self.layer.frame, string);
        text.text_align(TextAlign::Center);
        self.text = Some(text);
    }

    pub fn set_label(&mut self, label: Label) {
        self.label = Some(label);
    }

    /// Set the callback function for click action
    pub fn set_onclick<C>(&mut self, cb: C)
    where
        C: FnMut(&mut AppState) + 'static,
    {
        self.layer.on_click = Some(Box::new(cb));
    }

    pub fn set_click_event<E: AnyEvent>(&mut self, event: E) {
        self.layer.click_action = Some(Box::new(move |state: &mut AppState, source: String| {
            state.event_bus.dispatch_event(event, source);
        }));
    }

}

// *****************************************************************************************************
// Button :: Displayable
// *****************************************************************************************************

impl Displayable for Button {
    fn get_id(&self) -> u32 {
        self.layer.get_id()
    }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
        if let Some(text) = &mut self.text {
            text.set_id(id);
        }
        if let Some(label) = &mut self.label {
            label.set_id(id);
        }
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Button>()
    }

    fn get_layer(&self) -> &Layer { &self.layer }

    fn get_layer_mut(&mut self) -> &mut Layer { &mut self.layer }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn get_content_size(&self) -> Vector {
        self.layer.frame.size
    }

    fn move_to(&mut self, pos: (f32, f32)) {
        self.layer.frame.pos.x = pos.0;
        self.layer.frame.pos.y = pos.1;
    }

    /// Change the button font, color, and size
    fn set_theme(&mut self, theme: &mut Theme) {
        let ok = self.layer.apply_theme(theme);
        if !ok {
            return;
        }
        self.layer.bg_style = BackgroundStyle::Solid(theme.button_bg_color);
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Ready => {
                self.layer.on_ready();
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
                if let Some(view) = &mut self.text {
                    view.notify(event);
                }
            }
            _ => {}
        }
    }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        self.layer.frame.pos = self.layer.initial.pos + Vector::new(state.offset.0, state.offset.1);
        self.layer.tween_update();

        let events = self.layer.notifications.borrow_mut().events.filter::<LayerEvent>();
        for evt in events {
            match evt {
                LayerEvent::Hover(_id, _type_id, state) | LayerEvent::Click(_id, _type_id, state) => match state {
                    PlayState::Completed => {
                        // Cleanup?
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        if let Some(text) = &mut self.text {
            text.update(window, state);
        }
    }

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        let hover = self.layer.handle_mouse_over(pt);
        if hover {
            window.set_cursor(MouseCursor::Hand);
        } else {
            window.set_cursor(MouseCursor::Default);
        }
        hover
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        // TODO: Tween hover animation
        self.layer.draw_background(window);
        if let Some(text) = &mut self.text {
            text.render(theme, window);
        } else if let Some(label) = &mut self.label {
            label.render(theme, window);
        }
    }

    fn set_hover_animation(&mut self, props: PropSet) {
        self.layer.hover_effect = Some(props);
    }

    fn set_click_animation(&mut self, props: PropSet) {
        self.layer.click_effect = Some(props);
    }

    fn set_tint(&mut self, color: Color) {
        if let Some(text) = &mut self.text {
            text.layer.font_style = FontStyle::new(text.layer.font_style.get_size(), color);
        }
    }

    fn debug_out(&self) -> String {
        let mut rows: Vec<String> = Vec::new();
        let out = format!("{}{} {}", "", self.debug_id(), self.debug_frame());
        rows.push(out);
        if let Some(view) = &self.text {
            let out = format!("{}{} {}", "\n| ", view.debug_id(), view.debug_frame());
            rows.push(out);
        }
        if let Some(view) = &self.label {
            let out = format!("{}{} {}", "\n| ", view.debug_id(), view.debug_frame());
            rows.push(out);
        }
        let result = rows.join("");
        result
    }
}

// *****************************************************************************************************
// Button :: Responder
// *****************************************************************************************************

impl Responder for Button {
    fn handle_mouse_up(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        if pt.overlaps_rectangle(&self.layer.frame) {
            self.layer.handle_click_animation();
            if let Some(cb) = &mut self.layer.on_click {
                (&mut *cb)(state);
            }
            let path = self.layer.full_path();
            if let Some(cb) = &mut self.layer.click_action {
                (&mut *cb)(state, path);
            }
            return true;
        }
        false
    }
}
