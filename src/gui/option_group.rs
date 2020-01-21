/// OptionGroup
///
use crate::core::*;
use crate::events::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Color, Image},
    lifecycle::Window,
};
use std::any::TypeId;

use super::*;

/// Enum to define how the checkbox options are laid out within the group
pub enum OptionGroupLayout {
    /// Layout checkboxes with fixed vertical spacing between each
    Vertical(f32),
    /// First f32 is the fixed interval for placement of checkboxes that wrap to next line
    /// Second f32 is the vertical space between rows
    HorizontalGrid(f32, f32),
    /// First f32 is the horizontal gap between checkbox items
    /// Second f32 is the vertical space between rows
    HorizontalWrap(f32, f32),
}

//-- OptionGroup -----------------------------------------------------------------------

/// An object containing a collection of Checkbox objects with different layout options
pub struct OptionGroup {
    /// The base layer
    pub layer: Layer,
    /// Is it single or multi select?
    pub multi_select: bool,
    /// The CheckStyle of all included checkboxes
    pub check_style: CheckStyle,
    checkboxes: Vec<Checkbox>,
    layout: OptionGroupLayout,
}

impl OptionGroup {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);

        OptionGroup {
            layer,
            multi_select: false,
            check_style: CheckStyle::X,
            checkboxes: Vec::new(),
            layout: OptionGroupLayout::Vertical(10.0),
        }
    }

    /// Set the OptionGroupLayout
    pub fn set_layout(&mut self, layout: OptionGroupLayout) {
        self.layout = layout;
    }

    /// Set the list of string options and whether any are initially selected
    pub fn set_options(&mut self, options: Vec<(&str, bool)>) {
        let line_height = 20.0;
        let line_gap = 5.0;
        // Create checkboxes and set the default frames for vertical layout.
        // These will get updated in set_theme
        for (i, option) in options.iter().enumerate() {
            let xpos = self.layer.frame.x();
            let ypos = self.layer.frame.y() + (line_gap * (i + 1) as f32) + (line_height * i as f32);
            let frame = Rectangle::new((xpos, ypos), (self.layer.frame.width(), line_height));
            let mut checkbox = Checkbox::new(frame).with_text(option.0, option.1);
            checkbox.set_id(i as u32); // Assign the layer a unique id within the scope of this OptionGroup
            checkbox.check_style = self.check_style.clone();
            self.checkboxes.push(checkbox);
        }
    }

    /// Internal method to layout the Checkbox components based on the OptionGroupLayout
    fn layout_subviews(&mut self, theme: &mut Theme) {
        let line_height = 20.0;
        let mut row_size = Vector::new(0.0, 0.0);
        // The layout of checkboxes may depend on the content width, so update the theme which will re-render
        // the label text and provide actual sizing of checkbox content (instead of the entire row)
        for (i, checkbox) in self.checkboxes.iter_mut().enumerate() {
            checkbox.set_theme(theme);

            let frame = match self.layout {
                OptionGroupLayout::Vertical(vspace) => {
                    let xpos = self.layer.frame.x();
                    let ypos = self.layer.frame.y() + (line_height + vspace) * i as f32 + 10.0;
                    let frame = Rectangle::new((xpos, ypos), (self.layer.frame.width(), line_height));
                    frame
                }
                OptionGroupLayout::HorizontalGrid(interval, v_space) => {
                    let row_height = line_height + v_space;
                    let mut xpos = self.layer.frame.x();

                    if row_size.x + interval <= self.layer.frame.width() {
                        // If the next node fits on the current line, calculate the xpos
                        // and increase the width of the current row size.
                        xpos += row_size.x;
                        // Set the row_size width including the h_space
                        row_size.x += interval;
                    } else {
                        // Otherwise, push it to the next line.
                        row_size.x = interval;
                        row_size.y += row_height;
                    }
                    let ypos = self.layer.frame.y() + row_size.y + 10.0;
                    let frame = Rectangle::new((xpos, ypos), (interval, line_height));
                    // log::debug!("row_size={:?} add_size={:?} frame={:?}", row_size, add_size, frame);
                    frame
                }
                OptionGroupLayout::HorizontalWrap(h_space, v_space) => {
                    let row_height = line_height + v_space;
                    let add_size = theme.default_font.measure_text(&checkbox.text, theme.font_size);
                    // eprintln!("row_size={:?} // add_size={:?}", row_size, add_size);
                    let mut xpos = self.layer.frame.x();

                    if row_size.x + add_size.0 <= self.layer.frame.width() {
                        // If the next node fits on the current line, calculate the xpos
                        // and increase the width of the current row size.
                        xpos += row_size.x;
                        // Set the row_size width including the h_space
                        row_size.x += add_size.0 + h_space;
                    } else {
                        // Otherwise, push it to the next line.
                        row_size.x = add_size.0 + h_space;
                        row_size.y += row_height;
                    }
                    let ypos = self.layer.frame.y() + row_size.y + 10.0;
                    let frame = Rectangle::new((xpos, ypos), (add_size.0, line_height));
                    log::debug!("row_size={:?} add_size={:?} frame={:?}", row_size, add_size, frame);
                    frame
                }
            };
            checkbox.update_frame(frame);
        }
    }
}

