use std::collections::HashMap;

pub const FONT_INVALID: i32 = -1;
pub const FONT_REGULAR: &str = "regular"; // regular Latin font
pub const FONT_KOREAN_REGULAR: &str = "korean"; // regular Korean font
pub const FONT_MATERIAL_ICONS: &str = "material"; // Material icons font
pub const FONT_SWITCH_ICONS: &str = "switch"; // Switch icons font (see the HOS shared symbols font for an example)

pub struct FontStash(HashMap<String, i32>);

impl FontStash {
    pub fn insert(&mut self, k: &str, v: i32) {
        self.0.insert(k.into(), v);
    }

    pub fn get(&mut self, k: &str) -> Option<&i32> {
        self.0.get(k)
    }
}


// Platform interface to load fonts from disk or other sources (system / shared font...)
pub trait FontLoader {

    /**
     * Called once on init to load every font in the font stash.
     *
     * The implementation must use the Application::loadFont and
     * Application::loadFontFromMemory methods to load as much as possible
     * of the "built-in" fonts defined in the FONT_* constants above.
     */
    fn load_fonts(&self);

    /**
     * Convenience method to load a font from a file path
     * with some more logging.
     */
    fn load_font_from_file(&self, font_name: &str, font_path: &str) -> bool;

    /**
     * Can be called internally to load the Material icons font from resources.
     * Returns true if the operation succeeds.
     */
    fn load_material_from_resources(&self) -> bool;
}