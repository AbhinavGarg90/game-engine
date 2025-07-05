use glfw::WindowEvent;
use log::info;

use super::{applicationevent::{WindowCloseEvent, WindowFocusEvent, WindowResizeEvent}, keyevent::{Key, KeyPressedEvent, KeyReleasedEvent}, mousevent::{Action, MouseButton, MouseButtonPressedEvent, MouseButtonReleasedEvent, MouseMovedEvent, MouseScrollEvent}, EventType};


/*
    TODO: Match statements can probably be refactored in the future
*/

impl From<glfw::MouseButton> for MouseButton {
    fn from(button: glfw::MouseButton) -> Self {
        match button {
            glfw::MouseButton::Button1 => MouseButton::Button1,
            glfw::MouseButton::Button2 => MouseButton::Button2,
            glfw::MouseButton::Button3 => MouseButton::Button3,
            glfw::MouseButton::Button4 => MouseButton::Button4,
            glfw::MouseButton::Button5 => MouseButton::Button5,
            glfw::MouseButton::Button6 => MouseButton::Button6,
            glfw::MouseButton::Button7 => MouseButton::Button7,
            glfw::MouseButton::Button8 => MouseButton::Button8,
        }
    }
}

impl From<glfw::Action> for Action {
    fn from(action: glfw::Action) -> Action {
        match action {
            glfw::Action::Press => Action::Press,
            glfw::Action::Release => Action::Release,
            glfw::Action::Repeat => Action::Repeat,
        }
    }
}

impl From<glfw::Key> for Key {
    fn from(key: glfw::Key) -> Key {
        match key {
            glfw::Key::Space => Key::Space,
            glfw::Key::Apostrophe => Key::Apostrophe,
            glfw::Key::Comma => Key::Comma,
            glfw::Key::Minus => Key::Minus,
            glfw::Key::Period => Key::Period,
            glfw::Key::Slash => Key::Slash,
            glfw::Key::Num0 => Key::Num0,
            glfw::Key::Num1 => Key::Num1,
            glfw::Key::Num2 => Key::Num2,
            glfw::Key::Num3 => Key::Num3,
            glfw::Key::Num4 => Key::Num4,
            glfw::Key::Num5 => Key::Num5,
            glfw::Key::Num6 => Key::Num6,
            glfw::Key::Num7 => Key::Num7,
            glfw::Key::Num8 => Key::Num8,
            glfw::Key::Num9 => Key::Num9,
            glfw::Key::Semicolon => Key::Semicolon,
            glfw::Key::Equal => Key::Equal,
            glfw::Key::A => Key::A,
            glfw::Key::B => Key::B,
            glfw::Key::C => Key::C,
            glfw::Key::D => Key::D,
            glfw::Key::E => Key::E,
            glfw::Key::F => Key::F,
            glfw::Key::G => Key::G,
            glfw::Key::H => Key::H,
            glfw::Key::I => Key::I,
            glfw::Key::J => Key::J,
            glfw::Key::K => Key::K,
            glfw::Key::L => Key::L,
            glfw::Key::M => Key::M,
            glfw::Key::N => Key::N,
            glfw::Key::O => Key::O,
            glfw::Key::P => Key::P,
            glfw::Key::Q => Key::Q,
            glfw::Key::R => Key::R,
            glfw::Key::S => Key::S,
            glfw::Key::T => Key::T,
            glfw::Key::U => Key::U,
            glfw::Key::V => Key::V,
            glfw::Key::W => Key::W,
            glfw::Key::X => Key::X,
            glfw::Key::Y => Key::Y,
            glfw::Key::Z => Key::Z,
            glfw::Key::LeftBracket => Key::LeftBracket,
            glfw::Key::Backslash => Key::Backslash,
            glfw::Key::RightBracket => Key::RightBracket,
            glfw::Key::GraveAccent => Key::GraveAccent,
            glfw::Key::World1 => Key::World1,
            glfw::Key::World2 => Key::World2,
            glfw::Key::Escape => Key::Escape,
            glfw::Key::Enter => Key::Enter,
            glfw::Key::Tab => Key::Tab,
            glfw::Key::Backspace => Key::Backspace,
            glfw::Key::Insert => Key::Insert,
            glfw::Key::Delete => Key::Delete,
            glfw::Key::Right => Key::Right,
            glfw::Key::Left => Key::Left,
            glfw::Key::Down => Key::Down,
            glfw::Key::Up => Key::Up,
            glfw::Key::PageUp => Key::PageUp,
            glfw::Key::PageDown => Key::PageDown,
            glfw::Key::Home => Key::Home,
            glfw::Key::End => Key::End,
            glfw::Key::CapsLock => Key::CapsLock,
            glfw::Key::ScrollLock => Key::ScrollLock,
            glfw::Key::NumLock => Key::NumLock,
            glfw::Key::PrintScreen => Key::PrintScreen,
            glfw::Key::Pause => Key::Pause,
            glfw::Key::F1 => Key::F1,
            glfw::Key::F2 => Key::F2,
            glfw::Key::F3 => Key::F3,
            glfw::Key::F4 => Key::F4,
            glfw::Key::F5 => Key::F5,
            glfw::Key::F6 => Key::F6,
            glfw::Key::F7 => Key::F7,
            glfw::Key::F8 => Key::F8,
            glfw::Key::F9 => Key::F9,
            glfw::Key::F10 => Key::F10,
            glfw::Key::F11 => Key::F11,
            glfw::Key::F12 => Key::F12,
            glfw::Key::F13 => Key::F13,
            glfw::Key::F14 => Key::F14,
            glfw::Key::F15 => Key::F15,
            glfw::Key::F16 => Key::F16,
            glfw::Key::F17 => Key::F17,
            glfw::Key::F18 => Key::F18,
            glfw::Key::F19 => Key::F19,
            glfw::Key::F20 => Key::F20,
            glfw::Key::F21 => Key::F21,
            glfw::Key::F22 => Key::F22,
            glfw::Key::F23 => Key::F23,
            glfw::Key::F24 => Key::F24,
            glfw::Key::F25 => Key::F25,
            glfw::Key::Kp0 => Key::Kp0,
            glfw::Key::Kp1 => Key::Kp1,
            glfw::Key::Kp2 => Key::Kp2,
            glfw::Key::Kp3 => Key::Kp3,
            glfw::Key::Kp4 => Key::Kp4,
            glfw::Key::Kp5 => Key::Kp5,
            glfw::Key::Kp6 => Key::Kp6,
            glfw::Key::Kp7 => Key::Kp7,
            glfw::Key::Kp8 => Key::Kp8,
            glfw::Key::Kp9 => Key::Kp9,
            glfw::Key::KpDecimal => Key::KpDecimal,
            glfw::Key::KpDivide => Key::KpDivide,
            glfw::Key::KpMultiply => Key::KpMultiply,
            glfw::Key::KpSubtract => Key::KpSubtract,
            glfw::Key::KpAdd => Key::KpAdd,
            glfw::Key::KpEnter => Key::KpEnter,
            glfw::Key::KpEqual => Key::KpEqual,
            glfw::Key::LeftShift => Key::LeftShift,
            glfw::Key::LeftControl => Key::LeftControl,
            glfw::Key::LeftAlt => Key::LeftAlt,
            glfw::Key::LeftSuper => Key::LeftSuper,
            glfw::Key::RightShift => Key::RightShift,
            glfw::Key::RightControl => Key::RightControl,
            glfw::Key::RightAlt => Key::RightAlt,
            glfw::Key::RightSuper => Key::RightSuper,
            glfw::Key::Menu => Key::Menu,
            glfw::Key::Unknown => Key::Unknown,
        }
    }
}

