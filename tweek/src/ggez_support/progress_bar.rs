/// ProgressBar
extern crate ggez;

use ggez::graphics::{self, Color, DrawParam, Rect};
// use ggez::mint::{Point2, Vector2};
use ggez::{Context, GameResult};

use std::any::TypeId;
use std::f32;

use super::*;

//-- ProgressBar -----------------------------------------------------------------------

pub struct ProgressBarView {
    pub bg_layer: TweenLayer,
    pub fg_layer: TweenLayer,
    pub bg_image: Option<graphics::Mesh>,
    pub progress: f32, // between 0.0 and 1.0
}

impl ProgressBarView {
    pub fn new(frame: Rect) -> Self {
        let layer1 = TweenLayer::new(frame, DrawParam::new().color(graphics::BLACK));
        let layer2 = TweenLayer::new(frame, DrawParam::new().color(graphics::WHITE));

        ProgressBarView { bg_layer: layer1, fg_layer: layer2, bg_image: None, progress: 0.0 }
    }

    /// This should be called in the update() part of the run loop with the latest
    /// time-elapsed percentage
    pub fn set_progress(&mut self, value: f32) {
        // Must be between 0.0 and 1.0
        self.progress = value;
        self.fg_layer.frame.w = self.bg_layer.frame.w * self.progress;
    }

    pub fn set_track_color(&mut self, color: Color) {
        self.bg_layer.graphics.color = color;
    }

    pub fn set_progress_color(&mut self, color: Color) {
        self.fg_layer.graphics.color = color;
    }
}

impl TKDisplayable for ProgressBarView {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ProgressBarView>()
    }

    fn get_frame(&self) -> Rect {
        return self.bg_layer.frame;
    }

    fn set_theme(&mut self, _theme: &Theme) {
        // if let Some(label) = &mut self.label {
        //     label.layer.graphics.color = theme.fg_color;
        // }
    }

    fn update(&mut self) -> GameResult {
        Ok(())
    }
    fn render(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(bg) = &self.bg_image {
            graphics::draw(ctx, bg, self.bg_layer.graphics)?;
        } else {
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                self.bg_layer.frame,
                self.bg_layer.graphics.color,
            )?;
            graphics::draw(ctx, &mesh, self.bg_layer.graphics)?;
            self.bg_image = Some(mesh);
        }
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.fg_layer.frame,
            self.fg_layer.graphics.color,
        )?;
        graphics::draw(ctx, &mesh, self.fg_layer.graphics)?;
        Ok(())
    }
}
