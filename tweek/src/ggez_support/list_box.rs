/// The ListBox view is a simple list of strings in a rectangular space that is scrollable.
///
///
extern crate ggez;

use crate::core::*;

// use ggez::conf;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Rect};
use ggez::mint::{self, Point2};
use ggez::{Context, GameResult};

use std::any::TypeId;
use std::f32;
use std::ops::Range;

use super::*;

/// This is the multiplier that affects how quickly scrolling occurs.
const SCROLL_FACTOR: f32 = 1.0;
const SMOOTH_SCROLL: bool = true; // placeholder until it works.

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
    scroll_offset: mint::Point2<f32>,
    // use_canvas: bool,
}

impl ListBox {
    pub fn new(frame: &Rect, _ctx: &mut Context) -> Self {
        let mut layer = TweenLayer::new(frame.clone(), DrawParam::new().color(graphics::WHITE));
        layer.border_color = Some(Color::from_rgb_u32(0x333333));
        layer.border_width = 1.0;

        let row_hover = UITransition::new(vec![color(0xEEEEEE)], 0.0);
        let row_select = UITransition::new(vec![color(0xCCCCCC)], 0.3);

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
            scroll_offset: Point2 { x: 0.0, y: 0.0 },
            // use_canvas: false,
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
        // TODO: Create only the rows needed to fill the visible range. Experiment first in future TableView gui
        for (i, value) in ds.iter().enumerate() {
            let rect = Rect::new(
                self.layer.frame.x,
                self.layer.frame.y + self.row_height * i as f32,
                self.layer.frame.w,
                self.row_height,
            );
            let mut row = ListBoxRow::new(rect).with_text(value);

            row.load_defaults();
            self.rows.push(row);
        }
        self.datasource = ds;
        self
    }

    pub fn set_color(&mut self, color: &graphics::Color) {
        self.layer.graphics.color = color.clone();
    }

    pub fn get_visible_range(&self) -> Range<usize> {
        let shift = self.scroll_offset.y / self.row_height;
        let start = shift.floor() as usize;

        let row_count = {
            if SMOOTH_SCROLL {
                (self.layer.frame.h / self.row_height + shift.fract()).ceil() as usize
            } else {
                (self.layer.frame.h / self.row_height) as usize
            }
        };
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

    fn get_frame(&self) -> Rect {
        return self.layer.frame;
    }

    fn set_theme(&mut self, theme: &Theme) {
        for (_, row) in &mut self.rows.iter_mut().enumerate() {
            row.set_theme(&theme);
        }
    }

    fn get_perimeter_frame(&self) -> Option<Rect> {
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
    fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.apply_updates(&update.props);
            }
        }

        // Provide usize index values that are outside the range of the array
        // Q: If there's a better way of unwrapping these, please suggest.
        let hover_index = self.hover_row.unwrap_or(self.rows.len() + 1);
        let select_index = self.select_row.unwrap_or(self.rows.len() + 1);

        let range = self.get_visible_range();

