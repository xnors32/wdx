use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SortOrder {
    #[default]
    Desc,
    Asc,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SearchFilters {
    pub categories: Option<String>,
    pub purity: Option<String>,
    pub sorting: Option<String>,
    pub order: Option<SortOrder>,
    pub top_range: Option<String>,
    pub atleast: Option<String>,
    pub resolutions: Option<String>,
    pub ratios: Option<String>,
    pub colors: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub page: u32,
    pub filters: SearchFilters,
    pub seed: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchPage {
    pub items: Vec<super::wallpaper::WallpaperSummary>,
    pub current_page: u32,
    pub last_page: Option<u32>,
    pub seed: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ContentPurity {
    #[default]
    Sfw,
    Sketchy,
    Nsfw,
}
