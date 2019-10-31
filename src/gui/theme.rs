use crate::core::*;
use crate::events::*;
use crate::tools::*;

use quicksilver::graphics::Color;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

static ROBOTO_REGULAR: &[u8] = include_bytes!("../../static/Roboto-Regular.ttf");

pub const LIGHT_THEME: u32 = 1;
pub const DARK_THEME: u32 = 2;

/// Simple enum to define Theme type. Possibly useful for a theme selector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeType {
    /// light theme
    Light,
    /// dark theme
    Dark,
}

/// A single Theme holds all of the custom display attributes for rendering a Scene.
/// The current Theme is passed with every render() call to all child objects that implement
/// Displayable.
/// TODO: Move color primitives into a style object that can be versioned.
pub struct Theme {
    /// The display name. Should be unique (but not enforced)
    pub name: String,
    /// The theme type
    pub theme_type: ThemeType,
    /// Font size
    pub font_size: f32,
    /// Background color
    pub bg_color: Color,
    /// Foreground color
    pub fg_color: Color,
    /// Border color
    pub border_color: Color,
    /// Background color for input fields
    pub input_bg_color: Color,
    /// Foreground color for in put fields
    pub input_fg_color: Color,
    /// Button background color (primary?)
    pub button_bg_color: Color,
    /// Button foreground color
    pub button_fg_color: Color,
    /// Scrollbar slider color
    pub scrollbar_fg_color: Color,
    /// Cursor color
    pub cursor_color: Color,
    /// Border width
    pub border_width: f32,
    /// Default margins (internal padding) for horizontal and vertical
    pub margins: (f32, f32),
    /// The default field height for input controls and listbox rows
    pub field_height: f32,
    /// Default animation props for button hover events
    pub on_button_hover: PropSet,
    /// Default animation props for button click events
    pub on_button_click: PropSet,
    /// Default animation props for view hover events
    pub on_view_hover: PropSet,
    /// The default font for drawing or rendering text. Almost all font rendering happens through
    /// this font, regardless of font size or color. A DrawFont maintains its own GPU texture which
    /// makes performance good.
    pub default_font: DrawFont,
    /// A simple Fonts table
    fonts: Fonts,
}

impl Default for Theme {
    fn default() -> Self {
        // The default font is loaded from static bytes. However, custom fonts can be added using the
        // add_named_font() method as long as the file path is correct. In this project, files in the
        // "static" directory can be loaded with:
        //     theme.add_named_font(Theme::DEFAULT_FONT, "Helvetica.ttf");
        //     theme.set_default_font(Theme::DEFAULT_FONT);
        let font = DrawFont::from_bytes(ROBOTO_REGULAR.clone().to_vec(), None);

        Theme {
            name: "Default".to_string(),
            theme_type: ThemeType::Light,
            font_size: 16.0,
            bg_color: Color::WHITE,
            fg_color: Color::BLACK,
            border_color: Color::from_hex("#AAAAAA"),
            input_bg_color: Color::from_hex("#FFFFFF"),
            input_fg_color: Color::from_hex("#333333"),
            button_bg_color: Color::from_hex("#4373c2"),
            button_fg_color: Color::from_hex("#FFFFFF"),
            cursor_color: Color::from_hex("#80A4C2"),
            scrollbar_fg_color: Color::from_hex("#CCCCCC"),
            border_width: 1.0,
            margins: (5.0, 5.0),
            field_height: 20.0,
            on_button_hover: PropSet::new([color("#EEEEEE")].to_vec(), 0.2).for_type(TweenType::Hover),
            on_button_click: PropSet::new([color("#AAAAAA")].to_vec(), 0.2).for_type(TweenType::Click),
            on_view_hover: PropSet::new(Vec::new(), 0.0).for_type(TweenType::Hover),
            default_font: font,
            fonts: Fonts::new(),
        }
    }
}

impl Theme {
    /// Common name for default font
    pub const DEFAULT_FONT: &'static str = "default";
    /// Common name for default font
    pub const REGULAR_FONT: &'static str = "regular";
    /// Common name for bold font
    pub const BOLD_FONT: &'static str = "bold";
    /// Common name for italic font
    pub const ITALIC_FONT: &'static str = "italic";

