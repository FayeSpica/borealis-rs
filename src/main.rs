mod lib;

fn main() {
    let mut application = lib::core::application::Application::new("demo/title", 1920, 1080);

    // Run the app
    while application.main_loop() {

    }
}