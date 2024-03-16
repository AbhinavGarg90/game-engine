use engine_blue::engine::{application::Application, window::{linux::LinuxWindow, WindowType}};
use log::info;
mod sandbox;


fn main() 
{
    env_logger::init();
    let mut application = Application::new(WindowType::Linux);
    application.run();
}
