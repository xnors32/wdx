use dioxus::prelude::*;
use dioxus::signals::SyncSignal;

use crate::app::DownloadJob;
use crate::strings;

#[component]
pub fn DownloadsView() -> Element {
    let jobs = use_context::<SyncSignal<Vec<DownloadJob>>>();

    rsx! {
        div { class: "page-header",
            h2 { class: "page-title", "⬇ {strings::DOWNLOADS}" }
        }
        div { class: "page-body",
            if jobs.read().is_empty() {
                div { class: "state",
                    span { class: "state-icon", "📭" }
                    span { class: "state-title", "Belum ada unduhan" }
                    span { class: "state-desc", "Unduhan akan muncul di sini saat kamu mengunduh wallpaper dari Jelajahi." }
                }
            } else {
                div { class: "item-list",
                    for job in jobs.read().clone() {
                        div { class: "download-item",
                            // Status icon
                            div { class: "download-item-icon",
                                if job.status == "completed" { "✅" }
                                else if job.status == "failed" { "❌" }
                                else if job.status == "cancelled" { "⛔" }
                                else if job.status == "downloading" { "⬇" }
                                else { "🕐" }
                            }

                            div { class: "download-item-body",
                                span { class: "download-item-name", "{job.file_name}" }
                                // Progress bar for active downloads
                                if let Some((received, total)) = job.progress {
                                    {
                                        let pct = total.map(|t| if t > 0 { (received * 100 / t).min(100) } else { 0 }).unwrap_or(0);
                                        let total_str = total.map(|t| format!("{} KiB", t / 1024)).unwrap_or_else(|| "?".to_string());
                                        let recv_kib = received / 1024;
                                        rsx! {
                                            div { class: "progress-wrap",
                                                div { class: "progress-bar", style: "width:{pct}%;" }
                                            }
                                            span { class: "download-item-meta",
                                                "{recv_kib} KiB / {total_str}"
                                            }
                                        }
                                    }
                                }
                                if let Some(path) = (!job.file_path.is_empty()).then_some(job.file_path.clone()) {
                                    span { class: "download-item-meta", "📂 {path}" }
                                }
                                if let Some(error) = job.error.clone() {
                                    span { class: "download-item-meta status-error", "⚠ {error}" }
                                }
                            }

                            div { class: "download-item-actions",
                                // Status badge
                                {
                                    let badge_class = match job.status.as_str() {
                                        "completed"   => "badge badge-completed",
                                        "failed"      => "badge badge-failed",
                                        "cancelled"   => "badge badge-cancelled",
                                        "downloading" => "badge badge-downloading",
                                        _             => "badge badge-queued",
                                    };
                                    rsx! { span { class: "{badge_class}", "{job.status}" } }
                                }
                                if job.status == "downloading" {
                                    button {
                                        class: "btn btn-ghost btn-sm",
                                        onclick: move |_| job.cancelled.store(true, std::sync::atomic::Ordering::Relaxed),
                                        "✕"
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
