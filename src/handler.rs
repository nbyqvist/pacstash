use std::fs;

use actix_web::{
    get,
    http::{self, header},
    post,
    web::{Data, Path},
    HttpResponseBuilder, Responder,
};
use rand::{seq::SliceRandom, thread_rng};
use sqlx::SqlitePool;

use crate::{
    cached_package::{
        cache_package_to_disk_entry, create_cached_package, find_cached_package,
        purge_old_packages, CachedPackageIdentifier,
    },
    disk_cache::{path_of_cached_package, write_cached_file, DiskCacheEntry},
    error::ApplicationError,
    fetch::fetch_package,
    response::AppResponse,
    state::AppState,
    statistics::{fetch_statistics, StatisticsTemplate},
    upstream::find_upstream_by_name,
    upstream_mirror::{get_mirrors_for_upstream_id, should_cache_file},
};

#[get("/")]
pub async fn statistics_page(pool: Data<SqlitePool>) -> impl Responder {
    let mut conn = pool.acquire().await.ok().unwrap();
    let stats = fetch_statistics(&mut conn).await.ok().unwrap();
    StatisticsTemplate { stats }
}

#[get("/u/{upstream_name}/{repo}/{arch}/{filename}")]
pub async fn caching_package_endpoint(
    path: Path<(String, String, String, String)>,
    pool: Data<SqlitePool>,
    app_state: Data<AppState>,
) -> AppResponse {
    let cache_root = app_state.into_inner().cache_root.clone();
    let (upstream_name, repo, arch, filename) = path.into_inner();
    let mut conn = pool.acquire().await?;
    let maybe_upstream = find_upstream_by_name(&mut conn, &upstream_name).await?;
    let Some(upstream) = maybe_upstream else {
        log::info!("Missing upstream {}", upstream_name);
        return Err(ApplicationError::General(anyhow::anyhow!(format!(
            "Upstream with name {} does not exist",
            upstream_name.clone()
        ))));
    };

    if !should_cache_file(&filename) {
        log::info!("Not caching filename {}", filename);
        let mut mirrors = get_mirrors_for_upstream_id(&mut conn, upstream.id).await?;
        let mut rand = thread_rng();
        mirrors.shuffle(&mut rand);
        let file = fetch_package(mirrors, &arch, &repo, &filename).await?;
        // Todo: use file.tried_mirrors and file.mirror_id for status
        return Ok(HttpResponseBuilder::new(http::StatusCode::OK)
            .insert_header((
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/octet-stream"),
            ))
            .body(file.package));
    }

    let package_ident = CachedPackageIdentifier {
        upstream_id: upstream.id,
        arch: arch.clone(),
        repo: repo.clone(),
        filename: filename.clone(),
    };
    // We should cache this file
    let maybe_cached_file = find_cached_package(&mut conn, &package_ident).await?;

    let Some(cached_pkg) = maybe_cached_file else {
        log::info!("File {} not cached!", filename);
        // Cache miss fetch upstream
        let mut mirrors = get_mirrors_for_upstream_id(&mut conn, upstream.id).await?;
        let mut rand = thread_rng();
        mirrors.shuffle(&mut rand);
        let file = fetch_package(mirrors, &arch, &repo, &filename).await?;
        write_cached_file(
            cache_root,
            DiskCacheEntry {
                upstream_name: upstream.name,
                repo: repo.clone(),
                arch,
                filename,
            },
            &file.package,
        )?;
        let _created_id = create_cached_package(conn, package_ident, file.mirror_id).await?;
        return Ok(HttpResponseBuilder::new(http::StatusCode::OK).body(file.package));
    };

    log::info!("Cache hit {filename}");
    let entry = cache_package_to_disk_entry(upstream.name, cached_pkg);
    let pkg_path = path_of_cached_package(&cache_root, &entry);
    let content = fs::read(std::path::Path::new(&pkg_path))
        .map_err(|e| anyhow::anyhow!(format!("{:?}", e)))?;
    Ok(HttpResponseBuilder::new(http::StatusCode::OK)
        .insert_header((
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/octet-stream"),
        ))
        .body(content))
}

#[post("/rpc/expire_cache")]
pub async fn purge_expired_packages(
    pool: Data<SqlitePool>,
    app_state: Data<AppState>,
) -> AppResponse {
    let mut conn = pool.acquire().await?;
    purge_old_packages(&mut conn, app_state).await?;
    Ok(HttpResponseBuilder::new(http::StatusCode::OK).finish())
}
