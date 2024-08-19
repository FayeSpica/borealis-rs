use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::core::audio::AudioPlayer;
use crate::lib::core::font::FontLoader;
use crate::lib::core::input::InputManager;
use crate::lib::core::theme::ThemeVariant;
use crate::lib::core::video::VideoContext;

// Interface to provide everything platform specific required to run borealis: graphics context, inputs, audio...
// The best platform is automatically selected when the application starts, and cannot be changed by the user at the moment
pub trait Platform {

    /**
     * Returns the human-readable name of the platform.
     */
    fn get_name(&self) -> &str;

    /**
     * Called at every iteration of the main loop.
     * Must return false if the app should continue running
     * (for example, return false if the X button was pressed on the window).
     */
    fn main_loop_iteration(&mut self) -> bool;

    /**
     * Can be called at anytime to get the current system theme variant.
     *
     * For now, the variant is assumed to stay the same during the whole time
     * the app is running (no variant hot swap).
     *
     * As such, the result should be cached by the platform code.
     */
    fn get_theme_variant(&self) -> ThemeVariant;


    /**
     * Can be called at anytime to get the current locale
     *
     * For now, the locale is assumed to stay the same during the whole time
     * the app is running (no locale hot swap)
     *
     * As such, the result should be cached by the platform code.
     * The method should return one of the locale constants
     * defined in the i18n header file.
     */
    fn get_locale(&self) -> &str;

    /**
     * Returns the AudioPlayer for the platform.
     * Cannot return nullptr.
     */
    fn get_audio_player(&mut self) -> Rc<RefCell<Box<dyn AudioPlayer>>>;

    /**
     * Returns the VideoContext for the platform.
     * Cannot return nullptr.
     */
    fn get_video_context(&mut self) -> Rc<RefCell<Box<dyn VideoContext>>>;

    /**
     * Returns the InputManager for the platform.
     * Cannot return nullptr.
     */
    fn get_input_manager(&mut self) -> Rc<RefCell<Box<dyn InputManager>>>;

    /**
     * Returns the FontLoader for the platform.
     * Cannot return nullptr.
     */
    fn get_font_loader(&mut self) -> Rc<RefCell<Box<dyn FontLoader>>>;
}

/**
 * Selects and returns the best platform.
 */
pub fn create_platform() -> *mut Box<dyn Platform> {
    panic!()
}