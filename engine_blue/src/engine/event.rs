use std::mem::transmute;

pub mod applicationevent;
pub mod keyevent;
pub mod mousevent;

#[derive(Debug)]
pub struct Event {
    //convert eventype to enum later
    event_type: EventType,
}

// convert to enum of structs in the future?
#[derive(Debug, PartialEq)]
pub enum EventType {
    None = 0,
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

pub(crate) trait StaticEventType {
    fn get_static_type(&self) -> EventType;
}

impl EventType {
    fn to_string(&self) -> String {
        match self {
            EventType::None => "None".to_string(),
            EventType::WindowClose => "WindowClose".to_string(),
            EventType::WindowResize => "WindowResize".to_string(),
            EventType::WindowFocus => "WindowFocus".to_string(),
            EventType::WindowLostFocus => "WindowLostFocus".to_string(),
            EventType::WindowMoved => "WindowMoved".to_string(),
            EventType::AppTick => "AppTick".to_string(),
            EventType::AppUpdate => "AppUpdate".to_string(),
            EventType::AppRender => "AppRender".to_string(),
            EventType::KeyPressed => "KeyPressed".to_string(),
            EventType::KeyReleased => "KeyReleased".to_string(),
            EventType::MouseButtonPressed => "MouseButtonPressed".to_string(),
            EventType::MouseButtonReleased => "MouseButtonReleased".to_string(),
            EventType::MouseMoved => "MouseMoved".to_string(),
            EventType::MouseScrolled => "MouseScrolled".to_string(),
        }
    }
}

// TODO
pub enum EventCategory {}

impl Event {
    pub fn get_event_type(&self) -> &EventType {
        &self.event_type
    }
    // fn get_category_name() -> i32; // TODO: implement based on relevancy
}

pub struct EventDispatcher<'a> {
    event_ptr: Box<&'a dyn StaticEventType>
}

type EventFn<T: StaticEventType> = fn(&T) -> bool;

impl<'a> EventDispatcher<'a> {
    pub fn new(event: &'a dyn StaticEventType) -> EventDispatcher<'a> {
        EventDispatcher { event_ptr: Box::new(event) }
    }

    pub fn dispatch<T: StaticEventType + Default>(&self, func: EventFn<T>) -> bool {
        if T::get_static_type(&T::default()) == self.event_ptr.get_static_type() {
            unsafe {func(transmute::<Box<&'a dyn StaticEventType>, &T>(self.event_ptr))};
            return true;
        }
        false
    }

    fn get_category_flags() -> i32 {
        todo!()
    }
}

#[macro_export]
macro_rules! impl_new_functions {
    ($($struct_name:ident, $struct_enum:ident),*) => {
        $(
            impl Default for $struct_name {
                fn default() -> Self {
                    $struct_name { event: Event {
                        event_type: EventType::$struct_enum,
                    } }
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_get_static_type {
	($($struct_name:ident, $enum_name:ident),*) => {
	$(
		impl StaticEventType for $struct_name {
			fn get_static_type(&self) -> EventType {
                EventType::$enum_name
			}
		}
	)*
	};
}
