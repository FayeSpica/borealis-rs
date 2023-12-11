mod lib;

fn main() {
    let mut application = lib::core::application::Application::new("demo/title", 1280, 720);

    // Run the app
    while application.main_loop() {
        
    }
}
