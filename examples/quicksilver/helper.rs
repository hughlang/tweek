/// Miscellaneous helpers
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, MeshTask},
};
use tweek::prelude::*;

#[allow(dead_code)]
pub const NAV_SCENE: u32 = 100;
#[allow(dead_code)]
pub const BG_SCENE: u32 = 200;
#[allow(dead_code)]
pub const MAIN_SCENE: u32 = 300;
#[allow(dead_code)]
pub const FPS_TAG: u32 = 901;
#[allow(dead_code)]
pub const TITLE_TAG: u32 = 902;

pub struct DemoHelper {}

#[allow(dead_code)]
impl DemoHelper {
    pub fn get_draw_area(screen: Vector) -> Rectangle {
        let margin = 100.0;
        let stage_width = screen.x - margin * 2.0;
        let stage_height = screen.y - margin * 2.0;
        let draw_area = Rectangle::new(((screen.x - stage_width) / 2.0, 120.0), (stage_width, stage_height));
        draw_area
    }

    pub fn draw_grid(width: f32, height: f32, interval: f32, color: Color) -> MeshTask {
        let mut xpos = 0.0;
        // Vertical lines first
        let mut task = MeshTask::new(0);
        while xpos < width {
            let pts: [&Vector; 2] = [&Vector::new(xpos, 0.0), &Vector::new(xpos, height)];
            let mut line = DrawShape::line(&pts, color, 0.5);
            task.append(&mut line);
            xpos += interval;
        }
        let mut ypos = 0.0;
        while ypos < height {
            let pts: [&Vector; 2] = [&Vector::new(0.0, ypos), &Vector::new(width, ypos)];
            let mut line = DrawShape::line(&pts, color, 0.5);
            task.append(&mut line);
            ypos += interval;
        }
        task
    }

    pub fn make_next_prev_buttons(screen: &Vector) -> Vec<Button> {
        const BUTTON_WIDTH: f32 = 90.0;
        const BUTTON_HEIGHT: f32 = 40.0;

        let mut buttons: Vec<Button> = Vec::with_capacity(2);
        let ypos = 30.0;

        let style = FontStyle::new(18.0, Color::BLACK);
        let click_props = PropSet::new(vec![shift(2.0, 2.0)], 0.1).for_type(TweenType::Click);
        let hover_props =
            PropSet::new(vec![color(HexColors::Chocolate), tint(HexColors::White)], 0.2).for_type(TweenType::Hover);
        // ---- Previous ---------------------
        let frame = Rectangle::new((30.0, ypos), (BUTTON_WIDTH, BUTTON_HEIGHT));
        // let image = theme.font.render("Previous", &style).unwrap();
        let mut button = Button::new(frame)
            .background(BackgroundStyle::Solid(Color::from_hex(HexColors::Tan)))
            .with_text("Previous");
        button.layer.corner_radius = 5.0;
        button.layer.font_style = style;
        button.set_hover_animation(hover_props.clone());
        button.set_click_animation(click_props.clone());
        button.set_onclick(move |state| {
            state.event_bus.register_event(NavEvent::Back);
        });

        buttons.push(button);

        // ---- Next ---------------------
        let frame = Rectangle::new((screen.x - BUTTON_WIDTH - 30.0, ypos), (BUTTON_WIDTH, BUTTON_HEIGHT));
        // let image = theme.font.render("Next", &style).unwrap();
        let mut button =
            Button::new(frame).background(BackgroundStyle::Solid(Color::from_hex(HexColors::Tan))).with_text("Next");
        button.layer.font_style = style;
        button.layer.corner_radius = 5.0;

        button.set_hover_animation(hover_props.clone());
        button.set_click_animation(click_props.clone());
        button.set_onclick(move |state| {
            state.event_bus.register_event(NavEvent::Next);
        });
        buttons.push(button);

        buttons
    }

