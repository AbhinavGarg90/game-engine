use self::{applicationevent::{AppRenderEvent, AppTickEvent, AppUpdateEvent, WindowCloseEvent, WindowFocusEvent, WindowLostFocusEvent, WindowMovedEvent, WindowResizeEvent}, keyevent::{KeyPressedEvent, KeyReleasedEvent}, mousevent::{MouseButtonPressedEvent, MouseButtonReleasedEvent, MouseMovedEvent, MouseScrollEvent}};

pub mod applicationevent;
pub mod keyevent;
pub mod mousevent;
pub mod glfw_event_handler;

pub enum EventType {
    None ,
    WindowClose(WindowCloseEvent),
    WindowResize(WindowResizeEvent),
    WindowFocus(WindowFocusEvent),
    WindowLostFocus(WindowLostFocusEvent),
    WindowMoved(WindowMovedEvent),
    AppTick(AppTickEvent),
    AppUpdate(AppUpdateEvent),
    AppRender(AppRenderEvent),
    KeyPressed(KeyPressedEvent),
    KeyReleased(KeyReleasedEvent),
    MouseButtonPressed(MouseButtonPressedEvent),
    MouseButtonReleased(MouseButtonReleasedEvent),
    MouseMoved(MouseMovedEvent),
    MouseScroll(MouseScrollEvent),
}

pub(crate) trait DispatchesEvent {
    fn on_event(&mut self, e: &EventType) -> bool;
}

impl EventType {
    fn to_string(&self) -> String {
        match self {
            EventType::None => "None".to_string(),
            EventType::WindowClose(_) => "WindowClose".to_string(),
            EventType::WindowResize(_) => "WindowResize".to_string(),
            EventType::WindowFocus(_) => "WindowFocus".to_string(),
            EventType::WindowLostFocus(_) => "WindowLostFocus".to_string(),
            EventType::WindowMoved(_) => "WindowMoved".to_string(),
            EventType::AppTick(_) => "AppTick".to_string(),
            EventType::AppUpdate(_) => "AppUpdate".to_string(),
            EventType::AppRender(_) => "AppRender".to_string(),
            EventType::KeyPressed(_) => "KeyPressed".to_string(),
            EventType::KeyReleased(_) => "KeyReleased".to_string(),
            EventType::MouseButtonPressed(_) => "MouseButtonPressed".to_string(),
            EventType::MouseButtonReleased(_) => "MouseButtonReleased".to_string(),
            EventType::MouseMoved(_) => "MouseMoved".to_string(),
            EventType::MouseScroll(_) => "MouseScrolled".to_string(),
        }
    }
}
