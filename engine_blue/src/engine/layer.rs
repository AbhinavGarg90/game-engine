use super::event::Event;


pub trait Layer {
    fn on_attach(&self) {}
    fn on_detach(&self) {}
    fn on_update(&self) {}
    fn on_event(&self, event: &Event) {}
    fn get_debug_name(&self) -> String {
        "sample layer".to_string()
    }
}