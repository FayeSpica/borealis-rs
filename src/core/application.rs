const ORIGINAL_WINDOW_WIDTH: u32 = 1280;
const ORIGINAL_WINDOW_HEIGHT: u32 = 720;

pub struct Application {}

impl Application {
    /**
     * Inits the borealis application.
     * Returns Ok if it succeeded, Err otherwise.
     */
    pub fn init() -> anyhow::Result<Self> {
        Ok(Application {})
    }

    pub fn create_window(&self, title: &str) {}

    pub fn main_loop(&self) -> bool {
        true
    }
}
