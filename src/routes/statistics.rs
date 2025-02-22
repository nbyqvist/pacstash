use actix_web::{get, web::Data, Responder};
use sqlx::SqlitePool;
use crate::{statistics::fetch_statistics, templates::StatisticsTemplate};

#[get("/")]
pub async fn statistics_page(pool: Data<SqlitePool>) -> impl Responder {
    let mut conn = pool.acquire().await.ok().unwrap();
    let stats = fetch_statistics(&mut conn).await.ok().unwrap();
    StatisticsTemplate { stats }
}