use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};

use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        image::{
            web::{WebImage, WebImageResponse},
            ImageId,
        },
        CreateResponse,
    },
    media::{FileKind, MediaLibrary, PngImageFile},
};

use sqlx::PgPool;

use crate::{
    db, error,
    extractor::{RequestOrigin, TokenUser},
    service::{s3, storage, GcpAccessKeyStore, ServiceData},
};

/// Create an image from a media url
pub(super) async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    query: Json<<endpoints::image::web::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::Tag> {
    let url = request.into_inner().url;

    const MAX_RESPONSE_SIZE: usize = max(ANIMATION_BODY_SIZE_LIMIT, IMAGE_BODY_SIZE_LIMIT);

    let url_string = url.to_string();

    let id = db::image::web::create(&db, &claims.user_id, url_string);


}


    let client: reqwest::Client = reqwest::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // todo: this `?` should be a ClientError or "proxy/gateway error"
    let mut response: reqwest::Response = client.get(url).send().await?;

    let mut data = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        let chunk: Bytes = chunk;
        if data.len() + chunk.len() < MAX_RESPONSE_SIZE {
            data.extend_from_slice(&chunk[..]);
        } else {
            return Err(anyhow::anyhow!("todo: better error here (data too big)").into());
        }
    }

    log::trace!("data was {} bytes long", data.len());

    let mut hasher = sha2::Sha384::new();

    hasher.update(&data);

    let hash = hasher.finalize().to_vec();

    let mut txn = pool.begin().await?;

    // If we can find the image by hash, return early.

    let record = sqlx::query!(
        r#"
select id,
       kind as "kind: MediaKind"
from web_media_library
where hash = $1
for update
"#,
        &hash
    )
    .fetch_optional(&mut txn)
    .await?;

    if let Some(record) = record {
        let id = record.id;
        sqlx::query!(
            "insert into web_media_library_url (media_id, media_url) values ($1, $2) on conflict (media_id, media_url) do nothing",
            id,
            &url_string
        )
        .execute(&mut txn)
        .await?;

        txn.commit().await?;

        log::trace!("Found the hash");

        return Ok(HttpResponse::Created().json(UrlCreatedResponse {
            id,
            kind: record.kind.to_shared(),
        }));
    }

    let data = Arc::new(data);

    let kind = actix_web::web::block({
        let data = data.clone();
        move || crate::image_ops::detect_image_kind(&data)

}