    /// Add a font with a standard name. The Vec<u8> font data is stored in the Fonts hashmap.
    /// This should be called when creating a Theme (usually during startup)
    /// The path must be accurate or the code will panic and crash
    pub fn add_named_font(&mut self, name: &str, path: &str) {
        log::debug!("path={:?} name={:?}", path, name);
        let load = load_file_bytes(path);
        if load.is_ok() {
            let font_bytes = load.unwrap();
            assert!(font_bytes.len() > 0, "Font bytes was empty");
            self.fonts.add_font(name, font_bytes);
        } else {
            panic!("File was not loaded");
        }
    }

    /// Fetch saved font bytes. Font data added through add_named_font() can be retrieved using this method.
    pub fn data_for_font(&mut self, name: &str) -> Vec<u8> {
        if let Some(data) = self.fonts.get_font_data(name) {
            return data.clone();
        } else {
            let bytes = ROBOTO_REGULAR.clone().to_vec();
            return bytes;
        }
    }

    /// This method replaces self.default_font with the font already registered with the specified name.
    /// If the named font data does not exist, it returns false
    pub fn set_default_font(&mut self, name: &str) -> bool {
        if let Some(data) = self.fonts.get_font_data(name) {
            let font = DrawFont::from_bytes(data, None);
            self.default_font = font;
            return true;
        }
        false
    }

    // Method used to draw image and/or text and render as an Image
    // pub fn draw_image(&mut self, params: &DrawParams) -> Option<Image> {
    //     None
    // }
}

// ************************************************************************************
// Fonts
// ************************************************************************************

