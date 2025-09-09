use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_plauth);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Plauth<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.plauth", "ExamplePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_plauth)?;
    Ok(Plauth(handle))
}

/// Access to the plauth APIs.
pub struct Plauth<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Plauth<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        self.0
            .run_mobile_plugin("ping", payload)
            .map_err(Into::into)
    }

    pub fn authenticate(&self, payload: AuthRequest) -> crate::Result<AuthResponse> {
        self.0
            .run_mobile_plugin("authenticate", payload)
            .map_err(Into::into)
    }
}
