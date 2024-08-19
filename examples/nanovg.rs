extern crate glutin;
extern crate nanovg;
extern crate winit;

use glutin::platform::windows::WindowBuilderExtWindows;
use glutin::{ContextBuilder, GlProfile, GlRequest};
use nanovg::{Color, Context, Frame};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

fn main() {
    // 创建事件循环
    let event_loop = EventLoop::new();

    // 创建窗口
    let wb = WindowBuilder::new()
        .with_title("NanoVG with Glutin and Winit")
        .with_inner_size(LogicalSize::new(800.0, 600.0));

    // 创建 OpenGL 上下文
    let windowed_context = ContextBuilder::new()
        .with_gl(GlRequest::Latest)
        .with_gl_profile(GlProfile::Core)
        .build_windowed(wb, &event_loop)
        .unwrap()
        .make_current()
        .unwrap();

    // 初始化 NanoVG 上下文
    let mut vg = Context::create_gl3(nanovg::Antialias::On, nanovg::StencilStrokes::On)
        .expect("Failed to create NanoVG context!");

    // 渲染循环
    event_loop.run_return(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    windowed_context.resize(size);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                // 获取窗口大小
                let size = windowed_context.window().inner_size();
                let (width, height) = (size.width as f32, size.height as f32);

                // 设置视口并清除缓冲区
                unsafe {
                    gl::Viewport(0, 0, width as i32, height as i32);
                    gl::ClearColor(0.3, 0.3, 0.32, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
                }

                // 开始 NanoVG 绘制帧
                let mut frame = vg.frame((width, height), (1.0, 1.0), Frame::CLEAR);

                // 绘制矩形
                frame.path(|path| {
                    path.rect((100.0, 100.0), (400.0, 300.0));
                    path.fill(Color::rgb(255, 192, 0), Default::default());
                });

                // 结束绘制帧
                frame.end();

                // 交换前后缓冲区
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
