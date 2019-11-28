use super::*;
/// The ShapeView is simple shape holder with some Tweenable support
/// Warning: A ShapeView object will render above objects in Quicksilver's window.mesh because of the
/// ordering of MeshTasks.
use crate::core::*;
use crate::events::*;
use crate::tools::DrawShape;

use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Color, Mesh, MeshTask},
    lifecycle::Window,
};

use std::any::TypeId;

/// Enum type to define how a shape is drawn.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShapeDef {
    /// Line connecting two points
    Line,
    /// Circle
    Circle,
    /// Ellipse
    Ellipse,
    /// Rectangle or square
    Rectangle,
    /// Triangle
    Triangle,
    /// Quad
    Quad,
}

/// Enum to define how themes are applied to the shape
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShapeTheme {
    /// Normal
    Normal,
    /// Full screen overlay, locked in position
    Overlay,
}

//-- ShapeView -----------------------------------------------------------------------

/// A struct that represents a shape object created using ShapeHelper. Ultimately, it creates a MeshTask/Mesh that
/// has vertices and indices that the GPU uses to draw.
pub struct ShapeView {
    /// The base layer
    pub layer: Layer,
    /// Enum to define the shape
    pub shape_def: ShapeDef,
    /// The MeshTask which contains instructions for the GPU
    pub mesh_task: MeshTask,
    /// The offset is used when the parent Scene is moved and thus needs to inform child objects where to render
    pub offset: Vector,
    /// Define how theme is applied, etc
    pub shape_theme: ShapeTheme,
}

impl ShapeView {
    /// Constructor
    pub fn new(frame: Rectangle, shape_def: ShapeDef) -> Self {
        let layer = Layer::new(frame);
        ShapeView {
            layer,
            shape_def,
            mesh_task: MeshTask::new(0),
            offset: Vector::ZERO,
            shape_theme: ShapeTheme::Normal,
        }
    }

    /// Builder method to set the BackgroundStyle
    pub fn with_background(mut self, bg_style: BackgroundStyle) -> Self {
        self.layer.bg_style = bg_style;
        self
    }

    /// Builder method to set the BorderStyle
    pub fn with_border(mut self, style: BorderStyle) -> Self {
        self.layer.border_style = style;
        self
    }

    /// Method to update the MeshTask
    /// FIXME: This is a dumb idea
    pub fn build(&mut self) {
        let mut mesh = self.build_mesh();
        let mut task = MeshTask::new(0);
        task.vertices.append(&mut mesh.vertices);
        task.triangles.append(&mut mesh.triangles);
        self.mesh_task = task;
    }

    /// Builder method to copy vertices and triangles from Mesh
    pub fn with_mesh(mut self, mesh: &mut Mesh) -> Self {
        let mut task = MeshTask::new(0);
        task.vertices.append(&mut mesh.vertices);
        task.triangles.append(&mut mesh.triangles);
        self.mesh_task = task;
        // self.init_mesh = self.mesh_task.clone();
        self
    }

    fn build_mesh(&mut self) -> Mesh {
        let mesh = {
            let border: (Option<Color>, f32) = {
                match self.layer.border_style {
                    BorderStyle::None => (None, 0.0),
                    BorderStyle::SolidLine(color, width) => (Some(color), width),
                }
            };
            let color = Some(self.layer.bg_style.get_color());
            match self.shape_def {
                ShapeDef::Rectangle => DrawShape::rectangle(&self.layer.frame, color, border.0, border.1, 0.0),
                ShapeDef::Circle => DrawShape::circle(
                    &self.layer.frame.center(),
                    &self.layer.frame.width() / 2.0,
                    color,
                    border.0,
                    border.1,
                ),
                _ => Mesh::new(),
            }
        };
        mesh
    }

    /// Generate the Mesh from the animating props
    fn tween_mesh(&mut self) -> Mesh {
        let mesh = {
            let border: (Option<Color>, f32) = {
                match self.layer.border_style {
                    BorderStyle::None => (None, 0.0),
                    BorderStyle::SolidLine(color, width) => (Some(color), width),
                }
            };
            let color = Some(self.layer.transition.color);
            match self.shape_def {
                ShapeDef::Rectangle => {
                    DrawShape::rectangle(&self.layer.frame, color, border.0, border.1, self.layer.corner_radius)
                }
                ShapeDef::Circle => DrawShape::circle(
                    &self.layer.frame.center(),
                    &self.layer.frame.width() / 2.0,
                    color,
                    border.0,
                    border.1,
                ),
                _ => Mesh::new(),
            }
        };
        mesh
    }
}

impl Displayable for ShapeView {
    fn get_id(&self) -> u32 {
        self.layer.get_id()
    }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
        self.layer.type_id = self.get_type_id();
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ShapeView>()
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
        // TODO: decide if shapes should get themed
    }

    fn notify(&mut self, event: &DisplayEvent) {
        match event {
            DisplayEvent::Ready => {
                self.layer.on_ready();
            }
            DisplayEvent::Moved => {
                self.layer.on_move_complete();
                for (_, vertex) in self.mesh_task.vertices.iter_mut().enumerate() {
                    vertex.pos = Transform::translate(self.offset) * vertex.pos;
                }
            }
            _ => {}
        }
    }

    fn update(&mut self, _window: &mut Window, state: &mut AppState) {
        let mut notifier = Notifier::new();
        self.layer.notifications.borrow_mut().attach(&mut notifier);

        self.layer.tween_update(state);

        // let alerts = self.layer.notifications.borrow_mut().queue().clone();
        // for alert in alerts {
        //     match alert {
        //         Alert::Redraw => {
        //             self.build();
        //         }
        //     }
        // }
        self.layer.notifications.borrow_mut().clear();
    }

    fn render(&mut self, _theme: &mut Theme, window: &mut Window) {
        if self.layer.is_animating() {
            let mut mesh = self.tween_mesh();
            let mut task = MeshTask::new(0);
            task.vertices.append(&mut mesh.vertices);
            task.triangles.append(&mut mesh.triangles);
            // for (_, vertex) in task.vertices.iter_mut().enumerate() {
            //     vertex.pos = Transform::rotate(self.layer.transition.rotation)
            //         * Transform::translate(self.layer.anchor_pt)
            //         * vertex.pos;
            // }
            window.add_task(task);
        } else {
            let mut task = self.mesh_task.clone();
            if self.offset != Vector::ZERO {
                for (_, vertex) in task.vertices.iter_mut().enumerate() {
                    vertex.pos = Transform::translate(self.offset) * vertex.pos;
                }
            }
            window.add_task(task);
        }
    }

    fn set_hover_animation(&mut self, props: PropSet) {
        self.layer.hover_effect = Some(props);
    }

    fn handle_mouse_at(&mut self, pt: &Vector, _window: &mut Window) -> bool {
        // TODO: Use better hit-testing for non-rectangular items. See quicksilver Drawable
        // TODO: Override mouse cursor behavior for non-interactive shapes
        if self.layer.hover_effect.is_some() {
            return self.layer.handle_mouse_over(pt);
        }
        false
    }
}
