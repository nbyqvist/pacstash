use crate::upstream_mirror::{substitute_url_params, UpstreamMirror};
use anyhow::anyhow;

pub struct MirrorPackageFetch {
    pub mirror_id: i64,
    pub tried_mirrors: Vec<i64>,
    pub package: Vec<u8>,
}

pub async fn fetch_package(mirrors: Vec<UpstreamMirror>, arch: &String, repo: &String, filename: &String) -> anyhow::Result<MirrorPackageFetch> {
    let mut tried_mirror_ids = vec![];
    log::info!("Fetching {repo}/{arch}/{filename}");
    for mirror in mirrors.iter() {
        log::info!("Trying mirror {}", mirror.url);
        let mirror_base_url = substitute_url_params(mirror.url.clone(), arch.clone(), repo.clone());
        let url = format!("{}/{}", mirror_base_url, filename);
        let package = reqwest::get(url.clone()).await;
        match package {
            Ok(response) => {
                let package = response.bytes().await?;
                log::info!("Got package {} from mirror {}", filename, mirror.url);
                return Ok(MirrorPackageFetch{
                    mirror_id: mirror.id,
                    tried_mirrors: tried_mirror_ids.clone(),
                    package: package.to_vec(),
                })
            },
            Err(e) => {
                // Don't return yet, try next mirror
                log::warn!("Mirror {} failed to return package!", mirror.url);
                tried_mirror_ids.push(mirror.id);
                println!("Fetch error: {}. URL: {}", e, url);
            }
        }
    }

    Err(anyhow!("All mirrors failed to fetch"))
}