/// Simple file loader
pub fn load_file_bytes(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// A holder of loaded Fonts
pub struct Fonts {
    /// A HashMap where font bytes are stored
    font_data: HashMap<String, Vec<u8>>,
}

impl Fonts {
    /// Constructor
    pub fn new() -> Self {
        Fonts {
            // font_map: HashMap::new(),
            font_data: HashMap::new(),
        }
    }

    /// Purge all data if loading new fonts (when loading new Theme)
    pub fn reset(&mut self) {
        self.font_data.clear();
    }

    /// Check to see if font already registered before trying to add it again.
    pub fn has_font(&self, name: &str) -> bool {
        self.font_data.contains_key(name)
    }

    /// Method for adding font bytes
    pub fn add_font(&mut self, name: &str, bytes: Vec<u8>) {
        self.font_data.insert(name.to_string(), bytes.clone());
    }

    /// Get the Vec<u8> data for a font
    pub fn get_font_data(&self, name: &str) -> Option<Vec<u8>> {
        if let Some(data) = self.font_data.get(name) {
            return Some(data.clone());
        }
        None
    }
}

// ************************************************************************************
// ThemePicker
// ************************************************************************************

/// A holder of available themes that are wrapped in callback functions that are loaded
/// and selected at runtime.
pub struct ThemePicker {
    /// The theme that was last loaded/used. When load_theme is called, this value is changed.
    theme_id: u32,
    /// A helper array that can be used to show the list of themes that have been added.
    theme_list: Vec<(u32, String)>,
    /// Hashmap that stores the function closures that return a Theme, using the u32 id
    /// as the key.
    theme_map: HashMap<u32, Box<dyn Fn() -> Theme + 'static>>,
}

impl ThemePicker {
    /// Constructor
    pub fn new() -> Self {
        ThemePicker { theme_id: 0, theme_list: Vec::new(), theme_map: HashMap::new() }
    }

    /**
    *  Usage:
       let mut theme_picker = ThemePicker::new();
       theme_picker.add_theme(1, "Dark theme", || {
           let mut theme = ThemeBuilder::night_owl();
           // Modify theme if you want.
           theme
       });
       // Add more themes
    */
    pub fn add_theme<C>(&mut self, id: u32, name: &str, cb: C)
    where
        C: Fn() -> Theme + 'static,
    {
        self.theme_map.insert(id, Box::new(cb));
        if let Some(index) = self.theme_list.iter().position(|v| v.0 == id) {
            // Theme already added with this id, so remove it from the list
            self.theme_list.remove(index);
        }
        self.theme_list.push((id, name.to_string()));
    }

    /**
    *  USAGE:
       if let Some(theme) = self.theme_picker.load_theme(1) {
           self.theme = theme;
       }
    */
    pub fn load_theme(&mut self, id: u32) -> Option<Theme> {
        if let Some(cb) = self.theme_map.get(&id) {
            self.theme_id = id;
            return Some(cb());
        }
        None
    }

    /// Returns a reference to the list of loaded themes. This is useful for
    /// displaying a list of available themes
    pub fn get_theme_list(&self) -> &Vec<(u32, String)> {
        &self.theme_list
    }
}

// ************************************************************************************
// ThemeBuilder
// ************************************************************************************

/// A helper for creating themes.
pub struct ThemeBuilder {}

/// Some popular themes are provided here. In the future, a more modular system of adding themes may be implemented.
impl ThemeBuilder {
    /**
    Constructor for night owl dark theme
    FROM: https://github.com/sdras/night-owl-vscode-theme/blob/master/themes/Night%20Owl-color-theme.json
     */
    pub fn night_owl() -> Theme {
        let font = DrawFont::from_bytes(ROBOTO_REGULAR.clone().to_vec(), None);

        Theme {
            name: "Night Owl".to_string(),
            theme_type: ThemeType::Dark,
            font_size: 16.0,
            bg_color: Color::from_hex("#011627"),        // editor.background
            fg_color: Color::from_hex("#d6deeb"),        // editor.foreground
            border_color: Color::from_hex("#5f7e97"),    // input.border
            input_bg_color: Color::from_hex("#0b253a"),  // input.background
            input_fg_color: Color::from_hex("#ffffff"),  // input.foreground
            button_bg_color: Color::from_hex("#4373c2"), // selection.background
            button_fg_color: Color::from_hex("#ffffff"), // button.foreground
            cursor_color: Color::from_hex("#80a4c2"),    // editorCursor.foreground
            scrollbar_fg_color: Color::from_hex("#084D81"),
            on_button_hover: PropSet::new([color("#EEEEEE")].to_vec(), 0.2).for_type(TweenType::Hover),
            on_button_click: PropSet::new([shift(3.0, 3.0)].to_vec(), 0.0).for_type(TweenType::Click),
            on_view_hover: PropSet::new(Vec::new(), 0.0).for_type(TweenType::Hover),
            border_width: 1.0,
            margins: (5.0, 5.0),
            field_height: 20.0,
            default_font: font,
            fonts: Fonts::new(),
        }
    }

    /**
    Constructor for night owl light theme
    FROM: https://github.com/sdras/night-owl-vscode-theme/blob/master/themes/Night%20Owl-Light-color-theme.json
     */
    pub fn light_owl() -> Theme {
        let font = DrawFont::from_bytes(ROBOTO_REGULAR.clone().to_vec(), None);

        Theme {
            name: "Light Owl".to_string(),
            theme_type: ThemeType::Light,
            font_size: 16.0,
            bg_color: Color::from_hex("#FBFBFB"),        // editor.background
            fg_color: Color::from_hex("#403f53"),        // editor.foreground
            border_color: Color::from_hex("#d9d9d9"),    // input.border
            input_bg_color: Color::from_hex("#F0F0F0"),  // input.background
            input_fg_color: Color::from_hex("#403f53"),  // input.foreground
            button_bg_color: Color::from_hex("#7a8181"), // selection.background
            button_fg_color: Color::from_hex("#F0F0F0"), // button.foreground
            cursor_color: Color::from_hex("#90A7B2"),    // editorCursor.foreground
            scrollbar_fg_color: Color::from_hex("#CCCCCC"),
            on_button_hover: PropSet::new([color("#AAAAAA")].to_vec(), 0.2).for_type(TweenType::Hover),
            on_button_click: PropSet::new([shift(3.0, 3.0)].to_vec(), 0.0).for_type(TweenType::Click),
            on_view_hover: PropSet::new(Vec::new(), 0.0).for_type(TweenType::Hover),
            border_width: 1.0,
            margins: (5.0, 5.0),
            field_height: 20.0,
            default_font: font,
            fonts: Fonts::new(),
        }
    }
}
