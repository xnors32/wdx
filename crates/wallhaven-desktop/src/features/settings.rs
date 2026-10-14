use dioxus::prelude::*;
use dioxus::signals::SyncSignal;

use wallhaven_core::config::{self, UserSettings};
use wallhaven_core::storage::paths::AppPaths;

use crate::strings;

fn mask_key(key: &str) -> String {
    if key.len() <= 4 {
        "****".to_string()
    } else {
        format!("****{}", &key[key.len() - 4..])
    }
}

fn non_empty(value: String) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}

#[component]
pub fn SettingsView() -> Element {
    let mut api_key_input = use_signal(String::new);
    let mut has_key = use_signal(|| false);
    let mut key_masked = use_signal(String::new);
    let mut config_dir = use_signal(|| None::<std::path::PathBuf>);
    let mut download_dir = use_signal(String::new);
    let mut categories = use_signal(|| "111".to_string());
    let mut purity = use_signal(|| "100".to_string());
    let mut blur_nsfw = use_signal(|| true);
    let mut theme_pref = use_signal(|| "dark".to_string());
    let mut theme = use_context::<SyncSignal<String>>();
    let mut message = use_signal(String::new);
    let mut loaded = use_signal(|| false);
    let mut confirm_bit = use_signal(|| None::<usize>);

    use_effect(move || {
        if *loaded.read() {
            return;
        }
        loaded.set(true);
        if let Ok(paths) = AppPaths::resolve() {
            let _ = paths.ensure();
            config_dir.set(Some(paths.config_dir.clone()));
            let settings = config::load_settings(&paths.config_dir);
            categories.set(settings.preferred_categories);
            purity.set(settings.preferred_purity);
            download_dir.set(settings.download_dir.unwrap_or_default());
            blur_nsfw.set(settings.blur_nsfw_thumbnails);
            theme_pref.set(settings.theme);
            if config::has_api_key(&paths.config_dir) {
                has_key.set(true);
                if let Some(key) = config::read_api_key(&paths.config_dir) {
                    key_masked.set(mask_key(&key));
                }
            }
        }
    });

    let save_settings = move |_| {
        if let Some(dir) = config_dir.read().clone() {
            let new_key = api_key_input.read().trim().to_string();
            if !new_key.is_empty() {
                if let Err(error) = config::save_api_key(&dir, &new_key) {
                    message.set(error.to_string());
                    return;
                }
                has_key.set(true);
                key_masked.set(mask_key(&new_key));
                api_key_input.set(String::new());
            }
            let new_theme = theme_pref.read().clone();
            let settings = UserSettings {
                preferred_categories: categories.read().clone(),
                preferred_purity: purity.read().clone(),
                download_dir: non_empty(download_dir.read().clone()),
                blur_nsfw_thumbnails: *blur_nsfw.read(),
                theme: new_theme.clone(),
            };
            if let Err(error) = config::save_settings(&dir, &settings) {
                message.set(error.to_string());
                return;
            }
            // Apply theme live
            *theme.write() = new_theme;
            message.set("Pengaturan disimpan.".to_string());
        }
    };

    let delete_key = move |_| {
        if let Some(dir) = config_dir.read().clone() {
            let _ = config::delete_api_key(&dir);
            has_key.set(false);
            key_masked.set(String::new());
            message.set("API key dihapus.".to_string());
        }
    };

    let browse = move |_| {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            download_dir.set(folder.to_string_lossy().to_string());
        }
    };

    rsx! {
        div { class: "page-header",
            h2 { class: "page-title", "⚙ {strings::SETTINGS}" }
        }
        div { class: "page-body",
            div { class: "settings-layout",

                // ── API Key ──────────────────────────────────────────────────
                div { class: "settings-section",
                    div { class: "settings-section-header",
                        span { class: "settings-section-icon", "🔑" }
                        h3 { class: "settings-section-title", "API Key Wallhaven" }
                    }
                    div { class: "settings-section-body",
                        if *has_key.read() {
                            div { class: "key-display",
                                span { class: "key-dot", "●" }
                                span { "{key_masked.read()}" }
                            }
                        } else {
                            p { style: "font-size:0.825rem;color:var(--text-muted);margin:0;",
                                "Belum ada API key. NSFW membutuhkan key yang valid."
                            }
                        }
                        div { class: "settings-field",
                            label { class: "field-label", "Masukkan API key baru" }
                            input {
                                class: "text-input",
                                r#type: "password",
                                placeholder: "Paste API key dari wallhaven.cc...",
                                value: "{api_key_input.read()}",
                                oninput: move |evt| api_key_input.set(evt.value().clone())
                            }
                        }
                        div { class: "inline-row",
                            button { class: "btn btn-primary btn-sm", onclick: save_settings, "💾 Simpan Key" }
                            if *has_key.read() {
                                button { class: "btn btn-danger btn-sm", onclick: delete_key, "🗑 Hapus Key" }
                            }
                        }
                    }
                }

                // ── Download Directory ────────────────────────────────────
                div { class: "settings-section",
                    div { class: "settings-section-header",
                        span { class: "settings-section-icon", "📂" }
                        h3 { class: "settings-section-title", "Direktori Unduhan" }
                    }
                    div { class: "settings-section-body",
                        div { class: "settings-field",
                            label { class: "field-label", "Lokasi penyimpanan" }
                            span { class: "field-hint", "Biarkan kosong untuk menggunakan folder Downloads default." }
                            div { class: "inline-row",
                                input {
                                    class: "text-input",
                                    value: "{download_dir.read()}",
                                    oninput: move |evt| download_dir.set(evt.value().clone()),
                                    placeholder: "Kosong = folder Downloads"
                                }
                                button { class: "btn btn-secondary btn-sm", onclick: browse, "📁 Pilih" }
                            }
                        }
                    }
                }

                // ── Content Preferences ───────────────────────────────────
                div { class: "settings-section",
                    div { class: "settings-section-header",
                        span { class: "settings-section-icon", "🎨" }
                        h3 { class: "settings-section-title", "Preferensi Konten" }
                    }
                    div { class: "settings-section-body",
                        div { class: "settings-field",
                            label { class: "field-label", "Kategori default" }
                            div { class: "check-grid",
                                label { class: "check-pill",
                                    input { r#type: "checkbox", checked: categories.read().starts_with('1'), onchange: move |evt| {
                                        let mut chars: Vec<char> = categories.read().chars().collect();
                                        if chars.len() < 3 { chars = vec!['1','1','1']; }
                                        chars[0] = if evt.checked() { '1' } else { '0' };
                                        categories.set(chars.into_iter().collect());
                                    } }
                                    "General"
                                }
                                label { class: "check-pill",
                                    input { r#type: "checkbox", checked: categories.read().chars().nth(1) == Some('1'), onchange: move |evt| {
                                        let mut chars: Vec<char> = categories.read().chars().collect();
                                        if chars.len() < 3 { chars = vec!['1','1','1']; }
                                        chars[1] = if evt.checked() { '1' } else { '0' };
                                        categories.set(chars.into_iter().collect());
                                    } }
                                    "Anime"
                                }
                                label { class: "check-pill",
                                    input { r#type: "checkbox", checked: categories.read().chars().nth(2) == Some('1'), onchange: move |evt| {
                                        let mut chars: Vec<char> = categories.read().chars().collect();
                                        if chars.len() < 3 { chars = vec!['1','1','1']; }
                                        chars[2] = if evt.checked() { '1' } else { '0' };
                                        categories.set(chars.into_iter().collect());
                                    } }
                                    "People"
                                }
                            }
                        }

                        div { class: "settings-field",
                            label { class: "field-label", "Purity default" }
                            div { class: "check-grid",
                                label { class: "check-pill",
                                    input { r#type: "checkbox", checked: purity.read().starts_with('1'), onchange: move |evt| {
                                        let mut chars: Vec<char> = purity.read().chars().collect();
                                        if chars.len() < 3 { chars = vec!['1','0','0']; }
                                        chars[0] = if evt.checked() { '1' } else { '0' };
                                        purity.set(chars.into_iter().collect());
                                    } }
                                    "{strings::PURITY_SFW}"
                                }
                                label { class: "check-pill",
                                    input { r#type: "checkbox", checked: purity.read().chars().nth(1) == Some('1'), onchange: move |evt| {
                                        let old = purity.read().chars().nth(1) == Some('1');
                                        let mut chars: Vec<char> = purity.read().chars().collect();
                                        if chars.len() < 3 { chars = vec!['1','0','0']; }
                                        chars[1] = if evt.checked() { '1' } else { '0' };
                                        purity.set(chars.into_iter().collect());
                                        if evt.checked() && !old { confirm_bit.set(Some(1)); }
                                    } }
                                    "{strings::PURITY_SKETCHY}"
                                }
                                label { class: "check-pill",
                                    input {
                                        r#type: "checkbox",
                                        disabled: !*has_key.read(),
                                        checked: purity.read().chars().nth(2) == Some('1'),
                                        onchange: move |evt| {
                                            let old = purity.read().chars().nth(2) == Some('1');
                                            let mut chars: Vec<char> = purity.read().chars().collect();
                                            if chars.len() < 3 { chars = vec!['1','0','0']; }
                                            chars[2] = if evt.checked() { '1' } else { '0' };
                                            purity.set(chars.into_iter().collect());
                                            if evt.checked() && !old { confirm_bit.set(Some(2)); }
                                        }
                                    }
                                    if !*has_key.read() { "🔒 {strings::PURITY_NSFW}" } else { "{strings::PURITY_NSFW}" }
                                }
                            }
                            if !*has_key.read() {
                                span { class: "field-hint", "🔒 NSFW membutuhkan API key yang valid." }
                            }
                        }

                        div { class: "settings-field",
                            label { class: "check-pill",
                                input { r#type: "checkbox", checked: *blur_nsfw.read(), onchange: move |evt| blur_nsfw.set(evt.checked()) }
                                "🌫 Blur thumbnail non-SFW"
                            }
                        }
                    }
                }

                // ── Appearance ────────────────────────────────────────────
                div { class: "settings-section",
                    div { class: "settings-section-header",
                        span { class: "settings-section-icon", "🌙" }
                        h3 { class: "settings-section-title", "Tampilan" }
                    }
                    div { class: "settings-section-body",
                        div { class: "settings-field",
                            label { class: "field-label", "Tema" }
                            div { class: "check-grid",
                                label { class: "check-pill",
                                    input { r#type: "radio", name: "theme", value: "dark",
                                        checked: *theme_pref.read() == "dark",
                                        onchange: move |_| theme_pref.set("dark".to_string())
                                    }
                                    "🌙 Gelap"
                                }
                                label { class: "check-pill",
                                    input { r#type: "radio", name: "theme", value: "light",
                                        checked: *theme_pref.read() == "light",
                                        onchange: move |_| theme_pref.set("light".to_string())
                                    }
                                    "☀ Terang"
                                }
                            }
                        }
                    }
                }

                // ── Save button + message ─────────────────────────────────
                div { class: "inline-row",
                    button { class: "btn btn-primary", onclick: save_settings, "💾 Simpan Pengaturan" }
                }

                if !message.read().is_empty() {
                    div { class: "settings-message", "✓ {message.read()}" }
                }
            }
        }

        // ── Confirmation modal ────────────────────────────────────────────
        if let Some(bit) = *confirm_bit.read() {
            div { class: "modal-overlay",
                div { class: "modal",
                    p { class: "modal-title", "⚠ Konten Sensitif" }
                    p { class: "modal-body",
                        "Mengaktifkan konten sensitif atau dewasa mungkin tidak pantas untuk semua usia. Pastikan kamu berada di lingkungan yang tepat."
                    }
                    div { class: "modal-actions",
                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| confirm_bit.set(None),
                            "Saya mengerti"
                        }
                        button {
                            class: "btn btn-danger",
                            onclick: move |_| {
                                let mut chars: Vec<char> = purity.read().chars().collect();
                                if chars.len() < 3 { chars = vec!['1','0','0']; }
                                if bit == 1 { chars[1] = '0'; }
                                if bit == 2 { chars[2] = '0'; }
                                purity.set(chars.into_iter().collect());
                                confirm_bit.set(None);
                            },
                            "Batalkan"
                        }
                    }
                }
            }
        }
    }
}
