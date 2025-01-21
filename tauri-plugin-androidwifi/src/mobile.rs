use serde::de::DeserializeOwned;
use serde::{Serialize};
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_androidwifi);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Androidwifi<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.androidwifi", "WifiPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_androidwifi)?;
  Ok(Androidwifi(handle))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectWifiPayload {
    pub ssid: String,
}

/// Access to the androidwifi APIs.
pub struct Androidwifi<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Androidwifi<R> {
  pub fn get_wifi_details(&self, payload: Empty) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("getWifiDetails", payload)
      .map_err(Into::into)
  }

  pub fn connect_wifi(&self, payload: ConnectWifiPayload) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("connectWifi", payload)
      .map_err(Into::into)
  }
}
