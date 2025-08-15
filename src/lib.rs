use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(any(target_os = "windows", target_os = "linux"))]
mod desktop;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(any(target_os = "windows", target_os = "linux"))]
use desktop::Plauth;
#[cfg(any(target_os = "macos"))]
use macos::Plauth;
#[cfg(mobile)]
use mobile::Plauth;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the plauth APIs.
pub trait PlauthExt<R: Runtime> {
    fn plauth(&self) -> &Plauth<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PlauthExt<R> for T {
    fn plauth(&self) -> &Plauth<R> {
        self.state::<Plauth<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("plauth")
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::authenticate
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let plauth = mobile::init(app, api)?;
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            let plauth = desktop::init(app, api)?;
            #[cfg(target_os = "macos")]
            let plauth = macos::init(app, api)?;
            app.manage(plauth);
            Ok(())
        })
        .build()
}
