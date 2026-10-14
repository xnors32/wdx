use crate::domain::DownloadRecord;
use crate::storage::database::DbConnection;
use crate::CoreError;
use rusqlite::params;

#[derive(Clone)]
pub struct DownloadRepository {
    connection: DbConnection,
}

impl DownloadRepository {
    pub fn new(connection: DbConnection) -> Self {
        Self { connection }
    }

    pub fn save(&self, record: &DownloadRecord) -> Result<i64, CoreError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| CoreError::Storage("lock poisoned".to_string()))?;
        connection
            .execute(
                "INSERT INTO downloads (provider, provider_id, source_url, file_path, file_name, byte_size, status, error_summary, created_at, completed_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    record.provider,
                    record.provider_id,
                    record.source_url,
                    record.file_path,
                    record.file_name,
                    record.byte_size.map(|value| value as i64),
                    format!("{:?}", record.status).to_lowercase(),
                    record.error_summary,
                    record.created_at,
                    record.completed_at,
                ],
            )
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        Ok(connection.last_insert_rowid())
    }

    pub fn list(&self) -> Result<Vec<DownloadRecord>, CoreError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| CoreError::Storage("lock poisoned".to_string()))?;
        let mut statement = connection
            .prepare("SELECT id, provider, provider_id, source_url, file_path, file_name, byte_size, status, error_summary, created_at, completed_at FROM downloads ORDER BY id DESC")
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        let rows = statement
            .query_map([], |row| {
                Ok(DownloadRecord {
                    id: Some(row.get(0)?),
                    provider: row.get(1)?,
                    provider_id: row.get(2)?,
                    source_url: row.get(3)?,
                    file_path: row.get(4)?,
                    file_name: row.get(5)?,
                    byte_size: row.get::<_, Option<i64>>(6)?.map(|value| value as u64),
                    status: parse_status(&row.get::<_, String>(7)?),
                    error_summary: row.get(8)?,
                    created_at: row.get(9)?,
                    completed_at: row.get(10)?,
                })
            })
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        let mut records = Vec::new();
        for row in rows {
            records.push(row.map_err(|error| CoreError::Storage(error.to_string()))?);
        }
        Ok(records)
    }
}

fn parse_status(value: &str) -> crate::domain::DownloadStatus {
    match value {
        "completed" => crate::domain::DownloadStatus::Completed,
        "failed" => crate::domain::DownloadStatus::Failed,
        "cancelled" => crate::domain::DownloadStatus::Cancelled,
        "downloading" => crate::domain::DownloadStatus::Downloading,
        _ => crate::domain::DownloadStatus::Queued,
    }
}
