pub struct MouseMovedEvent {
    mouse_x: i32,
    mouse_y: i32,
}

impl MouseMovedEvent {
    pub fn new(x: i32, y: i32) -> Self {
        MouseMovedEvent {
            mouse_x: x,
            mouse_y: y,
        }
    }
    fn get_x(&self) -> i32 {
        self.mouse_x
    }

    fn get_y(&self) -> i32 {
        self.mouse_y
    }
}

pub struct MouseScrollEvent {
    x_offset: f64,
    y_offset: f64,
}

impl MouseScrollEvent {
    pub fn new(x_offset: f64, y_offset: f64) -> Self {
        MouseScrollEvent {
            x_offset,
            y_offset
        }
    }
    fn get_x_offset(&self) -> f64 {
        self.x_offset
    }

    fn get_y(&self) -> f64 {
        self.y_offset
    }
}

#[repr(i32)]
pub enum MouseButton {
    Button1 = 0,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
}


#[repr(i32)]
pub enum Action {
    Release,
    Press,
    Repeat,
}

pub struct MouseButtonEvent {
    button: MouseButton,
}

pub struct MouseButtonPressedEvent {
    mouse_button_event: MouseButtonEvent,
}

impl MouseButtonPressedEvent {
    pub fn new(button: MouseButton) -> Self {
        MouseButtonPressedEvent {
            mouse_button_event: MouseButtonEvent { button },
        }
    }
}

pub struct MouseButtonReleasedEvent {
    mouse_button_event: MouseButtonEvent,
}

impl MouseButtonReleasedEvent {
    pub fn new(button: MouseButton) -> Self {
        MouseButtonReleasedEvent {
            mouse_button_event: MouseButtonEvent { button },
        }
    }
}
