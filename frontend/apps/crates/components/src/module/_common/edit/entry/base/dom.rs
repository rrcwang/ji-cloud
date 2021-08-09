mod preview;
use preview::*;

mod regular;
use regular::*;

use super::state::*;
use dominator::Dom;
use std::rc::Rc;

use shared::domain::jig::{
    module::{
        body::{BodyExt, ModeExt, StepExt},
        ModuleId,
    },
    JigId,
};

pub fn render<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    is_preview: bool,
    jig_id: JigId,
    module_id: ModuleId,
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
) -> Vec<Dom>
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    if is_preview {
        vec![
            render_preview_header(RawData::kind(), state.clone()),
            render_preview_main(RawData::kind(), jig_id, module_id, state.clone()),
            render_preview_overlay(RawData::kind(), jig_id, module_id, state.clone()),
        ]
    } else {
        vec![
            render_main_bg(state.clone()),
            render_main(state.clone()),
            render_sidebar(state.clone()),
            render_header(state.clone()),
            render_footer(state.clone()),
            render_overlay(state.clone()),
        ]
    }
}
