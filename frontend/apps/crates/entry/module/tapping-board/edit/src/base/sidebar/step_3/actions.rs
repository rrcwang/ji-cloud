use super::state::*;
use utils::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use components::traces::bubble::TraceBubble;
use dominator::clone;

impl Step3 {

    pub fn start_preview(&self, index: usize) {

        let trace = self
            .base
            .traces
            .get(index)
            .unwrap_ji();

        let bounds = trace 
            .select_box
            .bounds
            .get()
            .unwrap_ji()
            .clone();

        let trace_meta = self
            .base
            .traces_meta
            .lock_ref()
            .get(index)
            .unwrap_ji()
            .clone();

        TraceBubble::set_unset_mutable(bounds, trace.audio.clone(), trace.text.clone(), trace_meta.bubble.clone());
    }
}
