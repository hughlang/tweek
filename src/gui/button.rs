/// Button
///
use super::*;
use crate::core::*;
use crate::events::*;

use glyph_brush::HorizontalAlign as HAlign;
use quicksilver::{
    geom::{Rectangle, Shape, Vector, Transform},
    graphics::{Color, FontStyle, Image, MeshTask},
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
    /// Optional image image
    image: Option<Image>,
    /// Cached mesh_task
    mesh_task: Option<MeshTask>,
    /// Holds scene offset value
    offset: Vector,
}

impl Button {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let mut layer = Layer::new(frame);
        layer.bg_style = BackgroundStyle::Solid(Color::from_hex("#AAAAAA"));
        // layer.init_props();
        let offset = Vector::ZERO;
        Button { layer, text: None, image: None, mesh_task: None, offset }
    }

    /// Builder method to apply the specified background style
    pub fn background(mut self, style: BackgroundStyle) -> Self {
        self.layer.bg_style = style;
        // self.layer.init_props();
        self
    }

    /// Define border style. Default is none
    pub fn border(mut self, style: BorderStyle) -> Self {
        self.layer.border_style = style;
        // self.layer.init_props();
        self
    }

    /// Builder method to set a text label for the button
    pub fn with_text(mut self, string: &str) -> Self {
        let mut text = Text::new(self.layer.frame, string);
        text.align_h(HAlign::Center);
        self.text = Some(text);
        self
    }

    /// Method to set a text label for the button.
    pub fn set_text(&mut self, string: &str) {
        let mut text = Text::new(self.layer.frame, string);
        text.align_h(HAlign::Center);
        self.text = Some(text);
    }

    /// Set an image image for the button
    pub fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    /// Set the callback function for click action
    pub fn set_onclick<C>(&mut self, cb: C)
    where
        C: FnMut(&mut AppState) + 'static,
    {
        self.layer.on_click = Some(Box::new(cb));
    }

}

// *****************************************************************************************************
// Button :: Displayable
// *****************************************************************************************************

impl Displayable for Button {

    fn get_id(&self) -> u32 { self.layer.get_id() }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Button>()
    }

    fn get_layer_mut(&mut self) -> &mut Layer {
        &mut self.layer
    }

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
        if self.layer.lock_style { return }
        self.layer.apply_theme(theme);
        self.layer.bg_style = BackgroundStyle::Solid(theme.button_bg_color);
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Ready => {
                self.layer.on_ready();
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
                if let Some(task) = &mut self.mesh_task {
                    for (_, vertex) in task.vertices.iter_mut().enumerate() {
                        vertex.pos = Transform::translate(self.offset) * vertex.pos;
                    }
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
                LayerEvent::Hover(id, type_id, state) | LayerEvent::Click(id, type_id, state) => {
                    match state {
                        PlayState::Completed => {
                            let out = self.debug_out();
                            eprintln!("{:?} –– {:?}", evt, out);

                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        }

        if let Some(view) = &mut self.text {
            view.update(window, state);
        }
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        // TODO: Tween hover animation
        self.layer.draw_background(window);
        if let Some(view) = &mut self.text {
            view.render(theme, window);
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

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        let hover = self.layer.handle_mouse_over(pt);
        // if hover { log::debug!("Button hover at pt={:?}", pt) }
        hover
    }

    fn debug_out(&self) -> String {
        let mut rows: Vec<String> = Vec::new();
        let frame = self.get_frame();
        let out = format!("{}<{}> [{}] Pos({:.1},{:.1}) Size({:.1},{:.1})", "", gui_print_type(&self.get_type_id()), self.get_id(), frame.pos.x, frame.pos.y, frame.size.x, frame.size.y);
        rows.push(out);
        if let Some(view) = &self.text {
            let frame = view.get_frame();
            let out = format!("{}<{}> [{}] Pos({:.1},{:.1}) Size({:.1},{:.1})", "\n| ", gui_print_type(&view.get_type_id()), view.get_id(), frame.pos.x, frame.pos.y, frame.size.x, frame.size.y);
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
            log::debug!("Click at: x={} y={}", pt.x, pt.y);
            self.layer.handle_click_animation();
            if let Some(cb) = &mut self.layer.on_click {
                (&mut *cb)(state);
            }

            return true;
        }
        false
    }
}
