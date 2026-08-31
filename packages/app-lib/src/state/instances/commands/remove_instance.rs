use crate::state::State;
use crate::state::instances::adapters::sqlite::instance_rows;
use crate::util::io;

pub(crate) async fn remove_instance(
    instance_id: &str,
    state: &State,
) -> crate::Result<()> {
    let instance = instance_rows::get_instance_by_id(instance_id, &state.pool)
        .await?
        .ok_or_else(|| {
            crate::ErrorKind::InputError("Unknown instance".to_string())
        })?;
    let _content_lock = state.lock_instance_content(instance_id).await;
    let _synced_options_lock = state.lock_synced_options().await;

    // 1. Unwatch directory so Windows doesn't lock it
    crate::state::instances::watcher::unwatch_instance_folder(
        &instance.path,
        &state.file_watcher,
        &state.directories,
    )
    .await;

    // 2. Remove physical directory from disk first
    let path = state.directories.instances_dir().join(&instance.path);
    if path.exists() {
        if let Err(e) = io::remove_dir_all(&path).await {
            tracing::warn!("Failed async remove_dir_all on {:?}: {e}, trying fallback", path);
            let _ = std::fs::remove_dir_all(&path);
        }
    }

    // 3. Remove generated instance files
    crate::api::instance::remove_generated_instance_files(instance_id, state)
        .await?;

    // 4. Delete instance database row and release locks
    delete_instance_row_and_locks(&instance.id, state).await?;

    Ok(())
}

async fn delete_instance_row_and_locks(
    instance_id: &str,
    state: &State,
) -> crate::Result<()> {
    // Keep these together so deleted instances cannot leave stale entries in the per-instance lock maps.
    instance_rows::delete_instance_by_id(instance_id, &state.pool).await?;
    state.remove_instance_locks(instance_id);

    Ok(())
}
