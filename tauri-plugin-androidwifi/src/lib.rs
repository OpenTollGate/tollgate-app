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

use commands::*;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Androidwifi;
#[cfg(mobile)]
use mobile::Androidwifi;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the androidwifi APIs.
pub trait AndroidwifiExt<R: Runtime> {
  fn androidwifi(&self) -> &Androidwifi<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AndroidwifiExt<R> for T {
  fn androidwifi(&self) -> &Androidwifi<R> {
    self.state::<Androidwifi<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("androidwifi")
    .invoke_handler(tauri::generate_handler![get_wifi_details, get_current_wifi_details, get_mac_address, connect_wifi])
    .setup(|app, api| {
      #[cfg(mobile)]
      let androidwifi = mobile::init(app, api)?;
      #[cfg(desktop)]
      let androidwifi = desktop::init(app, api)?;
      app.manage(androidwifi);
      Ok(())
    })
    .build()
}
