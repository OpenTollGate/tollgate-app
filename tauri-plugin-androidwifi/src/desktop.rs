use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use wifiscanner;

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Androidwifi<R>> {
  Ok(Androidwifi(app.clone()))
}

/// Access to the androidwifi APIs.
pub struct Androidwifi<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Androidwifi<R> {
  pub fn get_wifi_details(&self, _payload: Empty) -> crate::Result<Vec<WifiDetails>> {
    let wifis  = wifiscanner::scan().expect("Failed to scan wifi");
    let wifidetails = wifis.into_iter().map(|wifi| 
      WifiDetails {
        ssid: wifi.ssid,
        bssid: wifi.mac,
        rssi: wifi.signal_level,
        capabilities: wifi.security,
        frequency: wifi.channel,
        information_elements: vec![],
      }
    ).collect();
    Ok(wifidetails)
  }

  pub fn connect_wifi(&self, payload: ConnectWifiPayload) -> crate::Result<PingResponse> {
    todo!()
  }
  pub fn get_current_wifi_details(&self, payload: Empty) -> crate::Result<PingResponse> {
    todo!()
  }

  pub fn get_mac_address(&self, payload: Empty) -> crate::Result<MacAddress> {
    todo!()
  }
}
