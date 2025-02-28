use sqlx::SqliteConnection;

pub struct PackageCounts {
    pub upstream_name: String,
    pub package_count: i64,
}

pub async fn fetch_statistics(conn: &mut SqliteConnection) -> anyhow::Result<Vec<PackageCounts>> {
    let stats = sqlx::query!(
        "select
            u.name,
            count(c.id) as package_count
        from upstreams u
        join cached_packages c
            on c.upstream_id = u.id
        group by u.name"
    )
    .fetch_all(conn)
    .await?
    .into_iter()
    .map(|row| PackageCounts {
        upstream_name: row.name,
        package_count: row.package_count,
    })
    .collect::<Vec<_>>();
    Ok(stats)
}
