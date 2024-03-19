use super::layer::{self, Layer};
use std::{iter::Rev, vec};

pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
}

impl Default for LayerStack {
    fn default() -> Self {
        LayerStack { layers: Vec::new() }
    }
}

impl LayerStack {
    #[inline(always)]
    pub fn new() -> Self {
        LayerStack::default()
    }
    pub fn get_end(&mut self) -> Rev<std::slice::IterMut<'_, Box<(dyn Layer + 'static)>>> {
        self.layers.iter_mut().rev()
    }
    pub fn get_begin(&mut self) -> std::slice::IterMut<'_, Box<(dyn Layer + 'static)>> {
        self.layers.iter_mut()
    }
    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.insert(0, layer)
    }
    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layers.push(overlay);
    }
    pub fn pop_layer(&mut self, layer: Box<dyn Layer>) {
        match self
            .layers
            .iter()
            .position(|target| *target.get_debug_name() == layer.get_debug_name())
        {
            Some(i) => {
                self.layers.remove(i);
            }
            None => {}
        };
    }
    pub fn pop_overlay(&mut self, overlay: &impl Layer) {
        match self
            .layers
            .iter()
            .position(|target| *target.get_debug_name() == overlay.get_debug_name())
        {
            Some(i) => {
                self.layers.remove(i);
            }
            None => {}
        };
    }
}
