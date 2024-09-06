use std::collections::HashMap;

pub type FontStash = HashMap<String, i64>;

const USER_FONT_PATH: &str = "font/font.ttf";
const USER_ICON_PATH: &str = "font/font.ttf";

/// Platform interface to load fonts from disk or other sources (system / shared font...)
pub trait FontLoader {
    /**
     * Called once on init to load every font in the font stash.
     *
     * The implementation must use the Application::loadFont and
     * Application::loadFontFromMemory methods to load as much as possible
     * of the "built-in" fonts defined in the FONT_* constants above.
     */
    fn load_fonts();

    /**
     * Convenience method to load a font from a file path
     * with some more logging.
     */
    fn load_font_from_file(font_name: &str, file_path: &str) -> anyhow::Result<()>;

    /**
     * Can be called internally to load the Material icons font from resources.
     * Returns true if the operation succeeds.
     */
    fn load_material_from_resources() -> anyhow::Result<()>;
}
