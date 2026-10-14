# Arsitektur

## 1. Prinsip

1. UI tidak memanggil API provider dan tidak mengunduh file. Satu-satunya pengambilan jaringan yang boleh dilakukan WebView adalah memuat gambar thumbnail/preview dari host yang diizinkan (D-009).
2. Provider dan seluruh crate `core` tidak bergantung pada Dioxus.
3. Database tidak menyimpan byte gambar penuh.
4. Operasi jaringan dan file besar berjalan asynchronous atau di luar jalur render UI.
5. Model API provider (DTO) dipisahkan dari model domain aplikasi.
6. Tidak membuat trait atau abstraksi berlebihan sebelum ada kebutuhan nyata.
7. Tidak ada modul untuk menerapkan wallpaper pada MVP (D-002).

## 2. Struktur workspace (D-007)

```text
Cargo.toml                  # [workspace]
crates/
├── wallhaven-core/         # tanpa dependensi Dioxus
│   └── src/
│       ├── lib.rs
│       ├── domain/         # wallpaper.rs, search.rs, download.rs
│       ├── providers/
│       │   ├── mod.rs
│       │   └── wallhaven/  # client.rs, dto.rs, mapper.rs, limiter.rs
│       ├── services/       # search.rs, download.rs, favorites.rs, library.rs
│       ├── storage/        # database.rs, migrations/, repositories/, paths.rs
│       └── config/         # settings.rs
└── wallhaven-desktop/      # aplikasi Dioxus
    └── src/
        ├── main.rs
        ├── app.rs
        ├── strings.rs      # seluruh teks UI (D-011)
        ├── components/     # wallpaper_card, wallpaper_grid, search_bar, async_image, states
        └── features/       # discover, preview, downloads, favorites, library, settings
```

Manfaat pemisahan: aturan "provider tidak boleh bergantung pada Dioxus" ditegakkan oleh compiler, dan test `core` berjalan tanpa dependensi GUI (WebKitGTK).

Agent boleh menyesuaikan struktur kecil bila versi Dioxus atau compiler mengharuskan, dengan menjelaskan alasan dan memperbarui dokumen ini.

## 3. Aliran data

### Search
1. Pengguna mengubah query/filter di UI.
2. UI memanggil service search di `core`.
3. Service memvalidasi dan menormalkan parameter (misalnya menghapus bit NSFW bila tidak ada API key).
4. Provider Wallhaven mengirim request HTTP melalui rate limiter.
5. JSON diubah menjadi DTO, lalu dipetakan ke `WallpaperSummary`.
6. UI menerima state loading, data, atau error (hasil dari request lama diabaikan berdasarkan generasi request).
7. UI menampilkan thumbnail, bukan gambar resolusi penuh.

### Thumbnail dan preview
1. Kartu dan detail memakai URL thumbnail dari domain model (`thumbs.small` untuk grid, `thumbs.large` untuk detail).
2. WebView memuat gambar langsung dari host yang diizinkan CSP (lihat `SECURITY_AND_CONTENT.md` §3). Ini default sampai cache thumbnail (P1) dibuat.
3. Cache thumbnail (P1) memindahkan pengambilan gambar ke `core` dan menyajikannya ke WebView lewat mekanisme lokal (custom protocol/asset handler). Mekanisme yang dipilih bergantung pada versi Dioxus dan dicatat sebagai keputusan baru saat dikerjakan.
4. Kegagalan memuat gambar menampilkan placeholder, bukan merusak grid.

### Download
1. Pengguna memilih Download.
2. Service memeriksa metadata dan direktori tujuan.
3. Service membentuk nama file aman.
4. Service mengunduh ke file sementara di direktori tujuan (atau cache), dengan batas ukuran dan host allowlist.
5. Service memverifikasi hasil: respons sukses, ukuran bukan nol, tidak melebihi batas, dan tipe gambar valid berdasarkan byte awal file.
6. Service memindahkan file sementara ke nama akhir tanpa menimpa file yang ada.
7. Status dan metadata diperbarui (in-memory pada Fase 4; SQLite sejak Fase 5).
8. UI menerima status sukses atau error.

