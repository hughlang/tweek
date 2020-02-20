/// ListBox
///
use crate::core::*;
use crate::events::*;
use crate::tools::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Col, Background::Img, Color, Image, MeshTask},
    input::MouseCursor,
    lifecycle::Window,
};

#[allow(unused_imports)]
use image_rs::{imageops, DynamicImage, Pixel, RgbaImage};

use std::any::TypeId;
use std::f32;
use std::ops::Range;

use super::*;

/// This is the multiplier that affects how quickly scrolling occurs.
/// Not needed if 1.0 is the norm
const SCROLL_FACTOR: f32 = 1.0;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ListBoxState {
    /// User is idle
    Idle,
    /// Scrolling underway and selecting not allowed.
    Scrolling,
    /// At least one row has been selected
    Selected,
    /// The listbox is not the currently focused control
    Unfocused,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RowState {
    /// Default state
    Normal,
    /// Row shows hover state
    Hover,
    /// Row is selected/highlighted
    Selected,
}

/// A wrapper for holding data and rendering information for each row
pub struct RowData {
    /// The string value
    pub text: String,
    /// Current mouse state
    pub row_state: MouseState,
    /// Rendered font text
    pub render: Option<MeshTask>,
    /// If present, the RowData needs a Layer for managing animation
    pub layer: Option<Layer>,
}

impl RowData {
    pub fn new(text: String) -> Self {
        RowData { text, row_state: MouseState::None, render: None, layer: None }
    }
}

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
    /// The line separator between each row, including first and last
    pub row_border_style: BorderStyle,
    /// Is it single select or multiple?
    pub multiselect: bool,
    /// The datasource as an array of strings
    datasource: Vec<RowData>,
    /// The array of visible ListBoxRow objects
    rows: Vec<ListBoxRow>,
    /// Hover animation for the ListBoxRow. Unused
    pub on_row_hover: Option<PropSet>,
    /// Animation for row select
    pub on_row_select: Option<PropSet>,
    /// Stores the scroll offset
    scroll_offset: f32,
}

