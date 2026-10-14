use super::dto::{DetailItemDto, DetailResponseDto, SearchItemDto, SearchResponseDto};
use crate::domain::{WallpaperDetail, WallpaperSummary, WallpaperTag, WallpaperUploader};
use crate::error::CoreError;

pub fn map_search_response(dto: SearchResponseDto) -> Result<crate::domain::SearchPage, CoreError> {
    let mut items = Vec::with_capacity(dto.data.len());
    for item in dto.data {
        items.push(map_search_item(item)?);
    }
    Ok(crate::domain::SearchPage {
        items,
        current_page: dto.meta.current_page.unwrap_or(1).max(1),
        last_page: dto.meta.last_page,
        seed: dto.meta.seed,
    })
}

pub fn map_detail_response(dto: DetailResponseDto) -> Result<WallpaperDetail, CoreError> {
    let data = dto.data;
    let summary = map_detail_item_to_summary(&data)?;
    let tags = data
        .tags
        .unwrap_or_default()
        .into_iter()
        .map(|tag| WallpaperTag {
            name: tag.name,
            alias: tag.alias,
            category: tag.category,
        })
        .collect();
    let colors = data.colors.unwrap_or_default();
    let source = non_empty(data.source);
    let uploader = data
        .uploader
        .and_then(|u| u.username)
        .map(|username| WallpaperUploader {
            username: Some(username),
        });
    Ok(WallpaperDetail {
        summary,
        tags,
        colors,
        source,
        uploader,
    })
}

fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.trim().is_empty())
}

fn map_search_item(dto: SearchItemDto) -> Result<WallpaperSummary, CoreError> {
    if dto.id.trim().is_empty() {
        return Err(CoreError::Provider("wallpaper id is empty".to_string()));
    }
    let thumbs = dto.thumbs.unwrap_or_default();
    Ok(WallpaperSummary {
        provider: "wallhaven".to_string(),
        provider_id: dto.id,
        page_url: dto.url,
        image_url: dto.path,
        thumb_small_url: thumbs.small,
        thumb_large_url: thumbs.large,
        width: dto.dimension_x,
        height: dto.dimension_y,
        resolution: dto.resolution,
        file_size: dto.file_size,
        file_type: dto.file_type,
        category: dto.category,
        purity: dto.purity,
        views: dto.views,
        favorites: dto.favorites,
        source_created_at: dto.created_at,
    })
}

fn map_detail_item_to_summary(dto: &DetailItemDto) -> Result<WallpaperSummary, CoreError> {
    if dto.id.trim().is_empty() {
        return Err(CoreError::Provider("wallpaper id is empty".to_string()));
    }
    let thumbs = dto.thumbs.clone().unwrap_or_default();
    Ok(WallpaperSummary {
        provider: "wallhaven".to_string(),
        provider_id: dto.id.clone(),
        page_url: dto.url.clone(),
        image_url: dto.path.clone(),
        thumb_small_url: thumbs.small,
        thumb_large_url: thumbs.large,
        width: dto.dimension_x,
        height: dto.dimension_y,
        resolution: dto.resolution.clone(),
        file_size: dto.file_size,
        file_type: dto.file_type.clone(),
        category: dto.category.clone(),
        purity: dto.purity.clone(),
        views: dto.views,
        favorites: dto.favorites,
        source_created_at: dto.created_at.clone(),
    })
}
