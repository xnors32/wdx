# Catatan Keputusan (ADR ringan)

Keputusan di sini menjadi default sampai diubah dengan entri baru. Status:
- **Accepted**: disepakati pemilik proyek.
- **Proposed**: diusulkan pada revisi dokumen 2026-10-14; berlaku sebagai default dan dapat diubah pemilik proyek dengan mencatat entri baru.

## D-001 — Rust + Dioxus Desktop

Status: Accepted

Aplikasi dibuat dengan Rust untuk logika dan Dioxus Desktop dengan WebView untuk UI. Versi crate dipilih dan dikunci pada Fase 0 setelah memeriksa kompatibilitas dokumentasi (dicatat di D-012).

Alasan: UI dapat dibuat dengan komponen deklaratif Rust, dan pengembangan awal tidak membutuhkan renderer native custom.

## D-002 — Tidak menerapkan wallpaper pada MVP

Status: Accepted (alasan diperbarui 2026-10-14)

Tidak ada fungsi Set as Wallpaper, command compositor, wallpaper daemon, atau integrasi desktop environment pada MVP.

Alasan: menjaga MVP kecil dan portabel. Menerapkan wallpaper membutuhkan integrasi yang berbeda untuk tiap compositor/desktop environment, sehingga dibahas sebagai keputusan terpisah setelah MVP stabil.

Konsekuensi: tidak boleh ada dependensi wajib pada Hyprland, Niri, GNOME, KDE, `swww`, atau `hyprpaper`. Bila nanti dibuat, integrasi harus berupa adapter opsional di luar `wallhaven-core`.

## D-003 — Wallhaven adalah provider pertama

Status: Accepted

Implementasikan Wallhaven dahulu. Desain provider boleh extensible, tetapi provider kedua dan plugin dinamis tidak dibuat sebelum MVP teruji.

## D-004 — SFW sebagai default

Status: Accepted

Preferensi konten awal adalah SFW. Kategori lain ditampilkan sesuai dukungan API, autentikasi, pengaturan pengguna, dan kebijakan layanan.

## D-005 — SQLite menyimpan metadata, bukan gambar

Status: Accepted

Database menyimpan favorit, koleksi, metadata download, dan informasi terkait. File gambar disimpan di filesystem.

## D-006 — Linux terlebih dahulu

Status: Accepted

Target pertama adalah Linux, dengan Arch Linux sebagai target pengujian awal. Logika inti tidak bergantung pada package manager spesifik distro.

## D-007 — Cargo workspace: `wallhaven-core` dan `wallhaven-desktop`

Status: Proposed

Domain, provider, services, storage, dan config berada di crate `wallhaven-core` tanpa dependensi Dioxus. UI berada di crate `wallhaven-desktop`.

Alasan: aturan arsitektur ditegakkan compiler, dan test `core` berjalan cepat tanpa dependensi GUI.

## D-008 — Favorit dan koleksi bersifat lokal

Status: Proposed

Favorit (MVP) dan koleksi (P2) disimpan lokal di SQLite aplikasi. Sinkronisasi dengan favorit/koleksi akun Wallhaven (endpoint `/collections`) tidak dibuat pada MVP; membaca koleksi akun secara read-only tercantum di `ROADMAP.md`.

Alasan: menghindari ketergantungan pada API key untuk fitur dasar dan menjaga MVP kecil.

## D-009 — Pemuatan thumbnail oleh WebView dari host allowlist

Status: Proposed

Pada Fase 2, WebView memuat thumbnail dan preview langsung dari host allowlist (`th.wallhaven.cc`, `w.wallhaven.cc`) yang diizinkan CSP. Pemanggilan API, download, dan akses file tetap hanya lewat `core`. Cache thumbnail (P1) dapat memindahkan pengambilan gambar ke `core` dengan mekanisme lokal (custom protocol/asset handler), dan keputusan mekanismenya dicatat saat dikerjakan.

Alasan: sederhana untuk MVP, tetap memenuhi aturan "grid hanya memakai thumbnail", dan tetap dapat dibatasi lewat CSP.

## D-010 — Pagination bernomor

Status: Proposed

Discover memakai pagination bernomor (Sebelumnya/Berikutnya, "Halaman X dari Y") berdasarkan `meta` API, tanpa infinite scroll pada MVP.

Alasan: sejalan dengan aturan tidak ada prefetch seluruh halaman dan lebih mudah diuji.

## D-011 — Bahasa UI: Indonesia

Status: Proposed

Seluruh teks UI berbahasa Indonesia dan disimpan di satu modul `strings`. Nama komponen dan modul kode tetap berbahasa Inggris.

Alasan: konsistensi antara label navigasi dan pesan error; modul tunggal memudahkan penambahan locale.

## D-012 — Pilihan teknologi dicatat pada Fase 0

Status: Proposed (diisi pada Fase 0; menunggu konfirmasi pemilik proyek sebelum menjadi Accepted)

Agent mengisi tabel ini pada Fase 0 setelah memeriksa kompatibilitas, lalu mengubah statusnya menjadi Accepted setelah dikonfirmasi pemilik proyek.

| Area | Pilihan | Alasan |
| --- | --- | --- |
| Versi Dioxus | 0.7.3 dengan fitur `desktop` (`dioxus` + `dioxus-desktop`), CLI 0.7.10 | Sejajar dengan CLI yang tersedia; 0.8.0-alpha.1 belum dipilih untuk MVP |
| Runtime async | Tokio 1.x (`tokio` multi-thread) | Runtime bawaan `dioxus-desktop` dengan fitur `tokio_runtime`; layak untuk request jaringan dan file |
| HTTP client dan TLS | `reqwest` 0.11 + `native-tls` (OpenSSL sistem) | Satu client HTTP; TLS sistem tersedia di Arch, dan versi terkunci dapat ditambahkan lewat `Cargo.lock` |
| SQLite dan migrasi | `rusqlite` 0.40 dengan `modern_sqlite`, migrasi bernomor lewat `PRAGMA user_version` | Database lokal; migrasi Fase 5 dijalankan deterministik dari crate `storage`, tanpa crate migration tambahan |
| Error handling | `thiserror` 1.0 untuk `CoreError`, kategori sesuai ARCHITECTURE.md | Compiler memastikan enum; pesan pengguna bisa disederhanakan di UI |
| Logging | `tracing` + `tracing-subscriber` (env-filter) | Log teknis tanpa rahasia; API key tidak dicatat |
| Direktori Linux | `dirs` 6.0 (XDG) | Direktori standar config/data/cache/download, tanpa path hard-code |
| File dialog | `rfd` 0.17 (portal-aware), direncanakan Fase 4/6 | Kompatibel X11/Wayland lewat xdg-desktop-portal; fallback path manual |
| Mock HTTP untuk test | `wiremock` 0.6 | Unit test tanpa internet; dipakai di Fase 1 |
