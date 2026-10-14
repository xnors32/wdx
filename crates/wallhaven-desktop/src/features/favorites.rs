use dioxus::prelude::*;

use wallhaven_core::domain::{Favorite, WallpaperDetail};
use wallhaven_core::storage::database::Database;
use wallhaven_core::storage::favorites::FavoriteRepository;
use wallhaven_core::storage::paths::AppPaths;
use wallhaven_core::{CoreError, WallhavenClient};

use crate::strings;

#[derive(Clone, PartialEq)]
enum DetailStatus {
    Idle,
    Loading,
    Loaded(Box<WallpaperDetail>),
    Error(String),
}

fn load_client() -> WallhavenClient {
    let api_key = AppPaths::resolve()
        .ok()
        .and_then(|paths| wallhaven_core::config::read_api_key(&paths.config_dir));
    WallhavenClient::new(api_key).unwrap_or_else(|_| WallhavenClient::new(None).unwrap())
}

fn error_message(error: &CoreError) -> String {
    match error {
        CoreError::Unauthorized => strings::UNAUTHORIZED_ERROR.to_string(),
        CoreError::RateLimited => strings::RATE_LIMITED_ERROR.to_string(),
        CoreError::Network(_) => strings::CONNECTION_ERROR.to_string(),
        _ => strings::CONNECTION_ERROR.to_string(),
    }
}

fn format_bytes(value: Option<u64>) -> String {
    match value {
        Some(bytes) if bytes >= 1024 * 1024 => {
            format!("{:.1} MiB", bytes as f64 / (1024.0 * 1024.0))
        }
        Some(bytes) if bytes >= 1024 => format!("{:.1} KiB", bytes as f64 / 1024.0),
        Some(bytes) => format!("{bytes} B"),
        None => "—".to_string(),
    }
}

