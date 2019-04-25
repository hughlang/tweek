// Draw some multi-colored geometry to the screen
// This is a good place to get a feel for the basic structure of a Quicksilver app
extern crate quicksilver;
extern crate tweek;
use tweek::prelude::*;

mod demo_helper;
use demo_helper::*;

use quicksilver::{
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{create_immi_ctx, Background::Col, Color, Font, ImmiRender, ImmiStatus},
    lifecycle::{run, Settings, State, Window},
    Result,
};

use immi::{
    widgets::{image_button, label, Interaction},
    Alignment,
};

// A unit struct that we're going to use to run the Quicksilver functions
// If we wanted to store persistent state, we would put it in here.
struct DrawTest {
    ui_state: immi::UiState,
    theme: Theme,
}

impl State for DrawTest {
    // Initialize the struct
    fn new() -> Result<DrawTest> {
        let theme = StageHelper::load_theme();
        let test = DrawTest { ui_state: Default::default(), theme: theme };
        Ok(test)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Remove any lingering artifacts from the previous frame
        window.clear(Color::WHITE)?;
        // Draw a rectangle with a top-left corner at (100, 100) and a width and height of 32 with
        // a blue background
        // Draw a triangle with a red background, rotated by 45 degrees, and scaled down to half
        // its size
        window.draw_ex(
            &Triangle::new((500, 50), (450, 100), (650, 150)),
            Col(Color::RED),
            Transform::rotate(45) * Transform::scale((0.5, 0.5)),
            0,
        );
        let ui_status = ImmiStatus::new(window);
        let mut ui_render = ImmiRender::new_with_window(window, &self.theme.font);
        let ui_context = create_immi_ctx(ui_status, &mut ui_render)
            // Only take up half the screen with the immi widgets
            // .rescale(0.5, 0.5, &Alignment::center())
            ;

        // We completed with no errors
        Ok(())
    }
}

// The main isn't that important in Quicksilver: it just serves as an entrypoint into the event
// loop
fn main() {
    // Run with DrawTest as the event handler, with a window title of 'Draw Geometry' and a
    // size of (800, 600)
    run::<DrawTest>("Draw Geometry", Vector::new(800, 600), Settings::default());
}