### Favorites
1. UI memanggil service favorites.
2. Service mengubah status di database.
3. UI memperbarui state setelah operasi database sukses.
4. Kegagalan database ditampilkan; perubahan yang gagal tidak ditampilkan seolah tersimpan.

## 4. Lapisan

### UI (`wallhaven-desktop`)
Tanggung jawab: menampilkan state, menerima input, memanggil service, menampilkan loading/error/empty state.

Tidak boleh: membentuk URL API, menyimpan API key, menulis SQL, memproses file besar secara sinkron, atau memanggil command compositor.

### Domain (`core/domain`)
Model lintas modul: ID, `WallpaperSummary`, `WallpaperDetail`, status download, filter pencarian. Tidak bergantung pada Dioxus.

### Provider (`core/providers`)
Request HTTP, autentikasi provider, rate limiting, deserialisasi, pemetaan DTO ke domain, error khusus provider. MVP hanya Wallhaven.

### Services (`core/services`)
Use case: search, detail, download, favorit, library. Mengoordinasikan provider, database, cache, dan filesystem. Tidak mengandung markup UI.

### Storage (`core/storage`)
Koneksi SQLite, migrasi, repository, dan penentuan direktori config/data/cache.

### Configuration (`core/config`)
Memuat dan menyimpan preferensi non-rahasia, validasi, default aman. Sumber API key dan penyimpanannya mengikuti `SECURITY_AND_CONTENT.md` §1.

## 5. Kontrak provider

Buat kontrak provider hanya untuk operasi yang dibutuhkan MVP: search, detail, dan (untuk download) penyelesaian URL gambar penuh. Ilustrasi konsep:

```rust
pub struct SearchRequest {
    pub query: Option<String>,
    pub page: u32,
    pub filters: SearchFilters,
    pub seed: Option<String>, // untuk sorting random
}

pub struct SearchPage {
    pub items: Vec<WallpaperSummary>,
    pub current_page: u32,
    pub last_page: Option<u32>,
    pub seed: Option<String>,
}

pub trait WallpaperProvider {
    // Bentuk final menyesuaikan pendekatan async yang dipilih (D-012).
    // Jangan menyalin signature ini tanpa memeriksa kebutuhan object safety.
}
```

Ini ilustrasi, bukan kode final. Pilih pendekatan async trait yang cocok dengan versi Rust/dependensi, dan hindari `dyn` bila tidak dibutuhkan.

## 6. Error handling

Bedakan minimal:
- `Network`: koneksi gagal, timeout, DNS.
- `Provider`: API mengembalikan error, skema tak sesuai.
- `Unauthorized`: API key ditolak atau dibutuhkan (HTTP 401).
- `RateLimited`: batas request tercapai (HTTP 429).
- `Storage`: SQLite gagal.
- `Filesystem`: izin, ruang disk, file tidak ditemukan.
- `InvalidInput`: filter atau path tidak valid.
- `Cancelled`: operasi dibatalkan.

Pesan teknis boleh dicatat untuk debugging; pesan UI harus aman dan mudah dimengerti (lihat `UI_GUIDELINES.md` §7).

## 7. Direktori Linux

Gunakan direktori standar (crate `directories` atau setara yang dicatat di D-012):
- config: konfigurasi aplikasi non-rahasia;
- data: database dan metadata permanen;
- cache: thumbnail dan file sementara;
- download: default dari direktori Pictures/Downloads pengguna, dapat diubah di Settings.

Jangan menulis ke path yang di-hard-code untuk satu pengguna.

## 8. Platform

Catatan WebKitGTK, Wayland/X11, file picker, dan dependensi build ada di `PLATFORM_NOTES.md`.
