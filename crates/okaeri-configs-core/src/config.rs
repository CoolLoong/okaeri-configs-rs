use crate::error::ConfigResult;
use crate::metadata::ConfigMetadata;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// This trait is automatically implemented via the `#[derive(Config)]` macro.
pub trait Config: Sized + Serialize + DeserializeOwned + Default {
    fn metadata() -> ConfigMetadata;
    fn apply_env(&mut self) -> ConfigResult<()>;
}
