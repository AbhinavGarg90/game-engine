use engine_blue::engine::application::Application;
mod sandbox;


fn main() 
{
    env_logger::init();
    let s = sandbox::Sandbox::new();
    s.run();
}
