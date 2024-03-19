use egui::{CentralPanel, Painter, Rect, Ui};
use log::info;

use super::{event::{DispatchesEvent, EventType}, layer::Layer};

pub struct Egui {
    ui: egui::Area,
    pub(self) ctx: egui::Context,
}

impl Egui {
    pub fn new() -> Self {
        let ctx = egui::Context::default();
        let layer_id = "my area";
        let ui = egui::Area::new(layer_id);
        Egui {
            ctx,
            ui,
        }
    }
}

impl DispatchesEvent for Egui {
    fn on_event(&mut self, e: &EventType) -> bool {
        info!("EGUI catches event");
        true
    }
}

impl Layer for Egui {
    fn on_attach(&self) {}
    fn on_update(&self) {
        let mut egui_input = egui::RawInput::default();
        self.ctx.begin_frame(egui_input.take());
        let _ = self.ctx.run(egui_input, |ctx| {
            egui::CentralPanel::default().show(&ctx, |ui| {
                ui.label("Hello egui!");
                if ui.button("click me").clicked() {info!("button clicked")}
            });
        });

        let _full_output = self.ctx.end_frame();
    }
    fn get_debug_name(&self) -> String {
        "Egui".to_string()
    }
}
