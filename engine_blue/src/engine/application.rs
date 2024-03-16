use super::{
    event::{applicationevent::WindowCloseEvent, DispatchesEvent, EventType},
    layer::Layer,
    layerstack::LayerStack,
    window::{Window, WindowProps, WindowType},
};

pub struct Application {
    window: Window,
    running: bool,
    layer_stack: LayerStack,
}

impl DispatchesEvent for Application {
    fn on_event(&mut self, e: &EventType) -> bool {
        match e {
            EventType::WindowClose(e) => self.close_window(e),
            _ => {
                for layer in self.layer_stack.get_end() {
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
        let mut window = Window::new(WindowProps::default(), system_type);
        Application {
            window,
            running: true,
            layer_stack: LayerStack::default(),
        }
    }
    pub fn close_window(&mut self, event: &WindowCloseEvent) -> bool {
        self.running = false;
        true
    }
    // currently takes ownership, may change later
    fn push_layer(self, layer: Box<dyn Layer>) {}
}

impl Application {
    pub fn run(&mut self) {
        while self.running {
            unsafe {gl::ClearColor(0f32, 0f32, 1f32, 1f32);}
            unsafe {gl::Clear(gl::COLOR_BUFFER_BIT)}
            self.window.window_implementation.on_update();
        }
    }
}
