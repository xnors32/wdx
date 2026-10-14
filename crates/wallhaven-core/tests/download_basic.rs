use std::fs;
use wallhaven_core::storage::paths::AppPaths;
use wallhaven_core::{run_download, DownloadRequest};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn downloads_valid_image_to_non_ascii_path() {
    let server = MockServer::start().await;
    let body = [0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1a, b'\n', 0, 0, 0];
    Mock::given(method("GET"))
        .and(path("/image.png"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(body))
        .mount(&server)
        .await;

    let destination = std::env::temp_dir().join("wallhaven test Привет");
    let _ = fs::create_dir_all(&destination);

    let source_url = format!("{}/image.png", server.uri());
    let cancel = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let request = DownloadRequest {
        provider: "wallhaven".to_string(),
        provider_id: "abc123".to_string(),
        source_url: source_url.clone(),
        destination_dir: destination.clone(),
        max_bytes: 1024,
    };

    let (updates, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let on_progress = move |progress| {
        let _ = updates.send(progress);
    };

    let http =
        wallhaven_core::WallhavenClient::with_base_url(None, &format!("{}/api/v1/", server.uri()))
            .unwrap()
            .http();
    let record = run_download(http, request, on_progress, cancel).await;

    assert_eq!(
        record.status,
        wallhaven_core::domain::DownloadStatus::Completed
    );
    assert!(std::path::Path::new(&record.file_path).exists());
    assert!(record.file_name.ends_with(".png"));
    let _ = AppPaths::resolve();
    while rx.try_recv().is_ok() {}
}
