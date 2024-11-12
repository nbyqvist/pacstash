pub struct AppStateStruct {
    pub cache_root: String,
    pub pkg_max_age: u32,
}

pub type AppState = std::sync::Arc<AppStateStruct>;
