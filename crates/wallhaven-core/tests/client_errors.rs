use wallhaven_core::error::CoreError;
use wallhaven_core::providers::wallhaven::WallhavenClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn maps_unauthorized_for_search() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/search"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;
    let client =
        WallhavenClient::with_base_url(None, &format!("{}/api/v1/", server.uri())).unwrap();
    let request = wallhaven_core::domain::SearchRequest {
        query: None,
        page: 1,
        filters: wallhaven_core::domain::SearchFilters::default(),
        seed: None,
    };
    let result = client.search(&request).await;
    assert!(matches!(result, Err(CoreError::Unauthorized)));
}

#[tokio::test]
async fn maps_rate_limited_for_detail() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/w/abc123"))
        .respond_with(ResponseTemplate::new(429))
        .mount(&server)
        .await;
    let client =
        WallhavenClient::with_base_url(None, &format!("{}/api/v1/", server.uri())).unwrap();
    let result = client.wallpaper("abc123").await;
    assert!(matches!(result, Err(CoreError::RateLimited)));
}

#[tokio::test]
async fn maps_server_error_for_detail() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/w/abc123"))
        .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
        .mount(&server)
        .await;
    let client =
        WallhavenClient::with_base_url(None, &format!("{}/api/v1/", server.uri())).unwrap();
    let result = client.wallpaper("abc123").await;
    assert!(matches!(result, Err(CoreError::Provider(_))));
}

#[tokio::test]
async fn maps_malformed_json_for_search() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/search"))
        .respond_with(ResponseTemplate::new(200).set_body_string("{not json"))
        .mount(&server)
        .await;
    let client =
        WallhavenClient::with_base_url(None, &format!("{}/api/v1/", server.uri())).unwrap();
    let request = wallhaven_core::domain::SearchRequest {
        query: None,
        page: 1,
        filters: wallhaven_core::domain::SearchFilters::default(),
        seed: None,
    };
    let result = client.search(&request).await;
    assert!(matches!(result, Err(CoreError::Provider(_))));
}

#[tokio::test]
async fn rejects_invalid_wallpaper_id() {
    let client = WallhavenClient::new(None).unwrap();
    let result = client.wallpaper("../etc/passwd").await;
    assert!(matches!(result, Err(CoreError::InvalidInput(_))));
}
