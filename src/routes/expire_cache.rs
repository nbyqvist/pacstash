use actix_web::{
    http, post, web::Data, HttpResponseBuilder, Responder
};
use sqlx::SqlitePool;

use crate::{cached_package::purge_old_packages, error::ApplicationError, state::AppState};

#[post("/rpc/expire_cache")]
pub async fn purge_expired_packages(
    pool: Data<SqlitePool>,
    app_state: Data<AppState>,
) -> Result<impl Responder, ApplicationError> {
    let mut conn = pool.acquire().await?;
    purge_old_packages(&mut conn, app_state).await?;
    Ok(HttpResponseBuilder::new(http::StatusCode::OK).finish())
}
