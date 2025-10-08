use crate::error::ConfigResult;
use crate::formats::format::Format;
use crate::metadata::ConfigMetadata;


/// Middleware trait for extending configuration processing
///
/// This trait allows users to implement custom processing logic that runs
/// during configuration loading, saving, and merging operations.
///
/// # Example
///
/// ```rust
/// use crate::okaeri_configs_core::prelude::*;
///
/// struct LoggingMiddleware;
///
/// impl ConfigMiddleware for LoggingMiddleware {
///     fn on_save(&self, content: &mut String, _metadata: &ConfigMetadata, _format: Format) -> ConfigResult<()> {
///         println!("Saving config, size: {} bytes", content.len());
///         Ok(())
///     }
/// }
/// ```
pub trait ConfigMiddleware: Send + Sync {
    fn on_load(&self, _content: &mut String, _metadata: &ConfigMetadata, _format: Format) -> ConfigResult<()> {
        Ok(())
    }

    fn on_save(&self, _content: &mut String, _metadata: &ConfigMetadata, _format: Format) -> ConfigResult<()> {
        Ok(())
    }

    fn on_merge(
        &self,
        _old_content: &str,
        _new_content: &mut String,
        _metadata: &ConfigMetadata,
        _format: Format,
    ) -> ConfigResult<()> {
        Ok(())
    }
}