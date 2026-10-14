use crate::domain::Favorite;
use crate::storage::database::DbConnection;
use crate::CoreError;
use rusqlite::params;

#[derive(Clone)]
pub struct FavoriteRepository {
    connection: DbConnection,
}

impl FavoriteRepository {
    pub fn new(connection: DbConnection) -> Self {
        Self { connection }
    }

    pub fn add(&self, favorite: &Favorite) -> Result<(), CoreError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| CoreError::Storage("lock poisoned".to_string()))?;
        connection
            .execute(
                "INSERT OR REPLACE INTO favorites (provider, provider_id, page_url, thumb_url, resolution, category, purity, added_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    favorite.provider,
                    favorite.provider_id,
                    favorite.page_url,
                    favorite.thumb_url,
                    favorite.resolution,
                    favorite.category,
                    favorite.purity,
                    favorite.added_at,
                ],
            )
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        Ok(())
    }

    pub fn remove(&self, provider: &str, provider_id: &str) -> Result<(), CoreError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| CoreError::Storage("lock poisoned".to_string()))?;
        connection
            .execute(
                "DELETE FROM favorites WHERE provider = ?1 AND provider_id = ?2",
                params![provider, provider_id],
            )
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Favorite>, CoreError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| CoreError::Storage("lock poisoned".to_string()))?;
        let mut statement = connection
            .prepare("SELECT provider, provider_id, page_url, thumb_url, resolution, category, purity, added_at FROM favorites ORDER BY added_at DESC")
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        let rows = statement
            .query_map([], |row| {
                Ok(Favorite {
                    provider: row.get(0)?,
                    provider_id: row.get(1)?,
                    page_url: row.get(2)?,
                    thumb_url: row.get(3)?,
                    resolution: row.get(4)?,
                    category: row.get(5)?,
                    purity: row.get(6)?,
                    added_at: row.get(7)?,
                })
            })
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        let mut favorites = Vec::new();
        for row in rows {
            favorites.push(row.map_err(|error| CoreError::Storage(error.to_string()))?);
        }
        Ok(favorites)
    }

    pub fn contains(&self, provider: &str, provider_id: &str) -> Result<bool, CoreError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| CoreError::Storage("lock poisoned".to_string()))?;
        let count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM favorites WHERE provider = ?1 AND provider_id = ?2",
                params![provider, provider_id],
                |row| row.get(0),
            )
            .map_err(|error| CoreError::Storage(error.to_string()))?;
        Ok(count > 0)
    }
}
