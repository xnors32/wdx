# Pedoman UI

## 1. Arah visual

Antarmuka desktop modern, rapi, dan berfokus pada gambar. Untuk MVP, prioritaskan fungsi dan keterbacaan daripada animasi kompleks. UI tidak perlu meniru situs Wallhaven.

Layout awal:
- sidebar ringkas: Jelajahi, Unduhan, Favorit, Pustaka, Pengaturan;
- area utama: search dan filter di atas, grid wallpaper di bawah;
- detail: panel atau halaman preview yang jelas;
- status download terlihat tanpa menutupi area utama.

Ukuran jendela: tetapkan ukuran minimum yang masih menampilkan sidebar dan minimal dua kolom grid. Grid memakai kolom responsif (mis. CSS grid `auto-fill` dengan lebar kartu minimum).

## 2. Komponen utama

- `SearchBar`: query dan aksi pencarian.
- `FilterBar`: filter yang terverifikasi di `API_CONTRACTS.md` §2, plus chip filter aktif dan tombol reset.
- `WallpaperGrid`: daftar thumbnail.
- `WallpaperCard`: thumbnail, resolusi, badge kategori/purity, aksi favorit (setelah Fase 5).
- `Pager`: Sebelumnya/Berikutnya dan "Halaman X dari Y".
- `WallpaperDetail`: preview besar, tag, metadata, sumber, Download.
- `DownloadList`: status, progress, error.
- `LibraryView`: file lokal.
- `SettingsView`: API key, direktori download, preferensi konten.
- `EmptyState`, `LoadingState`, `ErrorState`: state umum yang dapat dipakai ulang.

## 3. Performa gambar

- Grid memakai `thumbs.small`; detail memakai `thumbs.large`. Gambar penuh hanya dimuat atas permintaan.
- Hindari merender ulang semua kartu saat progress satu download berubah.
- Gunakan lazy loading/virtualization bila tersedia pada versi Dioxus yang dipakai. Jangan menganggap `loading="lazy"` atau API WebView tertentu tersedia tanpa verifikasi.
- Tampilkan placeholder saat thumbnail dimuat atau gagal.

## 4. State yang wajib

Layar data async: idle (bila sesuai), loading, loaded, empty, error, refreshing (bila perlu).

Download: queued, downloading, completed, failed, cancelled (bila fitur cancel dibuat).

Gunakan enum/state eksplisit, bukan kumpulan boolean yang dapat saling bertentangan.

## 5. Interaksi dan aksesibilitas

- Enter di search bar memulai pencarian; Esc menutup panel detail.
- Setiap tombol punya label teks atau `aria-label` yang jelas; fokus keyboard terlihat; urutan Tab logis.
- Gambar punya `alt` yang bermakna (mis. ID dan resolusi).
- Penghapusan file lokal memerlukan konfirmasi.
- File yang sudah ada tidak ditimpa tanpa pilihan eksplisit.
- Favorit memberi umpan balik dan mencerminkan status yang benar-benar tersimpan.
- URL sumber dibuka lewat mekanisme aman dengan validasi scheme (hanya `https`).
- Pagination bernomor: tombol Sebelumnya/Berikutnya dinonaktifkan sesuai `current_page` dan `last_page`; ganti query atau filter kembali ke halaman 1 (D-010).

## 6. Konten sensitif

- Default SFW pada instalasi baru.
- Label pengaturan purity memakai bahasa yang jelas: "Aman (SFW)", "Sensitif (Sketchy)", "Dewasa (NSFW)".
- Mengaktifkan Sketchy atau NSFW menampilkan dialog konfirmasi satu kali yang menjelaskan bahwa konten dapat tidak pantas untuk semua usia.
- NSFW hanya dapat dipilih bila API key tersimpan; bila tidak, tampilkan petunjuk menambahkan key di Pengaturan dan jangan mengirim bit NSFW.
- Thumbnail non-SFW diburamkan secara default dengan opsi "tampilkan" per item. Blur adalah lapisan tambahan, bukan pengganti filter di request provider.
- Detail non-SFW tidak dimuat otomatis bila pengaturan melarangnya.
- Ikuti `SECURITY_AND_CONTENT.md`.

## 7. Bahasa dan pesan error

Semua teks UI berbahasa Indonesia dan disimpan di satu modul `strings` agar mudah diganti atau ditambah bahasa lain (D-011). Pesan ringkas, tidak menyalahkan pengguna, dan menyarankan tindakan:

- "Tidak dapat terhubung. Periksa koneksi lalu coba lagi."
- "API key ditolak. Periksa key di Pengaturan."
- "Terlalu banyak permintaan. Coba lagi sebentar lagi."
- "Folder tidak dapat ditulis. Pilih folder lain."
- "File sudah ada. Pilih nama lain atau konfirmasi penggantian."

Jangan menampilkan stack trace kepada pengguna biasa. Detail teknis dicatat di log tanpa rahasia.
