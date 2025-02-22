use sqlx::{prelude::FromRow, SqliteConnection};

#[derive(FromRow)]
pub struct UpstreamMirror {
    pub id: i64,
    pub upstream_id: i64,
    pub url: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

pub async fn get_mirrors_for_upstream_id(
    conn: &mut SqliteConnection,
    upstream_id: i64,
) -> anyhow::Result<Vec<UpstreamMirror>> {
    let mirrors = sqlx::query_as!(UpstreamMirror, "select id, upstream_id, url, created_at, updated_at from upstream_mirrors where upstream_id = ?", upstream_id)
    .fetch_all(conn).await?;

    Ok(mirrors)
}

pub fn should_cache_file(file: &str) -> bool {
    !(file.ends_with(".sig") || file.ends_with(".db"))
}

pub fn substitute_url_params(url: String, arch: String, repo: String) -> String {
    url.replace("$arch_v4", &arch)
        .replace("$arch_v3", &arch)
        .replace("$arch", &arch)
        .replace("$repo", &repo)
}
