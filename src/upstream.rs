use sqlx::{prelude::FromRow, SqliteConnection};

#[derive(FromRow)]
pub struct Upstream {
    pub id: i64,
    pub name: String,
    pub upstream_type: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

pub async fn find_upstream_by_name(
    conn: &mut SqliteConnection,
    name: &String,
) -> anyhow::Result<Option<Upstream>> {
    let upstream = sqlx::query_as!(Upstream, "select id, name, upstream_type, created_at, updated_at from upstreams where name = ? limit 1", name)
    .fetch_optional(conn).await?;

    Ok(upstream)
}
