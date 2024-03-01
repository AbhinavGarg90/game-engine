use crate::impl_get_static_type;

use super::StaticEventType;

use super::{Event, EventType};

pub struct MouseMovedEvent {
    event: Event,
    mouse_x: f64,
    mouse_y: f64,
}

impl MouseMovedEvent {
    fn get_x(&self) -> f64 {
        self.mouse_x
    }

    fn get_y(&self) -> f64 {
        self.mouse_y
    }
}

pub struct MouseScrollEvent {
    event: Event,
    mouse_x_offset: f64,
    mouse_y_offset: f64,
}

impl MouseScrollEvent {
    fn get_x_offset(&self) -> f64 {
        self.mouse_x_offset
    }

    fn get_y(&self) -> f64 {
        self.mouse_y_offset
    }
}

struct MouseButtonEvent {
    button: i32,
    event: Event,
}

struct MouseButtonPressedEvent {
    mouse_button_event: MouseButtonEvent,
}

impl MouseButtonPressedEvent {
    fn new(button: i32) -> Self {
        MouseButtonPressedEvent {
            mouse_button_event: MouseButtonEvent {
                button,
                event: Event {event_type: EventType::MouseButtonPressed}
            },
        }
    }
}

struct MouseButtonReleasedEvent {
    mouse_button_event: MouseButtonEvent,
}

impl MouseButtonReleasedEvent {
    fn new(button: i32) -> Self {
        MouseButtonReleasedEvent {
            mouse_button_event: MouseButtonEvent {
                button,
                event: Event {event_type: EventType::MouseButtonPressed}
            },
        }
    }
}


impl_get_static_type!(
    MouseMovedEvent, MoueMoved,
    MouseScrollEvent, MouseScrolled,
    MouseButtonPressedEvent, MouseButtonPressed,
    MouseButtonReleasedEvent, MouseButtonReleased
);