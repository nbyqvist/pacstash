use actix_web::web::Data;
use sqlx::{pool::PoolConnection, FromRow, Sqlite, SqliteConnection};

use crate::{disk_cache::{delete_cached_file, DiskCacheEntry}, state::AppState};

#[derive(FromRow)]
pub struct CachedPackage {
    pub id: i64,
    pub upstream_id: i64,
    pub repo: String,
    pub arch: String,
    pub filename: String,
    pub upstream_mirror_id: i64,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

pub fn cache_package_to_disk_entry(
    upstream_name: String,
    cached_package: CachedPackage,
) -> DiskCacheEntry {
    DiskCacheEntry {
        upstream_name,
        repo: cached_package.repo,
        arch: cached_package.arch,
        filename: cached_package.filename,
    }
}

pub struct CachedPackageIdentifier {
    pub upstream_id: i64,
    pub arch: String,
    pub repo: String,
    pub filename: String,
}

pub async fn create_cached_package(
    mut conn: PoolConnection<Sqlite>,
    req: CachedPackageIdentifier,
    mirror_id: i64,
) -> anyhow::Result<i64> {
    log::info!("Writing cache record for {}", req.filename);
    let id = sqlx::query!(
        "insert into cached_packages (upstream_id, upstream_mirror_id, repo, arch, filename) values (?, ?, ?, ?, ?) returning id",
        req.upstream_id,
        mirror_id,
        req.repo,
        req.arch,
        req.filename
    )
    .fetch_one(conn.as_mut()).await?.id;
    log::info!("Done!");
    Ok(id)
}

pub async fn find_cached_package(
    conn: &mut SqliteConnection,
    req: &CachedPackageIdentifier,
) -> anyhow::Result<Option<CachedPackage>> {
    let maybe_package = sqlx::query_as!(CachedPackage, "select id, upstream_id, repo, arch, filename, upstream_mirror_id, created_at, updated_at from cached_packages where upstream_id = ? and arch = ? and repo = ? and filename = ? limit 1", req.upstream_id, req.arch, req.repo, req.filename)
    .fetch_optional(conn).await?;

    Ok(maybe_package)
}

pub async fn purge_old_packages(
    conn: &mut SqliteConnection,
    app_state: Data<AppState>,
) -> anyhow::Result<()> {
    let app_state_inner = app_state.into_inner();
    let cache_root = app_state_inner.cache_root.clone();
    let max_age_sec = app_state_inner.pkg_max_age;
    let expired_pkgs = sqlx::query!(
        "select
            c.id, c.upstream_id, c.repo, c.arch, c.filename,
            c.upstream_mirror_id, c.created_at, c.updated_at,
            u.name
        from cached_packages c
        join upstreams u
            on u.id = c.upstream_id
        where c.created_at < (strftime('%s', 'now') - ?)",
        max_age_sec
    )
    .fetch_all(&mut *conn)
    .await?
    .into_iter()
    .map(|row| {
        (
            row.name,
            CachedPackage {
                id: row.id,
                upstream_id: row.upstream_id,
                repo: row.repo.clone(),
                arch: row.arch.clone(),
                filename: row.filename.clone(),
                upstream_mirror_id: row.upstream_mirror_id,
                created_at: row.created_at,
                updated_at: row.updated_at,
            },
        )
    }).collect::<Vec<_>>();

    log::info!("{} packages marked for deletion", expired_pkgs.len());

    for (upstream_name, cached_package) in expired_pkgs.iter() {
        let disk_entry = DiskCacheEntry {
            upstream_name: upstream_name.clone(),
            repo: cached_package.repo.clone(),
            arch: cached_package.arch.clone(),
            filename: cached_package.filename.clone(),
        };
        delete_cached_file(&cache_root, &disk_entry)?;
        sqlx::query!("delete from cached_packages where id = ?", cached_package.id).execute(&mut *conn).await?;
    }

    Ok(())
}
