use glfw::Window;
use super::StaticEventType;

use crate::impl_get_static_type;
use crate::impl_new_functions;

use super::Event;
use super::EventType;

#[derive(Debug)]
pub(crate) struct WindowResizeEvent {
    width: i32,
    height: i32,
    event: Event,
}

impl WindowResizeEvent {
    pub fn new(width:i32, height:i32) -> Self {
        WindowResizeEvent {
            width,
            height,
            event: Event {
                event_type: EventType::WindowResize,
            }
        }
    }
    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}

pub struct WindowCloseEvent {
    event: Event,
}

pub struct AppTickEvent {
    event: Event,
}

pub struct AppUpdateEvent {
    event: Event,
}

pub struct AppRenderEvent {
    event: Event,
}

impl_new_functions!(
    WindowCloseEvent, WindowClose,
    AppTickEvent, AppTick,
    AppUpdateEvent, AppUpdate,
    AppRenderEvent, AppRender
);  

impl_get_static_type!(
    WindowCloseEvent, WindowClose,
    AppTickEvent, AppTick,
    AppUpdateEvent, AppUpdate,
    AppRenderEvent, AppRender
);