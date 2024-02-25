use super::event::Event;

struct WindowProps {
    title: String,
    height: i32,
    width: i32,
}

impl WindowProps {
    fn new(title: String, height: i32, width: i32) -> Self {
        WindowProps {
            title,
            height,
            width,
        }
    }
}

impl Default for WindowProps {
    fn default() -> Self {
        WindowProps {
            title: "engine-blue".to_string(),
            height: 1280,
            width: 720,
        }
    }
}

pub type EventCallBackFn = fn(&Event);

pub(crate) struct Window<T: WindowInterface> {
    window_implementation: T,
}

pub trait WindowInterface {
    fn on_update();
    fn get_width() -> i32;
    fn get_height() -> i32;
    fn set_event_callback(callback: EventCallBackFn);
    fn set_vsync(enabled: bool);
    fn is_vsync() -> bool;
}
