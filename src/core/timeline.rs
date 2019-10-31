use super::tweek::*;
use super::{current_time, elapsed_time};
use crate::events::*;
/// A Timeline represents a group of Tween animations that each have a start and stop time in seconds
/// in the overall timeline.
use crate::gui::{Displayable, Layer, Theme};

use quicksilver::{
    geom::{Rectangle, Vector},
    lifecycle::Window,
};

use std::any::TypeId;

//-- Base -----------------------------------------------------------------------

/// A Sprite is basically a Displayable that belongs to a Timeline and it's Layer object
/// has a Tween animation ready to play. A Sprite also has a defined scheduled start and end time
/// that will determine when the animation starts playing. Since a Timeline likely has multiple
/// Sprites, the Timeline will choreograph when each Sprite starts animating.
pub struct Sprite {
    pub(self) view: Box<dyn Displayable>,
    /// The start time in float seconds
    pub start: f64,
    /// The end time in float seconds
    pub end: f64,
    /// The running state for this Tween
    pub state: PlayState,
}

impl Sprite {
    /// Constructor
    fn new(view: Box<dyn Displayable>, start: f64, end: f64) -> Self {
        Sprite { view, start, end, state: PlayState::Pending }
    }
}

//-- Main -----------------------------------------------------------------------

/// A Timeline contains a collection of Sprites that are basically views that will animate
/// according to their scheduled start and end times. The parent object of a Timeline is a Scene,
/// and is responsible for passing Displayable calls to the Timeline. Hence, a Timeline is kind of a
/// sub-Scene where the child views are animated.
pub struct Timeline {
    layer: Layer,
    sprites: Vec<Sprite>,
    tl_start: f64,
    /// Number of times to repeat this Timeline animation
    pub repeat_count: u32,
    /// Delay before repeating next execution
    pub repeat_delay: f32,
    /// Boolean to define whether this timeline repeats forever
    pub loop_forever: bool,
}

impl Timeline {
    /// Constructor
    pub fn new(frame: Rectangle) -> Self {
        let layer = Layer::new(frame);
        Timeline { layer, sprites: Vec::new(), tl_start: 0.0, repeat_count: 0, repeat_delay: 0.0, loop_forever: false }
    }

    pub fn add_sprite(&mut self, mut view: Box<dyn Displayable>, start: f64) {
        if view.get_layer_mut().has_animation() {
            log::error!("No Tween has been set for this view");
            return;
        }
        let end = start + view.get_tween_duration();
        // if end < EPSILON {
        //     log::error!("Animation duration is 0");
        //     return;
        // }
        if view.get_id() == 0 {
            view.set_id(self.sprites.len() as u32);
        }
        let sprite = Sprite::new(view, start, end);
        self.sprites.push(sprite);
    }

    /// Builder method to set the start time of the Tweens as either:
    /// Normal: All start at the same time
    /// Sequence: Tweens play sequentially
    pub fn align(&mut self, _alignment: SpriteAlign) {

        // let mut start = 0.0 as f64;
        // for id in &self.tween_ids {
        //     if let Some(range) = self.children.get_mut(&id) {
        //         let tween = range.tween.borrow();
        //         let total_secs = (&*tween).total_time();
        //         range.start = start;
        //         range.end = range.start + total_secs;
        //         log::debug!("align start={} end={}", range.start, range.end);

        //         match alignment {
        //             SpriteAlign::Normal => {}
        //             SpriteAlign::Sequence => {
        //                 start += total_secs;
        //             }
        //             _ => (),
        //         }
        //     }
        // }
        // self
    }

    /// Builder method to set a fixed offset delay for all Tweens in a timeline
    pub fn stagger(&mut self, offset: f64) {
        for (i, sprite) in &mut self.sprites.iter_mut().enumerate() {
            let duration = sprite.view.get_tween_duration();
            sprite.start = i as f64 * offset;
            sprite.end = sprite.start + duration;
            log::debug!("{}/ stagger start={} end={}", sprite.view.get_id(), sprite.start, sprite.end);
        }
    }

    /// Builder method to define the repeat count and delay time between plays
    // pub fn repeat(&mut self, count: u32, delay: f32) -> Self {
    //     self.repeat_count = count;
    //     self.repeat_delay = delay;
    //     self
    // }

    /// Calculate the total time for all animations in the Timeline
    pub fn total_time(&self) -> f64 {
        // let floats: Vec<f64> = self.sprites.map(|x| x.end).collect();
        // if let Some(max) = floats.iter().cloned().max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN")) {
        //     return max;
        // }
        0.0
    }
}

// ************************************************************************************
// Displayable trait
// ************************************************************************************

impl Displayable for Timeline {
    fn get_id(&self) -> u32 {
        self.layer.get_id()
    }

    fn set_id(&mut self, id: u32) {
        self.layer.set_id(id);
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Timeline>()
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
        for sprite in &mut self.sprites {
            sprite.view.set_theme(theme);
        }
    }

    fn notify(&mut self, event: &DisplayEvent) {
        for sprite in &mut self.sprites {
            sprite.view.notify(event);
        }
    }

    fn update(&mut self, window: &mut Window, state: &mut AppState) {
        let elapsed = elapsed_time(self.tl_start);
        for sprite in &mut self.sprites {
            if sprite.start <= elapsed && sprite.end > elapsed {
                // If not playing, start. Tween.play method checks play state first
                // TODO: get PlayState from layer and handle it here
                sprite.view.get_layer_mut().play();
            }
            sprite.view.update(window, state);
        }
        state.elapsed_time = elapsed_time(self.tl_start);
    }

    fn render(&mut self, theme: &mut Theme, window: &mut Window) {
        for sprite in &mut self.sprites {
            sprite.view.render(theme, window);
        }
    }

    fn handle_mouse_at(&mut self, pt: &Vector, window: &mut Window) -> bool {
        // TODO: Don't handle mouse movement if Timeline is currently playing
        for sprite in &mut self.sprites {
            let hover = sprite.view.handle_mouse_at(pt, window);
            if hover {
                return true;
            }
        }
        false
    }

    fn get_routes(&mut self) -> Vec<String> {
        let mut routes: Vec<String> = Vec::new();
        let base = self.node_key();
        routes.push(base.clone());

        for sprite in &mut self.sprites {
            for path in sprite.view.get_routes() {
                let route = format!("/{}/{}", &base, path);
                routes.push(route);
            }
        }
        routes
    }
}

// ************************************************************************************
// Playable trait
// ************************************************************************************

impl Playable for Timeline {
    /// The Timeline play method should only play the tweens where the start time
    /// is not greater than the current elapsed time.
    fn play(&mut self) {
        self.tl_start = current_time();
        for sprite in &mut self.sprites {
            let elapsed = elapsed_time(self.tl_start);
            if sprite.start <= elapsed && sprite.end > elapsed {
                log::debug!("Play animation for: {:?}", sprite.view.debug_out());
                sprite.view.get_layer_mut().play();
            }
        }
    }
}

//-- Support -----------------------------------------------------------------------

/**
 * From Greensock AS3:
 * Options are: "sequence" (aligns them one-after-the-other in a sequence), "start"
 * (aligns the start times of all of the objects (ignoring delays)), and "normal"
 * (aligns the start times of all the tweens (honoring delays)). The default is "normal".
 */
pub enum SpriteAlign {
    /// All tweens start at the same time
    Normal,
    /// Tweens play sequentially
    Sequence,
    /// Stagger start time by specified seconds
    Stagger(f32),
}
