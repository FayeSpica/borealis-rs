use std::collections::HashMap;
use nanovg::Color;
use once_cell::sync::Lazy;

#[derive(PartialEq)]
pub enum ThemeVariant {
    Light,
    Dark
}

pub struct ThemeValues {
    values: HashMap<String, Color>
}

impl ThemeValues {
    pub fn new(list: Vec<(&str, Color)>) -> ThemeValues {
        let mut values: HashMap<String, Color> = HashMap::new();
        for (name, color) in list {
            values.insert(name.into(), color);
        }
        ThemeValues {
            values
        }
    }
}

pub struct Theme  {
    theme_values: ThemeValues
}

impl Theme{
    pub fn new(values: ThemeValues) -> Theme {
        Theme {
            theme_values: values
        }
    }

    pub fn add_color(&mut self, name: &str, color: Color) {
        self.theme_values.values.insert(name.into(), color);
    }

    pub fn get_color(&self, name: &'_ str) -> Option<&Color> {
        self.theme_values.values.get(name)
    }
}

static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| {
    let light_theme_values = ThemeValues::new(
        vec![
            // Generic values
            ("brls/background", Color::from_rgb(235, 235, 235)),
            ("brls/text", Color::from_rgb(45, 45, 45)),
            ("brls/backdrop", Color::from_rgba(0, 0, 0, 178)),
            ("brls/click_pulse", Color::from_rgba(13, 182, 213, 38)), // same as highlight color1 with different opacity

            // Highlight
            ("brls/highlight/background", Color::from_rgb(252, 255, 248)),
            ("brls/highlight/color1", Color::from_rgb(13, 182, 213)),
            ("brls/highlight/color2", Color::from_rgb(80, 239, 217)),

            // AppletFrame
            ("brls/applet_frame/separator", Color::from_rgb(45, 45, 45)),

            // Sidebar
            ("brls/sidebar/background", Color::from_rgb(240, 240, 240)),
            ("brls/sidebar/active_item", Color::from_rgb(49, 79, 235)),
            ("brls/sidebar/separator", Color::from_rgb(208, 208, 208)),

            // Header
            ("brls/header/border", Color::from_rgb(207, 207, 207)),
            ("brls/header/rectangle", Color::from_rgb(127, 127, 127)),
            ("brls/header/subtitle", Color::from_rgb(140, 140, 140)),

            // Button
            ("brls/button/primary_enabled_background", Color::from_rgb(50, 79, 241)),
            ("brls/button/primary_disabled_background", Color::from_rgb(201, 201, 209)),
            ("brls/button/primary_enabled_text", Color::from_rgb(255, 255, 255)),
            ("brls/button/primary_disabled_text", Color::from_rgb(220, 220, 228)),

            ("brls/button/default_enabled_background", Color::from_rgb(255, 255, 255)),
            ("brls/button/default_disabled_background", Color::from_rgb(255, 255, 255)),
            ("brls/button/default_enabled_text", Color::from_rgb(45, 45, 45)),
            ("brls/button/default_disabled_text", Color::from_rgb(45, 45, 45)),

            ("brls/button/highlight_enabled_text", Color::from_rgb(49, 79, 235)),
            ("brls/button/highlight_disabled_text", Color::from_rgb(49, 79, 235)),

            ("brls/button/enabled_border_color", Color::from_rgb(45, 45, 45)),
            ("brls/button/disabled_border_color", Color::from_rgb(45, 45, 45)),
        ]
    );
    Theme::new(light_theme_values)
});

static DARK_THEME: Lazy<Theme> = Lazy::new(||{
    let dark_theme_value = ThemeValues::new(
        vec![
            // Generic values
            ("brls/background", Color::from_rgb(45, 45, 45)),
            ("brls/text", Color::from_rgb(255, 255, 255)),
            ("brls/backdrop", Color::from_rgba(0, 0, 0, 178)),
            ("brls/click_pulse", Color::from_rgba(25, 138, 198, 38)), // same as highlight color1 with different opacity

            // Highlight
            ("brls/highlight/background", Color::from_rgb(31, 34, 39)),
            ("brls/highlight/color1", Color::from_rgb(25, 138, 198)),
            ("brls/highlight/color2", Color::from_rgb(137, 241, 242)),

            // AppletFrame
            ("brls/applet_frame/separator", Color::from_rgb(255, 255, 255)),

            // Sidebar
            ("brls/sidebar/background", Color::from_rgb(50, 50, 50)),
            ("brls/sidebar/active_item", Color::from_rgb(0, 255, 204)),
            ("brls/sidebar/separator", Color::from_rgb(81, 81, 81)),

            // Header
            ("brls/header/border", Color::from_rgb(78, 78, 78)),
            ("brls/header/rectangle", Color::from_rgb(160, 160, 160)),
            ("brls/header/subtitle", Color::from_rgb(163, 163, 163)),

            // Button
            ("brls/button/primary_enabled_background", Color::from_rgb(1, 255, 201)),
            ("brls/button/primary_disabled_background", Color::from_rgb(83, 87, 86)),
            ("brls/button/primary_enabled_text", Color::from_rgb(52, 41, 55)),
            ("brls/button/primary_disabled_text", Color::from_rgb(71, 75, 74)),

            ("brls/button/default_enabled_background", Color::from_rgb(80, 80, 80)),
            ("brls/button/default_disabled_background", Color::from_rgb(80, 80, 80)),
            ("brls/button/default_enabled_text", Color::from_rgb(255, 255, 255)),
            ("brls/button/default_disabled_text", Color::from_rgb(255, 255, 255)),

            ("brls/button/highlight_enabled_text", Color::from_rgb(7, 247, 198)),
            ("brls/button/highlight_disabled_text", Color::from_rgb(7, 247, 198)),

            ("brls/button/enabled_border_color", Color::from_rgb(255, 255, 255)),
            ("brls/button/disabled_border_color", Color::from_rgb(255, 255, 255)),
        ]
    );
    Theme::new(dark_theme_value)
});

pub fn get_light_theme() -> &'static Theme {
    &LIGHT_THEME
}

pub fn get_dark_theme() -> &'static Theme {
    &DARK_THEME
}