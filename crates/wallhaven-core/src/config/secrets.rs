use crate::error::CoreError;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn api_key_path(config_dir: &Path) -> std::path::PathBuf {
    config_dir.join("api_key")
}

pub fn read_api_key(config_dir: &Path) -> Option<String> {
    let path = api_key_path(config_dir);
    let metadata = fs::metadata(&path).ok()?;
    let mode = metadata.permissions().mode() & 0o777;
    if mode != 0o600 {
        // Tighten loose permissions before returning the secret.
        let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o600));
    }
    let content = fs::read_to_string(path).ok()?;
    let trimmed = content.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

pub fn has_api_key(config_dir: &Path) -> bool {
    read_api_key(config_dir).is_some()
}

pub fn save_api_key(config_dir: &Path, key: &str) -> Result<(), CoreError> {
    let trimmed = key.trim();
    if trimmed.is_empty() {
        return Err(CoreError::InvalidInput("API key is empty".to_string()));
    }
    fs::create_dir_all(config_dir).map_err(|error| CoreError::Filesystem(error.to_string()))?;
    let _ = fs::set_permissions(config_dir, fs::Permissions::from_mode(0o700));
    let path = api_key_path(config_dir);
    fs::write(&path, trimmed).map_err(|error| CoreError::Filesystem(error.to_string()))?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
        .map_err(|error| CoreError::Filesystem(error.to_string()))?;
    Ok(())
}

pub fn delete_api_key(config_dir: &Path) -> Result<(), CoreError> {
    let path = api_key_path(config_dir);
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(CoreError::Filesystem(error.to_string())),
    }
}
