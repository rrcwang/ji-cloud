use components::module::edit::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;


impl DomRenderable for Overlay {
    fn render(state: Rc<Overlay>) -> Dom {
        html!("empty-fragment")
    }
}