impl ListBox {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);

        let row_hover = PropSet::new(vec![color("#EEEEEE")], 0.0).for_type(TweenType::Hover);
        let row_select = PropSet::new(vec![color("#0096FF"), tint("#FFFFFF")], 0.2).for_type(TweenType::Click);

        ListBox {
            layer,
            hover_row: None,
            select_row: None,
            row_height: 20.0,
            row_border_style: BorderStyle::None,
            multiselect: false,
            datasource: Vec::new(),
            rows: Vec::new(),
            on_row_hover: Some(row_hover),
            on_row_select: Some(row_select),
            scroll_offset: 0.0,
        }
    }

    /// Load datasource with RowData objects
    pub fn set_datasource(&mut self, values: Vec<String>) {
        for string in values {
            let row_data = RowData::new(string);
            self.datasource.push(row_data);
        }
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
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ListBox>()
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

    /// This function is responsible for building all of the ListBoxRows and pre-rendering text as
    /// meshes. This is necessary to support theme changes that can affect the row height and font color.
    fn set_theme(&mut self, theme: &mut Theme) {
        // Don't allow style to be locked
        let _ = self.layer.apply_theme(theme);

        // Create only the rows needed to fill the visible range. Each row is a template
        // that is populated during the render phase

        self.rows.clear();
        // FIXME: row height should be themed.
        let row_count = (self.layer.frame.size.y / self.row_height) as usize + 1;
        for i in 0..row_count {
            let rect = Rectangle::new(
                (self.layer.frame.pos.x, self.layer.frame.pos.y + self.row_height * i as f32),
                (self.layer.frame.size.x, self.row_height),
            );
            let mut row = ListBoxRow::new(rect);
            row.set_id(i as u32);
            row.set_theme(theme);
            self.rows.push(row);
        }

        // Render all string values in datasource as meshes to make performance better
        let frame = Rectangle::new_sized((self.layer.frame.size.x, self.row_height));

        for data in &mut self.datasource {
            let mut params = TextParams::new(self.layer.font_style)
                .frame(frame)
                .align(TextAlign::Left, VertAlign::Middle)
                .multiline(false);
            params.text = data.text.clone();
            if let Some(render) = theme.default_font.draw(params) {
                data.render = Some(render);
            } else {
                log::debug!(">>> mesh_task is None!");
            }
        }
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
    }

    /// The ListBox should manage the UI state of its rows.
    /// The mouse state of a row should determine whether to reset or not.
    /// Otherwise, leave it alone?
    fn update(&mut self, _window: &mut Window, state: &mut AppState) {
        self.layer.tween_update(state);

        self.datasource.iter_mut().for_each(|x| {
            if let Some(ref mut layer) = x.layer {
                layer.tween_update(state);
            }
        });
    }

    fn render(&mut self, _theme: &mut Theme, window: &mut Window) {
        if self.rows.len() == 0 {
            log::warn!("No rows found. Rows are created when set_theme is called.");
            return;
        }
        self.layer.draw_background(window);
        let frame = self.layer.frame;
        // log::debug!("frame={:?}", frame);
        // Create a mesh to hold row borders
        let mut graphics = MeshTask::new(0);
        let border: (Color, f32) = {
            match self.row_border_style {
                BorderStyle::None => (Color::WHITE, 0.0),
                BorderStyle::SolidLine(color, width) => (color, width),
            }
        };

        // Iterate through the rows that are in the visible range
        let range = self.get_visible_range();
        let shift_y = self.scroll_offset % self.row_height;

        let range_start = range.start;
        let last = self.rows.len() - 1;
        for (i, row) in &mut self.rows.iter_mut().enumerate() {
            // Calculate where row should render, using shift_y as the scroll offset
            let ypos = frame.pos.y - shift_y + self.row_height * i as f32;
            let rect = Rectangle::new((frame.pos.x, ypos), (frame.size.x, self.row_height));
            // Draw borders as row separators. Future: cache these lines
            if ypos > frame.y() {
                if border.1 > 0.0 {
                    let pts: [&Vector; 2] = [&Vector::new(rect.x(), ypos), &Vector::new(rect.x() + rect.width(), ypos)];
                    let mut line = DrawShape::line(&pts, border.0, border.1);
                    graphics.append(&mut line);
                }
            }

            let row_index = range_start + i;
            let mut bounds = rect.clone();
            if row_index < self.datasource.len() {
                let row_data = &self.datasource[row_index];
                if let Some(render) = &row_data.render {
                    let mut mesh = render.clone();
                    // All text meshes were created at origin 0, 0 and thus need to be translated to the actual
                    // row position.
                    for vertex in &mut mesh.vertices.iter_mut() {
                        vertex.pos = Transform::translate(rect.pos) * vertex.pos;
                    }
                    // If first visible row, check if the row rect overflows the top of the listbox.
                    // If last visible row, check if the row rect overflows the bottom of the listbox.
                    // Clip the mesh top or bottom as needed.
                    if i == 0 {
                        let y_overflow = frame.y() - rect.y();
                        if y_overflow > 0.0 {
                            bounds = Rectangle::new(frame.pos, (frame.width(), rect.height() - y_overflow));
                            UITools::clip_mesh(&mut mesh, &bounds, RectSide::Top);
                        }
                    } else if i == last {
                        let y_overflow = (rect.y() + rect.height()) - (frame.y() + frame.height());
                        if y_overflow > 0.0 {
                            bounds = Rectangle::new((frame.x(), rect.y()), (frame.width(), rect.height() - y_overflow));
                            UITools::clip_mesh(&mut mesh, &bounds, RectSide::Bottom);
                        }
                    }
                    let transition = {
                        if let Some(layer) = &row_data.layer {
                            Some(layer.transition.clone())
                        } else {
                            None
                        }
                    };
                    row.render_row(mesh, bounds, transition, window);
                }
            }
        }

        let content_height = self.datasource.len() as f32 * self.row_height;
        if let Some(rect) = UITools::get_scrollbar_frame(content_height, &frame, self.scroll_offset) {
            // FIXME: use mesh
            window.draw(&rect, Col(Color::from_hex(UITools::SCROLLBAR_COLOR)));
        }

        window.add_task(graphics);
        self.layer.draw_border(window);
    }

    fn set_hover_animation(&mut self, props: PropSet) {
        self.layer.hover_effect = Some(props);
    }

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        let range = self.get_visible_range();
        if pt.overlaps_rectangle(&self.layer.frame) {
            self.layer.mouse_state = MouseState::Hover;
            window.set_cursor(MouseCursor::Hand);
            let local_y = pt.y - self.layer.frame.pos.y;
            let mut index = (local_y / self.row_height).floor() as usize;
            index += range.start;
            if index < self.datasource.len() {
                self.hover_row = Some(index);
                // TODO: Finish implementing hover effect
                // log::debug!("hover index={:?} y={:?}", index, 0);
                // let data = &mut self.datasource[index];
                // data.row_state = MouseState::Hover;

                // // show hover animation
                // if let Some(transition) = &self.on_row_hover {
                //     let layer = self.rows[0].get_layer().clone();
                //     if let Some(layer) = &mut data.layer {
                //         if transition.duration > 0.0 {
                //             layer.animate_with_props(transition.clone());
                //         } else {
                //             layer.apply_props(&transition.props.clone());
                //         }
                //     }
                // }

                return true;
            }
        } else {
            window.set_cursor(MouseCursor::Default);
            self.layer.mouse_state = MouseState::None;
            self.hover_row = None;
        }
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
            let mut index = (local_y / self.row_height).floor() as usize;
            index += range.start;
            log::debug!("local_y={:?} clicked row={:?}", local_y, index);
            if index < self.datasource.len() {
                state.row_target = Some(index);
                self.select_row = Some(index);
                for (_, data) in &mut self.datasource.iter_mut().enumerate() {
                    if !self.multiselect && data.layer.is_some() {
                        data.layer = None;
                        data.row_state = MouseState::None;
                    }
                }

                if self.rows.len() > 0 {
                    let mut layer = self.rows[0].layer.clone();
                    let data = &mut self.datasource[index];
                    if data.row_state != MouseState::Select {
                        data.row_state = MouseState::Select;
                        if let Some(transition) = &self.on_row_select {
                            if transition.duration > 0.0 {
                                layer.animate_with_props(transition.clone(), true);
                            } else {
                                layer.apply_props(&transition.props.clone());
                            }
                        }
                        data.layer = Some(layer);
                    }
                }

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
    /// The base layer
    pub layer: Layer,
}

impl ListBoxRow {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);

        ListBoxRow { layer }
    }

    /// Method to render a row
    pub fn render_row(
        &mut self,
        mut mesh: MeshTask,
        bounds: Rectangle,
        transition: Option<Transition>,
        window: &mut Window,
    ) {
        if let Some(transition) = transition {
            // Draw a background rectangle with the transition.color
            let mut bg_mesh = DrawShape::rectangle(&bounds, Some(transition.color), None, 0.0, 0.0);
            let mut mesh_task = MeshTask::new(0);
            mesh_task.append(&mut bg_mesh);
            window.add_task(mesh_task);
            // Change the font color
            mesh.vertices.iter_mut().for_each(|x| x.col = transition.tint);
        }
        window.add_task(mesh);
    }
}

// *****************************************************************************************************
// ListBoxRow :: Displayable
// *****************************************************************************************************

impl Displayable for ListBoxRow {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ListBoxRow>()
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

    fn set_theme(&mut self, theme: &mut Theme) {
        let ok = self.layer.apply_theme(theme);
        if !ok {
            return;
        }
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
    }

    fn update(&mut self, _window: &mut Window, state: &mut AppState) {
        self.layer.tween_update(state);
    }
}
