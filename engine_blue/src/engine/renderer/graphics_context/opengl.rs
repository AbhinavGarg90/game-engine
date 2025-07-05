use glfw::{PWindow, Context};

use super::GraphicsContext;

pub(crate) struct OpenGlContext {
    pub glfw_window_handle: PWindow,
}

impl OpenGlContext {
    pub fn new(glfw_window_handle: PWindow) -> Self {
        OpenGlContext {
            glfw_window_handle
        }
    }
}

impl GraphicsContext for OpenGlContext {
    fn init(&mut self) {
        self.glfw_window_handle.make_current();
        gl::load_with(|symbol| self.glfw_window_handle.get_proc_address(symbol) as *const _);
    }

    fn swap_buffers(&mut self) {
       self.glfw_window_handle.swap_buffers(); 
    }
}
