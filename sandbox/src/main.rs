use engine_blue::engine::{application::Application, event::{DispatchesEvent, EventType}, layer::Layer, window::{linux::LinuxWindow, WindowType}};
use log::info;
mod sandbox;

#[derive(Debug)]
struct ExampleLayer {}
impl DispatchesEvent for ExampleLayer {
    fn on_event(&mut self, e: &EventType) -> bool {
        info!("example layer catches event");
        false
    }
}
impl Layer for ExampleLayer {
}

fn main() 
{
    env_logger::init();
    let mut application = Application::new(WindowType::Linux);
    let layer = ExampleLayer {};
    application.push_layer(Box::new(layer));
    application.run();
}
