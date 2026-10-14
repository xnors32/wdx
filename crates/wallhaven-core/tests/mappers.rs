use wallhaven_core::domain::WallpaperDetail;
use wallhaven_core::providers::wallhaven::dto::{DetailResponseDto, SearchResponseDto};
use wallhaven_core::providers::wallhaven::mapper;

#[test]
fn maps_search_fixture() {
    let json = include_str!("fixtures/wallhaven/search_page.json");
    let dto: SearchResponseDto = serde_json::from_str(json).unwrap();
    let page = mapper::map_search_response(dto).unwrap();
    assert_eq!(page.items.len(), 2);
    assert_eq!(page.current_page, 1);
    assert_eq!(page.last_page, Some(36));
    assert_eq!(page.seed, None);
    assert_eq!(page.items[0].provider_id, "abc123");
    assert_eq!(
        page.items[0].thumb_small_url.as_deref(),
        Some("https://th.wallhaven.cc/small/ab/abc123.jpg")
    );
    assert_eq!(page.items[1].purity.as_deref(), Some("sketchy"));
}

#[test]
fn maps_polymorphic_meta_fixture() {
    let json = include_str!("fixtures/wallhaven/search_exact_tag_random.json");
    let dto: SearchResponseDto = serde_json::from_str(json).unwrap();
    let page = mapper::map_search_response(dto).unwrap();
    assert!(page.items.is_empty());
    assert_eq!(page.current_page, 1);
    assert_eq!(page.last_page, Some(1));
    assert_eq!(page.seed.as_deref(), Some("a1B2c3"));
}

#[test]
fn maps_detail_fixture() {
    let json = include_str!("fixtures/wallhaven/wallpaper_detail.json");
    let dto: DetailResponseDto = serde_json::from_str(json).unwrap();
    let detail: WallpaperDetail = mapper::map_detail_response(dto).unwrap();
    assert_eq!(detail.summary.provider_id, "abc123");
    assert_eq!(detail.tags.len(), 1);
    assert_eq!(detail.tags[0].name, "anime");
    assert_eq!(detail.colors.len(), 3);
    assert_eq!(
        detail.uploader.as_ref().and_then(|u| u.username.as_deref()),
        Some("example-user")
    );
}
