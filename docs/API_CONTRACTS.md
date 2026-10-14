# Kontrak API dan Provider

## 1. Sumber resmi

- Wallhaven API: https://wallhaven.cc/help/api

Dokumentasi pihak ketiga tidak boleh menjadi satu-satunya sumber untuk endpoint atau aturan autentikasi. Jika dokumentasi resmi berubah, ikuti versi yang berlaku dan perbarui §2.

## 2. Snapshot fakta API terverifikasi

Diverifikasi dari halaman resmi pada **2026-10-14**. Verifikasi ulang bila snapshot lebih tua dari 90 hari atau respons nyata berbeda; perbarui tanggal bila sudah diperiksa. Dokumentasi resmi menyatakan API dapat berubah tanpa pemberitahuan dan disediakan "as-is".

### Endpoint (base: `https://wallhaven.cc/api/v1`)

| Tujuan | Endpoint | Catatan |
| --- | --- | --- |
| Detail wallpaper | `GET /w/<ID>` | Respons dibungkus `data`; memuat `tags` dan `uploader` |
| Search/listing | `GET /search` | Respons berisi `data` (list) dan `meta`; item tidak memuat `tags` |
| Info tag | `GET /tag/<ID>` | Di luar MVP |
| Pengaturan akun | `GET /settings` | Butuh API key; di luar MVP |
| Koleksi akun | `GET /collections`, `/collections/<USERNAME>`, `/collections/<USERNAME>/<ID>` | Di luar MVP (D-008) |

### Autentikasi
- API key dapat dikirim lewat query `?apikey=<KEY>` atau header `X-API-Key: <KEY>`.
- Aplikasi memakai **header** `X-API-Key` agar key tidak masuk URL, log, atau riwayat.
- Tanpa key, API menampilkan konten yang diizinkan untuk tamu. NSFW membutuhkan key yang valid.
- Bila key diberikan pada `/search`, pencarian memakai browsing settings dan filter default akun pengguna. Karena itu aplikasi **selalu mengirim** `categories`, `purity`, dan `sorting` secara eksplisit agar hasil sesuai UI.

### Parameter `/search`

| Parameter | Nilai | Keterangan |
| --- | --- | --- |
| `q` | teks; sintaks: `tag`, `-tag`, `+tag1 +tag2`, `@username`, `id:123`, `type:png`/`type:jpg`, `like:<ID>` | `id:` tidak dapat digabung dengan yang lain |
| `categories` | 3 bit `general/anime/people`, mis. `100`, `101`, `111` (default `111`) | 1 = aktif |
| `purity` | 3 bit `sfw/sketchy/nsfw`, mis. `100`, `110`, `111` (default `100`) | Bit NSFW membutuhkan API key |
| `sorting` | `date_added` (default), `relevance`, `random`, `views`, `favorites`, `toplist` | `hot` muncul di UI situs tetapi tidak tercantum pada daftar `sorting` API di snapshot ini; jangan dikirim sebelum diverifikasi |
| `order` | `desc` (default), `asc` | |
| `topRange` | `1d`, `3d`, `1w`, `1M` (default), `3M`, `6M`, `1y` | Hanya bermakna bila `sorting=toplist` |
| `atleast` | mis. `1920x1080` | Resolusi minimum |
| `resolutions` | daftar dipisah koma, mis. `1920x1080,1920x1200` | Resolusi persis |
| `ratios` | daftar dipisah koma, mis. `16x9,16x10` | |
| `colors` | satu warna hex dari daftar tetap (tanpa `#`), mis. `66cccc` | Daftar tetap tercantum di dokumentasi resmi; validasi terhadap daftar itu |
| `page` | integer mulai 1 | |
| `seed` | `[a-zA-Z0-9]{6}` | Opsional; untuk `sorting=random` |

Perilaku penting:
- Tanpa parameter tambahan, `/search` menampilkan wallpaper SFW terbaru. Ini menjadi tampilan awal Discover.
- Hasil dibatasi **24 item per halaman**; gunakan `meta` untuk pagination.
- `sorting=random` menghasilkan `seed` di `meta`; kirim seed yang sama pada halaman berikutnya agar tidak ada pengulangan.
- Dokumentasi menandai pagination "not actually infinite"; jangan mengasumsikan halaman tak terbatas.

### Bentuk respons

Item search (`data[]`), field yang terdokumentasi: `id`, `url`, `short_url`, `views`, `favorites`, `source`, `purity`, `category`, `dimension_x`, `dimension_y`, `resolution`, `ratio`, `file_size`, `file_type`, `created_at`, `colors`, `path`, `thumbs`.

`thumbs` berisi tiga ukuran: `small`, `large`, `original`.

Detail (`/w/<ID>` → `data`) memuat field di atas ditambah `uploader` (username, group, avatar) dan `tags[]` (`id`, `name`, `alias`, `category_id`, `category`, `purity`, `created_at`).

`meta` (search): `current_page`, `last_page`, `per_page`, `total`, `query`, `seed`.
- `meta.query` dapat berupa string, `null`, atau objek `{id, tag}` untuk pencarian tag eksak. Modelkan sebagai enum/`serde_json::Value`.
- `meta.seed` dapat `null`.