// *****************************************************************************************************
// OptionGroup :: Displayable
// *****************************************************************************************************

impl Displayable for OptionGroup {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<OptionGroup>()
    }

    fn get_layer(&self) -> &Layer {
        &self.layer
    }

    fn get_layer_mut(&mut self) -> &mut Layer {
        &mut self.layer
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn move_to(&mut self, pos: (f32, f32)) {
        self.layer.frame.pos.x = pos.0;
        self.layer.frame.pos.y = pos.1;
    }

    /// Change the font, color, and size
    fn set_theme(&mut self, theme: &mut Theme) {
        let ok = self.layer.apply_theme(theme);
        if !ok {
            return;
        }
        self.layout_subviews(theme);
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Ready => {
                self.layer.on_ready();
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
            }
            _ => {}
        }
        for checkbox in &mut self.checkboxes {
            checkbox.notify(event);
        }
    }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        self.layer.tween_update(state);
        for checkbox in &mut self.checkboxes {
            checkbox.update(window, state);
        }
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        // self.layer.draw_background(window);

        for checkbox in &mut self.checkboxes {
            checkbox.render(theme, window);
        }

        // Draw border
        self.layer.draw_border(window);
    }

    fn set_hover_animation(&mut self, props: PropSet) {
        self.layer.hover_effect = Some(props);
    }

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        if pt.overlaps_rectangle(&self.layer.frame) {
            for checkbox in &mut self.checkboxes {
                let hit = checkbox.handle_mouse_at(pt, window);
                if hit {
                    return true;
                }
            }
        }
        false
    }

    fn debug_out(&self) -> String {
        let mut rows: Vec<String> = Vec::new();
        let out = format!("{}{} {}", "", self.debug_id(), self.debug_frame());
        rows.push(out);
        for cb in &self.checkboxes {
            // For child objects, prepend the output with a linebreak and pipe+space to conform with scene hierarchy style
            let out = format!("{}{} {}", "\n| ", cb.debug_id(), cb.debug_frame());
            rows.push(out);
        }
        let result = rows.join(""); // Note, it doesn't work to try putting the linebreak in the join character.
        result
    }
}

// *****************************************************************************************************
// OptionGroup :: Responder
// *****************************************************************************************************

impl Responder for OptionGroup {
    fn get_field_value(&self) -> FieldValue {
        let results: Vec<usize> =
            self.checkboxes.iter().enumerate().filter(|(_, option)| option.is_checked).map(|(i, _)| i).collect();

        FieldValue::Selections(results)
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut AppState) -> bool {
        if pt.overlaps_rectangle(&self.layer.frame) {
            let mut hit_index: Option<usize> = None;
            for (i, checkbox) in &mut self.checkboxes.iter_mut().enumerate() {
                let hit = checkbox.handle_mouse_down(pt, state);
                if hit {
                    log::debug!("Click at: i={} pt={:?}", i, pt);
                    hit_index = Some(i);
                }
            }
            if let Some(idx) = hit_index {
                if !self.multi_select {
                    for (i, checkbox) in &mut self.checkboxes.iter_mut().enumerate() {
                        if i != idx {
                            checkbox.is_checked = false;
                            checkbox.clear_draw_cache();
                        }
                    }
                }
                return true;
            }
        }
        false
    }
}
