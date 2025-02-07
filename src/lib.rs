use tauri::{plugin::{Builder, TauriPlugin}, Runtime};

mod err;
mod logic;

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("downloader")
    .invoke_handler(tauri::generate_handler![
      // thread managment
      logic::thread::spawn_thread,
      logic::thread::destroy_thread,
      // downloader managment,
      logic::actions::download,
      logic::actions::pause,
      logic::actions::unpause,
      logic::actions::status,
      logic::actions::cancel,
    ])
    .build()
}