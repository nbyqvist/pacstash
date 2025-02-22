use actix_web::{get, web::{Data, Path}, Responder};
use sqlx::SqlitePool;
use crate::{cached_package::CachedPackageFull, templates::RepoViewTemplate};

#[get("/repo/{repo_name}")]
pub async fn view_repo_page(path: Path<String>, pool: Data<SqlitePool>) -> impl Responder {
    let repo_name = path.into_inner();
    let conn_res = pool.acquire().await;
    if conn_res.is_err() {
        return RepoViewTemplate {
            repo_name,
            packages: Err("Failed to acquire connection".to_string()),
        };
    }
    let mut conn = conn_res.unwrap();
    let packages = sqlx::query_as!(
        CachedPackageFull,
        "
        select
            c.id,
            c.upstream_id,
            c.repo,
            c.arch,
            c.filename,
            c.upstream_mirror_id,
            c.download_count,
            c.last_downloaded_at,
            c.created_at,
            c.updated_at
        from upstreams u
        join cached_packages c
            on c.upstream_id = u.id
        where u.name = ?
        order by c.repo, c.arch, c.filename
        ",
        repo_name
    )
    .fetch_all(conn.as_mut())
    .await;
    RepoViewTemplate {
        repo_name,
        packages: packages.map_err(|e| e.to_string()),
    }
}
