use crate::lib::core::platform::Platform;
use crate::lib::core::video::VideoContext;
use crate::lib::platforms::platform::GlfwPlatform;

pub struct Application {
    glfw_platform: GlfwPlatform
}

impl Application {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Application {
            glfw_platform: GlfwPlatform::new(title, width, height)
        }
    }

    pub fn main_loop(&mut self) -> bool {
        // if !self.glfw_platform.main_loop_iteration() {
        //     println!("false");
        //     return false;
        // }
        // Render
        self.frame();
        true
    }

    pub fn frame(&self) {
        self.glfw_platform.get_video_context().begin_frame();
        self.glfw_platform.get_video_context().clear(nanovg::Color::from_rgb(100, 0, 0));
        self.glfw_platform.get_video_context().end_frame();
    }
}