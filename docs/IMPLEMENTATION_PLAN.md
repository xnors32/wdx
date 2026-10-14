# Rencana Implementasi Bertahap

Kerjakan fase secara berurutan. Fase berikutnya dimulai setelah kriteria fase aktif terpenuhi.

Sumber API key selama Fase 1–5: variabel lingkungan `WALLHAVEN_API_KEY` (opsional, untuk pengembangan; tidak pernah dicatat di log). Penyimpanan key lewat UI baru dibuat di Fase 6.

## Fase 0 — Fondasi, workspace, dan keputusan teknis

Tugas:
- periksa repository dan versi Rust/Dioxus yang tersedia;
- buat Cargo workspace dengan crate `wallhaven-core` dan `wallhaven-desktop` (D-007);
- susun modul dasar sesuai `ARCHITECTURE.md`;
- pilih dan catat teknologi di D-012 (versi Dioxus, runtime async, HTTP/TLS, SQLite, error, logging, direktori, file dialog);
- buat konfigurasi minimal di `core`: penentuan direktori config/data/cache dan direktori download default; belum ada UI pengaturan;
- buat aplikasi Dioxus Desktop yang dapat dibuka, dengan navigasi placeholder dan modul `strings`;
- periksa kebutuhan platform di `PLATFORM_NOTES.md` dan catat hasilnya;
- siapkan lint/test dasar.

Kriteria selesai:
- `cargo fmt --all --check` dan `cargo check --workspace` berhasil;
- `core` dapat dikompilasi dan diuji tanpa dependensi Dioxus;
- aplikasi dapat dijalankan di environment pengembangan yang tersedia;
- D-012 terisi;
- tidak ada integrasi wallpaper sistem.

Jangan mengimplementasikan API, database, atau download pada fase ini kecuali diperlukan untuk membuktikan fondasi.

## Fase 1 — Wallhaven API client

Tugas:
- baca `API_CONTRACTS.md` §2 dan verifikasi terhadap dokumentasi resmi bila snapshot lebih tua dari 90 hari;
- satu `reqwest::Client` dengan timeout, User-Agent, header `X-API-Key` opsional, dan rate limiter (45/menit);
- DTO untuk `/search` dan `/w/<ID>` yang benar-benar dipakai; mapper ke `WallpaperSummary` dan `WallpaperDetail`;
- normalisasi parameter (bit purity/kategori, `topRange`, `seed`);
- tangani status HTTP (401, 429), timeout, dan error JSON;
- test unit untuk mapper, normalisasi parameter, dan pemetaan error memakai `docs/fixtures/wallhaven/`.

Kriteria selesai:
- tidak ada request dari komponen UI;
- tidak ada endpoint atau field yang dikarang;
- key tidak masuk log atau URL;
- test dengan fixture JSON lulus tanpa internet;
- kegagalan jaringan tidak membuat aplikasi crash.

## Fase 2 — Discover, search, filters, pagination

Tugas:
- search bar, grid thumbnail, `Pager`;
- tampilan awal: SFW terbaru;
- loading, empty, dan error state;
- filter MVP terverifikasi dan chip filter aktif;
- CSP dan pemuatan thumbnail dari host allowlist (D-009);
- cegah hasil request lama menimpa hasil terbaru.

Kriteria selesai:
- pengguna dapat mencari dan menelusuri halaman hasil;
- grid tidak meminta gambar resolusi penuh;
- filter dikirim sebagai parameter yang valid;
- UI tidak macet selama request.

## Fase 3 — Preview/detail

Tugas:
- buka detail dari kartu; preview `thumbs.large`;
- metadata, tag, warna, sumber, tautan ke halaman Wallhaven (validasi scheme);
- tangani gambar gagal dimuat;
- patuhi preferensi purity (default SFW).

Kriteria selesai:
- UI detail tidak bergantung pada model API mentah;
- field yang hilang tidak menyebabkan crash;
- konten mengikuti preferensi purity.

## Fase 4 — Download

Tugas:
- direktori download dari konfigurasi dasar, dan opsi file picker bila platform mendukung;
- nama file aman, unduh ke file sementara, validasi tipe gambar dan batas ukuran;
- status dan progress; riwayat download in-memory selama sesi (halaman Unduhan);
- tangani tabrakan nama, timeout, disk penuh, izin, dan retry manual;
- pembatalan bila dapat diimplementasikan dengan aman.

Kriteria selesai:
- file hasil dapat dibuka dari file manager;
- download gagal tidak meninggalkan file final yang rusak;
- file yang ada tidak ditimpa diam-diam;
- operasi file tidak memblokir UI.

## Fase 5 — SQLite, favorites, dan local library

Tugas:
- implementasi database dengan migrasi bernomor;
- favorit lokal dan tombol favorit di kartu/detail;
- simpan riwayat download di SQLite;
- local library dengan deteksi file hilang;
- cache thumbnail (P1) bila waktunya cukup; bila tidak, catat sebagai pekerjaan lanjutan;
- test repository.

Kriteria selesai:
- favorit bertahan setelah restart;
- database dibuat di direktori data yang benar;
- migrasi tidak menghapus data;
- file hilang ditangani dengan aman.

## Fase 6 — Settings dan konten

Tugas:
- `SettingsView`: input API key (masked), direktori download, preferensi purity/kategori;
- penyimpanan key sesuai `SECURITY_AND_CONTENT.md` §1 (permission file `0600`);
- dialog konfirmasi untuk Sketchy/NSFW dan blur thumbnail non-SFW;
- validasi setting dan petunjuk ketika fitur memerlukan key.

Kriteria selesai:
- perubahan setting disimpan dan dipakai;
- rahasia tidak masuk log;
- filter UI dan parameter request provider konsisten;
- akses provider tidak dilewati.

## Fase 7 — Stabilitas dan rilis awal

Tugas:
- audit error path; uji koneksi lambat atau gagal;
- uji path dengan spasi dan karakter non-ASCII;
- periksa penggunaan cache dan memori grid;
- uji di Wayland dan X11;
- tulis petunjuk build/run Linux; pastikan `Cargo.lock` dan instruksi dependensi jelas;
- siapkan CI sederhana yang menjalankan quality gate (untuk `core` tanpa dependensi GUI; untuk app sesuai paket yang tersedia).

Kriteria selesai:
- seluruh quality gate di `TESTING.md` lulus, atau kegagalan yang tersisa didokumentasikan;
- README menjelaskan fitur yang ada dan yang belum ada.

## Format laporan agent

Di akhir setiap fase, laporkan:
1. fase yang dikerjakan;
2. ringkasan perubahan;
3. file yang berubah;
4. perintah yang dijalankan dan hasilnya;
5. kriteria yang terpenuhi;
6. masalah yang tersisa;
7. keputusan baru yang dicatat di `DECISIONS.md`, dan konfirmasi bahwa tidak ada fitur di luar ruang lingkup `PRD.md` §5 yang ditambahkan.
