use crate::gl;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color};
use glutin::context::PossiblyCurrentContext;
use glutin::surface::{Surface, WindowSurface};
use winit::window::Window;

pub struct View {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl View {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        View {
            x,
            y,
            width,
            height,
        }
    }

    pub fn draw(
        &self,
        context: &PossiblyCurrentContext,
        surface: &Surface<WindowSurface>,
        window: &Window,
        canvas: &mut Canvas<OpenGl>,
    ) {
        canvas.clear_rect(
            self.x,
            self.y,
            self.width,
            self.height,
            Color::rgbf(1., 0., 0.),
        );
    }
}
