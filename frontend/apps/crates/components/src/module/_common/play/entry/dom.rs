use std::rc::Rc;

use futures_signals::{
    signal::{Mutable, SignalExt},
};

use dominator::{clone, events, html, Dom};

use utils::{events::ModuleResizeEvent, iframe::*, prelude::*, resize::*};

use super::{loading::dom::render_loading, state::*};
use crate::{
    instructions::player::{dom::render_instructions_player, state::InstructionsPlayer},
    module::_common::play::prelude::*,
};
use shared::domain::jig::module::body::{BodyExt, ModeExt, StepExt};

pub fn render_page_body<RawData, Mode, Step, Base>(
    state: Rc<GenericState<RawData, Mode, Step, Base>>,
) where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    let sig =
            state.phase.signal_cloned().map(clone!(state => move |phase| {
                let page_kind = match phase.as_ref() {
                    Phase::Loading(_) | Phase::WaitingIframeRaw(_) => ModulePageKind::GridPlain,
                    Phase::Ready(_) => ModulePageKind::Iframe
                };

                let has_resized_once = Mutable::new(!page_kind.is_resize());

                html!(page_kind.element_name(), {
                        .apply_if(page_kind.add_scrollable_attribute(), |dom| {
                            dom.property("scrollable", true)
                        })
                        .apply_if(page_kind.add_preview_attribute(), |dom| {
                            dom.property("preview", true)
                        })
                        .event(clone!(has_resized_once => move |event:ModuleResizeEvent| {
                            //in utils / global static
                            set_resize_info(event.data());
                            has_resized_once.set_neq(true);
                        }))
                        .children_signal_vec({
                            has_resized_once.signal()
                                .map(clone!(state, phase => move |has_resized_once| {
                                    if !has_resized_once {
                                        vec![]
                                    } else {
                                        match phase.as_ref() {
                                            Phase::Loading(_) => {
                                                vec![render_loading(state.clone())]
                                            },
                                            Phase::WaitingIframeRaw(on_raw) => {
                                                vec![render_iframe_wait_raw(state.clone(), on_raw.clone())]
                                            },
                                            Phase::Ready(ready) => {
                                                vec![render_player(state.clone(), ready.base.clone(), ready.jig_player, ready.play_started.clone())]
                                            },
                                        }
                                    }
                                }))
                                .to_signal_vec()
                        })
                })
            }));

    state.page_body_switcher.load(sig.for_each(|dom| {
        let body = dominator::body();
        body.set_inner_html("");
        dominator::append_dom(&body, dom);
        async move {}
    }));
}

//This is just a placeholder to get messages
//It'll be replaced when the iframe data arrives
fn render_iframe_wait_raw<RawData, Mode, Step, Base>(
    state: Rc<GenericState<RawData, Mode, Step, Base>>,
    on_raw: Rc<Box<dyn Fn(RawData)>>,
) -> Dom
where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    html!("empty-fragment", {
        .global_event(clone!(state, on_raw => move |evt:dominator_helpers::events::Message| {
            if let Ok(msg) = evt.try_serde_data::<IframeInit<RawData>>() {
                log::info!("got iframe data!");
                //on_raw was stashed from the original State::new()
                on_raw(msg.data);
            } else {
                log::info!("hmmm got other iframe message...");
            }
        }))
        .after_inserted(clone!(state => move |_elem| {
            let parent = web_sys::window()
                .unwrap_ji()
                .parent()
                .unwrap_ji()
                .unwrap_ji();
            //On mount - send an empty IframeInit message to let the parent know we're ready
            let msg = IframeInit::empty();

            let _ = parent.post_message(&msg.into(), "*");
        }))
    })
}

fn render_player<RawData, Mode, Step, Base>(
    state: Rc<GenericState<RawData, Mode, Step, Base>>,
    base: Rc<Base>,
    jig_player: bool,
    play_started: Mutable<bool>,
) -> Dom
where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    let instructions = base.get_instructions();
    let is_screenshot = utils::screenshot::is_screenshot_url();

    html!("empty-fragment", {
        .property("slot", "main")
        .child(Base::render(base.clone()))
        .apply_if(instructions.is_some() && !is_screenshot, clone!(state, base, play_started => move |dom| {
            dom
                .child_signal(play_started.signal().map(clone!(state, base => move |has_started| {
                    if has_started {
                        Some(render_instructions_player(
                            Rc::new(InstructionsPlayer::new(base.get_instructions().unwrap_ji())),
                            &state.audio_mixer
                        ))
                    } else {
                        None
                    }
                })))
        }))

        .apply_if(jig_player, |dom| {
            dom
                .global_event(clone!(state, base => move |evt:dominator_helpers::events::Message| {
                    if let Ok(msg) = evt.try_serde_data::<IframeAction<JigToModuleMessage>>() {
                        match msg.data {
                            JigToModuleMessage::Play => {
                                state.audio_mixer.play_all();
                            },
                            JigToModuleMessage::Pause => {
                                state.audio_mixer.pause_all();
                            },
                            JigToModuleMessage::TimerDone => {
                            }
                        }
                    } else {
                        log::info!("hmmm got other iframe message...");
                    }
                }))
                .after_inserted(clone!(state => move |_elem| {
                    let parent = web_sys::window()
                        .unwrap_ji()
                        .parent()
                        .unwrap_ji()
                        .unwrap_ji();
                    //On mount - send an empty IframeInit message to let the parent know we're ready
                    let msg = IframeInit::empty();

                    let _ = parent.post_message(&msg.into(), "*");
                }))
        })

        .apply_if(!is_screenshot, |dom| {
            dom.child_signal(play_started.signal().map(clone!(play_started => move |has_started| {
                if !has_started {
                    Some(html!("module-play-button", {
                        .event(clone!(base, play_started => move |_evt:events::Click| {
                            start_playback(base.clone(), &play_started);
                        }))
                        .after_inserted(clone!(state, base, play_started => move |_elem| {
                            if state.opts.skip_play {
                                start_playback(base.clone(), &play_started);
                            }
                        }))
                    }))
                } else {

                    if jig_player {
                        let parent = web_sys::window()
                            .unwrap_ji()
                            .parent()
                            .unwrap_ji()
                            .unwrap_ji();
                        let msg = IframeAction::new(ModuleToJigMessage::Start(base.get_timer_seconds()));
                        let _ = parent.post_message(&msg.into(), "*");
                    }
                    None
                }
            })))
        })
    })
}

fn start_playback<Base>(base: Rc<Base>, play_started: &Mutable<bool>)
where
    Base: BaseExt + 'static,
{
    play_started.set_neq(true);
    Base::play(base);
}
