use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Plauth<R>> {
    Ok(Plauth(app.clone()))
}

/// Access to the plauth APIs.
pub struct Plauth<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Plauth<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }

    pub fn authenticate(&self, payload: AuthRequest) -> crate::Result<AuthResponse> {
        Ok(AuthResponse {
            success: true,
            callback_url: None,
            error: None,
        })
    }
}
