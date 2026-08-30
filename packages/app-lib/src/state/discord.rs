use std::sync::{Arc, atomic::AtomicBool};

use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{Activity, Assets, Timestamps},
};
use tokio::sync::RwLock;

use crate::State;

pub struct DiscordGuard {
    client: Arc<RwLock<DiscordIpcClient>>,
    connected: Arc<AtomicBool>,
}

impl DiscordGuard {
    /// Initialize discord IPC client, and attempt to connect to it
    /// If it fails, it will still return a DiscordGuard, but the client will be unconnected
    pub fn init() -> crate::Result<DiscordGuard> {
        let dipc = DiscordIpcClient::new("1123683254248148992");

        Ok(DiscordGuard {
            client: Arc::new(RwLock::new(dipc)),
            connected: Arc::new(AtomicBool::new(false)),
        })
    }

    /// If the client failed connecting during init(), this will check for connection and attempt to reconnect
    /// This MUST be called first in any client method that requires a connection, because those can PANIC if the client is not connected
    /// (No connection is different than a failed connection, the latter will not panic and can be retried)
    pub async fn retry_if_not_ready(&self) -> bool {
        let mut client = self.client.write().await;
        if !self.connected.load(std::sync::atomic::Ordering::Relaxed) {
            if client.connect().is_ok() {
                self.connected
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                return true;
            }
            return false;
        }
        true
    }

    /// Set the activity for a running instance with start timestamp and modpack details
    pub async fn set_instance_activity(
        &self,
        instance_name: &str,
        start_time_secs: Option<i64>,
        reconnect_if_fail: bool,
    ) -> crate::Result<()> {
        let state = State::get().await?;
        let settings = crate::state::Settings::get(&state.pool).await?;
        if !settings.discord_rpc {
            Ok(self.clear_activity(true).await?)
        } else {
            Ok(self
                .force_set_instance_activity(instance_name, start_time_secs, reconnect_if_fail)
                .await?)
        }
    }

    /// Force set instance activity regardless of settings check
    pub async fn force_set_instance_activity(
        &self,
        instance_name: &str,
        start_time_secs: Option<i64>,
        reconnect_if_fail: bool,
    ) -> crate::Result<()> {
        if !self.retry_if_not_ready().await {
            return Ok(());
        }

        let mut activity = Activity::new()
            .details(instance_name)
            .state("Playing Modpack")
            .assets(
                Assets::new()
                    .large_image("modrinth_simple")
                    .large_text("PawsMC Launcher")
                    .small_image("modrinth_simple")
                    .small_text("PawsMC"),
            );

        if let Some(start) = start_time_secs {
            activity = activity.timestamps(Timestamps::new().start(start));
        }

        let mut client = self.client.write().await;
        let res = client.set_activity(activity.clone());

        if reconnect_if_fail {
            if let Err(_e) = res {
                client.reconnect()?;
                return Ok(client.set_activity(activity)?);
            }
        } else {
            res?;
        }

        Ok(())
    }

    /// Set the activity to the given message
    /// First checks if discord is disabled, and if so, clear the activity instead
    pub async fn set_activity(
        &self,
        msg: &str,
        reconnect_if_fail: bool,
    ) -> crate::Result<()> {
        // Check if discord is disabled, and if so, clear the activity instead
        let state = State::get().await?;
        let settings = crate::state::Settings::get(&state.pool).await?;
        if !settings.discord_rpc {
            Ok(self.clear_activity(true).await?)
        } else {
            Ok(self.force_set_activity(msg, reconnect_if_fail).await?)
        }
    }

    /// Sets the activity to the given message, regardless of if discord is disabled or offline
    pub async fn force_set_activity(
        &self,
        msg: &str,
        reconnect_if_fail: bool,
    ) -> crate::Result<()> {
        // Attempt to connect if not connected
        if !self.retry_if_not_ready().await {
            return Ok(());
        }

        let activity = Activity::new().details("PawsMC Launcher").state(msg).assets(
            Assets::new()
                .large_image("modrinth_simple")
                .large_text("PawsMC Launcher"),
        );

        let mut client = self.client.write().await;
        let res = client.set_activity(activity.clone());

        if reconnect_if_fail {
            if let Err(_e) = res {
                client.reconnect()?;
                return Ok(client.set_activity(activity)?);
            }
        } else {
            res?;
        }

        Ok(())
    }

    /// Clear the activity entirely ('disabling' the RPC until the next set_activity)
    pub async fn clear_activity(
        &self,
        reconnect_if_fail: bool,
    ) -> crate::Result<()> {
        if !self.retry_if_not_ready().await {
            return Ok(());
        }

        let mut client = self.client.write().await;
        let res = client.clear_activity();

        if reconnect_if_fail {
            if res.is_err() {
                client.reconnect()?;
                return Ok(client.clear_activity()?);
            }
        } else {
            res?;
        }
        Ok(())
    }

    /// Clear the activity, but if there is a running profile, set the activity to that instead
    pub async fn clear_to_default(
        &self,
        reconnect_if_fail: bool,
    ) -> crate::Result<()> {
        let state = State::get().await?;

        let settings = crate::state::Settings::get(&state.pool).await?;
        if !settings.discord_rpc {
            return self.clear_activity(true).await;
        }

        let running_instances = state.process_manager.get_all();
        if let Some(existing_child) = running_instances.first() {
            let start_time = existing_child.start_time.timestamp();
            self.set_instance_activity(
                &existing_child.instance_name,
                Some(start_time),
                reconnect_if_fail,
            )
            .await?;
        } else {
            self.force_set_activity("Browsing instances", reconnect_if_fail)
                .await?;
        }
        Ok(())
    }
}