Catatan DTO:
- Contoh `path` pada dokumentasi tidak seragam antara detail dan search (ada yang memuat segmen `full/`). Perlakukan `path` sebagai URL opak dan jangan menyusunnya sendiri dari ID.
- Format timestamp `created_at` adalah teks `YYYY-MM-DD HH:MM:SS` tanpa zona waktu eksplisit di contoh; simpan sebagai teks atau parse dengan toleran.
- `settings.per_page` pada respons `/settings` berupa string pada contoh; jangan menyalin asumsi tipe antar endpoint.

### Batas dan error
- Rate limit: **45 request per menit**. Melebihi batas menghasilkan HTTP `429`.
- `401 Unauthorized`: mengakses wallpaper NSFW tanpa key atau dengan key invalid, dan setiap key invalid lainnya.
- Header `Retry-After` tidak terdokumentasi; gunakan bila ada, jika tidak pakai backoff.

### Host gambar yang teramati pada contoh
- `wallhaven.cc` (halaman, API)
- `w.wallhaven.cc` (gambar penuh)
- `th.wallhaven.cc` (thumbnail)

Daftar ini menjadi dasar allowlist di `SECURITY_AND_CONTENT.md` §3. Perbarui bila respons nyata memakai host lain.

## 3. Aturan request

- Gunakan satu `reqwest::Client` yang dikelola provider; jangan membuat client baru per request.
- Atur timeout (koneksi dan total) yang masuk akal.
- Bentuk URL dengan builder query (percent-encoding otomatis), bukan penggabungan string.
- Pasang rate limiter di dalam client provider sehingga seluruh request (search, detail) berbagi anggaran 45/menit, dengan sisa anggaran untuk aksi pengguna.
- Gunakan User-Agent yang mengidentifikasi aplikasi.
- Periksa status HTTP sebelum memetakan body sebagai hasil normal.
- Hindari request duplikat saat pengguna mengetik cepat (debounce atau submit eksplisit).
- Jangan log API key atau header autentikasi.

## 4. Kontrak pencarian internal

`SearchRequest`/`SearchFilters` mendukung: query opsional, halaman, kategori, purity, sorting, order, topRange, resolusi minimum, daftar resolusi, rasio, warna, dan seed. Service menormalkan:
- bit NSFW dihapus bila tidak ada API key yang valid;
- `topRange` hanya dikirim untuk `toplist`;
- `seed` hanya diteruskan untuk `random`;
- `id:` tidak digabung dengan parameter query lain.

Hanya filter yang tercantum di §2 yang boleh muncul di UI.

## 5. Model respons

DTO mengikuti JSON Wallhaven; setelah deserialisasi, DTO dipetakan ke model domain. Komponen UI tidak boleh bergantung pada bentuk JSON mentah.

- Pakai `Option<T>` untuk field yang bisa hilang; abaikan field tak dikenal (jangan `deny_unknown_fields`).
- Buat dua model domain: `WallpaperSummary` (dari search) dan `WallpaperDetail` (dari `/w/<ID>`, menambah tag, uploader, dan warna).
- Fixture contoh ada di `docs/fixtures/wallhaven/`.

## 6. API key dan purity

- API key opsional.
- Jangan menyimpan key di kode, fixture, screenshot, test, atau log.
- Default aplikasi adalah SFW (`purity=100`).
- Sketchy tersedia tanpa key; NSFW membutuhkan key. Jangan menganggap purity tertentu tersedia untuk semua pengguna; tampilkan petunjuk bila ditolak.
- Filter aplikasi dipetakan ke parameter provider (bukan hanya menyembunyikan item setelah diterima).
- Jangan menyiasati pembatasan akses provider.

## 7. Pagination dan request race

- Minta halaman satu per satu; tidak ada prefetch seluruh halaman.
- Pakai request ID/generasi (atau pembatalan) agar hasil lama tidak menimpa hasil terbaru.
- Tombol Sebelumnya/Berikutnya mengikuti `current_page` dan `last_page` dari `meta`.
- Bila `last_page` tidak tersedia, jangan mengarang total halaman.
- Mengganti query atau filter mengembalikan ke halaman 1 dan mengganti `seed`.

## 8. Retry

Retry hanya untuk kegagalan sementara (sebagian kegagalan jaringan, `429` dengan backoff), dengan jumlah percobaan terbatas. Jangan retry otomatis untuk `401` atau input yang salah.

## 9. Provider masa depan

Provider baru harus:
1. menerapkan kontrak internal yang sama;
2. memetakan DTO sendiri ke model domain;
3. mengelola aturan autentikasi dan kontennya sendiri;
4. menyediakan metadata sumber yang jelas;
5. tidak memaksa perubahan komponen UI yang tidak perlu.

MVP tidak membutuhkan sistem plugin dinamis. Provider kedua ditambahkan sebagai modul Rust biasa setelah kebutuhan dan ketentuan API dikaji (lihat `SECURITY_AND_CONTENT.md` §2).
