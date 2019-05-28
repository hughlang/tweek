/// The ShapeView is simple shape holder with some Tweenable support
///
use crate::core::*;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, DrawTask, Mesh},
    lifecycle::Window,
};

use std::any::TypeId;

use super::*;

pub enum ShapeDef {
    /// The generic Circle uses a Rectange to define its frame, so it can be used for both Circle and Ellipse
    /// shapes.
    Circle(Rectangle, Option<Color>, Option<Color>, f32),
    /// The generic Rectangle also has a corner-radius option as the last parameter, so it is used for anything
    /// shaped like a rectangle. The two optional colors and f32 params represent: fill color, line color, and line width.
    Rectangle(Rectangle, Option<Color>, Option<Color>, f32, f32),
}

//-- ShapeView -----------------------------------------------------------------------

pub struct ShapeView {
    pub layer: TweenLayer,
    pub shape_def: ShapeDef,
    pub draw_task: DrawTask,
}

impl ShapeView {
    pub fn new(frame: Rectangle) -> Self {
        let def = ShapeDef::Rectangle(frame.clone(), None, None, 0.0, 0.0);
        let layer = TweenLayer::new(frame);
        ShapeView { layer: layer, shape_def: def, draw_task: DrawTask::new(0) }
    }

    pub fn with_mesh(mut self, mesh: &mut Mesh) -> Self {
        let mut task = DrawTask::new(0);
        task.vertices.append(&mut mesh.vertices);
        task.triangles.append(&mut mesh.triangles);
        self.draw_task = task;
        self
    }
}

impl TKDisplayable for ShapeView {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ShapeView>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.layer.frame;
    }

    fn set_theme(&mut self, _theme: &Theme) {}

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Ready => {
                self.layer.defaults = Tween::load_props(&self.layer);
            }
            _ => {}
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

    fn render(&mut self, _theme: &mut Theme, window: &mut Window) -> TKResult {
        window.add_task(self.draw_task.clone());
        // if let Some(transition) = &self.layer.on_hover {
        // } else {
        //     window.add_task(self.draw_task.clone());
        // }
        Ok(())
    }

    fn set_hover_animation(&mut self, props: &[Prop], seconds: f64) {
        let transition = UITransition::new(props.to_vec(), seconds);
        self.layer.on_hover = Some(transition);
    }

    fn handle_mouse_at(&mut self, pt: &Vector) -> bool {
        // TODO: Use better hit-testing for non-rectangular items. See quicksilver Drawable
        // TODO: Override mouse cursor behavior for non-interactive shapes
        return self.layer.handle_mouse_over(pt);
    }
}
