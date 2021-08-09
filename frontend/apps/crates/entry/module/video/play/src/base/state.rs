use components::{audio_mixer::AudioMixer, module::_common::play::prelude::*};
use shared::domain::jig::{
    module::{
        body::{
            Instructions,
            _groups::design::{Backgrounds, Sticker},
            video::{Mode, ModuleData as RawData, PlaySettings, Step},
        },
        ModuleId,
    },
    Jig, JigId,
};
use std::rc::Rc;
use utils::prelude::*;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: Jig,
    pub theme_id: ThemeId,
    pub audio_mixer: AudioMixer,
    pub instructions: Instructions,
    pub backgrounds: Backgrounds,
    pub stickers: Vec<Sticker>,
    pub play_settings: PlaySettings,
}

impl Base {
    pub async fn new(init_args: InitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let InitFromRawArgs {
            jig_id,
            module_id,
            audio_mixer,
            jig,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();
        let base_content = content.base;

        Rc::new(Self {
            jig_id,
            module_id,
            jig,
            theme_id,
            audio_mixer,
            instructions: base_content.instructions,
            backgrounds: base_content.backgrounds,
            stickers: base_content.stickers,
            play_settings: content.play_settings,
        })
    }
}

impl BaseExt for Base {
    fn get_instructions(&self) -> Option<Instructions> {
        Some(self.instructions.clone())
    }
}
