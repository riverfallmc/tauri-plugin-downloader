use std::{path::Path, time::Duration};
use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{fs::{self, File}, io::AsyncWriteExt};

#[derive(Debug)]
pub struct Downloadable {
  pub name: String,
  pub url: String,
  pub save: String,
  // for internal use
  pub paused: bool,
  pub speed: usize,
  pub size: u64,
  pub downloaded: u64,
  pub file: Option<File>
}

#[derive(Deserialize, Serialize)]
pub struct DownloadableInitialize {
  pub name: String,
  pub url: String,
  pub save: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadableOverview {
  pub name: String,
  pub url: String,
  pub save: String,
  // for internal use
  pub paused: bool,
  pub speed: usize,
  pub size: u64,
  pub downloaded: u64,
}

impl From<DownloadableInitialize> for Downloadable {
  fn from(s: DownloadableInitialize) -> Downloadable {
    Downloadable {
      name: s.name,
      url: s.url,
      save: s.save,
      paused: false,
      speed: 0,
      size: 0,
      downloaded: 0,
      file: None,
    }
  }
}

impl From<&Downloadable> for DownloadableOverview {
  fn from(s: &Downloadable) -> DownloadableOverview {
    DownloadableOverview {
      name: s.name.clone(),
      url: s.url.clone(),
      save: s.save.clone(),
      paused: s.paused,
      speed: s.speed,
      size: s.size,
      downloaded: s.downloaded,
    }
  }
}

static CLIENT: Lazy<Client> = Lazy::new(Client::new);

impl Downloadable {
  pub async fn download(
    &mut self
  ) -> Result<()> {
    let mut res = CLIENT.get(&self.url)
      .send()
      .await?;

    self.size = res.content_length()
      .unwrap_or_default();

    let save_path = Path::new(&self.save);

    if save_path.is_file() {
      fs::remove_file(&self.save)
        .await?;
    }

    fs::create_dir_all(save_path.parent().context("no parent dir")?)
      .await?;

    self.file = Some(File::create_new(&self.save).await?);

    while let Some(chunk) = res.chunk().await? {
      while self.paused {
        tokio::time::sleep(Duration::from_millis(500))
          .await;
      }

      self.downloaded += chunk.len() as u64;

      self.file
        .as_mut()
        .unwrap()
        .write_all(&chunk)
        .await?;
    }

    log::debug!("downloaded \"{}\"", self.name);

    Ok(())
  }

  pub async fn cancel(&mut self) -> Result<()> {
    self.pause();

    self.file = None;

    fs::remove_file(&self.save)
      .await
      .map_err(|e| anyhow!("unable to cancel Downloadable: {e}"))?;

    log::debug!("canceled \"{}\"", self.name);

    Ok(())
  }

  pub fn pause(&mut self) {
    self.paused = true;

    log::debug!("paused \"{}\"", self.name);
  }

  pub fn unpause(&mut self) {
    self.paused = false;

    log::debug!("unpaused \"{}\"", self.name);
  }

  pub fn is_downloaded(&self) -> bool {
    self.size == self.downloaded
  }
}