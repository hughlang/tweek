/// ListBox
///
use crate::core::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Background::Img, Color, FontStyle, Image},
    lifecycle::Window,
};

use std::any::TypeId;
use std::f32;
use std::ops::Range;

use super::*;

/// This is the multiplier that affects how quickly scrolling occurs.
const SCROLL_FACTOR: f32 = 1.0;

// *****************************************************************************************************
// ListBox
// *****************************************************************************************************

/// The ListBox is a simple vertical scrolling space with a collection of rows/cells that
/// can move in and out of view. When scrolling content, the rows that enter and leave the frame
/// will overflow the intended bounds of the listbox. For that reason, there is a process in the parent
/// Scene that will paint over the overflow with the default background color. This is a temporary
/// solution until there is a convenient means of clipping the rendered content.
///
/// A datasource will feed data to the ListBox as needed to populate the rows with content.
/// TBD: How to dynamically define the row content and specs?
///
pub struct ListBox {
    pub layer: TweenLayer,
    pub hover_row: Option<usize>,
    pub select_row: Option<usize>,
    pub row_height: f32,
    pub datasource: Vec<String>,
    rows: Vec<ListBoxRow>,
    defaults: Vec<Prop>,
    on_hover: Option<UITransition>,
    on_row_hover: Option<UITransition>,
    on_row_select: Option<UITransition>,
    scroll_offset: f32,
}

impl ListBox {
    pub fn new(frame: &Rectangle) -> Self {
        let mut layer = TweenLayer::new(frame.clone());
        layer.border_color = Some(Color::from_hex("#333333"));
        layer.border_width = 1.0;

        let row_hover = UITransition::new(vec![color(0xEEEEEE)], 0.0);
        let row_select = UITransition::new(vec![color(0xCCCCCC)], 0.1);

        ListBox {
            layer: layer,
            hover_row: None,
            select_row: None,
            row_height: 20.0,
            datasource: Vec::new(),
            rows: Vec::new(),
            defaults: Vec::new(),
            on_hover: None,
            on_row_hover: Some(row_hover),
            on_row_select: Some(row_select),
            scroll_offset: 0.0,
        }
    }

    pub fn set_on_hover(&mut self, props: Vec<Prop>, seconds: f64) {
        let transition = UITransition::new(props, seconds);
        self.on_hover = Some(transition);
    }

    pub fn set_on_row_hover(&mut self, props: Vec<Prop>, seconds: f64) {
        let transition = UITransition::new(props, seconds);
        self.on_row_hover = Some(transition);
    }

    pub fn datasource(mut self, ds: Vec<String>) -> Self {
        // Create only the rows needed to fill the visible range. Each row is a template
        // that is populated during the render phase
        let row_count = (self.layer.frame.size.y / self.row_height) as usize + 1;

        for i in 0..row_count {
            let rect = Rectangle::new(
                (self.layer.frame.pos.x, self.layer.frame.pos.y + self.row_height * i as f32),
                (self.layer.frame.size.x, self.row_height),
            );
            let row = ListBoxRow::new(rect);
            self.rows.push(row);
        }
        self.datasource = ds;
        self
    }

    pub fn get_visible_range(&self) -> Range<usize> {
        let shift = self.scroll_offset / self.row_height;
        let start = shift.floor() as usize;
        let row_count = (self.layer.frame.size.y / self.row_height + shift.fract()).ceil() as usize;
        // log::debug!("offset={:?} shift={:?}", self.scroll_offset, shift);
        // log::debug!("start={:?} count={:?}", start, row_count);
        return start..(start + row_count);
    }
}

// *****************************************************************************************************
// ListBox :: Displayable
// *****************************************************************************************************

impl TKDisplayable for ListBox {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ListBox>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn set_theme(&mut self, theme: &Theme) {
        self.layer.color = theme.bg_color;
        for (_, row) in &mut self.rows.iter_mut().enumerate() {
            row.set_theme(&theme);
            row.load_defaults();
        }
    }

    fn get_perimeter_frame(&self) -> Option<Rectangle> {
        let perimeter = self.layer.offset_by(0.0, -self.row_height, 0.0, -self.row_height);
        Some(perimeter)
    }

