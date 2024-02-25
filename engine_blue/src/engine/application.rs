use log::info;

use crate::engine::event::applicationevent::WindowResizeEvent;


pub trait Application
{
  fn run(&self) {
    info!("started running application");
    let e = WindowResizeEvent::new(40, 40);
    dbg!(e);
  }
}