fn open_external_url(url: String) {
    if url.starts_with("https://wallhaven.cc/") {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
}

#[component]
pub fn FavoritesView() -> Element {
    let db = use_context::<Database>();
    let mut favorites = use_signal(Vec::<Favorite>::new);
    let mut loaded = use_signal(|| false);

    // Detail popup state
    let mut selected_id = use_signal(|| None::<String>);
    let mut detail_status = use_signal(|| DetailStatus::Idle);
    let mut detail_generation = use_signal(|| 0u64);
    let mut image_failed = use_signal(|| false);

    let db_for_effect = db.clone();
    use_effect(move || {
        if *loaded.read() {
            return;
        }
        loaded.set(true);
        let db = db_for_effect.clone();
        spawn(async move {
            let repo = FavoriteRepository::new(db.connection());
            if let Ok(items) = repo.list() {
                favorites.set(items);
            }
        });
    });

    let mut open_detail = move |id: String| {
        selected_id.set(Some(id.clone()));
        image_failed.set(false);
        let gen = *detail_generation.read() + 1;
        detail_generation.set(gen);
        detail_status.set(DetailStatus::Loading);
        let client = load_client();
        spawn(async move {
            match client.detail(&id).await {
                Ok(detail) => {
                    if *detail_generation.read() == gen {
                        detail_status.set(DetailStatus::Loaded(Box::new(detail)));
                    }
                }
                Err(error) => {
                    if *detail_generation.read() == gen {
                        detail_status.set(DetailStatus::Error(error_message(&error)));
                    }
                }
            }
        });
    };

    let close_detail = move |_: Event<MouseData>| {
        selected_id.set(None);
        detail_status.set(DetailStatus::Idle);
    };

    rsx! {
        div { class: "page-header",
            h2 { class: "page-title", "♥ {strings::FAVORITES}" }
        }
        div { class: "page-body",
            if favorites.read().is_empty() {
                div { class: "state",
                    span { class: "state-icon", "🤍" }
                    span { class: "state-title", "Belum ada favorit" }
                    span { class: "state-desc", "Tambahkan wallpaper favorit dari halaman Jelajahi." }
                }
            } else {
                div { class: "fav-grid",
                    for favorite in favorites.read().clone() {
                        div {
                            class: "fav-card",
                            style: "cursor: pointer;",
                            onclick: {
                                let id = favorite.provider_id.clone();
                                move |_| open_detail(id.clone())
                            },
                            if let Some(url) = favorite.thumb_url.clone() {
                                img { src: "{url}", alt: "Thumbnail", class: "fav-card-thumb" }
                            } else {
                                div { class: "fav-card-thumb",
                                    style: "display:flex;align-items:center;justify-content:center;font-size:2rem;background:var(--bg-input);",
                                    "🖼"
                                }
                            }
                            div { class: "fav-card-body",
                                div { class: "fav-card-info",
                                    div { class: "fav-card-id truncate", "{favorite.provider_id}" }
                                    div { class: "fav-card-res", "{favorite.resolution.clone().unwrap_or_default()}" }
                                }
                                button {
                                    class: "btn btn-danger btn-sm btn-icon",
                                    title: "Hapus dari favorit",
                                    onclick: {
                                        let db = db.clone();
                                        let favorite = favorite.clone();
                                        let mut favorites = favorites;
                                        move |evt: Event<MouseData>| {
                                            evt.stop_propagation();
                                            let db = db.clone();
                                            let provider = favorite.provider.clone();
                                            let provider_id = favorite.provider_id.clone();
                                            spawn(async move {
                                                let repo = FavoriteRepository::new(db.connection());
                                                let _ = repo.remove(&provider, &provider_id);
                                                if let Ok(items) = repo.list() {
                                                    favorites.set(items);
                                                }
                                            });
                                        }
                                    },
                                    "✕"
                                }
                            }
                        }
                    }
                }
            }
        }

        // ── Detail popup ──
        if selected_id.read().is_some() {
            div {
                class: "modal-overlay detail-overlay",
                tabindex: "0",
                autofocus: "true",
                onclick: close_detail,
                onkeydown: move |evt| {
                    if evt.key() == Key::Escape {
                        selected_id.set(None);
                        detail_status.set(DetailStatus::Idle);
                    }
                },
                div {
                    class: "modal detail-modal",
                    onclick: move |evt| evt.stop_propagation(),
                    div { class: "detail-top-bar",
                        p { class: "detail-top-title", "Detail Wallpaper" }
                        button {
                            class: "btn btn-ghost btn-sm",
                            onclick: move |_| {
                                selected_id.set(None);
                                detail_status.set(DetailStatus::Idle);
                            },
                            "✕ Tutup"
                        }
                    }
                    match &*detail_status.read() {
                        DetailStatus::Loading => rsx! {
                            div { class: "state",
                                div { class: "spinner" }
                                span { class: "state-title", "{strings::LOADING}" }
                            }
                        },
                        DetailStatus::Error(message) => rsx! {
                            div { class: "state",
                                span { class: "state-icon", "⚠" }
                                span { class: "state-title", "{message}" }
                                button {
                                    class: "btn btn-secondary btn-sm",
                                    onclick: move |_| {
                                        if let Some(id) = selected_id.read().clone() {
                                            open_detail(id);
                                        }
                                    },
                                    "{strings::RETRY}"
                                }
                            }
                        },
                        DetailStatus::Loaded(detail) => {
                            let detail = detail.clone();
                            rsx! {
                                div { class: "detail-body",
                                    div { class: "detail-image-wrap",
                                        if !*image_failed.read() {
                                            if let Some(url) = detail.summary.thumb_large_url.clone() {
                                                img {
                                                    src: "{url}",
                                                    alt: "Preview",
                                                    onerror: move |_| image_failed.set(true)
                                                }
                                            }
                                        } else {
                                            div { class: "detail-image-failed",
                                                span { "🖼" }
                                                span { "Gagal memuat gambar" }
                                            }
                                        }
                                    }
                                    div { class: "detail-info",
                                        div { class: "detail-meta-grid",
                                            div { class: "meta-chip",
                                                div { class: "meta-chip-label", "ID" }
                                                div { class: "meta-chip-value", "{detail.summary.provider_id}" }
                                            }
                                            div { class: "meta-chip",
                                                div { class: "meta-chip-label", "Resolusi" }
                                                div { class: "meta-chip-value", "{detail.summary.resolution.clone().unwrap_or_default()}" }
                                            }
                                            div { class: "meta-chip",
                                                div { class: "meta-chip-label", "Ukuran" }
                                                div { class: "meta-chip-value", "{format_bytes(detail.summary.file_size)}" }
                                            }
                                            div { class: "meta-chip",
                                                div { class: "meta-chip-label", "Tipe" }
                                                div { class: "meta-chip-value", "{detail.summary.file_type.clone().unwrap_or_default()}" }
                                            }
                                            div { class: "meta-chip",
                                                div { class: "meta-chip-label", "Kategori" }
                                                div { class: "meta-chip-value", "{detail.summary.category.clone().unwrap_or_default()}" }
                                            }
                                            div { class: "meta-chip",
                                                div { class: "meta-chip-label", "Purity" }
                                                div { class: "meta-chip-value", "{detail.summary.purity.clone().unwrap_or_default()}" }
                                            }
                                        }
                                        if !detail.tags.is_empty() {
                                            div { class: "detail-tags",
                                                for tag in detail.tags.iter() {
                                                    span { class: "tag-pill", "#{tag.name}" }
                                                }
                                            }
                                        }
                                        if !detail.colors.is_empty() {
                                            div { class: "detail-colors",
                                                for color_hex in detail.colors.iter() {
                                                    div {
                                                        class: "color-dot",
                                                        style: "background: #{color_hex};",
                                                        title: "#{color_hex}"
                                                    }
                                                }
                                            }
                                        }
                                        div { class: "detail-actions",
                                            if let Some(page_url) = detail.summary.page_url.clone() {
                                                button {
                                                    class: "btn btn-ghost btn-sm",
                                                    onclick: move |_| open_external_url(page_url.clone()),
                                                    "↗ Wallhaven"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        DetailStatus::Idle => rsx! {
                            div { class: "state", div { class: "spinner" } }
                        },
                    }
                }
            }
        }
    }
}
