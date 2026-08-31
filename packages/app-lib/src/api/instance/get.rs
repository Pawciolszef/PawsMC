use crate::event::InstancePayloadType;
use crate::event::emit::emit_instance;
use crate::state::instances::adapters::sqlite::{content_rows, instance_rows};
use crate::state::instances::{
    ContentSet, ContentSetStatus, ContentSourceKind, Instance,
    InstanceLaunchOverrides, InstanceLink,
};
use crate::state::{
    InstanceInstallStage, InstanceMetadata, LauncherFeatureVersion, ModLoader,
    ReleaseChannel, State,
};
use chrono::Utc;
use std::collections::HashSet;
use std::path::Path;
use uuid::Uuid;

#[tracing::instrument]
pub async fn get(instance_id: &str) -> crate::Result<Option<InstanceMetadata>> {
    let state = State::get().await?;
    crate::state::get_instance(instance_id, &state.pool).await
}

#[tracing::instrument]
pub async fn get_many(
    instance_ids: &[&str],
) -> crate::Result<Vec<InstanceMetadata>> {
    let state = State::get().await?;
    crate::state::get_instances_metadata(instance_ids, &state.pool).await
}

#[tracing::instrument]
pub async fn list() -> crate::Result<Vec<InstanceMetadata>> {
    let state = State::get().await?;
    let _ = reconcile_instances_with_disk(&state).await;
    crate::state::list_instances(&state.pool).await
}

async fn reconcile_instances_with_disk(state: &State) -> crate::Result<()> {
    let instances_dir = state.directories.instances_dir();
    if !instances_dir.exists() {
        return Ok(());
    }

    // 1. Get all instances currently recorded in the SQLite database
    let registered_instances = instance_rows::list_instances(&state.pool).await?;

    let mut tracked_paths = HashSet::new();
    for inst in registered_instances {
        let full_path = instances_dir.join(&inst.path);
        if !full_path.exists() {
            tracing::info!(
                "Auto-pruning missing instance from database: {} (path: {})",
                inst.id,
                inst.path
            );
            let _ = instance_rows::delete_instance_by_id(&inst.id, &state.pool).await;
            state.remove_instance_locks(&inst.id);
        } else {
            tracked_paths.insert(inst.path);
        }
    }

    // 2. Scan the filesystem profiles directory for unmanaged folders
    let mut read_dir = match tokio::fs::read_dir(&instances_dir).await {
        Ok(rd) => rd,
        Err(_) => return Ok(()),
    };

    while let Ok(Some(entry)) = read_dir.next_entry().await {
        let full_path = entry.path();
        if !full_path.is_dir() {
            continue;
        }

        let folder_name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(_) => continue,
        };

        if tracked_paths.contains(&folder_name) {
            continue;
        }

        // Folder is physically present on disk in profiles/ but not in database. Auto-import!
        auto_import_instance_folder(&folder_name, &full_path, state).await;
    }

    Ok(())
}

async fn auto_import_instance_folder(
    folder_name: &str,
    folder_path: &Path,
    state: &State,
) {
    tracing::info!(
        "Auto-discovering new profile folder in profiles/: {}",
        folder_name
    );

    let profile_json_path = folder_path.join("profile.json");
    let (p_name, p_game_ver, p_loader, p_loader_ver, p_icon) =
        if profile_json_path.exists() {
            if let Ok(content) = tokio::fs::read_to_string(&profile_json_path).await {
                parse_profile_json(&content)
            } else {
                (None, None, None, None, None)
            }
        } else {
            (None, None, None, None, None)
        };

    let name = p_name.unwrap_or_else(|| folder_name.to_string());
    let loader = p_loader.unwrap_or_else(|| detect_loader_from_dir(folder_path));
    let game_version = p_game_ver
        .or_else(|| detect_game_version_from_name_or_dir(folder_name, folder_path))
        .unwrap_or_else(|| "1.21.4".to_string());
    let loader_version = p_loader_ver;
    let icon_path = p_icon;

    let now = Utc::now();
    let instance_id = format!("local:{}", Uuid::new_v4());
    let content_set_id = format!("content-set:{}", Uuid::new_v4());

    let instance = Instance {
        id: instance_id.clone(),
        path: folder_name.to_string(),
        applied_content_set_id: Some(content_set_id.clone()),
        install_stage: InstanceInstallStage::Installed,
        launcher_feature_version: LauncherFeatureVersion::MOST_RECENT,
        update_channel: ReleaseChannel::Release,
        name,
        icon_path,
        created: now,
        modified: now,
        last_played: None,
        submitted_time_played: 0,
        recent_time_played: 0,
    };

    let content_set = ContentSet {
        id: content_set_id,
        instance_id: instance_id.clone(),
        name: "Default".to_string(),
        source_kind: ContentSourceKind::Local,
        status: ContentSetStatus::Available,
        game_version,
        protocol_version: None,
        loader,
        loader_version,
        created: now,
        modified: now,
    };

    let launch_overrides = InstanceLaunchOverrides::empty(instance_id.clone());

    if let Ok(mut tx) = state.pool.begin().await {
        let res = async {
            instance_rows::insert_instance(&instance, &mut tx).await?;
            instance_rows::insert_default_instance_sync_preferences(
                &instance_id,
                &mut tx,
            )
            .await?;
            content_rows::insert_content_set(&content_set, &mut tx).await?;
            instance_rows::upsert_instance_link(
                &instance_id,
                &InstanceLink::Unmanaged,
                &mut tx,
            )
            .await?;
            instance_rows::replace_instance_groups(&instance_id, &[], &mut tx)
                .await?;
            instance_rows::upsert_instance_launch_overrides(
                &launch_overrides,
                &mut tx,
            )
            .await?;
            tx.commit().await?;
            Ok::<(), crate::Error>(())
        }
        .await;

        if res.is_ok() {
            crate::state::instances::watcher::watch_instance_folder(
                &instance.id,
                &instance.path,
                &state.file_watcher,
                &state.directories,
            )
            .await;

            let _ = emit_instance(&instance.id, InstancePayloadType::Created).await;
            tracing::info!(
                "Successfully auto-imported instance '{}' ({})",
                instance.name,
                instance.id
            );
        }
    }
}

