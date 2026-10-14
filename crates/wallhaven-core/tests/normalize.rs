use wallhaven_core::domain::{SearchFilters, SearchRequest};
use wallhaven_core::normalize_search;

#[test]
fn removes_nsfw_bit_without_api_key() {
    let request = SearchRequest {
        query: None,
        page: 1,
        filters: SearchFilters {
            purity: Some("111".to_string()),
            ..Default::default()
        },
        seed: None,
    };
    let params = normalize_search(&request, false);
    assert!(params.contains(&("purity".to_string(), "110".to_string())));
}

#[test]
fn sends_top_range_only_for_toplist() {
    let request = SearchRequest {
        query: Some("anime".to_string()),
        page: 2,
        filters: SearchFilters {
            sorting: Some("toplist".to_string()),
            top_range: Some("1w".to_string()),
            ..Default::default()
        },
        seed: None,
    };
    let params = normalize_search(&request, true);
    assert!(params.contains(&("topRange".to_string(), "1w".to_string())));
    assert!(params.contains(&("page".to_string(), "2".to_string())));
}

#[test]
fn ignores_top_range_for_other_sorting() {
    let request = SearchRequest {
        query: None,
        page: 1,
        filters: SearchFilters {
            sorting: Some("date_added".to_string()),
            top_range: Some("1w".to_string()),
            ..Default::default()
        },
        seed: None,
    };
    let params = normalize_search(&request, true);
    assert!(!params.iter().any(|(key, _)| key == "topRange"));
}

#[test]
fn sends_seed_only_for_random() {
    let request = SearchRequest {
        query: None,
        page: 1,
        filters: SearchFilters {
            sorting: Some("random".to_string()),
            ..Default::default()
        },
        seed: Some("a1B2c3".to_string()),
    };
    let params = normalize_search(&request, true);
    assert!(params.contains(&("seed".to_string(), "a1B2c3".to_string())));
}
