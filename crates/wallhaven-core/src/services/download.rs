use crate::domain::{DownloadProgress, DownloadRecord, DownloadStatus};
use crate::error::CoreError;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

const DEFAULT_MAX_BYTES: u64 = 100 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct DownloadRequest {
    pub provider: String,
    pub provider_id: String,
    pub source_url: String,
    pub destination_dir: PathBuf,
    pub max_bytes: u64,
}

impl DownloadRequest {
    pub fn with_default_limit(mut self) -> Self {
        if self.max_bytes == 0 {
            self.max_bytes = DEFAULT_MAX_BYTES;
        }
        self
    }
}

pub async fn run_download<P>(
    http: reqwest::Client,
    mut request: DownloadRequest,
    mut on_progress: P,
    cancelled: Arc<AtomicBool>,
) -> DownloadRecord
where
    P: FnMut(DownloadProgress) + Send + 'static,
{
    request = request.with_default_limit();
    let created_at = timestamp_now();
    let id_validation = validate_id(&request.provider_id);
    if let Err(error) = id_validation {
        on_progress(DownloadProgress::Failed(error.to_string()));
        return failed_record(&request, created_at, "invalid wallpaper id");
    }

    if let Err(error) = validate_source_url(&request.source_url) {
        on_progress(DownloadProgress::Failed(error.to_string()));
        return failed_record(&request, created_at, "invalid source URL");
    }

    on_progress(DownloadProgress::Started);

    if let Err(error) = fs::create_dir_all(&request.destination_dir) {
        let message = format!("cannot create destination directory: {error}");
        on_progress(DownloadProgress::Failed(message.clone()));
        return failed_record(&request, created_at, &message);
    }

    let source_url = request.source_url.clone();
    let response = match http.get(&source_url).send().await {
        Ok(response) => response,
        Err(error) => {
            let message = if error.is_timeout() {
                format!("request timed out: {error}")
            } else {
                error.to_string()
            };
            on_progress(DownloadProgress::Failed(message.clone()));
            return failed_record(&request, created_at, &message);
        }
    };

    let status = response.status();
    if !status.is_success() {
        let message = format!("download failed with HTTP {}", status.as_u16());
        on_progress(DownloadProgress::Failed(message.clone()));
        return failed_record(&request, created_at, &message);
    }

    let total_bytes = response.content_length();
    let temp_path = request
        .destination_dir
        .join(format!("wallhaven-{}.part", request.provider_id));
    let mut file = match fs::File::create(&temp_path) {
        Ok(file) => file,
        Err(error) => {
            let message = format!("cannot create temporary file: {error}");
            on_progress(DownloadProgress::Failed(message.clone()));
            return failed_record(&request, created_at, &message);
        }
    };

    let mut received: u64 = 0;
    let mut response = response;
    let mut cancelled_flag = false;
    let mut too_large = false;
    let mut network_error: Option<String> = None;

    loop {
        if cancelled.load(Ordering::Relaxed) {
            cancelled_flag = true;
            break;
        }
        match response.chunk().await {
            Ok(Some(chunk)) => {
                if received.saturating_add(chunk.len() as u64) > request.max_bytes {
                    too_large = true;
                    break;
                }
                if let Err(error) = file.write_all(&chunk) {
                    network_error = Some(format!("cannot write file: {error}"));
                    break;
                }
                received += chunk.len() as u64;
                on_progress(DownloadProgress::Progress {
                    bytes_received: received,
                    total_bytes,
                });
            }
            Ok(None) => break,
            Err(error) => {
                network_error = Some(error.to_string());
                break;
            }
        }
    }

    let _ = file.flush();
    drop(file);

    if cancelled_flag {
        let _ = fs::remove_file(&temp_path);
        let mut record = failed_record(&request, created_at, "cancelled");
        record.status = DownloadStatus::Cancelled;
        record.error_summary = Some("cancelled".to_string());
        on_progress(DownloadProgress::Cancelled {
            record: record.clone(),
        });
        return record;
    }

    if too_large {
        let _ = fs::remove_file(&temp_path);
        let message = format!("file exceeds {} bytes", request.max_bytes);
        on_progress(DownloadProgress::Failed(message.clone()));
        return failed_record(&request, created_at, &message);
    }

    if let Some(message) = network_error {
        let _ = fs::remove_file(&temp_path);
        on_progress(DownloadProgress::Failed(message.clone()));
        return failed_record(&request, created_at, &message);
    }

    let bytes = match fs::read(&temp_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            let _ = fs::remove_file(&temp_path);
            let message = format!("cannot read temporary file: {error}");
            on_progress(DownloadProgress::Failed(message.clone()));
            return failed_record(&request, created_at, &message);
        }
    };

    let extension = match image_extension(&bytes) {
        Some(extension) => extension,
        None => {
            let _ = fs::remove_file(&temp_path);
            let message = "downloaded file is not a supported image".to_string();
            on_progress(DownloadProgress::Failed(message.clone()));
            return failed_record(&request, created_at, &message);
        }
    };

    let final_path = unique_file_path(
        &request.destination_dir,
        &format!("wallhaven-{}", request.provider_id),
        extension,
    );

    if let Err(error) = fs::rename(&temp_path, &final_path) {
        let _ = fs::remove_file(&temp_path);
        let message = format!("cannot move file into place: {error}");
        on_progress(DownloadProgress::Failed(message.clone()));
        return failed_record(&request, created_at, &message);
    }

    let record = DownloadRecord {
        id: None,
        provider: request.provider.clone(),
        provider_id: request.provider_id.clone(),
        source_url: Some(request.source_url.clone()),
        file_path: final_path.to_string_lossy().to_string(),
        file_name: final_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("download")
            .to_string(),
        byte_size: Some(received),
        status: DownloadStatus::Completed,
        error_summary: None,
        created_at: created_at.clone(),
        completed_at: Some(timestamp_now()),
    };

    on_progress(DownloadProgress::Completed {
        record: record.clone(),
    });
    record
}

