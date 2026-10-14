use crate::CoreError;
use rusqlite::Connection;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub type DbConnection = Arc<Mutex<Connection>>;

const MIGRATIONS: [&str; 1] = [r#"
    CREATE TABLE IF NOT EXISTS favorites (
        provider       TEXT NOT NULL,
        provider_id    TEXT NOT NULL,
        page_url       TEXT,
        thumb_url      TEXT,
        resolution     TEXT,
        category       TEXT,
        purity         TEXT,
        added_at       TEXT NOT NULL,
        PRIMARY KEY (provider, provider_id)
    );

    CREATE TABLE IF NOT EXISTS downloads (
        id             INTEGER PRIMARY KEY,
        provider       TEXT NOT NULL,
        provider_id    TEXT NOT NULL,
        source_url     TEXT,
        file_path      TEXT NOT NULL,
        file_name      TEXT NOT NULL,
        byte_size      INTEGER,
        status         TEXT NOT NULL,
        error_summary  TEXT,
        created_at     TEXT NOT NULL,
        completed_at   TEXT
    );
    CREATE UNIQUE INDEX IF NOT EXISTS idx_downloads_file_path ON downloads(file_path);
    CREATE INDEX IF NOT EXISTS idx_downloads_provider_id ON downloads(provider, provider_id);
    "#];

#[derive(Clone)]
pub struct Database {
    connection: DbConnection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, CoreError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                CoreError::Storage(format!("cannot create database directory: {error}"))
            })?;
        }
        let connection =
            Connection::open(path).map_err(|error| CoreError::Storage(error.to_string()))?;
        Self::prepare(connection)
    }

    pub fn in_memory() -> Result<Self, CoreError> {
        let connection =
            Connection::open_in_memory().map_err(|error| CoreError::Storage(error.to_string()))?;
        Self::prepare(connection)
    }

    fn prepare(mut connection: Connection) -> Result<Self, CoreError> {
        connection
            .execute_batch("PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 5000;")
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        migrate(&mut connection)?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub fn connection(&self) -> DbConnection {
        self.connection.clone()
    }
}

fn migrate(connection: &mut Connection) -> Result<(), CoreError> {
    let current_version: i32 = connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|error| CoreError::Storage(error.to_string()))?;

    for (index, sql) in MIGRATIONS.iter().enumerate() {
        let version = (index + 1) as i32;
        if version <= current_version {
            continue;
        }
        let transaction = connection
            .transaction()
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        transaction
            .execute_batch(sql)
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        transaction
            .execute_batch(&format!("PRAGMA user_version = {version}"))
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        transaction
            .commit()
            .map_err(|error| CoreError::Storage(error.to_string()))?;
    }
    Ok(())
}
