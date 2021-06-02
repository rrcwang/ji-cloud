use shared::domain::jig::{JigId, Jig, module::{ModuleId, body::tapping_board::{Mode as RawMode, ModuleData as RawData}}};
use components::module::play::state::MainExt;
use utils::prelude::*;

pub struct Main {
    pub text: String,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: Option<Jig>,
}

impl Main {
    pub async fn new(jig_id: JigId, module_id: ModuleId, jig: Option<Jig>, raw:RawData, ) -> Self {

        Self {
            jig_id,
            module_id,
            jig,
            text: serde_json::to_string(&raw).unwrap_ji()
        }
    }
}

impl MainExt for Main {
}
