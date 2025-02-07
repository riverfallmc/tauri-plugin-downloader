use std::sync::Arc;
use anyhow::anyhow;
use tauri::{Runtime, WebviewWindow};
use super::downloader::Downloader;
use once_cell::sync::OnceCell;
use tokio::{sync::Mutex, task::JoinHandle};
use crate::err::Result;

static THREAD: OnceCell<Arc<Mutex<Option<JoinHandle<()>>>>> = OnceCell::new();
static DOWNLOADER: OnceCell<Arc<Mutex<Downloader>>> = OnceCell::new();

fn get_thread() -> &'static Arc<Mutex<Option<JoinHandle<()>>>> {
  THREAD.get_or_init(|| Arc::new(Mutex::new(None)))
}

pub(crate) fn get_downloader() -> &'static Arc<Mutex<Downloader>> {
  DOWNLOADER.get_or_init(|| Arc::new(Mutex::new(Downloader::new())))
}

#[tauri::command]
pub(crate) async fn spawn_thread<R: Runtime>(
  window: WebviewWindow<R>
) -> Result<()> {
  let win = Arc::new(window);

  let mut guard = get_thread()
    .lock()
    .await;

  if guard.is_some() {
    return Err(anyhow!("Downloader thread already spawned!").into());
  }

  let thread = tokio::spawn(async move {
    get_downloader()
      .lock()
      .await
      .listen(win)
      .await;
  });

  *guard = Some(thread);

  Ok(())
}

#[tauri::command]
pub(crate) async fn destroy_thread() -> Result<()> {
  let mut guard = get_thread()
    .lock()
    .await;

  if let Some(thread) = guard.as_ref() {
    thread.abort();

    *guard = None;

    return Ok(());
  }

  Err(anyhow!("Downloader thread not found").into())
}