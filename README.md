# Wallhaven Desktop

Aplikasi desktop Linux berbasis Rust + Dioxus untuk mencari, melihat, mengunduh, dan mengelola wallpaper dari Wallhaven.cc.

> Status: rancangan MVP (revisi 2026-10-14). Dokumen ini adalah sumber aturan proyek sampai keputusan baru dicatat di `docs/DECISIONS.md`.

## Mulai untuk coding agent

1. Baca `AGENTS.md` sampai selesai.
2. Baca `docs/PRD.md` (fitur dan batasan) dan `docs/DECISIONS.md` (keputusan yang berlaku).
3. Baca `docs/ARCHITECTURE.md` dan `docs/IMPLEMENTATION_PLAN.md`.
4. Sebelum menyentuh provider, baca `docs/API_CONTRACTS.md` (berisi snapshot fakta API yang sudah diverifikasi).
5. Kerjakan **satu fase saja** sesuai rencana implementasi.
6. Jalankan quality gate di `docs/TESTING.md` sebelum menyatakan pekerjaan selesai.

## Ringkasan keputusan

- Bahasa: Rust. UI: Dioxus Desktop (WebView). Struktur: Cargo workspace dengan crate `core` terpisah dari crate aplikasi (D-007).
- Target awal: Linux; Arch Linux untuk pengujian awal. Wayland dan X11 sama-sama diuji (`docs/PLATFORM_NOTES.md`).
- Sumber konten: Wallhaven API v1. Default SFW.
- Fitur MVP: pencarian, filter, grid thumbnail, detail/preview, download, favorit lokal, library lokal, pengaturan.
- Pasca-MVP (P2): koleksi lokal, download queue lanjutan. Daftar lengkap ada di `docs/ROADMAP.md`.
- Aplikasi ini tidak mengatur wallpaper desktop dan tidak bergantung pada compositor, desktop environment, atau tool wallpaper tertentu (D-002).
- Provider dirancang agar sumber lain dapat ditambahkan nanti, tetapi hanya Wallhaven yang dibuat pada MVP.

## Peta dokumen

| Dokumen | Isi |
| --- | --- |
| `AGENTS.md` | Aturan wajib untuk coding agent |
| `CLAUDE.md` | Pintu masuk untuk Claude Code, merujuk ke `AGENTS.md` |
| `docs/PRD.md` | Kebutuhan produk, ruang lingkup, kriteria penerimaan |
| `docs/ARCHITECTURE.md` | Lapisan, modul, aliran data, workspace |
| `docs/API_CONTRACTS.md` | Snapshot fakta API Wallhaven dan kontrak provider |
| `docs/DATA_MODEL.md` | Model domain dan penyimpanan lokal |
| `docs/UI_GUIDELINES.md` | Layout, state, interaksi, bahasa UI |
| `docs/IMPLEMENTATION_PLAN.md` | Fase kerja dan kriteria selesai |
| `docs/TESTING.md` | Quality gate dan aturan test |
| `docs/SECURITY_AND_CONTENT.md` | API key, konten sensitif, keamanan jaringan dan file |
| `docs/PLATFORM_NOTES.md` | Catatan WebKitGTK, Wayland/X11, file picker, dependensi Arch |
| `docs/ROADMAP.md` | Ide pasca-MVP (bukan komitmen) |
| `docs/DECISIONS.md` | Catatan keputusan (ADR ringan) |
| `docs/fixtures/wallhaven/` | Contoh JSON berbasis dokumentasi resmi untuk test mapper |

## Urutan prioritas bila dokumen bertentangan

1. `docs/DECISIONS.md` (entri terbaru menang)
2. `docs/SECURITY_AND_CONTENT.md`
3. `docs/PRD.md`
4. `docs/ARCHITECTURE.md`, `docs/API_CONTRACTS.md`, `docs/DATA_MODEL.md`
5. `docs/UI_GUIDELINES.md`, `docs/TESTING.md`, `docs/IMPLEMENTATION_PLAN.md`

Jika konflik masih belum terselesaikan, berhenti, tanyakan kepada pemilik proyek, lalu catat hasilnya di `docs/DECISIONS.md`.

## Prinsip penting

Jika implementasi belum jelas, pilih solusi paling sederhana yang memenuhi dokumen, atau minta klarifikasi. Perubahan ruang lingkup selalu dicatat secara eksplisit, tidak dilakukan diam-diam.

## Build dan run (Linux)

Dependensi sistem (Arch Linux contoh):

```bash
sudo pacman -S --needed webkit2gtk-4.1 gtk3 xdotool
```

Build:

```bash
cargo build --workspace
```

Jalankan aplikasi desktop:

```bash
cargo run -p wallhaven-desktop
```

Jalankan quality gate:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Catatan:
- tanpa API key, aplikasi memakai mode tamu (default SFW);
- API key, ketika disimpan via Pengaturan, ditulis ke file `api_key` dengan permission `0600`;
- database dan cache ditulis ke direktori data/cache standar Linux, bukan ke direktori repo;
- cache thumbnail belum diimplementasikan; thumbnail dimuat langsung dari host yang diizinkan.
