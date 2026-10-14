use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WallpaperSummary {
    pub provider: String,
    pub provider_id: String,
    pub page_url: Option<String>,
    pub image_url: Option<String>,
    pub thumb_small_url: Option<String>,
    pub thumb_large_url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub resolution: Option<String>,
    pub file_size: Option<u64>,
    pub file_type: Option<String>,
    pub category: Option<String>,
    pub purity: Option<String>,
    pub views: Option<u64>,
    pub favorites: Option<u64>,
    pub source_created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WallpaperTag {
    pub name: String,
    pub alias: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WallpaperUploader {
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WallpaperDetail {
    pub summary: WallpaperSummary,
    pub tags: Vec<WallpaperTag>,
    pub colors: Vec<String>,
    pub source: Option<String>,
    pub uploader: Option<WallpaperUploader>,
}
