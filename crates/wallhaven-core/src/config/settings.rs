use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSettings {
    pub preferred_categories: String,
    pub preferred_purity: String,
    pub download_dir: Option<String>,
    pub blur_nsfw_thumbnails: bool,
    pub theme: String,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            preferred_categories: "111".to_string(),
            preferred_purity: "100".to_string(),
            download_dir: None,
            blur_nsfw_thumbnails: true,
            theme: "dark".to_string(),
        }
    }
}

pub fn settings_path(config_dir: &Path) -> std::path::PathBuf {
    config_dir.join("settings.json")
}

pub fn load_settings(config_dir: &Path) -> UserSettings {
    let path = settings_path(config_dir);
    fs::read_to_string(path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

pub fn save_settings(
    config_dir: &Path,
    settings: &UserSettings,
) -> Result<(), crate::error::CoreError> {
    if let Some(parent) = config_dir.parent() {
        let _ = parent;
    }
    fs::create_dir_all(config_dir)
        .map_err(|error| crate::error::CoreError::Filesystem(error.to_string()))?;
    let content = serde_json::to_string_pretty(settings)
        .map_err(|error| crate::error::CoreError::Storage(error.to_string()))?;
    fs::write(settings_path(config_dir), content)
        .map_err(|error| crate::error::CoreError::Filesystem(error.to_string()))?;
    Ok(())
}