    fn load_defaults(&mut self) {
        let transition = UITransition::new(vec![color(0xDDDDDD)], 0.1);
        self.on_hover = Some(transition);
        // TODO: Move this to another method to seamlessly load props and save them.
        self.defaults = Tween::load_props(&self.layer);
    }

    /// The ListBox should manage the UI state of its rows.
    /// The mouse state of a row should determine whether to reset or not.
    /// Otherwise, leave it alone?
    fn update(&mut self) -> TKResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.apply_updates(&update.props);
            }
        }

        // Provide usize index values that are outside the range of the array
        // Q: If there's a better way of unwrapping these, please suggest.
        let hover_index = self.hover_row.unwrap_or(self.datasource.len() + 1);
        let select_index = self.select_row.unwrap_or(self.datasource.len() + 1);

        let range = self.get_visible_range();
        let offset = range.start;

        for (i, row) in &mut self.rows.iter_mut().enumerate() {
            &row.update()?;
            if i + offset == select_index {
                match row.layer.mouse_state {
                    MouseState::Select => {
                        if let Some(animation) = &row.layer.animation {
                            if animation.state == TweenState::Completed {
                                row.layer.mouse_state = MouseState::None;
                                // row.layer.apply_updates(&row.defaults);
                                row.layer.animation = None;
                                self.select_row = None;
                            }
                        } else {
                            row.layer.mouse_state = MouseState::None;
                            // row.layer.apply_updates(&row.defaults);
                            row.layer.animation = None;
                        }
                    }
                    MouseState::Hover => {
                        row.layer.mouse_state = MouseState::Select;
                        if let Some(transition) = &self.on_row_select {
                            if transition.seconds > 0.0 {
                                let mut tween = Tween::with(i, &row.layer)
                                    .to(&transition.props.to_vec())
                                    .duration(transition.seconds)
                                    // .debug()
                                    ;
                                &tween.play();
                                row.layer.animation = Some(tween);
                            } else {
                                row.layer.apply_updates(&transition.props.clone());
                            }
                        }
                    }
                    _ => (),
                }
            } else if i + offset == hover_index {
                match row.layer.mouse_state {
                    MouseState::None => {
                        row.layer.mouse_state = MouseState::Hover;
                        // show hover animation
                        if let Some(transition) = &self.on_row_hover {
                            if transition.seconds > 0.0 {
                                let mut tween = Tween::with(i, &row.layer)
                                    .to(&transition.props.to_vec())
                                    .duration(transition.seconds);
                                &tween.play();
                                row.layer.animation = Some(tween);
                            } else {
                                row.layer.apply_updates(&transition.props.clone());
                            }
                        }
                    }
                    _ => (),
                }
            } else {
                match row.layer.mouse_state {
                    MouseState::None => {}
                    MouseState::Hover => {
                        // if i != hover_index {
                        row.layer.mouse_state = MouseState::None;
                        row.layer.apply_updates(&row.defaults); // FIXME: dry
                        row.layer.animation = None;
                        // }
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) -> TKResult {
        let frame = self.layer.frame;

        // TODO: Render background?

        // Iterate through the rows that are in the visible range
        let range = self.get_visible_range();
        let shift_y = self.scroll_offset % self.row_height;
        let offset = range.start;
        for (i, row) in &mut self.rows.iter_mut().enumerate() {
            let ypos = frame.pos.y - shift_y + self.row_height * i as f32;
            let rect = Rectangle::new((frame.pos.x, ypos), (frame.size.x, self.row_height));
            window.draw(&rect, Col(row.layer.color));

            if offset + i < self.datasource.len() {
                let data = &self.datasource[offset + i];
                row.render_with_text(&data, &rect, theme, window);
            }
        }

        let content_height = self.datasource.len() as f32 * self.row_height;
        if let Some(rect) = UITools::get_scrollbar_frame(content_height, &self.layer.frame, self.scroll_offset) {
            window.draw(&rect, Col(Color::from_hex(UITools::SCROLLBAR_COLOR)));
        }

        if let Some(color) = self.layer.border_color {
            for line in self.layer.get_border_lines(self.layer.border_width) {
                window.draw_ex(&line.with_thickness(line.t), Col(color), Transform::IDENTITY, 0);
            }
        }

        Ok(())
    }

    fn set_hover_animation(&mut self, props: &[Prop], seconds: f64) {
        self.defaults = Tween::load_props(&self.layer);
        let transition = UITransition::new(props.to_vec(), seconds);
        self.on_hover = Some(transition);
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        let range = self.get_visible_range();
        if pt.overlaps_rectangle(&self.layer.frame) {
            self.layer.mouse_state = MouseState::Hover;
            let local_y = pt.y - self.layer.frame.pos.y;
            let mut index = (local_y / self.row_height).floor() as usize;
            index += range.start;
            if index < self.datasource.len() {
                // log::debug!("hover index={:?} y={:?}", index, 0);
                self.hover_row = Some(index);
                return true;
            }
        }
        self.layer.mouse_state = MouseState::None;
        self.hover_row = None;
        false
    }
}

// *****************************************************************************************************
// ListBox :: TKResponder
// *****************************************************************************************************

impl TKResponder for ListBox {
    fn has_focus(&self) -> bool {
        false
    }

    /// Calculate whether x, y are in the rect bounds of any child row with maths.
    /// Identify the row by getting x, y offset from listbox origin.
    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut TKState) -> bool {
        let range = self.get_visible_range();

        if pt.overlaps_rectangle(&self.layer.frame) {
            let local_y = pt.y - self.layer.frame.pos.y;
            // example: row_height=20. If local_y=50, then it is row_index=2
            let mut index = (local_y / self.row_height).floor() as usize;
            index += range.start;
            log::debug!("local_y={:?} clicked row={:?}", local_y, index);
            if index < self.datasource.len() {
                state.row_target = Some(index);
                self.select_row = Some(index);
                return true;
            }
        }
        false
    }

    /// Add or subtract from the layer content offset
    fn handle_mouse_scroll(&mut self, pt: &Vector, _state: &mut TKState) {
        match self.layer.mouse_state {
            MouseState::Hover => {
                // Calculate upper_limit as the content size outside of the frame.
                let upper_limit = self.datasource.len() as f32 * self.row_height - self.layer.frame.height();
                let eval_y = ((self.scroll_offset + pt.y) * SCROLL_FACTOR).min(upper_limit);
                // log::debug!("pt.y={:?} eval_y={:?}", pt.y, eval_y);
                self.scroll_offset = eval_y.max(0.0);
            }
            _ => (),
        }
    }
}

// *****************************************************************************************************
// ListBoxRow
// *****************************************************************************************************

pub struct ListBoxRow {
    pub row_id: usize, // this is optional
    pub layer: TweenLayer,
    pub string: String,
    pub image_text: Option<Image>,
    pub defaults: Vec<Prop>,
    // hover_animation: Option<UITransition>,
    // mouse_state: MouseState,
    // onclick: Option<Box<FnMut(TKAction, &mut TKState) + 'static>>,
}

impl ListBoxRow {
    pub fn new(frame: Rectangle) -> Self {
        let layer = TweenLayer::new(frame);

        ListBoxRow {
            row_id: 0,
            layer: layer,
            string: String::default(),
            image_text: None,
            defaults: Vec::new(),
            // hover_animation: None,
            // mouse_state: MouseState::None,
            // onclick: None,
        }
    }

    pub fn row(mut self, id: usize) -> Self {
        self.row_id = id;
        self
    }

    pub fn render_with_text(&mut self, string: &str, rect: &Rectangle, theme: &mut Theme, window: &mut Window) {
        let style = FontStyle::new(theme.font_size, Color::BLACK);
        let mut text = Text::new(&rect, string).margin(8.0, 5.0);
        text.layer.font_style = style;
        let _ = text.render(theme, window);
    }
}

// *****************************************************************************************************
// ListBoxRow :: Displayable
// *****************************************************************************************************

impl TKDisplayable for ListBoxRow {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ListBoxRow>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn set_theme(&mut self, theme: &Theme) {
        self.layer.color = theme.bg_color;
    }

    fn load_defaults(&mut self) {
        self.defaults = Tween::load_props(&self.layer);
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
}
