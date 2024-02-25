use self::linux::LinuxWindow;

use super::event::Event;
pub mod linux;

#[derive(Debug)]
pub struct WindowProps {
    title: String,
    height: u32,
    width: u32,
}

impl WindowProps {
    pub fn new(title: String, height: u32, width: u32) -> Self {
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
            height: 720,
            width: 1280,
        }
    }
}

pub type EventCallBackFn = fn(&Event);

pub struct Window<T: WindowInterface> {
    pub window_implementation: T,
}
impl Window<LinuxWindow> {
    pub fn new(props: WindowProps) -> Self {
        Window {
            window_implementation: LinuxWindow::new(props).unwrap()
        }
    }
}
pub trait WindowInterface {
    fn on_update(&mut self);
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn set_event_callback(&mut self, mcallback: EventCallBackFn);
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
}
