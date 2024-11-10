use sqlx::{pool::PoolConnection, Sqlite, SqliteConnection};

use crate::disk_cache::DiskCacheEntry;

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

pub fn cache_package_to_disk_entry(upstream_name: String, cached_package: CachedPackage) -> DiskCacheEntry {
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
    let maybe_package = sqlx::query!("select id, upstream_id, repo, arch, filename, upstream_mirror_id, created_at, updated_at from cached_packages where upstream_id = ? and arch = ? and repo = ? and filename = ? limit 1", req.upstream_id, req.arch, req.repo, req.filename)
    .fetch_optional(conn).await?.map(|row| CachedPackage {
        id: row.id,
        upstream_id: row.upstream_id,
        arch: row.arch,
        repo: row.repo,
        filename: row.filename,
        upstream_mirror_id: row.upstream_mirror_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
    });

    Ok(maybe_package)
}
