use crate::Result;
use crate::data::ModLoader;
use crate::install::events::{emit_install_job, InstallProgressReporter};
use crate::install::model::{
    InstallCleanup, InstallJobDisplay, InstallJobState, InstallJobStatus,
    InstallPhaseDetails, InstallPhaseId, InstallProgress,
    InstallProgressSecondary, InstallRequest, InstallTarget,
};
use crate::install::store;
use crate::state::InstanceLink;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, RwLock};

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent("PawsMC/2.0.0 (https://pawsmc.pawciol.eu; PrismLauncher-compatible)")
        .build()
        .unwrap_or_default()
});

/// Official Feed The Beast Public API base URL (used by Prism Launcher)
const FTB_API_BASE: &str = "https://api.feed-the-beast.com/v1/modpacks/public";

// In-memory cache for pack details to ensure lightning-fast searching and browsing
static PACK_CACHE: LazyLock<RwLock<HashMap<i64, (std::time::Instant, Arc<FtbModpackDetails>)>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
const CACHE_TTL_SECS: u64 = 600; // 10 minutes

// ---------------------------------------------------------------------------
// API Data Structures
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbAllPacksResponse {
    #[serde(default)]
    pub packs: Vec<i64>,
    #[serde(default)]
    pub total: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbArt {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(rename = "type", default)]
    pub art_type: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbAuthor {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type", default)]
    pub author_type: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbTag {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbSpecs {
    #[serde(default)]
    pub minimum: Option<i64>,
    #[serde(default)]
    pub recommended: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbVersionTarget {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type", default)]
    pub target_type: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbVersionInfo {
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type", default)]
    pub version_type: Option<String>,
    #[serde(default)]
    pub updated: Option<i64>,
    #[serde(default)]
    pub specs: Option<FtbSpecs>,
    #[serde(default)]
    pub targets: Option<Vec<FtbVersionTarget>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbModpackDetails {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub synopsis: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub art: Vec<FtbArt>,
    #[serde(default)]
    pub authors: Vec<FtbAuthor>,
    #[serde(default)]
    pub tags: Vec<FtbTag>,
    #[serde(default)]
    pub versions: Vec<FtbVersionInfo>,
    #[serde(default)]
    pub installs: Option<i64>,
    #[serde(default)]
    pub plays: Option<i64>,
    #[serde(default)]
    pub updated: Option<i64>,
    #[serde(default)]
    pub refreshed: Option<i64>,
    #[serde(default)]
    pub featured: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbCurseForgeRef {
    #[serde(default)]
    pub project: Option<serde_json::Value>,
    #[serde(default)]
    pub file: Option<serde_json::Value>,
}

impl FtbCurseForgeRef {
    pub fn project_id(&self) -> Option<i64> {
        self.project.as_ref().and_then(|v| match v {
            serde_json::Value::Number(n) => n.as_i64(),
            serde_json::Value::String(s) => s.parse::<i64>().ok(),
            _ => None,
        })
    }

    pub fn file_id(&self) -> Option<i64> {
        self.file.as_ref().and_then(|v| match v {
            serde_json::Value::Number(n) => n.as_i64(),
            serde_json::Value::String(s) => s.parse::<i64>().ok(),
            _ => None,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbVersionFile {
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type", default)]
    pub file_type: Option<String>,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub sha1: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(rename = "clientonly", default)]
    pub client_only: Option<bool>,
    #[serde(rename = "serveronly", default)]
    pub server_only: Option<bool>,
    #[serde(default)]
    pub optional: Option<bool>,
    #[serde(default)]
    pub updated: Option<i64>,
    #[serde(default)]
    pub curseforge: Option<FtbCurseForgeRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbVersionManifest {
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type", default)]
    pub version_type: Option<String>,
    #[serde(default)]
    pub updated: Option<i64>,
    #[serde(default)]
    pub targets: Vec<FtbVersionTarget>,
    #[serde(default)]
    pub specs: Option<FtbSpecs>,
    #[serde(default)]
    pub files: Vec<FtbVersionFile>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FtbModpackSummary {
    pub id: i64,
    pub name: String,
    pub synopsis: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub authors: Vec<String>,
    pub installs: i64,
    pub plays: i64,
    pub tags: Vec<String>,
    pub updated: Option<i64>,
    pub versions: Vec<FtbVersionInfo>,
    pub latest_game_version: Option<String>,
    pub latest_loader: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn details_to_summary(pack: &FtbModpackDetails) -> FtbModpackSummary {
    let icon_art = pack
        .art
        .iter()
        .find(|a| a.art_type.as_deref() == Some("square") || a.art_type.as_deref() == Some("logo"))
        .or_else(|| pack.art.first());
    let icon_url = icon_art.and_then(|a| a.url.clone());

    let banner_art = pack
        .art
        .iter()
        .find(|a| a.art_type.as_deref() == Some("splash") || a.art_type.as_deref() == Some("banner"));
    let banner_url = banner_art.and_then(|a| a.url.clone());

    let authors: Vec<String> = pack
        .authors
        .iter()
        .filter_map(|a| a.name.clone())
        .collect();

    let tags: Vec<String> = pack.tags.iter().filter_map(|t| t.name.clone()).collect();

    let latest_version = pack.versions.first();
    let (latest_game_version, latest_loader) = if let Some(ver) = latest_version {
        let targets = ver.targets.as_deref().unwrap_or(&[]);
        let mc = targets
            .iter()
            .find(|t| {
                t.name.as_deref().map(|n| n.eq_ignore_ascii_case("minecraft")).unwrap_or(false)
                    || t.target_type.as_deref() == Some("game")
            })
            .and_then(|t| t.version.clone());
        let loader = targets
            .iter()
            .find(|t| t.target_type.as_deref() == Some("modloader"))
            .and_then(|t| t.name.clone());
        (mc, loader)
    } else {
        (None, None)
    };

    // Calculate best updated timestamp (from pack or latest version)
    let updated = pack
        .updated
        .filter(|&u| u > 0)
        .or_else(|| latest_version.and_then(|v| v.updated).filter(|&u| u > 0));

    FtbModpackSummary {
        id: pack.id,
        name: pack.name.clone(),
        synopsis: pack.synopsis.clone().unwrap_or_default(),
        description: pack.description.clone(),
        icon_url,
        banner_url,
        authors: if authors.is_empty() {
            vec!["FTB Team".to_string()]
        } else {
            authors
        },
        installs: pack.installs.unwrap_or(0),
        plays: pack.plays.unwrap_or(0),
        tags,
        updated,
        versions: pack.versions.clone(),
        latest_game_version,
        latest_loader,
    }
}

// ---------------------------------------------------------------------------
// Public API Functions
// ---------------------------------------------------------------------------

/// Fetch all modpack IDs from the official FTB API
pub async fn get_all_ftb_pack_ids() -> Result<Vec<i64>> {
    let url = format!("{FTB_API_BASE}/modpack/all");
    let res = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .map_err(|e| crate::ErrorKind::InputError(format!("Failed to connect to FTB API: {e}")))?;

    if !res.status().is_success() {
        return Err(crate::ErrorKind::InputError(format!(
            "FTB API returned HTTP status {}",
            res.status()
        ))
        .into());
    }

    let all: FtbAllPacksResponse = res
        .json()
        .await
        .map_err(|e| crate::ErrorKind::InputError(format!("Failed to parse FTB /modpack/all: {e}")))?;

    Ok(all.packs)
}

/// Fetch modpack details from official FTB API, utilizing in-memory cache
pub async fn get_ftb_modpack_details(modpack_id: i64) -> Result<FtbModpackDetails> {
    // Check cache
    {
        let cache = PACK_CACHE.read().unwrap();
        if let Some((instant, details)) = cache.get(&modpack_id) {
            if instant.elapsed().as_secs() < CACHE_TTL_SECS {
                return Ok((**details).clone());
            }
        }
    }

    let url = format!("{FTB_API_BASE}/modpack/{modpack_id}");
    let res = HTTP_CLIENT
        .get(&url)
        .send()
        .await
        .map_err(|e| crate::ErrorKind::InputError(format!("Failed to fetch FTB modpack {modpack_id}: {e}")))?;

    if !res.status().is_success() {
        return Err(crate::ErrorKind::InputError(format!(
            "FTB API returned HTTP {} for modpack {modpack_id}",
            res.status()
        ))
        .into());
    }

    let details: FtbModpackDetails = res.json().await.map_err(|e| {
        crate::ErrorKind::InputError(format!("Failed to parse FTB modpack {modpack_id} details: {e}"))
    })?;

    // Store in cache
    {
        let mut cache = PACK_CACHE.write().unwrap();
        cache.insert(
            modpack_id,
            (std::time::Instant::now(), Arc::new(details.clone())),
        );
    }

    Ok(details)
}

/// Fetch version manifest for an FTB modpack
pub async fn get_ftb_modpack_version_manifest(
    modpack_id: i64,
    version_id: i64,
) -> Result<FtbVersionManifest> {
    let url = format!("{FTB_API_BASE}/modpack/{modpack_id}/{version_id}");
    let res = HTTP_CLIENT.get(&url).send().await.map_err(|e| {
        crate::ErrorKind::InputError(format!(
            "Failed to fetch FTB manifest for {modpack_id}/{version_id}: {e}"
        ))
    })?;

    if !res.status().is_success() {
        return Err(crate::ErrorKind::InputError(format!(
            "FTB API returned HTTP {} for manifest {modpack_id}/{version_id}",
            res.status()
        ))
        .into());
    }

    let text = res.text().await.map_err(|e| {
        crate::ErrorKind::InputError(format!("Failed to read response body: {e}"))
    })?;

    let manifest: FtbVersionManifest = serde_json::from_str(&text).map_err(|e| {
        crate::ErrorKind::InputError(format!("Failed to parse FTB version manifest: {e}"))
    })?;

    Ok(manifest)
}

/// Search and list modern FTB modpacks with accurate sorting and full metadata
pub async fn search_ftb_modpacks(
    term: Option<String>,
    limit: Option<usize>,
    sort: Option<String>,
) -> Result<Vec<FtbModpackSummary>> {
    let all_pack_ids = get_all_ftb_pack_ids().await?;
    let limit = limit.unwrap_or(20).clamp(1, 100);

    // Concurrently fetch pack details for uncached / top packs
    let mut details_list = Vec::new();
    let term_lower = term.as_ref().map(|t| t.trim().to_lowercase()).filter(|t| !t.is_empty());

    // We fetch details in chunks to avoid overwhelming the network
    let ids_to_fetch: Vec<i64> = all_pack_ids;
    for chunk in ids_to_fetch.chunks(15) {
        let futures = chunk.iter().map(|&id| async move { get_ftb_modpack_details(id).await.ok() });
        let results = futures::future::join_all(futures).await;
        for opt in results.into_iter().flatten() {
            details_list.push(opt);
        }
        // If we have enough matches when searching, we can break early
        if let Some(ref q) = term_lower {
            let matched_count = details_list
                .iter()
                .filter(|p| {
                    p.name.to_lowercase().contains(q)
                        || p.synopsis.as_deref().unwrap_or("").to_lowercase().contains(q)
                })
                .count();
            if matched_count >= limit * 2 {
                break;
            }
        }
    }

    // Filter by search term
    let filtered: Vec<FtbModpackDetails> = if let Some(ref q) = term_lower {
        details_list
            .into_iter()
            .filter(|p| {
                p.name.to_lowercase().contains(q)
                    || p.slug.as_deref().unwrap_or("").to_lowercase().contains(q)
                    || p.synopsis.as_deref().unwrap_or("").to_lowercase().contains(q)
                    || p.description.as_deref().unwrap_or("").to_lowercase().contains(q)
                    || p.tags.iter().any(|t| t.name.as_deref().unwrap_or("").to_lowercase().contains(q))
                    || p.authors.iter().any(|a| a.name.as_deref().unwrap_or("").to_lowercase().contains(q))
            })
            .collect()
    } else {
        details_list
    };

    // Convert to summaries
    let mut summaries: Vec<FtbModpackSummary> = filtered.iter().map(details_to_summary).collect();

    // Sort according to requested sort option
    let sort_key = sort.as_deref().unwrap_or("relevance");
    match sort_key {
        "updated" | "newest" => {
            summaries.sort_by(|a, b| b.updated.unwrap_or(0).cmp(&a.updated.unwrap_or(0)));
        }
        "downloads" | "installs" => {
            summaries.sort_by(|a, b| b.installs.cmp(&a.installs));
        }
        "follows" | "plays" => {
            summaries.sort_by(|a, b| b.plays.cmp(&a.plays));
        }
        "name" => {
            summaries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }
        _ => {
            // "relevance" or default: if query given, best match first; otherwise newest updated first
            if let Some(ref q) = term_lower {
                summaries.sort_by(|a, b| {
                    let a_exact = a.name.to_lowercase().contains(q);
                    let b_exact = b.name.to_lowercase().contains(q);
                    b_exact.cmp(&a_exact).then_with(|| b.updated.unwrap_or(0).cmp(&a.updated.unwrap_or(0)))
                });
            } else {
                summaries.sort_by(|a, b| b.updated.unwrap_or(0).cmp(&a.updated.unwrap_or(0)));
            }
        }
    }

    if summaries.len() > limit {
        summaries.truncate(limit);
    }

    Ok(summaries)
}

/// Download and install a complete FTB modpack into a ready-to-use Minecraft instance
pub async fn install_ftb_modpack(
    modpack_id: i64,
    version_id: Option<i64>,
    instance_name: Option<String>,
) -> Result<String> {
    let details = get_ftb_modpack_details(modpack_id).await?;
    if details.versions.is_empty() {
        return Err(crate::ErrorKind::InputError(
            "This FTB modpack has no available versions".to_string(),
        )
        .into());
    }

    let target_version_id = version_id.unwrap_or_else(|| {
        details
            .versions
            .iter()
            .rev()
            .find(|v| v.version_type.as_deref() == Some("release"))
            .or_else(|| details.versions.last())
            .map(|v| v.id)
            .unwrap_or(details.versions[0].id)
    });
    let manifest = get_ftb_modpack_version_manifest(modpack_id, target_version_id).await?;

    // Determine Minecraft version and Modloader from targets
    let mut game_version = "1.20.1".to_string();
    let mut modloader = ModLoader::Forge;
    let mut loader_version = None;

    for target in &manifest.targets {
        let name = target.name.as_deref().unwrap_or("").to_lowercase();
        let t_type = target.target_type.as_deref().unwrap_or("").to_lowercase();
        if name == "minecraft" || t_type == "game" {
            if let Some(ref v) = target.version {
                game_version = v.clone();
            }
        } else if t_type == "modloader" || name == "forge" || name == "neoforge" || name == "fabric" || name == "quilt" {
            loader_version = target.version.clone();
            if name.contains("neoforge") {
                modloader = ModLoader::NeoForge;
            } else if name.contains("fabric") {
                modloader = ModLoader::Fabric;
            } else if name.contains("quilt") {
                modloader = ModLoader::Quilt;
            } else {
                modloader = ModLoader::Forge;
            }
        }
    }

    let icon_url = details
        .art
        .iter()
        .find(|a| a.art_type.as_deref() == Some("square") || a.art_type.as_deref() == Some("logo"))
        .or_else(|| details.art.first())
        .and_then(|a| a.url.clone());

    let resolved_name = instance_name.unwrap_or_else(|| {
        format!("{} ({})", details.name, manifest.name)
    });

    tracing::info!(
        "Creating FTB instance: {} (MC: {}, Loader: {:?} {:?})",
        resolved_name,
        game_version,
        modloader,
        loader_version
    );

    // Create the instance
    let instance_meta = crate::api::instance::create(
        resolved_name.clone(),
        game_version,
        modloader,
        loader_version,
        None,
        None,
        InstanceLink::ImportedModpack {
            project_id: None,
            version_id: None,
            name: Some(details.name.clone()),
            version_number: Some(manifest.name.clone()),
            filename: None,
        },
    )
    .await?;

    let instance_id = instance_meta.instance.id.clone();
    let instance_full_path = match crate::api::instance::get_full_path(&instance_id).await {
        Ok(p) => p,
        Err(e) => {
            let _ = crate::api::instance::remove(&instance_id).await;
            return Err(e);
        }
    };

    let app_state = crate::State::get().await?;
    let _ = crate::state::instances::commands::set_instance_install_stage(
        &instance_id,
        crate::state::InstanceInstallStage::PackInstalling,
        &app_state.pool,
    )
    .await;

    // Initialize InstallJob so the top-right download progress notification displays
    let job_id = uuid::Uuid::new_v4();
    let mut job_state = InstallJobState::new(InstallRequest::CreateModpackInstance {
        location: crate::api::pack::install_from::CreatePackLocation::FromVersionId {
            project_id: format!("ftb-{}", modpack_id),
            version_id: target_version_id.to_string(),
            title: resolved_name.clone(),
            icon_url: icon_url.clone(),
        },
        post_install_edit: None,
    });
    job_state.display = Some(InstallJobDisplay {
        title: resolved_name.clone(),
        icon: icon_url.clone(),
    });
    job_state.target = InstallTarget::NewInstance {
        instance_id: Some(instance_id.clone()),
    };
    job_state.cleanup = InstallCleanup::DeleteNewInstance {
        instance_id: Some(instance_id.clone()),
    };

    let record = store::insert(
        job_id,
        &job_state,
        InstallJobStatus::Running,
        &app_state,
    )
    .await?;
    let _ = emit_install_job(&record.snapshot()).await;

    let reporter = InstallProgressReporter::new(job_id, job_state.clone());

    // Cache icon if square art is available
    if let Some(ref url) = icon_url {
        if let Ok(bytes) = crate::util::fetch::fetch(
            url,
            None,
            None,
            None,
            &app_state.fetch_semaphore,
            &app_state.pool,
        )
        .await
        {
            if let Ok(icon_path) = crate::api::instance::cache_icon(bytes, &app_state).await {
                let _ = crate::api::instance::edit_icon(&instance_id, Some(&icon_path)).await;
            }
        }
    }

    // Filter files to download (skip server-only files)
    let files_to_download: Vec<FtbVersionFile> = manifest
        .files
        .into_iter()
        .filter(|f| !f.server_only.unwrap_or(false))
        .collect();

    let total_files = files_to_download.len() as u64;
    let total_bytes: u64 = files_to_download.iter().filter_map(|f| f.size).sum();
    let downloaded_files = Arc::new(AtomicU64::new(0));
    let downloaded_bytes = Arc::new(AtomicU64::new(0));

    tracing::info!(
        "Downloading {} files ({} bytes) for FTB modpack {}",
        total_files,
        total_bytes,
        resolved_name
    );

    let _ = reporter
        .update(
            InstallPhaseId::DownloadingContent,
            Some(InstallProgress {
                current: 0,
                total: total_files,
                secondary: Some(InstallProgressSecondary {
                    current: 0,
                    total: total_bytes,
                }),
            }),
            InstallPhaseDetails::Modpack {
                project_id: Some(format!("ftb-{}", modpack_id)),
                version_id: Some(target_version_id.to_string()),
                title: Some(resolved_name.clone()),
            },
        )
        .await;

    // Download files concurrently in batches of 32
    for chunk in files_to_download.chunks(32) {
        let futures = chunk.iter().map(|file| {
            let instance_dir = instance_full_path.clone();
            let downloaded_files = downloaded_files.clone();
            let downloaded_bytes = downloaded_bytes.clone();
            let file_size = file.size.unwrap_or(0);
            async move {
                // Clean relative path (remove leading "./" or ".\\")
                let rel_path = file
                    .path
                    .trim_start_matches('.')
                    .trim_start_matches('/')
                    .trim_start_matches('\\');
                let target_dir = if rel_path.is_empty() {
                    instance_dir.clone()
                } else {
                    instance_dir.join(rel_path)
                };

                let target_file_path = target_dir.join(&file.name);

                // Skip download if file already exists with expected size
                if let Ok(metadata) = tokio::fs::metadata(&target_file_path).await {
                    if let Some(expected_size) = file.size {
                        if metadata.len() == expected_size {
                            downloaded_files.fetch_add(1, Ordering::Relaxed);
                            downloaded_bytes.fetch_add(file_size, Ordering::Relaxed);
                            return Ok::<(), crate::Error>(());
                        }
                    } else if metadata.len() > 0 {
                        downloaded_files.fetch_add(1, Ordering::Relaxed);
                        downloaded_bytes.fetch_add(metadata.len(), Ordering::Relaxed);
                        return Ok::<(), crate::Error>(());
                    }
                }

                // Determine download URL
                let download_url = if let Some(ref u) = file.url {
                    if !u.is_empty() {
                        u.clone()
                    } else {
                        "".to_string()
                    }
                } else if let Some(ref cf) = file.curseforge {
                    if let Some(file_id) = cf.file_id() {
                        format!(
                            "https://edge.forgecdn.net/files/{}/{}/{}",
                            file_id / 1000,
                            file_id % 1000,
                            file.name
                        )
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                };

                if download_url.is_empty() {
                    tracing::warn!("No download URL for file {} in FTB pack", file.name);
                    downloaded_files.fetch_add(1, Ordering::Relaxed);
                    return Ok(());
                }

                if let Err(e) = tokio::fs::create_dir_all(&target_dir).await {
                    tracing::warn!("Failed to create directory {}: {}", target_dir.display(), e);
                    return Err(e.into());
                }

                // Download file
                match HTTP_CLIENT.get(&download_url).send().await {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            if let Ok(bytes) = resp.bytes().await {
                                let len = bytes.len() as u64;
                                if let Err(e) = tokio::fs::write(&target_file_path, bytes).await {
                                    tracing::warn!(
                                        "Failed to write file {}: {}",
                                        target_file_path.display(),
                                        e
                                    );
                                }
                                downloaded_bytes.fetch_add(len, Ordering::Relaxed);
                            }
                        } else {
                            tracing::warn!(
                                "HTTP {} downloading {} from {}",
                                resp.status(),
                                file.name,
                                download_url
                            );
                            downloaded_bytes.fetch_add(file_size, Ordering::Relaxed);
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to download {}: {}", file.name, e);
                        downloaded_bytes.fetch_add(file_size, Ordering::Relaxed);
                    }
                }

                downloaded_files.fetch_add(1, Ordering::Relaxed);
                Ok::<(), crate::Error>(())
            }
        });

        let _ = futures::future::join_all(futures).await;

        let cur_files = downloaded_files.load(Ordering::Relaxed);
        let cur_bytes = downloaded_bytes.load(Ordering::Relaxed);
        let _ = reporter
            .update(
                InstallPhaseId::DownloadingContent,
                Some(InstallProgress {
                    current: cur_files,
                    total: total_files,
                    secondary: Some(InstallProgressSecondary {
                        current: cur_bytes,
                        total: total_bytes,
                    }),
                }),
                InstallPhaseDetails::Modpack {
                    project_id: Some(format!("ftb-{}", modpack_id)),
                    version_id: Some(target_version_id.to_string()),
                    title: Some(resolved_name.clone()),
                },
            )
            .await;
    }

    // Install Minecraft assets/jar/libraries for this loader/version
    if let Err(e) = crate::launcher::install_minecraft_for_instance_id_with_reporter(
        &instance_id,
        false,
        Some(reporter.clone()),
    )
    .await
    {
        tracing::warn!(
            "Initial Minecraft installation for FTB pack {}: {}, will install on launch",
            instance_id,
            e
        );
    }

    let _ = reporter
        .update(
            InstallPhaseId::Finalizing,
            None,
            InstallPhaseDetails::Empty,
        )
        .await;

    let app_state = crate::State::get().await?;
    let mut final_state = job_state.clone();
    final_state.progress.phase = InstallPhaseId::Finalizing;
    final_state.display = Some(InstallJobDisplay {
        title: resolved_name.clone(),
        icon: icon_url,
    });
    if let Ok(Some(record)) = store::complete_success(job_id, &final_state, &app_state).await {
        let _ = emit_install_job(&record.snapshot()).await;
    }

    let _ = crate::state::instances::commands::set_instance_install_stage(
        &instance_id,
        crate::state::InstanceInstallStage::Installed,
        &app_state.pool,
    )
    .await;
    let _ = crate::event::emit::emit_instance(
        &instance_id,
        crate::event::InstancePayloadType::Edited,
    )
    .await;

    tracing::info!("FTB modpack {} successfully installed to {}", resolved_name, instance_id);
    Ok(instance_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ftb_search_skies() {
        let results = search_ftb_modpacks(Some("skies".to_string()), Some(10), None).await.unwrap();
        assert!(!results.is_empty(), "Expected FTB Skies modpacks in search results");
        let found_skies = results.iter().any(|p| p.name.to_lowercase().contains("skies"));
        assert!(found_skies, "Expected at least one modpack containing 'skies'");
        println!("Found {} FTB Skies packs:", results.len());
        for p in &results {
            println!("  - ID: {}, Name: {}, Icon: {:?}", p.id, p.name, p.icon_url);
        }
        let manifest = get_ftb_modpack_version_manifest(134, 100421).await;
        println!("Manifest 134/100421 result: {:?}", manifest.as_ref().map(|m| &m.name));
        if let Err(ref e) = manifest {
            println!("Manifest error: {:?}", e);
        }
        assert!(manifest.is_ok(), "Manifest should deserialize successfully");
    }
}