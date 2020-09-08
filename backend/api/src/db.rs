pub(crate) mod category;
pub(crate) mod image;
pub(crate) mod user;
pub(crate) mod meta;

use config::DB_POOL_CONNECTIONS;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

pub async fn get_pool(connect_options: PgConnectOptions) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(DB_POOL_CONNECTIONS)
        .connect_with(connect_options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
