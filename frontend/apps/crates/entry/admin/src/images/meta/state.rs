use shared::domain::{
    meta::{ImageStyleId, AffiliationId, AgeRangeId},
    category::CategoryId,
    image::{ImageId, ImageSearchQuery, ImageMetadata},
};
use futures_signals::signal::Mutable;
use dominator_helpers::futures::AsyncLoader;
use std::collections::HashSet;
use web_sys::HtmlInputElement;
use std::cell::RefCell;
use components::image::tag::ImageTag;

pub struct State {
    pub id: ImageId, 
    pub section: Mutable<Section>,
    pub loader: AsyncLoader,
    pub loaded: Mutable<bool>,
    pub delete_modal: Mutable<bool>,
    pub file_input: RefCell<Option<HtmlInputElement>>,
}

impl State {
    pub fn new(id: ImageId, is_new: bool) -> Self {

        let section = {
            if is_new 
                { Section::General}
            else
                { Section::Summary}
        };

        Self {
            id,
            section: Mutable::new(section),
            loader: AsyncLoader::new(),
            loaded: Mutable::new(false),
            delete_modal: Mutable::new(false),
            file_input: RefCell::new(None),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Section {
    General,
    Categories,
    Summary 
}

impl Section {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::General => "general",
            Self::Categories => "categories",
            Self::Summary => "summary",
        }
    }
}

impl From<&str> for Section {
    fn from(value: &str) -> Self { 
        match value {
            "general" => Self::General,
            "categories" => Self::Categories,
            "summary" => Self::Summary,
            _ => panic!("unknown value!")
        }
    }
}

#[derive(Clone)]
pub struct MutableImage {
    pub id: Mutable<ImageId>,
    pub name: Mutable<String>,
    pub description: Mutable<String>,
    pub is_premium: Mutable<bool>,
    pub styles: Mutable<HashSet<ImageStyleId>>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub tag_indices: Mutable<HashSet<i16>>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub orig: ImageMetadata,
}

impl From<ImageMetadata> for MutableImage {
    fn from(image:ImageMetadata) -> Self {

        let orig = image.clone();

        Self {
            id: Mutable::new(image.id),
            name: Mutable::new(image.name),
            description: Mutable::new(image.description),
            is_premium: Mutable::new(image.is_premium),
            styles: {
                let mut styles = HashSet::with_capacity(image.styles.len());
                for id in image.styles.into_iter() {
                    styles.insert(id);
                }
                Mutable::new(styles)
            },
            age_ranges: {
                let mut age_ranges = HashSet::with_capacity(image.age_ranges.len());
                for id in image.age_ranges.into_iter() {
                    age_ranges.insert(id);
                }
                Mutable::new(age_ranges)
            },
            affiliations: {
                let mut affiliations = HashSet::with_capacity(image.affiliations.len());
                for id in image.affiliations.into_iter() {
                    affiliations.insert(id);
                }
                Mutable::new(affiliations)
            },
            tag_indices: {
                let mut tag_indices = HashSet::with_capacity(image.tags.len());
                for id in image.tags.into_iter() {
                    tag_indices.insert(id.0);
                }
                Mutable::new(tag_indices)
            },
            categories: {
                let mut categories = HashSet::with_capacity(image.categories.len());
                for id in image.categories.into_iter() {
                    categories.insert(id);
                }
                Mutable::new(categories)
            },

            orig
        }
    }
}
