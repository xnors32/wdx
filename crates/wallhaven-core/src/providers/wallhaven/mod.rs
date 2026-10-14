pub mod client;
pub mod dto;
pub mod mapper;
pub mod normalize;

pub use client::WallhavenClient;
pub use normalize::{is_valid_color, normalize_search, ALLOWED_COLORS};
