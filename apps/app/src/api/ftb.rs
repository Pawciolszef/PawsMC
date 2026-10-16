use crate::api::Result;
use theseus::pack::ftb::{
    FtbModpackDetails, FtbModpackSummary, FtbVersionManifest,
    get_ftb_modpack_details as fetch_ftb_details,
    get_ftb_modpack_version_manifest as fetch_ftb_manifest,
    install_ftb_modpack as do_install_ftb_modpack,
    search_ftb_modpacks as fetch_ftb_search,
};

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("ftb")
        .invoke_handler(tauri::generate_handler![
            search_ftb_modpacks,
            get_ftb_modpack_details,
            get_ftb_modpack_version_manifest,
            install_ftb_modpack,
        ])
        .build()
}

#[tauri::command]
pub async fn search_ftb_modpacks(
    term: Option<String>,
    limit: Option<usize>,
    sort: Option<String>,
) -> Result<Vec<FtbModpackSummary>> {
    Ok(fetch_ftb_search(term, limit, sort).await?)
}

#[tauri::command]
pub async fn get_ftb_modpack_details(
    modpack_id: i64,
) -> Result<FtbModpackDetails> {
    Ok(fetch_ftb_details(modpack_id).await?)
}

#[tauri::command]
pub async fn get_ftb_modpack_version_manifest(
    modpack_id: i64,
    version_id: i64,
) -> Result<FtbVersionManifest> {
    Ok(fetch_ftb_manifest(modpack_id, version_id).await?)
}

#[tauri::command]
pub async fn install_ftb_modpack(
    modpack_id: i64,
    version_id: Option<i64>,
    instance_name: Option<String>,
) -> Result<String> {
    Ok(do_install_ftb_modpack(modpack_id, version_id, instance_name).await?)
}