    pub fn make_theme_buttons(screen: &Vector) -> Vec<Button> {
        const BUTTON_WIDTH: f32 = 90.0;
        const BUTTON_HEIGHT: f32 = 30.0;

        let mut buttons: Vec<Button> = Vec::with_capacity(2);
        let ypos = screen.y - 20.0 - BUTTON_HEIGHT;

        let style = FontStyle::new(18.0, Color::BLACK);
        let click_props = PropSet::new(vec![shift(2.0, 2.0)], 0.1).for_type(TweenType::Click);
        let hover_props =
            PropSet::new(vec![color(HexColors::Silver), tint(HexColors::White)], 0.2).for_type(TweenType::Hover);
        // ---- Previous ---------------------
        let frame = Rectangle::new((screen.x - 32.0 - BUTTON_WIDTH * 2.0, ypos), (BUTTON_WIDTH, BUTTON_HEIGHT));
        // let image = theme.font.render("Previous", &style).unwrap();
        let mut button = Button::new(frame)
            .background(BackgroundStyle::Solid(Color::from_hex(HexColors::WhiteSmoke)))
            .with_text("Light Theme");
        // button.layer.corner_radius = 5.0;
        button.layer.font_style = style;
        button.set_hover_animation(hover_props.clone());
        button.set_click_animation(click_props.clone());
        button.set_onclick(move |state| {
            state.event_bus.register_event(ThemeEvent::Change(LIGHT_THEME));
        });
        buttons.push(button);

        // ---- Next ---------------------
        let frame = Rectangle::new((screen.x - BUTTON_WIDTH - 30.0, ypos), (BUTTON_WIDTH, BUTTON_HEIGHT));
        // let image = theme.font.render("Next", &style).unwrap();
        let mut button = Button::new(frame)
            .background(BackgroundStyle::Solid(Color::from_hex(HexColors::WhiteSmoke)))
            .with_text("Dark Theme");
        button.layer.font_style = style;
        // button.layer.corner_radius = 5.0;

        button.set_hover_animation(hover_props.clone());
        button.set_click_animation(click_props.clone());
        button.set_onclick(move |state| {
            state.event_bus.register_event(ThemeEvent::Change(DARK_THEME));
        });
        buttons.push(button);

        buttons
    }

    pub fn build_nav_scene(screen: Vector) -> Scene {
        let frame = Rectangle::new((0.0, 0.0), (screen.x, screen.y));
        let mut scene = Scene::new(frame).with_id(NAV_SCENE, "Nav Scene");

        let mut buttons = DemoHelper::make_next_prev_buttons(&screen);
        buttons.drain(..).for_each(|x| {
            let _ = scene.add_control(Box::new(x));
        });

        let mut buttons = DemoHelper::make_theme_buttons(&screen);
        buttons.drain(..).for_each(|x| {
            let _ = scene.add_control(Box::new(x));
        });

        let x = 20.0;
        let y = screen.y - 40.0;
        let frame = Rectangle::new((x, y), (80.0, 20.0));
        let mut text = Text::new(frame, "");
        text.layer.font_style = FontStyle::new(14.0, Color::RED);
        text.set_id(FPS_TAG);
        scene.add_control(Box::new(text));

        let width = 300.0;
        let height = 30.0;
        let ypos = 30.0;
        let frame = Rectangle::new(((screen.x - width) / 2.0, ypos), (width, height));
        let mut text = Text::new(frame, "");
        text.layer.font_style = FontStyle::new(18.0, Color::BLACK);
        text.set_id(TITLE_TAG);
        text.text_align(TextAlign::Center);
        // text.layer.apply_props(&[border(1.0, "#333333")]);

        scene.add_control(Box::new(text));

        scene
    }

    pub fn load_theme() -> Theme {
        let mut theme = Theme::default();
        theme.font_size = 18.0;
        theme.bg_color = Color::from_hex("#FFFFEE");

        theme
    }
}
