use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus::signals::SyncSignal;
use wallhaven_core::config;
use wallhaven_core::domain::DownloadProgress;
use wallhaven_core::domain::{
    Favorite, SearchFilters, SearchRequest, SortOrder, WallpaperDetail, WallpaperSummary,
};
use wallhaven_core::storage::database::Database;
use wallhaven_core::storage::downloads::DownloadRepository;
use wallhaven_core::storage::favorites::FavoriteRepository;
use wallhaven_core::storage::paths::AppPaths;
use wallhaven_core::{run_download, CoreError, DownloadRequest, WallhavenClient};

use crate::app::DownloadJob;
use crate::strings;

// Accumulated items across pages for infinite scroll
#[derive(Clone, PartialEq)]
struct InfiniteState {
    items: Vec<WallpaperSummary>,
    current_page: u32,
    last_page: Option<u32>,
    seed: Option<String>,
}

#[derive(Clone, PartialEq)]
enum DetailStatus {
    Idle,
    Loading,
    Loaded(Box<WallpaperDetail>),
    Error(String),
}

fn non_empty(value: String) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
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

fn error_message(error: &CoreError) -> String {
    match error {
        CoreError::Unauthorized => strings::UNAUTHORIZED_ERROR.to_string(),
        CoreError::RateLimited => strings::RATE_LIMITED_ERROR.to_string(),
        CoreError::Network(_) => strings::CONNECTION_ERROR.to_string(),
        _ => strings::CONNECTION_ERROR.to_string(),
    }
}

fn load_client() -> WallhavenClient {
    let api_key = AppPaths::resolve()
        .ok()
        .and_then(|paths| wallhaven_core::config::read_api_key(&paths.config_dir));
    WallhavenClient::new(api_key).unwrap_or_else(|_| WallhavenClient::new(None).unwrap())
}

fn timestamp_now() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

