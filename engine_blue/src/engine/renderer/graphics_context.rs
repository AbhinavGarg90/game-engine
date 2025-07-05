pub mod opengl;

pub(crate) trait GraphicsContext {
    fn init(&mut self);
    fn swap_buffers(&mut self);
}
