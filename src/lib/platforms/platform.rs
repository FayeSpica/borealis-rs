use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use glfw::{Context, Glfw, PWindow};
use crate::lib::core::audio::{AudioPlayer, NullAudioPlayer};
use crate::lib::core::font::FontLoader;
use crate::lib::core::input::InputManager;
use crate::lib::core::platform::Platform;
use crate::lib::core::theme::ThemeVariant;
use crate::lib::core::video::VideoContext;
use crate::lib::platforms::font::GLFWFontLoader;
use crate::lib::platforms::video::GLFWVideoContext;
use crate::lib::platforms::input::GLFWInputManager;

pub struct GlfwPlatform {
    audio_player: Rc<RefCell<Box<dyn AudioPlayer>>>,
    input_manager: Rc<RefCell<Box<dyn InputManager>>>,
    font_loader: Rc<RefCell<Box<dyn FontLoader>>>,
    video_context: Rc<RefCell<Box<dyn VideoContext>>>,
    g: Rc<RefCell<Glfw>>,
    window: Rc<RefCell<PWindow>>,
}

impl GlfwPlatform {
    pub fn new(title: &str, width: u32, height: u32) -> GlfwPlatform {
        let mut glfw_video_context = GLFWVideoContext::new(title, width, height);
        GlfwPlatform {
            audio_player: Rc::new(RefCell::new(Box::new(NullAudioPlayer::new()))),
            input_manager: Rc::new(RefCell::new(Box::new(GLFWInputManager::new(Rc::clone(&glfw_video_context.get_glfw()))))),
            font_loader: Rc::new(RefCell::new(Box::new(GLFWFontLoader::new()))),
            g: glfw_video_context.get_glfw(),
            window: glfw_video_context.get_glfw_window(),
            video_context: Rc::new(RefCell::new(Box::new(glfw_video_context))),
        }
    }
}

impl Platform for GlfwPlatform {

    fn get_name(&self) -> &str {
        "GLFW"
    }

    fn main_loop_iteration(&mut self) -> bool {
        let mut is_active = false;
        let window = self.window.borrow_mut();
        let mut g = self.g.borrow_mut();
        while !is_active {

            is_active = unsafe { glfw::ffi::glfwGetWindowAttrib(window.window_ptr(), glfw::ffi::ICONIFIED) != 0 };

            if is_active {
                g.poll_events();
            } else {
                g.wait_events();
            }
        }
        !window.should_close()
    }

    fn get_theme_variant(&self) -> ThemeVariant {
        if let Ok(value) = std::env::var("BOREALIS_THEME") {
            match value.as_str() {
                "DARK" => {
                    ThemeVariant::Dark
                }
                _ => {
                    ThemeVariant::Light
                }
            }
        } else {
            ThemeVariant::Light
        }
    }

    fn get_locale(&self) -> &str {
        "CN"
    }

    fn get_audio_player(&mut self) -> Rc<RefCell<Box<dyn AudioPlayer>>> {
        Rc::clone(&self.audio_player)
    }

    fn get_video_context(&mut self) -> Rc<RefCell<Box<dyn VideoContext>>> {
        Rc::clone(&self.video_context)
    }

    fn get_input_manager(&mut self) -> Rc<RefCell<Box<dyn InputManager>>> {
        Rc::clone(&self.input_manager)
    }

    fn get_font_loader(&mut self) -> Rc<RefCell<Box<dyn FontLoader>>> {
        Rc::clone(&self.font_loader)
    }
}