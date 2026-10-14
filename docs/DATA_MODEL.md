# Model Data dan Penyimpanan

## 1. Prinsip

- Metadata, favorit, koleksi lokal, dan riwayat download disimpan di SQLite.
- File gambar disimpan di filesystem; kolom database tidak menyimpan byte gambar.
- Simpan URL sumber dan provider ID agar metadata dapat ditelusuri.
- Data remote dapat berubah atau hilang; aplikasi tahan terhadap field kosong dan file lokal yang hilang.
- Favorit dan koleksi bersifat lokal dan tidak disinkronkan dengan akun Wallhaven (D-008).

## 2. Model domain awal

Nama dan tipe final boleh disesuaikan setelah DTO diperiksa terhadap respons nyata.

### WallpaperSummary (dari search)
- `provider`: identifier provider, mis. `wallhaven`;
- `provider_id`: ID wallpaper dari provider;
- `page_url`, `image_url`: URL halaman dan gambar penuh, bila tersedia;
- `thumb_small_url`, `thumb_large_url`: URL thumbnail untuk grid dan detail;
- `width`, `height`, `resolution`: dimensi dan teks resolusi;
- `file_size`, `file_type`: ukuran dan tipe file bila tersedia;
- `category`, `purity`: nilai provider yang sudah divalidasi;
- `views`, `favorites`: statistik bila tersedia;
- `source_created_at`: waktu dari sumber, sebagai teks.

`tags` tidak termasuk karena respons search tidak memuatnya.

### WallpaperDetail (dari detail)
Semua field `WallpaperSummary` ditambah: `tags` (nama, alias, kategori tag), `colors`, `source` (URL sumber karya bila ada), dan `uploader` (username, opsional).

### Identitas
Gunakan `(provider, provider_id)` sebagai identitas remote internal. ID dari dua provider berbeda tidak dijamin unik.

### DownloadRecord
- ID lokal;
- provider dan provider ID;
- path file lokal dan nama file;
- ukuran byte bila diketahui;
- status: `queued`, `downloading`, `completed`, `failed`, `cancelled` (bila fitur cancel ada);
- waktu dibuat/selesai;
- error ringkas opsional.

Pada Fase 4 riwayat ini hanya in-memory; mulai Fase 5 disimpan di SQLite.

### Favorite
- provider, provider ID, waktu ditambahkan;
- metadata minimum agar tampil offline: `page_url`, `thumb_small_url`, `resolution`, `category`, `purity`.

### Collection dan CollectionItem (P2, lokal)
- Collection: ID lokal, nama, waktu dibuat/diubah.
- CollectionItem: ID koleksi, provider, provider ID, urutan opsional.

## 3. Skema awal konseptual

Bukan migrasi SQL final. Sesuaikan tipe dan constraint setelah memilih crate SQLite (D-012). Wallhaven tidak menyediakan judul wallpaper, sehingga tidak ada kolom `title`.

```sql
CREATE TABLE IF NOT EXISTS favorites (
    provider       TEXT NOT NULL,
    provider_id    TEXT NOT NULL,
    page_url       TEXT,
    thumb_url      TEXT,
    resolution     TEXT,
    category       TEXT,
    purity         TEXT,
    added_at       TEXT NOT NULL,
    PRIMARY KEY (provider, provider_id)
);

CREATE TABLE IF NOT EXISTS downloads (
    id             INTEGER PRIMARY KEY,
    provider       TEXT NOT NULL,
    provider_id    TEXT NOT NULL,
    source_url     TEXT,
    file_path      TEXT NOT NULL,
    file_name      TEXT NOT NULL,
    byte_size      INTEGER,
    status         TEXT NOT NULL,
    error_summary  TEXT,
    created_at     TEXT NOT NULL,
    completed_at   TEXT
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_downloads_file_path ON downloads(file_path);
CREATE INDEX IF NOT EXISTS idx_downloads_provider_id ON downloads(provider, provider_id);

-- P2
CREATE TABLE IF NOT EXISTS collections (
    id             INTEGER PRIMARY KEY,
    name           TEXT NOT NULL UNIQUE,
    created_at     TEXT NOT NULL,
    updated_at     TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS collection_items (
    collection_id  INTEGER NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    provider       TEXT NOT NULL,
    provider_id    TEXT NOT NULL,
    position       INTEGER,
    added_at       TEXT NOT NULL,
    PRIMARY KEY (collection_id, provider, provider_id)
);
```

Jangan menjalankan SQL contoh ini tanpa menyesuaikannya dengan sistem migrasi proyek. Aktifkan foreign key di koneksi SQLite bila memakai `REFERENCES`.

## 4. Migrasi dan versi skema

- Migrasi bernomor, deterministik, dan dijalankan otomatis saat startup.
- Catat versi skema (tabel migrasi dari crate yang dipilih atau `PRAGMA user_version`).
- Jangan menghapus database untuk mengatasi migrasi gagal; tampilkan error dan hentikan dengan aman.
- Perubahan skema berikutnya menambah migrasi baru; migrasi yang sudah dipakai pengguna tidak diedit tanpa alasan dan strategi yang jelas.

## 5. Filesystem

- Gunakan direktori data/cache standar Linux.
- Simpan download ke direktori pilihan pengguna.
- Unduh ke file sementara, lalu rename setelah verifikasi berhasil.
- Periksa tabrakan nama file; jangan menimpa diam-diam.
- Hindari path traversal dan perilaku symlink yang tidak aman.
- Bila file hilang, pertahankan metadata dan tampilkan status "hilang".
- Jangan menghapus file eksternal hanya karena metadata database tidak menemukannya.
- Penamaan file aman: `wallhaven-<id>.<ext>` (ID tervalidasi `[a-z0-9]+`, ekstensi dari tipe gambar tervalidasi) sebagai default.

## 6. Cache thumbnail (P1)

- Cache thumbnail secara lokal untuk mengurangi request dan mempercepat scrolling.
- Cache dapat dibangun ulang; kehilangan cache tidak merusak database atau favorit.
- Batasi ukuran cache (mis. batas total MB yang dapat dikonfigurasi) atau sediakan pembersihan yang aman; hanya menghapus isi direktori cache aplikasi.
- Jangan menjadikan gambar resolusi penuh sebagai cache.
