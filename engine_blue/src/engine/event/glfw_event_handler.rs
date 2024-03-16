use glfw::WindowEvent;
use log::info;

use super::{applicationevent::{WindowCloseEvent, WindowFocusEvent, WindowResizeEvent}, keyevent::{KeyPressedEvent, KeyReleasedEvent}, mousevent::{Action, MouseButton, MouseButtonPressedEvent, MouseButtonReleasedEvent, MouseMovedEvent, MouseScrollEvent}, EventType};

impl From<glfw::MouseButton> for MouseButton {
    fn from(button: glfw::MouseButton) -> Self {
        unsafe {std::mem::transmute(button)}
    }
}

impl From<glfw::Action> for Action {
    fn from(action: glfw::Action) -> Action {
        unsafe {std::mem::transmute(action)}
    }
}


pub trait ToEventType {
    fn to_event_type(e: WindowEvent) -> EventType{
        match e {
            WindowEvent::Pos(x, y) => EventType::MouseMoved(MouseMovedEvent::new(x, y)),
            WindowEvent::Size(width, height) => EventType::WindowResize(WindowResizeEvent::new(width, height)),
            WindowEvent::Close => EventType::WindowClose(WindowCloseEvent {}),
            WindowEvent::Refresh => EventType::None, // TODO
            WindowEvent::Focus(occurred) => EventType::WindowFocus(WindowFocusEvent::new(occurred)),
            WindowEvent::Iconify(_) => EventType::None,
            WindowEvent::FramebufferSize(_, _) => EventType::None,
            WindowEvent::MouseButton(mouse_button, glfw::Action::Press, _) => 
                EventType::MouseButtonPressed(MouseButtonPressedEvent::new(mouse_button.into())),
            WindowEvent::MouseButton(mouse_button, glfw::Action::Release, _) => 
                EventType::MouseButtonReleased(MouseButtonReleasedEvent::new(mouse_button.into())),
            WindowEvent::MouseButton(_, glfw::Action::Repeat, _) => {
                info!("mouse repeated not implemented yet");
                EventType::None
            }
            WindowEvent::CursorPos(_, _) => EventType::None,
            WindowEvent::CursorEnter(bool) => EventType::None,
            WindowEvent::Scroll(x_offset, y_offset) => EventType::MouseScroll(MouseScrollEvent::new(x_offset, y_offset)),
            WindowEvent::Key(key, _, glfw::Action::Press, _) => EventType::KeyPressed(KeyPressedEvent::new(key as i32, 0)),
            WindowEvent::Key(key, _, glfw::Action::Release, _) => EventType::KeyReleased(KeyReleasedEvent::new(key as i32)),
            WindowEvent::Key(Key, _, glfw::Action::Repeat, _) => EventType::None,
            WindowEvent::Char(_) => EventType::None,
            WindowEvent::CharModifiers(_, _) => EventType::None,
            WindowEvent::FileDrop(_) => EventType::None,
            WindowEvent::Maximize(_) => EventType::None,
            WindowEvent::ContentScale(_, _) => EventType::None,

        }
    }
}
impl ToEventType for WindowEvent {}
