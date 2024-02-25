use engine_blue::engine::application::Application;

pub struct Sandbox;

impl Sandbox {
    pub fn new() -> Self {
        Sandbox {}
    }
}
impl Application for Sandbox {}
