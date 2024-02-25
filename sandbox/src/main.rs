use engine_blue::engine::{application::Application, window::linux::LinuxWindow};
mod sandbox;


fn main() 
{
    env_logger::init();
    let mut application = Application::new::<LinuxWindow>();
    application.run();
}