#[component]
pub fn DiscoverView() -> Element {
    let mut query = use_signal(String::new);
    let mut categories = use_signal(|| "111".to_string());
    let mut purity = use_signal(|| "100".to_string());
    let mut sorting = use_signal(|| "date_added".to_string());
    let mut order = use_signal(|| "desc".to_string());
    let mut top_range = use_signal(|| "1M".to_string());
    let mut atleast = use_signal(String::new);
    let mut resolutions = use_signal(String::new);
    let mut ratios = use_signal(String::new);
    let mut colors = use_signal(String::new);

    // Pending (belum diterapkan) untuk filter radio-button
    let mut pending_atleast = use_signal(String::new);
    let mut pending_resolutions = use_signal(String::new);
    let mut pending_ratios = use_signal(String::new);
    let mut pending_colors = use_signal(String::new);

    // Infinite scroll state
    let mut inf = use_signal(|| None::<InfiniteState>);
    let mut is_loading = use_signal(|| false);
    let mut is_loading_more = use_signal(|| false);
    let mut load_error = use_signal(|| None::<String>);
    let mut generation = use_signal(|| 0u64);
    let mut did_init = use_signal(|| false);

    let db = use_context::<Database>();
    let mut selected_id = use_signal(|| None::<String>);
    let mut detail_status = use_signal(|| DetailStatus::Idle);
    let mut detail_generation = use_signal(|| 0u64);
    let mut image_failed = use_signal(|| false);
    let mut is_favorite = use_signal(|| false);

    let db_for_fav = db.clone();
    use_effect(move || {
        if let Some(id) = selected_id.read().clone() {
            let db = db_for_fav.clone();
            let mut is_favorite = is_favorite;
            spawn(async move {
                let repo = FavoriteRepository::new(db.connection());
                if let Ok(value) = repo.contains("wallhaven", &id) {
                    is_favorite.set(value);
                }
            });
        } else {
            is_favorite.set(false);
        }
    });

    let mut jobs = use_context::<SyncSignal<Vec<DownloadJob>>>();
    let mut job_counter = use_signal(|| 0u64);

    // Build SearchRequest from current filter signals
    let build_request = move |page: u32, seed: Option<String>| -> SearchRequest {
        let q = query.read().clone();
        let filters = SearchFilters {
            categories: Some(categories.read().clone()),
            purity: Some(purity.read().clone()),
            sorting: Some(sorting.read().clone()),
            order: Some(if *order.read() == "asc" {
                SortOrder::Asc
            } else {
                SortOrder::Desc
            }),
            top_range: if *sorting.read() == "toplist" {
                non_empty(top_range.read().clone())
            } else {
                None
            },
            atleast: non_empty(atleast.read().clone()),
            resolutions: non_empty(resolutions.read().clone()),
            ratios: non_empty(ratios.read().clone()),
            colors: non_empty(colors.read().clone()),
        };
        SearchRequest {
            query: non_empty(q),
            page,
            filters,
            seed: if *sorting.read() == "random" {
                seed
            } else {
                None
            },
        }
    };

    // Fresh search — discards accumulated items, starts from page 1
    let mut search_fresh = move || {
        let gen = *generation.read() + 1;
        generation.set(gen);
        inf.set(None);
        is_loading.set(true);
        is_loading_more.set(false);
        load_error.set(None);

        let request = build_request(1, None);
        let client = load_client();
        spawn(async move {
            match client.search(&request).await {
                Ok(page_data) => {
                    if *generation.read() == gen {
                        inf.set(Some(InfiniteState {
                            items: page_data.items,
                            current_page: 1,
                            last_page: page_data.last_page,
                            seed: page_data.seed,
                        }));
                        is_loading.set(false);
                    }
                }
                Err(e) => {
                    if *generation.read() == gen {
                        load_error.set(Some(error_message(&e)));
                        is_loading.set(false);
                    }
                }
            }
        });
    };

    // Load next page — appends items to the current list
    let mut load_more = move || {
        let state = inf.read().clone();
        let Some(state) = state else { return };
        if *is_loading_more.read() {
            return;
        }
        let next = state.current_page + 1;
        if let Some(last) = state.last_page {
            if next > last {
                return;
            }
        }
        let gen = *generation.read();
        is_loading_more.set(true);

        let request = build_request(next, state.seed.clone());
        let client = load_client();
        spawn(async move {
            match client.search(&request).await {
                Ok(page_data) => {
                    if *generation.read() == gen {
                        let mut current = inf.write();
                        if let Some(ref mut s) = *current {
                            s.items.extend(page_data.items);
                            s.current_page = next;
                            s.last_page = page_data.last_page;
                            if s.seed.is_none() {
                                s.seed = page_data.seed;
                            }
                        }
                        is_loading_more.set(false);
                    }
                }
                Err(_) => {
                    if *generation.read() == gen {
                        is_loading_more.set(false);
                    }
                }
            }
        });
    };

    // Initial load with saved settings
    use_effect(move || {
        if !*did_init.read() {
            if let Ok(paths) = AppPaths::resolve() {
                let settings = config::load_settings(&paths.config_dir);
                categories.set(settings.preferred_categories);
                purity.set(settings.preferred_purity);
            }
            did_init.set(true);
            search_fresh();
        }
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

    let db_button = db.clone();
    let mut start_download = move |source_url: String, provider_id: String| {
        let job_id = *job_counter.read() + 1;
        job_counter.set(job_id);
        let mut job = DownloadJob::new(job_id, provider_id.clone(), source_url.clone());
        job.status = "queued".to_string();
        jobs.write().push(job);

        let cancel = jobs
            .read()
            .iter()
            .find(|j| j.id == job_id)
            .unwrap()
            .cancelled
            .clone();
        let dest_dir = dirs::download_dir().unwrap_or_else(|| PathBuf::from("."));
        let request = DownloadRequest {
            provider: "wallhaven".to_string(),
            provider_id: provider_id.clone(),
            source_url: source_url.clone(),
            destination_dir: dest_dir,
            max_bytes: wallhaven_core::services::download::default_max_bytes(),
        };
        let http = WallhavenClient::new(None).unwrap().http();
        let db = db.clone();
        spawn(async move {
            let on_progress = move |progress: DownloadProgress| match progress {
                DownloadProgress::Started => {
                    if let Some(job) = jobs.write().iter_mut().find(|j| j.id == job_id) {
                        job.status = "downloading".to_string();
                    }
                }
                DownloadProgress::Progress {
                    bytes_received,
                    total_bytes,
                } => {
                    if let Some(job) = jobs.write().iter_mut().find(|j| j.id == job_id) {
                        job.status = "downloading".to_string();
                        job.progress = Some((bytes_received, total_bytes));
                    }
                }
                DownloadProgress::Completed { record } => {
                    if let Some(job) = jobs.write().iter_mut().find(|j| j.id == job_id) {
                        job.status = "completed".to_string();
                        job.file_name = record.file_name.clone();
                        job.file_path = record.file_path.clone();
                        job.progress = None;
                        job.error = None;
                    }
                }
                DownloadProgress::Failed(message) => {
                    if let Some(job) = jobs.write().iter_mut().find(|j| j.id == job_id) {
                        job.status = "failed".to_string();
                        job.error = Some(message);
                    }
                }
                DownloadProgress::Cancelled { .. } => {
                    if let Some(job) = jobs.write().iter_mut().find(|j| j.id == job_id) {
                        job.status = "cancelled".to_string();
                    }
                }
            };
            let record = run_download(http, request, on_progress, cancel).await;
            let repo = DownloadRepository::new(db.connection());
            let _ = repo.save(&record);
        });
    };

    rsx! {
        div { class: "page-header",
            h2 { class: "page-title", "🔍 {strings::DISCOVER}" }
        }
        div { class: "page-body",

            // ── Toolbar ──
            div { class: "toolbar",
                div { class: "search-wrap",
                    span { class: "search-icon", "🔍" }
                    input {
                        class: "search-input",
                        r#type: "search",
                        placeholder: "{strings::SEARCH_PLACEHOLDER}",
                        value: "{query.read()}",
                        oninput: move |evt| query.set(evt.value().clone()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter {
                                search_fresh();
                            }
                        }
                    }
                }
                button { class: "btn btn-primary", onclick: move |_| search_fresh(), "Cari" }
            }

            // ── Filters ──
            div { class: "filter-row",
                div { class: "filter-group",
                    span { class: "filter-label", "Kategori" }
                    div { class: "check-group",
                        label { class: "check-label",
                            input { r#type: "checkbox", checked: categories.read().starts_with('1'), onchange: move |evt| {
                                let mut chars: Vec<char> = categories.read().chars().collect();
                                if chars.len() < 3 { chars = vec!['1','1','1']; }
                                chars[0] = if evt.checked() { '1' } else { '0' };
                                categories.set(chars.into_iter().collect());
                            } }
                            "{strings::CATEGORY_GENERAL}"
                        }
                        label { class: "check-label",
                            input { r#type: "checkbox", checked: categories.read().chars().nth(1) == Some('1'), onchange: move |evt| {
                                let mut chars: Vec<char> = categories.read().chars().collect();
                                if chars.len() < 3 { chars = vec!['1','1','1']; }
                                chars[1] = if evt.checked() { '1' } else { '0' };
                                categories.set(chars.into_iter().collect());
                            } }
                            "{strings::CATEGORY_ANIME}"
                        }
                        label { class: "check-label",
                            input { r#type: "checkbox", checked: categories.read().chars().nth(2) == Some('1'), onchange: move |evt| {
                                let mut chars: Vec<char> = categories.read().chars().collect();
                                if chars.len() < 3 { chars = vec!['1','1','1']; }
                                chars[2] = if evt.checked() { '1' } else { '0' };
                                categories.set(chars.into_iter().collect());
                            } }
                            "{strings::CATEGORY_PEOPLE}"
                        }
                    }
                }

                div { class: "filter-group",
                    span { class: "filter-label", "Purity" }
                    div { class: "check-group",
                        label { class: "check-label",
                            input { r#type: "checkbox", checked: purity.read().starts_with('1'), onchange: move |evt| {
                                let mut chars: Vec<char> = purity.read().chars().collect();
                                if chars.len() < 3 { chars = vec!['1','0','0']; }
                                chars[0] = if evt.checked() { '1' } else { '0' };
                                purity.set(chars.into_iter().collect());
                            } }
                            "{strings::PURITY_SFW}"
                        }
                        label { class: "check-label",
                            input { r#type: "checkbox", checked: purity.read().chars().nth(1) == Some('1'), onchange: move |evt| {
                                let mut chars: Vec<char> = purity.read().chars().collect();
                                if chars.len() < 3 { chars = vec!['1','0','0']; }
                                chars[1] = if evt.checked() { '1' } else { '0' };
                                purity.set(chars.into_iter().collect());
                            } }
                            "{strings::PURITY_SKETCHY}"
                        }
                        label { class: "check-label",
                            input { r#type: "checkbox", checked: purity.read().chars().nth(2) == Some('1'), onchange: move |evt| {
                                let mut chars: Vec<char> = purity.read().chars().collect();
                                if chars.len() < 3 { chars = vec!['1','0','0']; }
                                chars[2] = if evt.checked() { '1' } else { '0' };
                                purity.set(chars.into_iter().collect());
                            } }
                            "{strings::PURITY_NSFW}"
                        }
                    }
                }

                div { class: "filter-group",
                    span { class: "filter-label", "{strings::SORT_LABEL}" }
                    select { class: "filter-select", value: "{sorting.read()}", onchange: move |evt| sorting.set(evt.value().clone()),
                        option { value: "date_added", "Terbaru" }
                        option { value: "relevance", "Relevansi" }
                        option { value: "random", "Acak" }
                        option { value: "views", "Views" }
                        option { value: "favorites", "Favorit" }
                        option { value: "toplist", "Toplist" }
                    }
                }

                div { class: "filter-group",
                    span { class: "filter-label", "{strings::ORDER_LABEL}" }
                    select { class: "filter-select", value: "{order.read()}", onchange: move |evt| order.set(evt.value().clone()),
                        option { value: "desc", "↓ Terbaru" }
                        option { value: "asc", "↑ Terlama" }
                    }
                }

                if *sorting.read() == "toplist" {
                    div { class: "filter-group",
                        span { class: "filter-label", "{strings::TOP_RANGE_LABEL}" }
                        select { class: "filter-select", value: "{top_range.read()}", onchange: move |evt| top_range.set(evt.value().clone()),
                            option { value: "1d", "1 hari" }
                            option { value: "3d", "3 hari" }
                            option { value: "1w", "1 minggu" }
                            option { value: "1M", "1 bulan" }
                            option { value: "3M", "3 bulan" }
                            option { value: "6M", "6 bulan" }
                            option { value: "1y", "1 tahun" }
                        }
                    }
                }

                div { class: "filter-group",
                    span { class: "filter-label", "Resolusi min." }
                    div { class: "radio-group",
                        for (val, label) in [
                            ("", "Semua"),
                            ("1280x720", "720p"),
                            ("1920x1080", "1080p"),
                            ("2560x1440", "1440p"),
                            ("3840x2160", "4K"),
                        ] {
                            label { class: "radio-pill",
                                input {
                                    r#type: "radio",
                                    name: "atleast",
                                    checked: *pending_atleast.read() == val,
                                    onchange: move |_| pending_atleast.set(val.to_string()),
                                }
                                "{label}"
                            }
                        }
                    }
                }
                div { class: "filter-group",
                    span { class: "filter-label", "Resolusi" }
                    div { class: "radio-group",
                        for (val, label) in [
                            ("", "Semua"),
                            ("1280x720", "720p"),
                            ("1920x1080", "1080p"),
                            ("2560x1440", "1440p"),
                            ("3840x2160", "4K"),
                            ("1920x1200", "1920×1200"),
                            ("2560x1080", "UltraWide"),
                        ] {
                            label { class: "radio-pill",
                                input {
                                    r#type: "radio",
                                    name: "resolutions",
                                    checked: *pending_resolutions.read() == val,
                                    onchange: move |_| pending_resolutions.set(val.to_string()),
                                }
                                "{label}"
                            }
                        }
                    }
                }
                div { class: "filter-group",
                    span { class: "filter-label", "Rasio" }
                    div { class: "radio-group",
                        for (val, label) in [
                            ("", "Semua"),
                            ("16x9", "16:9"),
                            ("16x10", "16:10"),
                            ("21x9", "21:9"),
                            ("4x3", "4:3"),
                            ("9x16", "9:16"),
                            ("1x1", "1:1"),
                        ] {
                            label { class: "radio-pill",
                                input {
                                    r#type: "radio",
                                    name: "ratios",
                                    checked: *pending_ratios.read() == val,
                                    onchange: move |_| pending_ratios.set(val.to_string()),
                                }
                                "{label}"
                            }
                        }
                    }
                }
                div { class: "filter-group",
                    span { class: "filter-label", "Warna" }
                    div { class: "radio-group color-radio-group",
                        label { class: "radio-pill",
                            input {
                                r#type: "radio",
                                name: "colors",
                                checked: pending_colors.read().is_empty(),
                                onchange: move |_| pending_colors.set(String::new()),
                            }
                            "Semua"
                        }
                        for (hex, label) in [
                            ("660000", "Merah tua"), ("990000", "Merah"), ("cc0000", "Merah cerah"),
                            ("cc3333", "Mawar"), ("ea4c88", "Pink"), ("993399", "Ungu tua"),
                            ("663399", "Ungu"), ("333399", "Biru tua"), ("0066cc", "Biru"),
                            ("0099cc", "Biru langit"), ("66cccc", "Teal"), ("77cc33", "Hijau muda"),
                            ("669900", "Hijau"), ("336600", "Hijau tua"), ("666600", "Zaitun"),
                            ("999900", "Kuning tua"), ("cccc33", "Kuning"), ("ffff00", "Kuning cerah"),
                            ("ffcc33", "Emas"), ("ff9900", "Oranye"), ("ff6600", "Oranye tua"),
                            ("cc6633", "Coklat muda"), ("996633", "Coklat"), ("663300", "Coklat tua"),
                            ("ffffff", "Putih"), ("e8e8e8", "Abu terang"), ("aaaaaa", "Abu"),
                            ("555555", "Abu gelap"), ("000000", "Hitam"),
                        ] {
                            label {
                                class: if *pending_colors.read() == hex { "color-radio-pill color-radio-pill--selected" } else { "color-radio-pill" },
                                title: "{label}",
                                input {
                                    r#type: "radio",
                                    name: "colors",
                                    checked: *pending_colors.read() == hex,
                                    onchange: move |_| pending_colors.set(hex.to_string()),
                                }
                                span {
                                    class: "color-swatch",
                                    style: "background: #{hex};",
                                }
                            }
                        }
                    }
                }

                div { class: "filter-actions",
                    button {
                        class: "btn btn-primary btn-sm",
                        onclick: move |_| {
                            atleast.set(pending_atleast.read().clone());
                            resolutions.set(pending_resolutions.read().clone());
                            ratios.set(pending_ratios.read().clone());
                            colors.set(pending_colors.read().clone());
                            search_fresh();
                        },
                        "✓ Terapkan"
                    }
                    button { class: "btn btn-ghost btn-sm", onclick: move |_| {
                        categories.set("111".to_string());
                        purity.set("100".to_string());
                        sorting.set("date_added".to_string());
                        order.set("desc".to_string());
                        top_range.set("1M".to_string());
                        atleast.set(String::new());
                        resolutions.set(String::new());
                        ratios.set(String::new());
                        colors.set(String::new());
                        pending_atleast.set(String::new());
                        pending_resolutions.set(String::new());
                        pending_ratios.set(String::new());
                        pending_colors.set(String::new());
                        query.set(String::new());
                        search_fresh();
                    }, "↺ {strings::RESET}" }
                }
            }

            // ── Detail modal (popup) ──
            if selected_id.read().is_some() {
                div {
                    class: "modal-overlay detail-overlay",
                    tabindex: "0",
                    autofocus: "true",
                    // Klik backdrop = tutup
                    onclick: move |_| {
                        selected_id.set(None);
                        detail_status.set(DetailStatus::Idle);
                    },
                    onkeydown: move |evt| {
                        if evt.key() == Key::Escape {
                            selected_id.set(None);
                            detail_status.set(DetailStatus::Idle);
                        }
                    },
                    div {
                        class: "modal detail-modal",
                        // Cegah klik di dalam modal menutup overlay
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
                                let download_source = detail.summary.image_url.clone();
                                let download_id = detail.summary.provider_id.clone();
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
                                                if let Some(source_url) = download_source.clone() {
                                                    button {
                                                        class: "btn btn-primary btn-sm",
                                                        onclick: move |_| start_download(source_url.clone(), download_id.clone()),
                                                        "⬇ Unduh"
                                                    }
                                                }
                                                button {
                                                    class: if *is_favorite.read() { "btn btn-danger btn-sm" } else { "btn btn-secondary btn-sm" },
                                                    onclick: move |_| {
                                                        let db = db_button.clone();
                                                        let detail = detail.clone();
                                                        let currently = *is_favorite.read();
                                                        let mut is_favorite = is_favorite;
                                                        spawn(async move {
                                                            let repo = FavoriteRepository::new(db.connection());
                                                            if currently {
                                                                let _ = repo.remove("wallhaven", &detail.summary.provider_id);
                                                            } else {
                                                                let favorite = Favorite {
                                                                    provider: detail.summary.provider.clone(),
                                                                    provider_id: detail.summary.provider_id.clone(),
                                                                    page_url: detail.summary.page_url.clone(),
                                                                    thumb_url: detail.summary.thumb_small_url.clone(),
                                                                    resolution: detail.summary.resolution.clone(),
                                                                    category: detail.summary.category.clone(),
                                                                    purity: detail.summary.purity.clone(),
                                                                    added_at: timestamp_now(),
                                                                };
                                                                let _ = repo.add(&favorite);
                                                            }
                                                            if let Ok(value) = repo.contains("wallhaven", &detail.summary.provider_id) {
                                                                is_favorite.set(value);
                                                            }
                                                        });
                                                    },
                                                    if *is_favorite.read() { "♥ Hapus Favorit" } else { "♡ Favorit" }
                                                }
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

            // ── Results ──
            if *is_loading.read() {
                div { class: "state",
                    div { class: "spinner" }
                    span { class: "state-title", "{strings::LOADING}" }
                }
            } else if let Some(err) = load_error.read().clone() {
                div { class: "state",
                    span { class: "state-icon", "⚠" }
                    span { class: "state-title", "{err}" }
                    button { class: "btn btn-secondary", onclick: move |_| search_fresh(), "{strings::RETRY}" }
                }
            } else if let Some(state) = inf.read().clone() {
                if state.items.is_empty() {
                    div { class: "state",
                        span { class: "state-icon", "🔍" }
                        span { class: "state-title", "Tidak ada hasil" }
                        span { class: "state-desc", "{strings::EMPTY}" }
                    }
                } else {
                    div { class: "card-grid",
                        for item in state.items.clone() {
                            div {
                                class: "card",
                                onclick: move |_| open_detail(item.provider_id.clone()),
                                if let Some(url) = item.thumb_small_url.clone() {
                                    img { src: "{url}", alt: "{strings::THUMB_ALT}" }
                                } else {
                                    div { class: "card-placeholder", "🖼" }
                                }
                                div { class: "card-meta",
                                    span { class: "card-id", "{item.provider_id}" }
                                    span { class: "card-res", "{item.resolution.clone().unwrap_or_default()}" }
                                }
                            }
                        }
                    }
                    // Sentinel: IntersectionObserver fires click() when it enters viewport
                    {
                        let has_more = state.last_page
                            .map(|last| state.current_page < last)
                            .unwrap_or(false);
                        rsx! {
                            div {
                                id: "scroll-sentinel",
                                style: "height: 1px;",
                                onmounted: move |_| {
                                    spawn(async move {
                                        let _ = document::eval(r#"
                                            (function() {
                                                var el = document.getElementById('scroll-sentinel');
                                                if (!el) return;
                                                if (el._observer) el._observer.disconnect();
                                                var obs = new IntersectionObserver(function(entries) {
                                                    if (entries[0].isIntersecting) { el.click(); }
                                                }, { rootMargin: '300px' });
                                                obs.observe(el);
                                                el._observer = obs;
                                            })();
                                        "#).await;
                                    });
                                },
                                onclick: move |_| {
                                    if has_more && !*is_loading_more.read() {
                                        load_more();
                                    }
                                }
                            }
                            if *is_loading_more.read() {
                                div { class: "state", style: "padding: 1.5rem;",
                                    div { class: "spinner" }
                                }
                            } else if !has_more {
                                div { class: "state", style: "padding: 1rem;",
                                    span { style: "font-size:0.8rem;color:var(--text-dim);",
                                        "— Semua wallpaper sudah dimuat —"
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
