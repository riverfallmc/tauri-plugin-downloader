use anyhow::anyhow;

use crate::err::Result;
use super::{downloadable::{DownloadableInitialize, DownloadableOverview}, thread::get_downloader};

#[tauri::command]
pub async fn download(
  downloadable: DownloadableInitialize
) -> Result<()> {
  let clone = get_downloader()
    .clone();

  let mut downloader = clone
    .lock()
    .await;

  downloader.set_current(downloadable.into())
    .await?;

  Ok(())
}

#[tauri::command]
pub async fn pause() -> Result<()> {
  let clone = get_downloader()
    .clone();

  let mut downloader = clone
    .lock()
    .await;

  downloader.pause();

  Ok(())
}

#[tauri::command]
pub async fn unpause() -> Result<()> {
  let clone = get_downloader()
    .clone();

  let mut downloader = clone
    .lock()
    .await;

  downloader.unpause();

  Ok(())
}

#[tauri::command]
pub async fn status() -> Result<DownloadableOverview> {
  let clone = get_downloader()
    .clone();

  let downloader = clone
    .lock()
    .await;

  if let Some(current) = downloader.get_current() {
    return Ok(current.into())
  }

  Err(anyhow!("downloadable not found").into())
}

#[tauri::command]
pub async fn cancel() -> Result<()> {
  let clone = get_downloader()
    .clone();

  let mut downloader = clone
    .lock()
    .await;

  downloader.cancel()
    .await?;

  Ok(())
}