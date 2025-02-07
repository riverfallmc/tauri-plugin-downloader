const COMMANDS: &[&str] = &[
  "spawn_thread",
  "destroy_thread",

  "download",
  "pause",
  "unpause",
  "status",
  "cancel",
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}
