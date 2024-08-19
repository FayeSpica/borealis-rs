use std::cell::{Cell, RefCell, RefMut};
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
    audio_player: Box<dyn AudioPlayer>,
    input_manager: Box<dyn InputManager>,
    font_loader: Box<dyn FontLoader>,
    video_context: Box<dyn VideoContext>,
}

impl GlfwPlatform {
    pub fn new(title: &str, width: u32, height: u32) -> GlfwPlatform {
        let mut glfw_video_context = GLFWVideoContext::new(title, width, height);
        GlfwPlatform {
            audio_player: Box::new(NullAudioPlayer::new()),
            input_manager: Box::new(GLFWInputManager::new(&mut glfw_video_context.g)),
            font_loader: Box::new(GLFWFontLoader::new()),
            video_context: Box::new(glfw_video_context),
        }
    }
}

impl Platform for GlfwPlatform {

    fn get_name(&self) -> &str {
        "GLFW"
    }

    fn main_loop_iteration(&mut self) -> bool {
        self.video_context.main_loop_iteration()
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

    fn get_audio_player(&self) -> & Box<dyn AudioPlayer> {
        &self.audio_player
    }

    fn get_video_context(&self) -> & Box<dyn VideoContext> {
        &self.video_context
    }

    fn get_input_manager(&self) -> & Box<dyn InputManager> {
        &self.input_manager
    }

    fn get_font_loader(&self) -> & Box<dyn FontLoader> {
        &self.font_loader
    }

    fn get_audio_player_mut(&mut self) -> &mut Box<dyn AudioPlayer> {
        &mut self.audio_player
    }

    fn get_video_context_mut(&mut self) -> &mut Box<dyn VideoContext> {
        &mut self.video_context
    }

    fn get_input_manager_mut(&mut self) -> &mut Box<dyn InputManager> {
        &mut self.input_manager
    }

    fn get_font_loader_mut(&mut self) -> &mut Box<dyn FontLoader> {
        &mut self.font_loader
    }
}