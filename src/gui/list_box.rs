/// ListBox
///
use crate::core::*;
use crate::tools::*;
use crate::events::*;

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
///
pub struct ListBox {
    /// The base layer
    pub layer: Layer,
    /// The index of the currently hovered row
    pub hover_row: Option<usize>,
    /// The index of the currently selected row
    pub select_row: Option<usize>,
    /// The row height
    pub row_height: f32,
    /// The datasource as an array of strings
    pub datasource: Vec<String>,
    /// The array of visible ListBoxRow objects
    rows: Vec<ListBoxRow>,
    /// Hover animation for the ListBox
    on_hover: Option<PropSet>,
    /// Hover animation for the ListBoxRow.
    /// FIXME: Unused?
    on_row_hover: Option<PropSet>,
    /// Animation for row select
    on_row_select: Option<PropSet>,
    /// Stores the scroll offset
    scroll_offset: f32,
}

impl ListBox {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let mut layer = Layer::new(frame);
        layer.border_style = BorderStyle::SolidLine(Color::from_hex("#333333"), 1.0);

        let row_hover = PropSet::new(vec![color("#EEEEEE")], 0.0)
            .for_type(TweenType::Hover);
        let row_select = PropSet::new(vec![color("#CCCCCC")], 0.1)
            .for_type(TweenType::Click);