pub fn default_max_bytes() -> u64 {
    DEFAULT_MAX_BYTES
}

fn failed_record(request: &DownloadRequest, created_at: String, error: &str) -> DownloadRecord {
    DownloadRecord {
        id: None,
        provider: request.provider.clone(),
        provider_id: request.provider_id.clone(),
        source_url: Some(request.source_url.clone()),
        file_path: request.destination_dir.to_string_lossy().to_string(),
        file_name: format!("wallhaven-{}.part", request.provider_id),
        byte_size: None,
        status: DownloadStatus::Failed,
        error_summary: Some(error.to_string()),
        created_at,
        completed_at: None,
    }
}

fn validate_id(id: &str) -> Result<(), CoreError> {
    if id.is_empty() || !id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
        return Err(CoreError::InvalidInput(format!(
            "invalid wallpaper id: {id}"
        )));
    }
    Ok(())
}

fn validate_source_url(value: &str) -> Result<(), CoreError> {
    let parsed = reqwest::Url::parse(value)
        .map_err(|_| CoreError::InvalidInput("invalid source URL".to_string()))?;
    #[cfg(any(test, debug_assertions))]
    if parsed.scheme() == "http"
        && matches!(parsed.host_str(), Some("127.0.0.1") | Some("localhost"))
    {
        return Ok(());
    }
    if parsed.scheme() != "https" {
        return Err(CoreError::InvalidInput(
            "source URL must use https".to_string(),
        ));
    }
    match parsed.host_str() {
        Some("w.wallhaven.cc") | Some("wallhaven.cc") => Ok(()),
        _ => Err(CoreError::InvalidInput(
            "source host is not allowed".to_string(),
        )),
    }
}

fn image_extension(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        Some("jpg")
    } else if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("png")
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        Some("gif")
    } else if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        Some("webp")
    } else {
        None
    }
}

fn unique_file_path(directory: &Path, base: &str, extension: &str) -> PathBuf {
    let mut candidate = directory.join(format!("{base}.{extension}"));
    if !candidate.exists() {
        return candidate;
    }
    for index in 1.. {
        candidate = directory.join(format!("{base} ({index}).{extension}"));
        if !candidate.exists() {
            return candidate;
        }
    }
    unreachable!()
}

fn timestamp_now() -> String {
    // Keep timestamps dependency-free and sortable enough for in-memory use.
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
