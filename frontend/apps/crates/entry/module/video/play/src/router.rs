use super::state::{create_state, AppState};
use components::module::_common::play::entry::dom::render_page_body;
use dominator::clone;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::SignalExt;
use shared::domain::jig::ModuleKind;
use std::cell::RefCell;
use std::rc::Rc;
use utils::routes::{ModuleRoute, Route};

pub struct Router {
    loader: AsyncLoader,
    app: RefCell<Option<Rc<AppState>>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            loader: AsyncLoader::new(),
            app: RefCell::new(None),
        }
    }
}

pub fn render(state: Rc<Router>) {
    state.clone().loader.load(
        dominator::routing::url()
            .signal_ref(|url| Route::from_url(&url))
            .for_each(clone!(state => move |route| {
                match route {
                    Route::Module(route) => {
                        match route {
                            ModuleRoute::Play(kind, jig_id, module_id) => {
                                match kind {
                                    ModuleKind::Video => {
                                        let app = create_state(jig_id, module_id);
                                        render_page_body(app.clone());
                                        *state.app.borrow_mut() = Some(app);
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    },
                    _ => {}
                };
                async {}
            })),
    );
}
