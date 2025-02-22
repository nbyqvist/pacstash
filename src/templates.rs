use askama_actix::Template;

use crate::{cached_package::CachedPackageFull, statistics::PackageCounts};

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFoundTemplate;

#[derive(Template)]
#[template(path = "statistics.html")]
pub struct StatisticsTemplate {
    pub stats: Vec<PackageCounts>,
}

#[derive(Template)]
#[template(path = "repo.html")]
pub struct RepoViewTemplate {
    pub repo_name: String,
    pub packages: Result<Vec<CachedPackageFull>, String>,
}
