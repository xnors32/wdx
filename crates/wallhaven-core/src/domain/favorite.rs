use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Favorite {
    pub provider: String,
    pub provider_id: String,
    pub page_url: Option<String>,
    pub thumb_url: Option<String>,
    pub resolution: Option<String>,
    pub category: Option<String>,
    pub purity: Option<String>,
    pub added_at: String,
}
