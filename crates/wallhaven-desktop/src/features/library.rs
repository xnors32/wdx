use dioxus::prelude::*;
use std::path::Path;

use wallhaven_core::domain::DownloadRecord;
use wallhaven_core::storage::database::Database;
use wallhaven_core::storage::downloads::DownloadRepository;

use crate::strings;

#[component]
pub fn LibraryView() -> Element {
    let db = use_context::<Database>();
    let mut records = use_signal(Vec::<DownloadRecord>::new);
    let mut loaded = use_signal(|| false);

    use_effect(move || {
        if *loaded.read() {
            return;
        }
        loaded.set(true);
        let db = db.clone();
        spawn(async move {
            let repo = DownloadRepository::new(db.connection());
            if let Ok(items) = repo.list() {
                records.set(items);
            }
        });
    });

    rsx! {
        div { class: "page-header",
            h2 { class: "page-title", "📁 {strings::LIBRARY}" }
        }
        div { class: "page-body",
            if records.read().is_empty() {
                div { class: "state",
                    span { class: "state-icon", "📂" }
                    span { class: "state-title", "Pustaka kosong" }
                    span { class: "state-desc", "File yang kamu unduh akan dikelola di sini." }
                }
            } else {
                div { class: "item-list",
                    for record in records.read().clone() {
                        {
                            let exists = Path::new(&record.file_path).exists();
                            let completed = record.status == wallhaven_core::domain::DownloadStatus::Completed;
                            rsx! {
                                div { class: "download-item",
                                    div { class: "download-item-icon",
                                        if completed && exists { "✅" }
                                        else if completed && !exists { "⚠" }
                                        else { "🕐" }
                                    }
                                    div { class: "download-item-body",
                                        span { class: "download-item-name", "{record.file_name}" }
                                        span { class: "download-item-meta truncate", "📂 {record.file_path}" }
                                    }
                                    div { class: "download-item-actions",
                                        if completed && exists {
                                            span { class: "badge badge-completed", "tersedia" }
                                        } else if completed && !exists {
                                            span { class: "badge badge-failed", "hilang" }
                                        } else {
                                            span { class: "badge badge-queued", "{record.status:?}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
