use futures::stream::BoxStream;
use shared::domain::image::{user::UserImage, ImageId, ImageKind};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

pub async fn create(pool: &PgPool, user_id: &Uuid, url_string: String, kind: ImageKind) -> sqlx::Result<ImageId> {
    let mut txn = pool.begin().await?;

    // If we can already find the image, return early.
    if let Some(record) = sqlx::query!(
            r#"
    select media_id,
    from web_media_library_url
    inner join web_media_library on id = media_id
    where media_url = $1"#,
       &url_string
    )
    .fetch_optional(pool.as_ref())
    .await?
   {
       log::trace!("Found the url");
       return Ok(record._id);
   }

    let id: ImageId = sqlx::query!(
        //language=SQL
        r#"
insert into web_media_library_url (user_id, media_url)
values ($1, $2)
returning id as "id: ImageId"
"#,
        user_id,
        url
    )
    .fetch_one(&mut txn)
    .await?
    .id;

    sqlx::query!("insert into web_media_upload (image_id) values ($1)", id.0)
        .execute(&mut txn)
        .await?;

    txn.commit().await?;

    Ok(id)
}

pub async fn get(db: &PgPool, user_id: Uuid, image_id: ImageId) -> sqlx::Result<Option<WebImage>> {
    sqlx::query_as!(
        UserImage,
        // language=SQL
        r#"
select id as "id: ImageId", kind as "kind: ImageKind"
from web_image_library
         inner join user_image_upload
                    on user_image_library.id = user_image_upload.image_id
where user_id = $1
  and id = $2
  and processing_result is true
        "#,
        user_id,
        image_id.0,
    )
    .fetch_optional(db)
    .await
}