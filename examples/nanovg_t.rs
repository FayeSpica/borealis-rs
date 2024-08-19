use borealis_rs::gl_config_picker;
use glutin::config::{Config, ConfigTemplateBuilder, GlConfig};
use glutin::context::{ContextApi, ContextAttributesBuilder};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use std::error::Error;
use winit::event_loop::EventLoop;
use winit::window::Window;

fn main() -> Result<(), Box<dyn Error>> {
    // 创建事件循环
    let event_loop = EventLoop::new().unwrap();

    let window_attributes = Window::default_attributes()
        .with_transparent(true)
        .with_title("borealis");

    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(cfg!(cgl_backend));

    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

    let (mut window, gl_config) =
        match display_builder.build(&event_loop, template, gl_config_picker) {
            Ok(ok) => ok,
            Err(e) => {
                event_loop.exit();
                return Ok(());
            }
        };

    println!("Picked a config with {} samples", gl_config.num_samples());

    let raw_window_handle = window
        .as_ref()
        .and_then(|window| window.window_handle().ok())
        .map(|handle| handle.as_raw());

    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    Ok(())
}
