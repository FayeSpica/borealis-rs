use std::collections::HashMap;
use once_cell::sync::Lazy;

pub struct StyleValues {
    values: HashMap<String, f32>
}

impl StyleValues {
    pub fn new(list: Vec<(&str, f32)>) -> StyleValues {
        let mut values: HashMap<String, f32> = HashMap::new();
        for (name, value) in list {
            values.insert(name.into(), value);
        }
        StyleValues {
            values
        }
    }

    pub fn add_metric(&mut self, name: &str, value: f32) {
        self.values.insert(name.into(), value);
    }

    pub fn get_metric(&self, name: &'_ str) -> Option<&f32> {
        self.values.get(name)
    }
}

// Simple wrapper around StyleValues for the array operator
pub struct Style  {
    style_values: StyleValues
}

impl Style {
    pub fn new(values: StyleValues) -> Style {
        Style {
            style_values: values
        }
    }

    pub fn add_metric(&mut self, name: &str, value: f32) {
        self.style_values.add_metric(name, value);
    }

    pub fn get_metric(&self, name: &'_ str) -> Option<&f32> {
        self.style_values.get_metric(name)
    }
}

pub static STYLE: Lazy<Style> = Lazy::new(||{
    Style::new(StyleValues::new(
        vec![
            // Animations
            ( "brls/animations/show", 250.0),
            ( "brls/animations/show_slide", 125.0),

            ( "brls/animations/highlight", 100.0),
            ( "brls/animations/highlight_shake", 15.0),

            ( "brls/animations/label_scrolling_timer", 1500.0),
            ( "brls/animations/label_scrolling_speed", 0.05),

            // Highlight
            ( "brls/highlight/stroke_width", 5.0),
            ( "brls/highlight/corner_radius", 0.5),
            ( "brls/highlight/shadow_width", 2.0),
            ( "brls/highlight/shadow_offset", 10.0),
            ( "brls/highlight/shadow_feather", 10.0),
            ( "brls/highlight/shadow_opacity", 128.0),

            // AppletFrame
            ( "brls/applet_frame/padding_sides", 30.0),

            ( "brls/applet_frame/header_height", 88.0),
            ( "brls/applet_frame/header_padding_top_bottom", 15.0),
            ( "brls/applet_frame/header_padding_sides", 35.0),
            ( "brls/applet_frame/header_image_title_spacing", 18.0),
            ( "brls/applet_frame/header_title_font_size", 28.0),
            ( "brls/applet_frame/header_title_top_offset", 7.0),

            ( "brls/applet_frame/footer_height", 73.0),
            ( "brls/applet_frame/footer_padding_top_bottom", 20.0),
            ( "brls/applet_frame/footer_padding_sides", 25.0),

            // TabFrame
            ( "brls/tab_frame/sidebar_width", 410.0),
            ( "brls/tab_frame/content_padding_top_bottom", 42.0), // unused by the library, here for users
            ( "brls/tab_frame/content_padding_sides", 60.0), // unused by the library, here for users

            // Sidebar
            ( "brls/sidebar/border_height", 16.0),
            ( "brls/sidebar/padding_top", 32.0),
            ( "brls/sidebar/padding_bottom", 47.0),
            ( "brls/sidebar/padding_left", 80.0),
            ( "brls/sidebar/padding_right", 30.0),
            ( "brls/sidebar/item_height", 70.0),
            ( "brls/sidebar/item_accent_margin_top_bottom", 9.0),
            ( "brls/sidebar/item_accent_margin_sides", 8.0),
            ( "brls/sidebar/item_accent_rect_width", 4.0),
            ( "brls/sidebar/item_font_size", 22.0),
            ( "brls/sidebar/separator_height", 30.0),

            // Label
            ( "brls/label/default_font_size", 20.0),
            ( "brls/label/default_line_height", 1.65),
            ( "brls/label/scrolling_animation_spacing", 50.0),
            ( "brls/label/highlight_padding", 2.0),

            // Header
            ( "brls/header/padding_top_bottom", 11.0),
            ( "brls/header/padding_right", 11.0),
            ( "brls/header/rectangle_width", 5.0),
            ( "brls/header/rectangle_height", 22.0),
            ( "brls/header/rectangle_margin", 10.0),
            ( "brls/header/font_size", 18.0),

            // Button
            ( "brls/button/padding_top_bottom", 15.0),
            ( "brls/button/padding_sides", 25.0),
            ( "brls/button/corner_radius", 5.0),
            ( "brls/button/text_size", 18.0),
            ( "brls/button/primary_highlight_padding", 2.0),
            ( "brls/button/border_thickness", 2.0),

            // Generic shadow
            ( "brls/shadow/width", 2.0),
            ( "brls/shadow/feather", 10.0),
            ( "brls/shadow/opacity", 63.75),
            ( "brls/shadow/offset", 10.0),
        ]
    ))
});