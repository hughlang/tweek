// Draw some multi-colored geometry to the screen
// This is a good place to get a feel for the basic structure of a Quicksilver app
use tweek::prelude::*;

mod demo_helper;
use demo_helper::*;

#[allow(unused_imports)]
use quicksilver::{
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color, Font},
    lifecycle::{run, Settings, State, Window},
    Result,
};


// A unit struct that we're going to use to run the Quicksilver functions
// If we wanted to store persistent state, we would put it in here.
#[allow(dead_code)]
struct DrawTest {
    theme: Theme,
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
impl State for DrawTest {
    // Initialize the struct
    fn new() -> Result<DrawTest> {
        let theme = StageHelper::load_theme();
        let test = DrawTest { theme: theme };
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
