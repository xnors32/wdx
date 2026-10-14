use dioxus::prelude::*;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use wallhaven_core::storage::database::Database;
use wallhaven_core::storage::paths::AppPaths;

use crate::features::discover::DiscoverView;
use crate::features::downloads::DownloadsView;
use crate::features::favorites::FavoritesView;
use crate::features::library::LibraryView;
use crate::features::settings::SettingsView;
use crate::strings;

#[derive(Clone)]
pub struct DownloadJob {
    pub id: u64,
    pub provider_id: String,
    pub source_url: String,
    pub file_name: String,
    pub file_path: String,
    pub status: String,
    pub progress: Option<(u64, Option<u64>)>,
    pub error: Option<String>,
    pub cancelled: Arc<AtomicBool>,
}

impl DownloadJob {
    pub fn new(id: u64, provider_id: String, source_url: String) -> Self {
        Self {
            id,
            provider_id: provider_id.clone(),
            source_url: source_url.clone(),
            file_name: format!("wallhaven-{provider_id}.part"),
            file_path: String::new(),
            status: "queued".to_string(),
            progress: None,
            error: None,
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[component]
pub fn App() -> Element {
    let mut active = use_signal(|| "discover".to_string());
    let jobs = use_signal_sync(Vec::<DownloadJob>::new);

    let db = AppPaths::resolve()
        .and_then(|paths| {
            let _ = paths.ensure();
            Database::open(&paths.data_dir.join("wallhaven.db"))
        })
        .unwrap_or_else(|_| Database::in_memory().unwrap());

    let initial_theme = AppPaths::resolve()
        .map(|paths| {
            let _ = paths.ensure();
            wallhaven_core::config::load_settings(&paths.config_dir).theme
        })
        .unwrap_or_else(|_| "dark".to_string());
    let theme = use_signal_sync(|| initial_theme.clone());

    // Provide shared download history to child views.
    use_context_provider(|| db);
    use_context_provider(|| jobs);
    use_context_provider(|| theme);

    rsx! {
        div { class: format!("app theme-{}", theme.read()),
            nav { class: "sidebar",
                div { class: "sidebar-header",
                    div { class: "sidebar-logo", "🖼" }
                    p { class: "sidebar-title", "{strings::APP_TITLE}" }
                }
                ul { class: "sidebar-nav",
                    li {
                        button {
                            class: if *active.read() == "discover" { "nav-item active" } else { "nav-item" },
                            onclick: move |_| active.set("discover".to_string()),
                            span { class: "nav-icon", "🔍" }
                            "{strings::DISCOVER}"
                        }
                    }
                    li {
                        button {
                            class: if *active.read() == "downloads" { "nav-item active" } else { "nav-item" },
                            onclick: move |_| active.set("downloads".to_string()),
                            span { class: "nav-icon", "⬇" }
                            "{strings::DOWNLOADS}"
                        }
                    }
                    li {
                        button {
                            class: if *active.read() == "favorites" { "nav-item active" } else { "nav-item" },
                            onclick: move |_| active.set("favorites".to_string()),
                            span { class: "nav-icon", "♥" }
                            "{strings::FAVORITES}"
                        }
                    }
                    li {
                        button {
                            class: if *active.read() == "library" { "nav-item active" } else { "nav-item" },
                            onclick: move |_| active.set("library".to_string()),
                            span { class: "nav-icon", "📁" }
                            "{strings::LIBRARY}"
                        }
                    }
                    li { class: "divider" }
                    li {
                        button {
                            class: if *active.read() == "settings" { "nav-item active" } else { "nav-item" },
                            onclick: move |_| active.set("settings".to_string()),
                            span { class: "nav-icon", "⚙" }
                            "{strings::SETTINGS}"
                        }
                    }
                }
            }
            main { class: "content",
                match active.read().as_str() {
                    "discover"  => rsx! { DiscoverView {} },
                    "downloads" => rsx! { DownloadsView {} },
                    "favorites" => rsx! { FavoritesView {} },
                    "library"   => rsx! { LibraryView {} },
                    "settings"  => rsx! { SettingsView {} },
                    _ => rsx! { p { "{strings::NOT_AVAILABLE}" } },
                }
            }
        }
    }
}
