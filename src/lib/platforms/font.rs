use crate::lib::core::font::{FONT_MATERIAL_ICONS, FONT_SWITCH_ICONS, FontLoader};

// Font loader that reads everything from resources
pub struct GLFWFontLoader {

}

impl GLFWFontLoader {
    pub fn new() -> Self {
        Self {

        }
    }
}

static USER_REGULAR_PATH: &str = "User-Regular.ttf";
static INTER_FONT_PATH: &str = "inter/Inter-Switch.ttf";
static USER_SWITCH_ICONS_PATH: &str = "User-Switch-Icons.ttf";

static MATERIAL_ICONS_PATH: &str = "material/MaterialIcons-Regular.ttf";

impl FontLoader for GLFWFontLoader {
    fn load_fonts(&self) {
        // Regular
        // Try to use user-provided font first, fallback to Inter
        // todo!()

        // Switch icons
        // Only supports user-provided font
        self.load_font_from_file(FONT_SWITCH_ICONS, USER_SWITCH_ICONS_PATH);

        // Material icons
        self.load_material_from_resources();
    }

    fn load_font_from_file(&self, font_name: &str, font_path: &str) -> bool {
        todo!()
    }


    fn load_material_from_resources(&self) -> bool {
        return self.load_font_from_file(FONT_MATERIAL_ICONS, MATERIAL_ICONS_PATH)
    }
}