pub trait KeyEvent {
    fn get_key_code(&self) -> i32;
}

#[derive(Debug, PartialEq)]
pub struct KeyPressedEvent {
    key_code: i32, // leaving as int to avoid pain of creating enum
    repeat_count: i32,
}

impl KeyPressedEvent {
    pub fn new(key_code: i32, repeat_count: i32) -> Self {
        KeyPressedEvent {
            key_code,
            repeat_count,
        }
    }
}

impl KeyEvent for KeyPressedEvent {
    fn get_key_code(&self) -> i32 {
        self.key_code
    }
}

pub struct KeyReleasedEvent {
    key_code: i32,
}

impl KeyReleasedEvent {
    pub fn new(key_code: i32) -> Self {
        KeyReleasedEvent {
            key_code,
        }
    }
}

impl KeyEvent for KeyReleasedEvent {
    fn get_key_code(&self) -> i32 {
        self.key_code
    }
}
