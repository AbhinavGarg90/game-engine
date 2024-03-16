use super::EventType;

#[derive(Debug)]
pub(crate) struct WindowResizeEvent {
    width: i32,
    height: i32,
}

impl WindowResizeEvent {
    pub fn new(width: i32, height: i32) -> Self {
        WindowResizeEvent { width, height }
    }
    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}

pub struct WindowCloseEvent {}

pub struct WindowFocusEvent {
    occurred: bool,
}

impl WindowFocusEvent {
    pub fn new(occurred: bool) -> Self {
        WindowFocusEvent {
            occurred
        }
    }
}

pub struct WindowLostFocusEvent {}

pub struct WindowMovedEvent {}

pub struct AppTickEvent {}

pub struct AppUpdateEvent {}

pub struct AppRenderEvent {}