        for (i, row) in &mut self.rows[range].iter_mut().enumerate() {
            &row.update()?;
            if i == select_index {
                match row.mouse_state {
                    MouseState::Select => {
                        if let Some(animation) = &row.layer.animation {
                            if animation.state == TweenState::Completed {
                                row.mouse_state = MouseState::None;
                                // row.layer.apply_updates(&row.defaults);
                                row.layer.animation = None;
                                self.select_row = None;
                            }
                        } else {
                            row.mouse_state = MouseState::None;
                            // row.layer.apply_updates(&row.defaults);
                            row.layer.animation = None;
                        }
                    }
                    MouseState::Hover => {
                        row.mouse_state = MouseState::Select;
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
            } else if i == hover_index {
                match row.mouse_state {
                    MouseState::None => {
                        row.mouse_state = MouseState::Hover;
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
                match row.mouse_state {
                    MouseState::None => {}
                    MouseState::Hover => {
                        // if i != hover_index {
                        row.mouse_state = MouseState::None;
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

    fn render(&mut self, ctx: &mut Context) -> GameResult {
        let frame = self.layer.frame;
        let mut builder = graphics::MeshBuilder::new();
        builder.rectangle(DrawMode::fill(), frame, self.layer.graphics.color);

        // Iterate through the rows that are in the visible range
        let range = self.get_visible_range();
        let shift_y = self.scroll_offset.y % self.row_height;

        for (i, row) in &mut self.rows[range].iter_mut().enumerate() {
            let ypos = {
                if SMOOTH_SCROLL {
                    frame.y - shift_y + self.row_height * i as f32
                } else {
                    frame.y + self.row_height * i as f32
                }
            };
            let rect = Rect::new(frame.x, ypos, frame.w, self.row_height);
            builder.rectangle(DrawMode::fill(), rect, row.layer.graphics.color);
            if let Some(label) = &mut row.label {
                label.render_inside(&rect, ctx)?;
            }
        }

        // Draw the border last since the rows will overwrite some of it.
        if let Some(color) = self.layer.border_color {
            builder.polygon(
                DrawMode::stroke(self.layer.border_width),
                &[
                    Point2 { x: frame.x, y: frame.y },
                    Point2 { x: frame.right(), y: frame.top() },
                    Point2 { x: frame.right(), y: frame.bottom() },
                    Point2 { x: frame.left(), y: frame.bottom() },
                    Point2 { x: frame.x, y: frame.y },
                ],
                color,
            )?;
        }

        let mesh = builder.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;

        // Draw queued text, which are queued inside the LabelView
        graphics::draw_queued_text(ctx, graphics::DrawParam::default())?;

        Ok(())
    }

    fn set_hover_animation(&mut self, props: &[Prop], seconds: f64) {
        self.defaults = Tween::load_props(&self.layer);
        let transition = UITransition::new(props.to_vec(), seconds);
        self.on_hover = Some(transition);
    }
}

// *****************************************************************************************************
// ListBox :: TKResponder
// *****************************************************************************************************

impl TKResponder for ListBox {
    fn has_focus(&self) -> bool {
        false
    }

    fn handle_mouse_at(&mut self, x: f32, y: f32) -> bool {
        let range = self.get_visible_range();

        if self.layer.frame.contains(mint::Point2 { x, y }) {
            let local_y = y - self.layer.frame.y;
            let mut index = (local_y / self.row_height).floor() as usize;
            index += range.start;
            if index < self.rows.len() {
                self.hover_row = Some(index);
                return true;
            }
        }
        false
    }

    fn handle_mouse_down(&mut self, _x: f32, _y: f32, _state: &mut TKState) -> bool {
        false
    }

    /// Calculate whether x, y are in the rect bounds of any child row with maths.
    /// Identify the row by getting x, y offset from listbox origin.
    fn handle_mouse_up(&mut self, x: f32, y: f32, state: &mut TKState) -> bool {
        let range = self.get_visible_range();

        if self.layer.frame.contains(mint::Point2 { x, y }) {
            let local_y = y - self.layer.frame.y;
            // example: row_height=20. If local_y=50, then it is row_index=2
            let mut index = (local_y / self.row_height).floor() as usize;
            index += range.start;
            if index < self.rows.len() {
                log::debug!("local_y={:?} clicked row={:?}", local_y, index);
                state.row_target = Some(index);
                self.select_row = Some(index);
                return true;
            }
        }
        false
    }

    /// Add or subtract from the layer content offset
    fn handle_mouse_scroll(&mut self, _x: f32, y: f32, _state: &mut TKState) {
        let upper_limit = self.datasource.len() as f32 * self.row_height - self.layer.frame.h;
        let eval_y = ((self.scroll_offset.y + y) * SCROLL_FACTOR).max(0.0).min(upper_limit);
        // log::debug!("-------------- self.scroll_offset.y={:?} y={:?}", self.scroll_offset.y, y);
        self.scroll_offset.y = eval_y;
    }
}

// *****************************************************************************************************
// ListBoxRow
// *****************************************************************************************************

pub struct ListBoxRow {
    pub row_id: usize, // this is optional
    pub layer: TweenLayer,
    pub label: Option<LabelView>,
    pub defaults: Vec<Prop>,
    // hover_animation: Option<UITransition>,
    mouse_state: MouseState,
    // onclick: Option<Box<FnMut(TKAction, &mut TKState) + 'static>>,
}

impl ListBoxRow {
    pub fn new(frame: Rect) -> Self {
        let layer = TweenLayer::new(frame.clone(), DrawParam::new().color(graphics::WHITE));

        ListBoxRow {
            row_id: 0,
            layer: layer,
            label: None,
            defaults: Vec::new(),
            // hover_animation: None,
            mouse_state: MouseState::None,
            // onclick: None,
        }
    }

    pub fn row(mut self, id: usize) -> Self {
        self.row_id = id;
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        let frame = self.layer.inset_by(8.0, 3.0, 8.0, 3.0); // TODO: make configurable
        let label = LabelView::new(&frame, text);
        self.label = Some(label);
        self
    }

    // pub fn default_animation(mut self) -> Self {
    //     self.set_hover_animation(vec![color(HexColors::AliceBlue)], 0.0);
    //     self
    // }
}

// *****************************************************************************************************
// ListBoxRow :: Displayable
// *****************************************************************************************************

impl TKDisplayable for ListBoxRow {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ListBoxRow>()
    }

    fn get_frame(&self) -> Rect {
        return self.layer.frame;
    }

    fn set_theme(&mut self, theme: &Theme) {
        if let Some(label) = &mut self.label {
            label.layer.graphics.color = theme.fg_color;
            label.layer.font = theme.font;
        }
    }

    fn load_defaults(&mut self) {
        self.defaults = Tween::load_props(&self.layer);
    }

    fn update(&mut self) -> GameResult {
        if let Some(tween) = &mut self.layer.animation {
            tween.tick();
            if let Some(update) = tween.update() {
                self.layer.apply_updates(&update.props);
            }
        }
        Ok(())
    }

    // fn render(&mut self, ctx: &mut Context) -> GameResult {
    //     let mesh = graphics::Mesh::new_rectangle(
    //         ctx,
    //         graphics::DrawMode::fill(),
    //         self.layer.frame,
    //         self.layer.graphics.color,
    //     )?;

    //     let _result = graphics::draw(ctx, &mesh, self.layer.graphics);

    //     if let Some(label) = &mut self.label {
    //         log::debug!("listboxrow frame={:?} text={:?}", self.layer.frame, label.string);
    //         label.render_inside(&self.layer.frame, ctx)?;
    //     }

    //     Ok(())
    // }

    fn render_inside(&mut self, rect: &Rect, ctx: &mut Context) -> GameResult {
        let mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect.clone(), self.layer.graphics.color)?;

        graphics::draw(ctx, &mesh, self.layer.graphics)?;

        if let Some(label) = &mut self.label {
            label.render_inside(&rect, ctx)?;
        }

        Ok(())
    }
}
