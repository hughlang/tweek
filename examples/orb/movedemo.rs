extern crate orbrender;
extern crate tween;

use tween::*;

pub fn main() -> Result<(), String> {

    let tween = Tween::default();

    tween.to(vec![move_x(10.0), move_y(10.0)])
        .duration(5.0).play();

    Ok(())
}