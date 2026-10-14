use wallhaven_core::domain::Favorite;
use wallhaven_core::storage::database::Database;
use wallhaven_core::storage::downloads::DownloadRepository;
use wallhaven_core::storage::favorites::FavoriteRepository;

#[test]
fn favorites_round_trip() {
    let db = Database::in_memory().unwrap();
    let repo = FavoriteRepository::new(db.connection());
    let favorite = Favorite {
        provider: "wallhaven".to_string(),
        provider_id: "abc123".to_string(),
        page_url: Some("https://wallhaven.cc/w/abc123".to_string()),
        thumb_url: None,
        resolution: Some("1920x1080".to_string()),
        category: Some("anime".to_string()),
        purity: Some("100".to_string()),
        added_at: "1".to_string(),
    };
    repo.add(&favorite).unwrap();
    assert!(repo.contains("wallhaven", "abc123").unwrap());
    let items = repo.list().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].provider_id, "abc123");
    repo.remove("wallhaven", "abc123").unwrap();
    assert!(!repo.contains("wallhaven", "abc123").unwrap());
}

#[test]
fn downloads_repository_lists_saved_record() {
    let db = Database::in_memory().unwrap();
    let repo = DownloadRepository::new(db.connection());
    // Build and execute a tiny valid PNG download via local HTTP is not available;
    // instead save a completed record directly through repository.
    let record = wallhaven_core::domain::DownloadRecord {
        id: None,
        provider: "wallhaven".to_string(),
        provider_id: "abc123".to_string(),
        source_url: Some("https://w.wallhaven.cc/x.png".to_string()),
        file_path: "/tmp/wallhaven-abc123.png".to_string(),
        file_name: "wallhaven-abc123.png".to_string(),
        byte_size: Some(8),
        status: wallhaven_core::domain::DownloadStatus::Completed,
        error_summary: None,
        created_at: "1".to_string(),
        completed_at: Some("2".to_string()),
    };
    let id = repo.save(&record).unwrap();
    assert!(id > 0);
    let items = repo.list().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].status,
        wallhaven_core::domain::DownloadStatus::Completed
    );
}
