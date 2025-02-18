const COMMANDS: &[&str] = &[
  "get_wifi_details",
  "get_current_wifi_details",
  "connect_wifi",
  "get_mac_address",
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
