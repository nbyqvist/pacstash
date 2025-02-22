use actix_web::{
     get, web::{Data, Path}, HttpResponse, Responder
};
use serde_json::json;
use sqlx::SqlitePool;

use crate::{error::ApplicationError, upstream::find_upstream_by_name, upstream_mirror::UpstreamMirror};

#[get("/api/mirrors/{upstream_name}")]
pub async fn get_mirrors_for_upstream(
    path: Path<String>,
    pool: Data<SqlitePool>,
) -> Result<impl Responder, ApplicationError> {
    let upstream_name = path.into_inner();
    let mut conn = pool.acquire().await?;
    let maybe_upstream = find_upstream_by_name(&mut conn, &upstream_name).await?;
    let Some(upstream) = maybe_upstream else {
        return Ok(HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": format!("upstream with name {} not found", upstream_name),
        })));
    };

    let mirrors = sqlx::query_as!(
        UpstreamMirror,
        "select
            um.id, um.upstream_id, um.url, um.created_at, um.updated_at
        from upstream_mirrors um
        where um.upstream_id = ?",
        upstream.id
    ).fetch_all(conn.as_mut())
    .await?;
    Ok(HttpResponse::Ok().json(mirrors))
}
