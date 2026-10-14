use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadStatus {
    Queued,
    Downloading,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub enum DownloadProgress {
    Started,
    Progress {
        bytes_received: u64,
        total_bytes: Option<u64>,
    },
    Completed {
        record: DownloadRecord,
    },
    Failed(String),
    Cancelled {
        record: DownloadRecord,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadRecord {
    pub id: Option<i64>,
    pub provider: String,
    pub provider_id: String,
    pub source_url: Option<String>,
    pub file_path: String,
    pub file_name: String,
    pub byte_size: Option<u64>,
    pub status: DownloadStatus,
    pub error_summary: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}
