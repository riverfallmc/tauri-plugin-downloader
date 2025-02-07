use std::{sync::Arc, time::Duration};
use serde::Serialize;
use tauri::{Emitter, Runtime, WebviewWindow};

use crate::err::Result;
use super::downloadable::Downloadable;

#[derive(Default)]
pub struct Downloader {
  current: Option<Downloadable>,
}

#[derive(Serialize, Clone)]
struct MsgForEval {
  message: String
}

#[derive(Serialize, Clone)]
struct NameForEval {
  name: String,
}

impl Downloader {
  pub fn new() -> Self {
    Downloader {
      current: None
    }
  }

  pub fn pause(&mut self) {
    if let Some(current) = &mut self.current {
      current.pause();
    }
  }

  pub fn unpause(&mut self) {
    if let Some(current) = &mut self.current {
      current.unpause();
    }
  }

  pub async fn cancel(&mut self) -> Result<()> {
    if let Some(current) = &mut self.current {
      current.cancel()
        .await?;
    }

    Ok(())
  }

  pub async fn set_current(
    &mut self,
    downloadable: Downloadable
  ) -> Result<()> {
    if let Some(current) = &mut self.current {
      if !current.is_downloaded() {
        current.cancel()
          .await?;
      }
    }

    self.current = Some(downloadable);

    Ok(())
  }

  pub fn get_current(
    &self
  ) -> &Option<Downloadable> {
    &self.current
  }

  pub async fn listen<R: Runtime>(
    &mut self,
    window: Arc<WebviewWindow<R>>
  ) {
    loop {
      if self.current.is_none() {
        tokio::time::sleep(Duration::from_millis(500))
          .await; // спим 1/2 секунды

        continue;
      }

      let current = self.current
        .as_mut()
        .unwrap();

      let result = current
        .download()
        .await;

      match result {
        Err(err) => {
          let _ = window.emit("dwnerror", MsgForEval {
            message: err.to_string()
          });
          log::error!("failed to download the file \"{}:{}\": {err}", current.name, current.save);
        },
        Ok(_) => {
          let _ = window.emit("dwnok", NameForEval {
            name: current.name.clone()
          });

          log::info!("file \"{}\" downloaded", current.name)
        }
      };

      self.current = None;
    }
  }
}