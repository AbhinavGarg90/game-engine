use log::info;

use crate::engine::event::applicationevent::WindowResizeEvent;

use super::{
    event::{applicationevent::WindowCloseEvent, EventCategory, EventDispatcher, EventType, StaticEventType},
    window::{linux::LinuxWindow, EventCallBackFn, Window, WindowInterface, WindowProps},
};


pub struct Application<T: WindowInterface> {
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
    pub fn close_window(event: &WindowCloseEvent) -> bool {
        self.running = false;
        true
    }
    // currently takes ownership, may change later
    fn on_event(event: dyn StaticEventType) {
        let dispatcher = EventDispatcher::new(&event);
        if let EventType::WindowClose = event.get_static_type() {
            dispatcher.dispatch();
        }
    }
}

impl<T: WindowInterface> Application<T> {
    pub fn run(&mut self) {
        while !self.window.window_implementation.window_should_close() {
            self.window.window_implementation.on_update();
        }
    }
}
