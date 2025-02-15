use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Empty {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiDetails {
  pub ssid: String,
  pub bssid: String,
  pub rssi: String,
  pub capabilities: String,
  pub frequency: String,
  pub information_elements: Vec<InformationElement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InformationElement {
  pub id: i64,
  pub id_ext: i64,
  pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
  pub wifis: Option<Vec<WifiDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MacAddress {
    pub mac_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectWifiPayload {
    pub ssid: String,
}
