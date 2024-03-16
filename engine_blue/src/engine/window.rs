use self::linux::LinuxWindow;

pub mod linux;

#[derive(Debug)]
pub struct WindowProps {
    title: String,
    height: u32,
    width: u32,
}

#[derive(Debug)]
pub enum WindowType {
    Linux
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

pub struct Window {
    pub window_implementation: Box<dyn WindowInterface>,
}
impl Window {
    pub fn new(props: WindowProps, system_type: WindowType) -> Self {
        match system_type {
        WindowType::Linux => Window {
            window_implementation: Box::new(LinuxWindow::new(props).unwrap())
        },
        _ => panic!("Not implemented for other window types yet")
        }
    }
}
pub trait WindowInterface {
    fn on_update(&mut self);
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
    fn window_should_close(&self) -> bool;
}
