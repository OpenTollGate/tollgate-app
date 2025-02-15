use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::AndroidwifiExt;

#[command]
pub(crate) async fn get_wifi_details<R: Runtime>(
    app: AppHandle<R>,
    payload: Empty,
) -> Result<Vec<WifiDetails>> {
    app.androidwifi().get_wifi_details(payload)
}

#[command]
pub(crate) async fn connect_wifi<R: Runtime>(
    app: AppHandle<R>,
    payload: ConnectWifiPayload,
) -> Result<PingResponse> {
    app.androidwifi().connect_wifi(payload)
}

#[command]
pub(crate) async fn get_current_wifi_details<R: Runtime>(app: AppHandle<R>, payload: Empty) -> crate::Result<PingResponse> {
    app.androidwifi().get_current_wifi_details(payload)
}

#[command]
pub(crate) async fn get_mac_address<R: Runtime>(app: AppHandle<R>, payload: Empty) -> crate::Result<PingResponse> {
    app.androidwifi().get_current_wifi_details(payload)
}
