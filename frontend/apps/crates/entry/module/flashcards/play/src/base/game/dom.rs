use dominator::{html, Dom, DomBuilder, clone};
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;
use std::rc::Rc;
use super::state::*;
use components::module::_groups::cards::{
    lookup::{self, Side},
    play::card::dom::{render_card, render_card_mixin, CardOptions, Size},
    edit::{
        config,
        state::*
    },
};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, ReadOnlyMutable}
};

use shared::domain::jig::module::body::{
    ThemeId,
    ModeExt,
    _groups::cards::{Mode, Step, Card},
    flashcards::DisplayMode,
};
use rand::prelude::*;

use utils::prelude::*;

pub fn render(state: Rc<Game>) -> Dom {
    html!("flashcards-main", {
        .property("slot", "main")
        .children_signal_vec(
            state.current.signal_cloned()
                .map(clone!(state => move |current| {
                    let mut children:Vec<Dom> = Vec::new();

                    let theme_id = state.base.theme_id.clone();
                    let mode = state.base.mode.clone();


                    let Current { card, other, side } = current;

                    if state.base.settings.display_mode == DisplayMode::Single {

                        let mut options = CardOptions::new(&card, theme_id, mode, side, Size::Flashcards);
                        options.back_card = Some(&other);


                        children.push(render_card_mixin(options, flip_controller(state.clone(), true)));

                    } else {
                        let mut options = CardOptions::new(&card, theme_id, mode, side, Size::Flashcards);
                        options.flipped = true;

                        children.push(render_card(options));

                        let mut options = CardOptions::new(&other, theme_id, mode, side.negate(), Size::Flashcards);

                        children.push(render_card_mixin(options, flip_controller(state.clone(), false)));
                    }

                    children
                }))
                .to_signal_vec()
        )
        .child(html!("button-icon", {
            .property("icon", "white-circle-blue-arrow")
            .property("slot", "next")
            .event(clone!(state => move |evt:events::Click| {
                state.next();
            }))
        }))
    })
}

fn flip_controller(state: Rc<Game>, initial: bool) -> impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    move |dom| {
        dom
            .property_signal("flipped", state.gate.signal().map(move |gate| {
                if gate == Gate::Waiting || gate == Gate::FinishingFlip {
                    initial
                } else {
                    !initial
                }
            }))
            .event(clone!(state => move |evt:events::Click| {
                Game::flip(state.clone());
            }))
    }
}
