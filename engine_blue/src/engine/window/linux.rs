use std::error::Error;

use gl::types;
use glfw::{
    fail_on_errors, Context, Glfw, GlfwReceiver, InitError, PWindow, WindowEvent, WindowMode,
};
use log::info;

use crate::engine::event;

use super::{EventCallBackFn, WindowInterface, WindowProps};

pub struct LinuxWindow {
    title: String,
    width: u32,
    height: u32,
    vsync: bool,
    event_callback_fn: Option<EventCallBackFn>,
    glfw_handle: Glfw,
    glfw_window_handle: PWindow,
    glfw_event_handle: GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl LinuxWindow {
    pub fn new(props: WindowProps) -> Result<Self, Box<dyn Error>> {
        info!("creating a window with: {props:?}");
        // unwrapping purposely as failing to start glfw should stop program
        // must be checked for implementation if/when making multiple windows
        let mut glfw_handle = glfw::init(fail_on_errors!())?;
        let (mut glfw_window_handle, glfw_event_handle) = match glfw_handle.create_window(
            props.width,
            props.height,
            &props.title,
            WindowMode::Windowed,
        ) {
            Some(ret) => ret,
            None => return Err(Box::from("failed to initialize")),
        };
        glfw_window_handle.make_current();
        // Set window pointer // add if needed
        // VSYNC is set automatically for now
        //  ThreadSafeGlfw used for setting if needed
        glfw_window_handle.set_all_polling(true);

        gl::load_with(|symbol| glfw_window_handle.get_proc_address(symbol) as *const _);
        Ok(LinuxWindow {
            title: props.title,
            width: props.width,
            height: props.height,
            glfw_event_handle,
            glfw_window_handle,
            glfw_handle,
            vsync: true,
            event_callback_fn: None,
        })
    }
    fn handle_event(&self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorPos(x, y) => info!("xpos: {x} ypos: {y}"),
            _ => info!("other event"),
        }
    }
}

impl WindowInterface for LinuxWindow {
    fn on_update(&mut self) {
        self.glfw_window_handle.swap_buffers();

        self.glfw_handle.poll_events();
        for (_, event) in glfw::flush_messages(&self.glfw_event_handle) {
            info!("{event:?}");
            self.handle_event(&event);
        }
    }
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn set_event_callback(&mut self, callback: EventCallBackFn) {
        self.event_callback_fn = Some(callback);
    }
    fn set_vsync(&mut self, enabled: bool) {
        // does nothing at the moment
        // may be removed later if not useful
    }
    fn is_vsync(&self) -> bool {
        self.vsync
    }
    fn window_should_close(&self) -> bool {
        self.glfw_window_handle.should_close()
    }
}
