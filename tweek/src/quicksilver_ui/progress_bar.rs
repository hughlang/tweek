/// ProgressBar
use crate::core::*;

use quicksilver::{
    geom::Rectangle,
    graphics::{Background::Col, Color, Image},
    lifecycle::Window,
};
use std::any::TypeId;

use super::*;

//-- ProgressBar -----------------------------------------------------------------------

pub struct ProgressBarView {
    pub bg_layer: TweenLayer,
    pub fg_layer: TweenLayer,
    pub bg_image: Option<Image>,
    pub progress: f32, // between 0.0 and 1.0
}

impl ProgressBarView {
    pub fn new(frame: Rectangle) -> Self {
        let layer1 = TweenLayer::new(frame);
        let layer2 = TweenLayer::new(frame);

        ProgressBarView { bg_layer: layer1, fg_layer: layer2, bg_image: None, progress: 0.0 }
    }

    /// This should be called in the update() part of the run loop with the latest
    /// time-elapsed percentage
    pub fn set_progress(&mut self, value: f32) {
        // Must be between 0.0 and 1.0
        self.progress = value;
        self.fg_layer.frame.size.x = self.bg_layer.frame.width() * self.progress;
    }

    pub fn set_track_color(&mut self, color: Color) {
        self.bg_layer.color = color;
    }

    pub fn set_progress_color(&mut self, color: Color) {
        self.fg_layer.color = color;
    }
}

impl TKDisplayable for ProgressBarView {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ProgressBarView>()
    }

    fn get_frame(&self) -> Rectangle {
        return self.bg_layer.frame;
    }

    fn set_theme(&mut self, _theme: &Theme) {
        // if let Some(label) = &mut self.label {
        //     label.layer.color = theme.fg_color;
        // }
    }

    fn update(&mut self) -> TKResult {
        Ok(())
    }

    fn render(&mut self, _theme: &Theme, window: &mut Window) -> TKResult {
        window.draw(&self.bg_layer.frame, Col(self.bg_layer.color));
        window.draw(&self.fg_layer.frame, Col(self.fg_layer.color));
        Ok(())
    }
}
