use log::info;

use crate::engine::event::applicationevent::WindowResizeEvent;

use super::window::{linux::LinuxWindow, Window, WindowInterface, WindowProps};


pub struct Application<T: WindowInterface>{
    window: Window<T>,
    running: bool,
}

impl Application<LinuxWindow> {
    pub fn new<L: WindowInterface>() -> Self {
        Application {
            window: Window::new(WindowProps::default()),
            running: true,
        }
    }
}

impl<T: WindowInterface> Application<T> {
    pub fn run(&mut self) {
        while self.running {
            self.window.window_implementation.on_update();
        }
    }
}