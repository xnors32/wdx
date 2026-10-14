use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResponseDto {
    pub data: Vec<SearchItemDto>,
    pub meta: SearchMetaDto,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ThumbsDto {
    pub small: Option<String>,
    pub large: Option<String>,
    pub original: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchItemDto {
    pub id: String,
    pub url: Option<String>,
    pub views: Option<u64>,
    pub favorites: Option<u64>,
    pub source: Option<String>,
    pub purity: Option<String>,
    pub category: Option<String>,
    pub dimension_x: Option<u32>,
    pub dimension_y: Option<u32>,
    pub resolution: Option<String>,
    pub ratio: Option<String>,
    pub file_size: Option<u64>,
    pub file_type: Option<String>,
    pub created_at: Option<String>,
    pub colors: Option<Vec<String>>,
    pub path: Option<String>,
    pub thumbs: Option<ThumbsDto>,
}

#[derive(Debug, Deserialize)]
pub struct SearchMetaDto {
    pub current_page: Option<u32>,
    pub last_page: Option<u32>,
    pub per_page: Option<u32>,
    pub total: Option<u64>,
    pub query: Option<MetaQueryDto>,
    pub seed: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MetaQueryDto {
    Exact { id: i64, tag: String },
    Text(String),
    Null(Option<()>),
}

#[derive(Debug, Deserialize)]
pub struct DetailResponseDto {
    pub data: DetailItemDto,
}

#[derive(Debug, Deserialize)]
pub struct DetailItemDto {
    pub id: String,
    pub url: Option<String>,
    pub views: Option<u64>,
    pub favorites: Option<u64>,
    pub source: Option<String>,
    pub purity: Option<String>,
    pub category: Option<String>,
    pub dimension_x: Option<u32>,
    pub dimension_y: Option<u32>,
    pub resolution: Option<String>,
    pub ratio: Option<String>,
    pub file_size: Option<u64>,
    pub file_type: Option<String>,
    pub created_at: Option<String>,
    #[allow(dead_code)]
    pub colors: Option<Vec<String>>,
    pub path: Option<String>,
    pub thumbs: Option<ThumbsDto>,
    pub tags: Option<Vec<DetailTagDto>>,
    pub uploader: Option<UploaderDto>,
}

#[derive(Debug, Deserialize)]
pub struct DetailTagDto {
    pub id: Option<i64>,
    pub name: String,
    pub alias: Option<String>,
    pub category: Option<String>,
    #[allow(dead_code)]
    pub category_id: Option<i64>,
    #[allow(dead_code)]
    pub purity: Option<String>,
    #[allow(dead_code)]
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UploaderDto {
    pub username: Option<String>,
    #[allow(dead_code)]
    pub group: Option<String>,
}
