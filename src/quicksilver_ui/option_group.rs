/// OptionGroup
///
use crate::core::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Line, Rectangle, Shape, Vector},
    graphics::{Background::Col, Color, Image},
    lifecycle::Window,
};
use std::any::TypeId;

use super::*;

pub enum OptionGroupLayout {
    Vertical(f32),            // Layout checkboxes with fixed vertical spacing between each
    HorizontalGrid(f32, f32), // First f32 is the fixed interval for placement of checkboxes that wrap to next line
    HorizontalWrap(f32, f32), // Layout checkboxes with fixed horizontal spacing between each and wrap to next line
}

//-- OptionGroup -----------------------------------------------------------------------

#[allow(dead_code)]
pub struct OptionGroup {
    pub layer: TweenLayer,
    pub multi_select: bool,
    pub check_style: CheckStyle,
    checkboxes: Vec<Checkbox>,
    layout: OptionGroupLayout,
}

impl OptionGroup {
    pub fn new(frame: Rectangle) -> Self {
        let layer = TweenLayer::new(frame);

        OptionGroup {
            layer: layer,
            multi_select: false,
            check_style: CheckStyle::X,
            checkboxes: Vec::new(),
            layout: OptionGroupLayout::Vertical(10.0),
        }
    }

    pub fn set_layout(&mut self, layout: OptionGroupLayout) {
        self.layout = layout;
    }

    pub fn set_options(&mut self, options: Vec<(&str, bool)>) {
        let line_height = 20.0;

        // Create checkboxes and set the default frames for vertical layout.
        // These will get updated in set_theme
        for (i, option) in options.iter().enumerate() {
            let xpos = self.layer.frame.x();
            let ypos = self.layer.frame.y() + line_height * i as f32;
            let frame = Rectangle::new((xpos, ypos), (self.layer.frame.width(), line_height));
            let mut checkbox = Checkbox::new(frame).with_text(option.0, option.1);
            checkbox.check_style = self.check_style.clone();
            self.checkboxes.push(checkbox);
        }
    }

    fn layout_subviews(&mut self, theme: &Theme) {
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
                        // If the next element fits on the current line, calculate the xpos
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
                    // eprintln!("row_size={:?} add_size={:?} frame={:?}", row_size, add_size, frame);
                    frame
                }
                OptionGroupLayout::HorizontalWrap(h_space, v_space) => {
                    let row_height = line_height + v_space;
                    let add_size = checkbox.get_content_size();
                    let mut xpos = self.layer.frame.x();

                    if row_size.x + add_size.x <= self.layer.frame.width() {
                        // If the next element fits on the current line, calculate the xpos
                        // and increase the width of the current row size.
                        xpos += row_size.x;
                        // Set the row_size width including the h_space
                        row_size.x += add_size.x + h_space;
                    } else {
                        // Otherwise, push it to the next line.
                        row_size.x = add_size.x + h_space;
                        row_size.y += row_height;
                    }
                    let ypos = self.layer.frame.y() + row_size.y + 10.0;
                    let frame = Rectangle::new((xpos, ypos), (add_size.x, line_height));
                    // eprintln!("row_size={:?} add_size={:?} frame={:?}", row_size, add_size, frame);
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

impl TKDisplayable for OptionGroup {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<OptionGroup>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    /// Change the font, color, and size
    fn set_theme(&mut self, theme: &Theme) {
        if theme.border_width > 0.0 {
            self.layer.border_width = theme.border_width;
            self.layer.border_color = Some(theme.border_color);
        }
        self.layout_subviews(theme);
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Activate => {}
            DisplayEvent::Deactivate => {}
            DisplayEvent::Ready => {}
            _ => (),
        }
    }

    fn update(&mut self) -> TKResult {
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

        for checkbox in &mut self.checkboxes {
            checkbox.render(theme, window)?;
        }

        // Draw border
        if let Some(color) = self.layer.border_color {
            for line in self.layer.get_border_lines(self.layer.border_width) {
                window.draw(&line.with_thickness(line.t), Col(color));
            }
        }

        Ok(())
    }

    fn set_hover_animation(&mut self, props: &[Prop], seconds: f64) {
        self.layer.defaults = Tween::load_props(&self.layer);
        let transition = UITransition::new(props.to_vec(), seconds);
        self.layer.on_hover = Some(transition);
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        if pt.overlaps_rectangle(&self.layer.frame) {
            for checkbox in &mut self.checkboxes {
                let hit = checkbox.handle_mouse_at(pt);
                if hit {
                    return true;
                }
            }
        }
        false
    }
}

// *****************************************************************************************************
// OptionGroup :: TKResponder
// *****************************************************************************************************

impl TKResponder for OptionGroup {
    fn get_field_value(&self) -> FieldValue {
        let results: Vec<usize> =
            self.checkboxes.iter().enumerate().filter(|(_, option)| option.is_checked).map(|(i, _)| i).collect();

        FieldValue::Selections(results)
    }

    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut TKState) -> bool {
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
                        }
                    }
                }
                return true;
            }
        }
        false
    }
}
