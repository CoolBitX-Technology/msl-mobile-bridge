use crate::panic::{Result, ToResult};
use cardano_message_signing::error::{DeserializeError, JsError};

impl<T> ToResult<T> for std::result::Result<T, DeserializeError> {
  fn into_result(self) -> Result<T> {
    self.map_err(|e| e.to_string())
  }
}

impl<T> ToResult<T> for std::result::Result<T, JsError> {
  fn into_result(self) -> Result<T> {
    self.map_err(|e| e.to_string())
  }
}

impl<T> ToResult<T> for Result<T> {
  fn into_result(self) -> Result<T> {
    self
  }
}
