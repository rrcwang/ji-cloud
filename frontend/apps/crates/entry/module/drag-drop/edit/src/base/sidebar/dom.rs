use components::module::_common::edit::prelude::*;
use dominator::{clone, html, Dom};
use std::rc::Rc;
use super::state::*;
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::drag_drop::Step;

use super::{
    step_1::{
        dom::render_step_1,
        state::Step1
    },
    step_2::{
        dom::render_step_2,
        state::Step2
    },
    step_3::dom::render_step_3,
    step_4::dom::render_step_4,
    step_5::{
        dom::render_step_5,
        state::Step5
    },
};

impl DomRenderable for Sidebar {
    fn render(state: Rc<Sidebar>) -> Dom {
        html!("empty-fragment", {
            .style("display", "contents")
            .child_signal(state.base.step.signal_cloned().map(clone!(state => move |step| {
                match step {
                    Step::One => Some(render_step_1(Step1::new(state.base.clone()))),
                    Step::Two => Some(render_step_2(Step2::new(state.base.clone()))),
                    Step::Three => Some(render_step_3()),
                    Step::Four => Some(render_step_4()),
                    Step::Five => Some(render_step_5(Step5::new(state.base.clone()))),
                    _ => None
                }
            })))
        })
    }
}
