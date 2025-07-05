use std::{mem::{self, size_of}, os::raw::c_void};

use gl::types::{GLenum, GLuint};

use super::{
    event::{applicationevent::WindowCloseEvent, DispatchesEvent, EventType},
    layer::{self, Layer},
    layerstack::LayerStack,
    window::{Window, WindowProps, WindowType},
};

pub struct Application {
    window: Window,
    running: bool,
    layer_stack: LayerStack,
    vertex_array_handle: u32,
    index_buffer_handle: u32,
}

impl DispatchesEvent for Application {
    fn on_event(&mut self, e: &EventType) -> bool {
        match e {
            EventType::WindowClose(e) => self.close_window(e),
            _ => {
                let layer_stack_end = self.layer_stack.get_end();
                for layer in layer_stack_end {
                    dbg!(layer.get_debug_name());
                    if (**layer).on_event(e) {
                        return true
                    }
                }
                false
            }
        }
    }
}

impl Application {
    pub fn new(system_type: WindowType) -> Application {
        let window = Window::new(WindowProps::default(), system_type);
        let mut vertex_array_handle: GLuint = 0;
        let mut index_buffer_handle: GLuint = 0;
        let vertices = [
            -0.5, -0.5, 0.0,
			 0.5, -0.5, 0.0,
			 0.0,  0.5, 0.0

        ];
        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array_handle);
            gl::BindVertexArray(vertex_array_handle);
            gl::GenBuffers(1, &mut vertex_array_handle as *mut u32);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_array_handle);
            gl::BufferData(vertex_array_handle, 9, mem::transmute(&vertices[0]), gl::STATIC_DRAW);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<f32>() as i32, 0x0 as *const c_void);

            gl::GenBuffers(1, &mut index_buffer_handle as *mut u32);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_handle);

            let indices = [ 0, 1, 2 ];
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 3, mem::transmute(&indices[0]), gl::STATIC_DRAW);

        }
        Application {
            window,
            running: true,
            layer_stack: LayerStack::default(),
            vertex_array_handle,
            index_buffer_handle,
        }
    }
    pub fn close_window(&mut self, event: &WindowCloseEvent) -> bool {
        self.running = false;
        true
    }
    // currently takes ownership, may change later
    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layer_stack.push_layer(layer);
    }
    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layer_stack.push_overlay(overlay);
    }
}

impl Application {
    pub fn run(&mut self) {
        while self.running {
            unsafe {gl::ClearColor(0f32, 0f32, 1f32, 1f32);}
            unsafe {gl::Clear(gl::COLOR_BUFFER_BIT)}
            let layer_stack_iter = self.layer_stack.get_begin();
            for layer in layer_stack_iter {
                layer.on_update();
            }
            for event in self.window.window_implementation.on_update().iter() {
                self.on_event(event);
            }
        }
    }
}
