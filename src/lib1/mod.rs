pub(crate) mod core;
pub(crate) mod platforms;
pub(crate) mod views;


#[cfg(test)]
mod tests {
    #[test]
    pub fn test_main() {
        let mut application = lib::core::application::Application::new("demo/title", 1920, 1080);

        // Run the app
        while application.main_loop() {

        }
    }
}