use shared::{
    api::endpoints::{ApiEndpoint, image::meta},
    domain::image::*,
    error::image::*
};
use core::{
    path::api_url,
    fetch::{api_with_auth, api_with_auth_empty, FetchResult, upload_file}
};
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use url::Url;
use web_sys::File;

#[derive(Debug, Clone)]
pub struct MetaOptions {
    pub styles: Vec<(String, String)>,
    pub age_ranges: Vec<(String, String)>,
    pub affiliations: Vec<(String, String)>,
}

impl MetaOptions {
    pub async fn load() -> Result<Self, ()> {
        _load_meta_options().await
            .map_err(|err| {
                log::error!("{:?}", err);
                ()
            })
            .map(|res| {
                Self {
                    styles: 
                        vec![
                            ("1".to_string(), "style 1".to_string()),
                            ("2".to_string(), "style 2".to_string()),
                        ],

                        /*res.styles
                            .into_iter()
                            .map(|style| {
                                let label = "LABEL HERE".to_string();
                                let id = style.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                            */
                    age_ranges: 
                        vec![
                            ("1".to_string(), "age 1".to_string()),
                            ("2".to_string(), "age 2".to_string()),
                        ],
                        /*
                        res.age_ranges
                            .into_iter()
                            .map(|age_range| {
                                let label = "LABEL HERE".to_string();
                                let id = age_range.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                            */
                    affiliations: 
                        vec![
                            ("1".to_string(), "affiliation 1".to_string()),
                            ("2".to_string(), "affiliation 2".to_string()),
                        ],
                        /*
                        res.affiliations
                            .into_iter()
                            .map(|affiliation| {
                                let label = "LABEL HERE".to_string();
                                let id = affiliation.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                            */
                }
            })
    }
}

async fn _load_meta_options() -> FetchResult < <meta::Get as ApiEndpoint>::Res, <meta::Get as ApiEndpoint>::Err> {
    log::info!("{}", api_url(meta::Get::PATH));
    api_with_auth::<_, _, ()>(&api_url(meta::Get::PATH), meta::Get::METHOD, None).await
}