fn parse_profile_json(
    content: &str,
) -> (
    Option<String>,
    Option<String>,
    Option<ModLoader>,
    Option<String>,
    Option<String>,
) {
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(content) {
        let name = val
            .get("name")
            .and_then(|v| v.as_str())
            .or_else(|| val.pointer("/metadata/name").and_then(|v| v.as_str()))
            .map(|s| s.to_string());
        let game_version = val
            .get("game_version")
            .and_then(|v| v.as_str())
            .or_else(|| {
                val.pointer("/metadata/game_version").and_then(|v| v.as_str())
            })
            .map(|s| s.to_string());
        let loader_str = val
            .get("loader")
            .and_then(|v| v.as_str())
            .or_else(|| val.pointer("/metadata/loader").and_then(|v| v.as_str()));
        let loader = loader_str.and_then(|s| match s.to_lowercase().as_str() {
            "fabric" => Some(ModLoader::Fabric),
            "forge" => Some(ModLoader::Forge),
            "neoforge" => Some(ModLoader::NeoForge),
            "quilt" => Some(ModLoader::Quilt),
            "vanilla" => Some(ModLoader::Vanilla),
            _ => None,
        });
        let loader_ver = val
            .get("loader_version")
            .and_then(|v| v.as_str())
            .or_else(|| {
                val.pointer("/metadata/loader_version/id")
                    .and_then(|v| v.as_str())
            })
            .or_else(|| {
                val.pointer("/metadata/loader_version")
                    .and_then(|v| v.as_str())
            })
            .map(|s| s.to_string());
        let icon = val
            .get("icon")
            .and_then(|v| v.as_str())
            .or_else(|| val.pointer("/metadata/icon").and_then(|v| v.as_str()))
            .map(|s| s.to_string());
        (name, game_version, loader, loader_ver, icon)
    } else {
        (None, None, None, None, None)
    }
}

fn detect_loader_from_dir(dir: &Path) -> ModLoader {
    if dir.join(".fabric").exists() {
        return ModLoader::Fabric;
    }
    if dir.join(".quilt").exists() {
        return ModLoader::Quilt;
    }
    let mods_dir = dir.join("mods");
    if mods_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&mods_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if name.contains("neoforge") {
                    return ModLoader::NeoForge;
                }
                if name.contains("forge") && !name.contains("fabric") {
                    return ModLoader::Forge;
                }
                if name.contains("fabric") {
                    return ModLoader::Fabric;
                }
            }
        }
        return ModLoader::Fabric;
    }
    ModLoader::Vanilla
}

fn detect_game_version_from_name_or_dir(name: &str, dir: &Path) -> Option<String> {
    let re = regex::Regex::new(r"1\.\d+(\.\d+)?").ok()?;
    if let Some(m) = re.find(name) {
        let v = m.as_str();
        if v == "1.8" {
            return Some("1.8.9".to_string());
        }
        return Some(v.to_string());
    }

    let mods_dir = dir.join("mods");
    if mods_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&mods_dir) {
            for entry in entries.flatten() {
                let mname = entry.file_name().to_string_lossy().to_string();
                if let Some(m) = re.find(&mname) {
                    let v = m.as_str();
                    if v == "1.8" {
                        return Some("1.8.9".to_string());
                    }
                    return Some(v.to_string());
                }
            }
        }
    }

    None
}
