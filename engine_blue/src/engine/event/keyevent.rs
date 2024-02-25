use super::{Event, EventType};

pub trait KeyEvent {
    fn get_key_code(&self) -> i32;
}

struct KeyPressedEvent {
    key_code: i32,
    repeat_count: i32,
    event: Event,
}

impl KeyPressedEvent {
    fn new(key_code: i32, repeat_count: i32) -> Self {
        KeyPressedEvent {
            key_code,
            repeat_count,
            event: Event {
                event_type: EventType::KeyPressed,
            },
        }
    }
}

impl KeyEvent for KeyPressedEvent {
    fn get_key_code(&self) -> i32 {
        self.key_code
    }
}

struct KeyReleasedEvent {
    key_code: i32,
    event: Event,
}

impl KeyReleasedEvent {
    fn new(key_code: i32) -> Self {
        KeyReleasedEvent {
            key_code,
            event: Event {
                event_type: EventType::KeyReleased,
            },
        }
    }
}

impl KeyEvent for KeyReleasedEvent {
    fn get_key_code(&self) -> i32 {
        self.key_code
    }
}
