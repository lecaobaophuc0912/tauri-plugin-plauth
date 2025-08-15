use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use tauri_swift_runtime::{PluginApiExt, PluginHandleExt};

use crate::models::*;
tauri_swift_runtime::swift_plugin_binding!(init_plugin_plauth);

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Plauth<R>> {
    let api: PluginApiExt<_, _> = _api.into();
    let handle = api.register_swift_plugin(init_plugin_plauth)?;
    Ok(Plauth(handle))
}

/// Access to the plauth APIs.
pub struct Plauth<R: Runtime>(PluginHandleExt<R>);

impl<R: Runtime> Plauth<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }

    pub fn authenticate(&self, payload: AuthRequest) -> crate::Result<AuthResponse> {
        self.0
            .run_swift_plugin("authenticate", payload)
            .map_err(Into::into)
    }
}
