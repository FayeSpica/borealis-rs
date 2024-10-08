use std::cell::RefCell;
use std::ffi::c_int;
use std::rc::Rc;
use gl::{BLEND, COLOR_BUFFER_BIT, CULL_FACE, DEPTH_BUFFER_BIT, DEPTH_TEST, SCISSOR_TEST, STENCIL_BUFFER_BIT, STENCIL_TEST};
use glfw::{Context, fail_on_errors, Glfw, PWindow};
use glfw::ffi::{CONTEXT_VERSION_MAJOR, CONTEXT_VERSION_MINOR, glfwMakeContextCurrent, glfwSetFramebufferSizeCallback, glfwSetInputMode, glfwSwapBuffers, glfwSwapInterval, GLFWwindow, glfwWindowHint, OPENGL_CORE_PROFILE, OPENGL_PROFILE, STICKY_KEYS, TRUE};
use log::info;
use nanovg::{Color};

// GLFW Video Context
pub struct GLFWVideoContext {
    pub g: Glfw,
    pub window: glfw::PWindow,
    pub nvg_context: nanovg::Context,
}

extern "C" fn glfw_window_framebuffer_size_callback(window: *mut GLFWwindow, width: c_int, height: c_int) {
    if width < 0 || height < 0 {
        return;
    }

    unsafe {
        gl::Viewport(0, 0, width, height);
    }

    // todo: Application::on_window_resized(width, height);
}

impl GLFWVideoContext {
    pub fn new(window_title: &str, window_width: u32, window_height: u32) -> GLFWVideoContext {

        unsafe {
            glfwWindowHint(CONTEXT_VERSION_MAJOR, 4);
            glfwWindowHint(CONTEXT_VERSION_MINOR, 3);
            glfwWindowHint(OPENGL_PROFILE, OPENGL_CORE_PROFILE);
        }

        let mut g = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, _events) = g.create_window(window_width, window_height, window_title, glfw::WindowMode::Windowed)
            .expect("glfw: failed to create window");

        unsafe {
            // Configure window
            glfwSetInputMode(window.window_ptr(), STICKY_KEYS, TRUE);
            glfwMakeContextCurrent(window.window_ptr());
            glfwSetFramebufferSizeCallback(window.window_ptr(), Some(glfw_window_framebuffer_size_callback));

            // Load OpenGL routines using glad
            gl_loader::init_gl();
            gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
            glfwSwapInterval(1);

            info!("glfw: GL Vendor: {:#?}", gl::GetString(gl::VENDOR));
            info!("glfw: GL Renderer: {:#?}", gl::GetString(gl::RENDERER));
            info!("glfw: GL Version: {:#?}", gl::GetString(gl::VERSION));
        }

        // Initialize nanovg
        let mut nvg_context = nanovg::ContextBuilder::new()
            .stencil_strokes()
            .antialias()
            .build()
            .expect("glfw: unable to init nanovg");

        // Setup scaling
        glfw_window_framebuffer_size_callback(window.window_ptr(), window_width as c_int, window_height as c_int);

        GLFWVideoContext{
            g,
            window,
            nvg_context,
        }
    }
}

impl crate::lib::core::video::VideoContext for GLFWVideoContext {
    fn clear(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.red(), color.green(), color.blue(), 1.0);
            gl::Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT | STENCIL_BUFFER_BIT);
        }
    }

    fn begin_frame(&self) {

    }

    fn end_frame(&self) {
        unsafe {
            glfwSwapBuffers(self.window.window_ptr());
        }
    }

    fn reset_state(&self) {
        unsafe {
            gl::Disable(CULL_FACE);
            gl::Disable(BLEND);
            gl::Disable(DEPTH_TEST);
            gl::Disable(SCISSOR_TEST);
            gl::Disable(STENCIL_TEST);
        }
    }

    fn main_loop_iteration(&mut self) -> bool {
        let mut is_active = false;
        let window = &mut self.window;
        let mut g = &mut self.g;
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

    fn get_nvg_context(&self) -> &nanovg::Context {
        &self.nvg_context
    }
}