impl From<glfw::WindowEvent> for EventType {
    fn from(e: glfw::WindowEvent) -> EventType {
        match e {
            WindowEvent::Pos(x, y) => EventType::MouseMoved(MouseMovedEvent::new(x, y)),
            WindowEvent::Size(width, height) => EventType::WindowResize(WindowResizeEvent::new(width, height)),
            WindowEvent::Close => EventType::WindowClose(WindowCloseEvent {}),
            WindowEvent::Focus(occurred) => EventType::WindowFocus(WindowFocusEvent::new(occurred)),
            WindowEvent::MouseButton(mouse_button, glfw::Action::Press, _) => 
                EventType::MouseButtonPressed(MouseButtonPressedEvent::new(mouse_button.into())),
            WindowEvent::MouseButton(mouse_button, glfw::Action::Release, _) => 
                EventType::MouseButtonReleased(MouseButtonReleasedEvent::new(mouse_button.into())),
            WindowEvent::Scroll(x_offset, y_offset) => EventType::MouseScroll(MouseScrollEvent::new(x_offset, y_offset)),
            WindowEvent::Key(key, _, glfw::Action::Press, _) => EventType::KeyPressed(KeyPressedEvent::new(key.into(), 0)),
            WindowEvent::Key(key, _, glfw::Action::Release, _) => EventType::KeyReleased(KeyReleasedEvent::new(key.into())),
            _  => {
                info!("this event has not been implemented yet");
                EventType::None
            }
            // WindowEvent::CursorPos(_, _) => EventType::None,
            // WindowEvent::CursorEnter(bool) => EventType::None,
            // WindowEvent::Key(_, _, glfw::Action::Repeat, _) => EventType::None,
            // WindowEvent::Char(_) => EventType::None,
            // WindowEvent::CharModifiers(_, _) => EventType::None,
            // WindowEvent::FileDrop(_) => EventType::None,
            // WindowEvent::Maximize(_) => EventType::None,
            // WindowEvent::ContentScale(_, _) => EventType::None,
            // WindowEvent::Iconify(_) => EventType::None,
            // WindowEvent::FramebufferSize(_, _) => EventType::None,
            // WindowEvent::Refresh => EventType::None, // TODO
        }
    }
}
