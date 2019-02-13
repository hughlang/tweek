/// This file will contain various helpers that will make it easier to use Tweek
/// in conjunction with ggez. Some ideas:
/// * A progress/timeline widget that can display timeline status information
/// * Buttons for play/pause/restart
///
///
extern crate ggez;

struct GGezHelper {

}

struct GGButton {
    rect: ggez::graphics::Rect,
    label: ggez::graphics::Text,

}