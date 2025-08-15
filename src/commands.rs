use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::PlauthExt;
use crate::Result;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.plauth().ping(payload)
}

#[command]
pub(crate) async fn authenticate<R: Runtime>(
    app: AppHandle<R>,
    payload: AuthRequest,
) -> Result<AuthResponse> {
    app.plauth().authenticate(payload)
}
