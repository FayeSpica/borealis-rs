use nanovg::Context;
use nanovg_sys::{nvgBeginFrame, nvgBeginPath, NVGcolor, nvgEndFrame, nvgFill, nvgFillColor, nvgRect};

pub struct View {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl View {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        View { x, y, width, height }
    }

    pub fn draw(&self, vg: &Context) {
        // 默认的绘制方法，子类可以重写此方法
        unsafe {
            nvgBeginFrame(vg.raw(), 800.0, 600.0, 1.0);
            nvgBeginPath(vg.raw());
            nvgRect(vg.raw(), self.x, self.y, self.width, self.height);
            nvgFillColor(vg.raw(), NVGcolor{ rgba: [0.0, 1.0, 1.0, 1.0] });
            nvgFill(vg.raw());
            nvgEndFrame(vg.raw());
        }
    }
}