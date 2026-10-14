use crate::CoreError;
use std::path::PathBuf;

/// Standard Linux application directories resolved through the `dirs` crate.
#[derive(Debug, Clone)]
pub struct AppPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub download_dir: Option<PathBuf>,
}

impl AppPaths {
    /// Resolve directories. Parent paths are returned even if they do not yet exist.
    pub fn resolve() -> Result<Self, CoreError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| CoreError::Filesystem("cannot resolve config directory".to_string()))?
            .join("wallhaven-desktop");
        let data_dir = dirs::data_dir()
            .ok_or_else(|| CoreError::Filesystem("cannot resolve data directory".to_string()))?
            .join("wallhaven-desktop");
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| CoreError::Filesystem("cannot resolve cache directory".to_string()))?
            .join("wallhaven-desktop");

        Ok(Self {
            config_dir,
            data_dir,
            cache_dir,
            download_dir: dirs::download_dir().or_else(dirs::picture_dir),
        })
    }

    /// Create the configuration, data, and cache directories when requested.
    pub fn ensure(&self) -> Result<(), CoreError> {
        for dir in [&self.config_dir, &self.data_dir, &self.cache_dir] {
            std::fs::create_dir_all(dir).map_err(|e| {
                CoreError::Filesystem(format!("cannot create {}: {e}", dir.display()))
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_application_paths() {
        let paths = AppPaths::resolve().expect("application paths");
        assert!(paths.config_dir.ends_with("wallhaven-desktop"));
        assert!(paths.data_dir.ends_with("wallhaven-desktop"));
        assert!(paths.cache_dir.ends_with("wallhaven-desktop"));
    }
}