        ListBox {
            layer,
            hover_row: None,
            select_row: None,
            row_height: 20.0,
            datasource: Vec::new(),
            rows: Vec::new(),
            on_hover: None,
            on_row_hover: Some(row_hover),
            on_row_select: Some(row_select),
            scroll_offset: 0.0,
        }
    }

    /// Builder method to set the datasource
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

    /// Based on the scroll_offset, determine what rows are visible in the scrollable frame
    fn get_visible_range(&self) -> Range<usize> {
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

impl Displayable for ListBox {

    fn get_id(&self) -> u32 { self.layer.get_id() }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ListBox>()
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

    fn set_theme(&mut self, theme: &mut Theme) {
        if self.layer.lock_style { return }
        self.layer.apply_theme(theme);
        for (_, row) in &mut self.rows.iter_mut().enumerate() {
            row.set_theme(theme);
            // FIXME: This is the only usage
            // row.load_defaults();
        }
    }

    fn get_perimeter_frame(&self) -> Option<Rectangle> {
        let perimeter = self.layer.offset_by(0.0, -self.row_height, 0.0, -self.row_height);
        Some(perimeter)
    }

    // fn load_defaults(&mut self) {
    //     let transition = PropSet::new(vec![color("#DDDDDD")], 0.1);
    //     self.on_hover = Some(transition);
    //     // TODO: Move this to another method to seamlessly load props and save them.
    //     self.layer.defaults = Tween::load_props(&self.layer);
    // }

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
    }

    /// The ListBox should manage the UI state of its rows.
    /// The mouse state of a row should determine whether to reset or not.
    /// Otherwise, leave it alone?
    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        let offset = Vector::new(state.offset.0, state.offset.1);
        self.layer.frame.pos = self.layer.initial.pos + offset;
        self.layer.tween_update();

        // Provide usize index values that are outside the range of the array
        // Q: If there's a better way of unwrapping these, please suggest.
        let hover_index = self.hover_row.unwrap_or(self.datasource.len() + 1);
        let select_index = self.select_row.unwrap_or(self.datasource.len() + 1);

        let range = self.get_visible_range();

        for (i, row) in &mut self.rows.iter_mut().enumerate() {
            row.update(window, state);
            if i + range.start == select_index {
                match row.layer.mouse_state {
                    MouseState::Select => {
                        if let Some(animation) = &row.layer.animation {
                            if animation.state == PlayState::Completed {
                                row.layer.mouse_state = MouseState::None;
                                row.layer.animation = None;
                                self.select_row = None;
                            }
                        } else {
                            row.layer.mouse_state = MouseState::None;
                            row.layer.animation = None;
                        }
                    }
                    MouseState::Hover => {
                        row.layer.mouse_state = MouseState::Select;
                        if let Some(transition) = &self.on_row_select {
                            if transition.duration > 0.0 {
                                row.layer.animate_with_props(transition.clone());
                            } else {
                                row.layer.apply_props(&transition.props.clone());
                            }
                        }
                    }
                    _ => (),
                }
            } else if i + range.start == hover_index {
                match row.layer.mouse_state {
                    MouseState::None => {
                        row.layer.mouse_state = MouseState::Hover;
                        // show hover animation
                        if let Some(transition) = &self.on_row_hover {
                            if transition.duration > 0.0 {
                                row.layer.animate_with_props(transition.clone());
                            } else {
                                row.layer.apply_props(&transition.props.clone());
                            }
                        }
                    }
                    _ => (),
                }
            } else {
                match row.layer.mouse_state {
                    MouseState::None => {}
                    MouseState::Hover => {
                        if i != hover_index {
                            row.layer.mouse_state = MouseState::None;
                            row.layer.animation = None;
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        let frame = self.layer.frame;

        // TODO: Render background?

        // Iterate through the rows that are in the visible range
        let range = self.get_visible_range();
        let shift_y = self.scroll_offset % self.row_height;
        let offset = range.start;
        for (i, row) in &mut self.rows.iter_mut().enumerate() {
            let ypos = frame.pos.y - shift_y + self.row_height * i as f32;
            let rect = Rectangle::new((frame.pos.x, ypos), (frame.size.x, self.row_height));
            window.draw(&rect, Col(row.layer.transition.color));

            if offset + i < self.datasource.len() {
                let data = &self.datasource[offset + i];
                row.render_with_text(rect, &data, theme, window);
            }
        }

        let content_height = self.datasource.len() as f32 * self.row_height;
        if let Some(rect) = UITools::get_scrollbar_frame(content_height, &self.layer.frame, self.scroll_offset) {
            window.draw(&rect, Col(Color::from_hex(UITools::SCROLLBAR_COLOR)));
        }

        self.layer.draw_border(window);
    }

    fn set_hover_animation(&mut self, props: PropSet) {
        self.layer.hover_effect = Some(props);
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
// ListBox :: Responder
// *****************************************************************************************************

impl Responder for ListBox {

    /// Calculate whether x, y are in the rect bounds of any child row with maths.
    /// Identify the row by getting x, y offset from listbox origin.
    fn handle_mouse_down(&mut self, pt: &Vector, state: &mut AppState) -> bool {
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
    fn handle_mouse_scroll(&mut self, pt: &Vector, _state: &mut AppState) {
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

/// Defines a row within a ListBox. In general, most interaction is handled by the ListBox parent
pub struct ListBoxRow {
    /// Optional id value
    // pub row_id: usize, // this is optional
    /// The base layer
    pub layer: Layer,
    /// The string value for the row
    pub string: String,
}

impl ListBoxRow {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);

        ListBoxRow {
            // row_id: 0,
            layer,
            string: String::default(),
        }
    }

    /// Render live text
    pub fn render_with_text(&mut self, rect: Rectangle, string: &str, theme: &mut Theme, window: &mut Window) {
        let style = FontStyle::new(theme.font_size, Color::BLACK);
        let mut text = Text::new(rect, string).margin(8.0, 5.0);

        text.layer.font_style = style;
        let _ = text.render(theme, window);
    }
}

// *****************************************************************************************************
// ListBoxRow :: Displayable
// *****************************************************************************************************

impl Displayable for ListBoxRow {

    fn get_id(&self) -> u32 { self.layer.get_id() }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ListBoxRow>()
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

    fn set_theme(&mut self, theme: &mut Theme) {
        if self.layer.lock_style { return }
        self.layer.apply_theme(theme);
    }

    // fn load_defaults(&mut self) {
    //     self.layer.defaults = Tween::load_props(&self.layer);
    // }

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
    }

    fn update(&mut self, _window: &mut Window, _state: &mut AppState) {
        self.layer.tween_update();
    }
}
