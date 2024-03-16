use super::event::{DispatchesEvent};



pub trait Layer: DispatchesEvent {
    fn on_attach(&self) {}
    fn on_detach(&self) {}
    fn on_update(&self) {}
    fn get_debug_name(&self) -> String {
        "sample layer".to_string()
    }
}