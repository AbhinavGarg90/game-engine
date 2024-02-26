use super::layer::{self, Layer};
use std::vec;

pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
}

impl LayerStack {
    fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }
    fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layers.insert(0, overlay)
    }
    fn pop_layer(&mut self, layer: Box<dyn Layer>) {
            match self.layers
                .iter()
                .position(|target| *target.get_debug_name() == layer.get_debug_name()) {
                    Some(i) => {self.layers.remove(i);},
                    None =>  {},
                };
    }
    fn pop_overlay(&mut self, overlay: &impl Layer) {
            match self.layers
                .iter()
                .position(|target| *target.get_debug_name() == overlay.get_debug_name()) {
                    Some(i) => {self.layers.remove(i);},
                    None =>  {},
                };
    }

}
