use tauri::ipc::InvokeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TauriCommandError {
  #[error("Anyhow error: {0}")]
  Anyhow(#[from] anyhow::Error),
}

impl From<TauriCommandError> for InvokeError {
  fn from(error: TauriCommandError) -> Self {
    InvokeError::from(error.to_string())
  }
}

#[allow(unused)]
pub(crate) type Result<T> = anyhow::Result<T, TauriCommandError>;
