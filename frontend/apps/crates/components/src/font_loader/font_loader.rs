/*
 * FontLoader is intended to be created and then held at a top-level
 * though it's okay to call the loader with the same font multiple times, it'll just skip it the second
 *
 * error handling is non-existant atm
 */

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use std::collections::HashSet;
use futures::stream::{FuturesUnordered, StreamExt};
use wasm_bindgen_futures::JsFuture;
use utils::{path, prelude::*};

const LOAD_BATCH_SIZE:usize = 10;

const ALL_FONTS:[Font;5] = [
    Font::ShesekRegular,
    Font::FrankRuhlMedium,
    Font::ArchitectsDaughterRegular,
    Font::RobotoSlabRegular,
    Font::CaveatMedium,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Font {
    ShesekRegular,
    FrankRuhlMedium,
    ArchitectsDaughterRegular,
    RobotoSlabRegular,
    CaveatMedium,
}

struct LoaderInfo {
    pub name: &'static str,
    pub format: &'static str,
    pub url: String,
    pub unicode_range: Option<&'static str>,
}

impl Font {
    fn get_loader_info(self) -> LoaderInfo {
        let (name, filepath, format) = match self {
            Self::ShesekRegular => ("Shesek - Regular", "shesek-regular-fm.woff2", "woff2"),
            Self::FrankRuhlMedium => ("Frank Ruhl Libre - Medium", "Frank_Ruhl_Libre/FrankRuhlLibre-Medium.ttf", "truetype"),
            Self::ArchitectsDaughterRegular => ("Architects Daughter - Regular", "Architects_Daughter/ArchitectsDaughter-Regular.ttf", "truetype"),
            Self::RobotoSlabRegular => ("Roboto Slab - Regular", "Roboto_Slab/static/RobotoSlab-Regular.ttf", "truetype"),
            Self::CaveatMedium=> ("Caveat - Medium", "Caveat/static/Caveat-Medium.ttf", "truetype"),
        };

        let unicode_range = match self {
            //Hebrew
            Self::ShesekRegular | Self::FrankRuhlMedium => Some("U+0590-05FF, U+FB1D-FB4F"),
            _ => None
        };

        LoaderInfo {
            name,
            url: path::ui(&format!("fonts/{}", filepath)),
            format, 
            unicode_range
        }
    }
}

#[wasm_bindgen(inline_js=r#"
export function add_font(name, url, format, unicode_range) {
    let descriptors = {};
    if(unicode_range && unicode_range != "") {
        descriptors.unicodeRange = unicode_range;
    }
    const face = new FontFace(name, `url(${url}) format('${format}')`, descriptors);
    document.fonts.add(face);
}

export function fonts_ready() {
    return document.fonts.ready.then(() => console.log("fonts are ready!"));
}
"#)]

extern "C" {
    fn add_font(name: &str, url:String, format:&str, unicode_range: &str);
    fn fonts_ready() -> Promise;
}

pub struct FontLoader {
    has_queued: HashSet<Font>
}

impl FontLoader {
    pub fn new() -> Self {
        Self {
            has_queued: HashSet::new()
        }
    }

    pub async fn load_all(&mut self) {
        self.load(&ALL_FONTS).await;
    }

    pub async fn load(&mut self, fonts:&[Font]) {

        let mut fonts:Vec<Font> = fonts
            .iter()
            .filter(|font| !self.has_queued.contains(&font))
            .map(|font| *font).collect();

        for font in fonts.iter() {
            self.has_queued.insert(*font);
            let LoaderInfo {name, url, unicode_range, format}  = font.get_loader_info();
            add_font(name, url, format, unicode_range.unwrap_or_default());
        }

        JsFuture::from(fonts_ready()).await;

    }
}
