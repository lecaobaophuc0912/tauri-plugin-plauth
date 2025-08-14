use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Plauth;
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
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let plauth = mobile::init(app, api)?;
      #[cfg(desktop)]
      let plauth = desktop::init(app, api)?;
      app.manage(plauth);
      Ok(())
    })
    .build()
}
