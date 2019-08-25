/// This AppDelegate is provided as an example of how to integrate with the Quicksilver backend and translate runloop
/// calls to the Tweek UI architecture. The Tweek struct is used as the interface to communicate calls and events
/// to its own array of Scenes and down the hierarchy of child views and controls. Tweek should not have a visible
/// layer that is rendered, but instead is handled by child Scenes etc.
///
/// When creating your own application, you may choose to create your own version of this AppDelegate that conforms
/// to your application needs. This example is designed for a simple UI designed for showcasing the examples in this
/// project.
///
use crate::core::*;
use crate::gui::*;
use crate::events::*;

use std::any::TypeId;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    input::{ButtonState, Key, MouseButton, MouseCursor},
    lifecycle::{Event, State, Window},
    Error, Result,
};

// // Misc
pub const BG_SCENE_ID: u32 = 100;
const FPS_INTERVAL: usize = 40;
const FPS_TAG: u32 = 901;
const TITLE_TAG: u32 = 902;

//-- Main -----------------------------------------------------------------------

/// AppDelegate serves as a layer between the backend runloop and Tweek UI.
///
pub struct AppDelegate {
    stage: Stage,
    nav_scene: Scene,
    theme: Theme,
    theme_picker: ThemePicker,
    app_state: AppState,
    scene_builders: Vec<Box<dyn Fn() -> Stage + 'static>>,
    view_index: usize,
    frames: usize,
}

impl AppDelegate {
    /// Constructor
    pub fn new(screen: Vector) -> Self {
        let mut theme = Theme::default();
        theme.font_size = 18.0;
        theme.bg_color = Color::from_hex("#FFFFEE");

        let mut theme_picker = ThemePicker::new();
        theme_picker.add_theme(LIGHT_THEME, "Light theme", || {
            let theme = ThemeBuilder::light_owl();
            theme
        });
        theme_picker.add_theme(DARK_THEME, "Dark theme", || {
            let theme = ThemeBuilder::night_owl();
            theme
        });
        let frame = Rectangle::new_sized(screen);
        let nav_scene = Scene::new(frame);
        let stage = Stage::new(frame);
        let app = AppDelegate {
            stage,
            nav_scene,
            theme,
            theme_picker,
            app_state: AppState::new(),
            scene_builders: Vec::new(),
            view_index: 0,
            frames: 0,
        };
        app
    }

    pub fn set_nav_scene(&mut self, scene: Scene) {
        self.nav_scene = scene;
    }

    /// Save a Scene closure to load later
    pub fn register_scene<C>(&mut self, cb: C)
    where
        C: Fn() -> Stage + 'static,
    {
        self.scene_builders.push(Box::new(cb));
    }

    /// Application lifecycle event called before runloop starts
    pub fn application_ready(&mut self) {
        self.load_scene();
    }

    pub fn load_scene(&mut self)
    {
        if let Some(cb) = self.scene_builders.get_mut(self.view_index) {
            let mut group = cb();
            self.nav_scene.set_field_value(&FieldValue::Text(group.title), TypeId::of::<Text>(), TITLE_TAG);
            self.stage.scenes.clear();
            self.stage.scenes.append(&mut group.scenes);
            self.stage.notify(&DisplayEvent::Ready);
        }
    }
}


// ************************************************************************************
// ************************************************************************************

#[allow(dead_code)]
#[allow(unused_variables)]
impl State for AppDelegate {
    fn new() -> Result<AppDelegate> {
        Err(Error::ContextError("The AppDelegate should not be run directly. ".to_string()))
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        for event in self.app_state.event_bus.into_iter() {
            if let Ok(evt) = event.downcast_ref::<NavEvent>() {
                log::debug!("NavEvent={:?}", evt);
                match evt {
                    NavEvent::Next => {
                        self.view_index += 1;
                        if self.view_index == self.scene_builders.len() {
                            self.view_index = 0;
                        }
                        self.load_scene();
                        return Ok(());
                    }
                    NavEvent::Back => {
                        if self.view_index == 0 {
                            self.view_index = self.scene_builders.len() - 1;
                        } else {
                            self.view_index -= 1;
                        }
                        self.load_scene();
                        return Ok(());
                    }
                    _ => ()
                }
            }
            if let Ok(evt) = event.downcast_ref::<SceneEvent>() {
                log::debug!("SceneEvent={:?}", evt);
                match evt {
                    SceneEvent::Show(_) => {
                        self.nav_scene.is_interactive = false;
                    }
                    SceneEvent::Hide(_) => {
                        self.nav_scene.is_interactive = true;
                    }
                    // _ => ()
                }
                self.stage.handle_event(evt);
            }
            if let Ok(evt) = event.downcast_ref::<ThemeEvent>() {
                log::debug!("ThemeEvent={:?}", evt);
                match evt {
                    ThemeEvent::Change(id) => {
                        if let Some(theme) = self.theme_picker.load_theme(*id) {
                            self.theme = theme;
                            self.stage.set_theme(&mut self.theme);
                        }
                    }
                    _ => ()
                }
            }
        }

        self.app_state.zero_offset();
        let _ = self.stage.update(window, &mut self.app_state);
        let _ = self.nav_scene.update(window, &mut self.app_state);

        self.frames += 1;
        if (self.frames % FPS_INTERVAL) == 0 {
            let out = format!("FPS: {:.2}", window.current_fps());
            self.nav_scene.set_field_value(&FieldValue::Text(out), TypeId::of::<Text>(), FPS_TAG);
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Remove any lingering artifacts from the previous frame

        // TODO: Theme should define this color
        window.clear(self.theme.bg_color)?;

        let _ = self.stage.render(&mut self.theme, window);
        let _ = self.nav_scene.render(&mut self.theme, window);

        Ok(())
    }

    #[allow(unused_assignments)]
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Focused => {
                log::debug!("event={:?}", event);
            }
            Event::MouseMoved(pt) => {
                let mut hover: bool = false;
                // FIXME: This hover value overrides previous result.
                // hover = self.scene.handle_mouse_at(pt);
                if self.nav_scene.is_interactive {
                    hover = self.nav_scene.handle_mouse_at(pt);
                }
                if !hover {
                    hover = self.stage.handle_mouse_at(pt);
                }
                if hover {
                    window.set_cursor(MouseCursor::Hand);
                } else {
                    window.set_cursor(MouseCursor::Default);
                }
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                self.stage.handle_mouse_down(&window.mouse().pos(), &mut self.app_state);
                if self.nav_scene.is_interactive {
                    self.nav_scene.handle_mouse_down(&window.mouse().pos(), &mut self.app_state);
                }
            }
            Event::MouseButton(MouseButton::Left, ButtonState::Released) => {
                if self.nav_scene.is_interactive {
                    self.nav_scene.handle_mouse_up(&window.mouse().pos(), &mut self.app_state);
                }
                self.stage.handle_mouse_up(&window.mouse().pos(), &mut self.app_state);
            }
            Event::MouseWheel(xy) => {
                self.stage.handle_mouse_scroll(xy, &mut self.app_state);
            }
            Event::Key(key, ButtonState::Pressed) => match key {
                Key::Escape => {
                    window.close();
                }
                _ => {
                    self.stage.handle_key_command(key, window);
                }
            },
            Event::Typed(c) => {
                self.stage.handle_key_press(*c, window);
            }
            _ => {}
        };
        Ok(())
    }
}

