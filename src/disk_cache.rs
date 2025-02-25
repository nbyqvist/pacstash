use std::{fs, path::Path};

pub struct DiskCacheEntry {
    pub upstream_name: String,
    pub repo: String,
    pub arch: String,
    pub filename: String,
}

impl DiskCacheEntry {
    fn dir_path(&self, cache_root: String) -> String {
        let base = Path::new(&cache_root);
        let full = base
            .join(&self.upstream_name)
            .join(&self.repo)
            .join(&self.arch);
        full.to_string_lossy().to_string()
    }
}

pub fn path_of_cached_package(cache_root: &String, entry: &DiskCacheEntry) -> String {
    let base = Path::new(&cache_root);
    let full = base
        .join(&entry.upstream_name)
        .join(&entry.repo)
        .join(&entry.arch)
        .join(&entry.filename);
    full.to_string_lossy().to_string()
}

pub fn write_cached_file(
    cache_root: String,
    entry: &DiskCacheEntry,
    content: &Vec<u8>,
) -> anyhow::Result<()> {
    log::info!("Storing file {}", entry.filename);
    let base_path = entry.dir_path(cache_root);
    log::info!("mkdir -p {}", base_path);
    fs::create_dir_all(&base_path)?;
    let full_path = Path::new(&base_path).join(&entry.filename);
    log::info!("write {}", &entry.filename);

    fs::write(full_path, content)?;
    log::info!("Done!");
    Ok(())
}

pub fn delete_cached_file(cache_root: &String, disk_entry: &DiskCacheEntry) -> anyhow::Result<()> {
    let path = path_of_cached_package(cache_root, disk_entry);
    log::info!("Deleting path {}", path);
    fs::remove_file(path)?;
    Ok(())
}
