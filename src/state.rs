pub struct AppStateStruct {
    pub cache_root: String,
}

pub type AppState = std::sync::Arc<AppStateStruct>;