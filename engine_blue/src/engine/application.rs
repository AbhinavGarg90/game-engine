use log::info;

use crate::engine::event::applicationevent::WindowResizeEvent;

use super::{event::{Event, EventType}, window::{linux::LinuxWindow, Window, WindowInterface, WindowProps}};


pub struct Application<T: WindowInterface>{
    window: Window<T>,
    running: bool,
}

impl Application<LinuxWindow> {
    pub fn new() -> Application<LinuxWindow> {
        let mut window = Window::new(WindowProps::default());
        Application {
            window, 
            running: true,
        }
    }
    pub fn close_window(&mut self) -> bool{
        self.running = false;
        true 
    }
}

impl<T: WindowInterface> Application<T> {
    pub fn run(&mut self) {
        while !self.window.window_implementation.window_should_close(){
            self.window.window_implementation.on_update();
        }
    }
}