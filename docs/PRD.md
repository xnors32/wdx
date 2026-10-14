# Product Requirements Document (PRD)

## 1. Ringkasan

Wallhaven Desktop adalah aplikasi desktop Linux untuk mencari, melihat, mengunduh, dan mengatur wallpaper tanpa membuka situs web di browser. Logika aplikasi ditulis dengan Rust; UI memakai Dioxus Desktop.

Pada MVP aplikasi berfungsi sebagai katalog dan pengelola wallpaper. Aplikasi tidak memasang wallpaper ke desktop (D-002).

## 2. Masalah pengguna

Pengguna ingin:
- mencari wallpaper dari aplikasi desktop;
- melihat thumbnail dan detail sebelum mengunduh;
- menyimpan gambar ke disk;
- mengelola favorit;
- menelusuri gambar yang sudah diunduh tanpa meminta data jaringan lagi.

## 3. Pengguna sasaran

- Pengguna Linux desktop; Arch Linux sebagai target pengujian awal.
- Pengguna yang menyukai wallpaper anime, ilustrasi, foto, dan kategori lain yang tersedia dari provider.
- Pengguna yang ingin satu UI untuk sumber online dan file lokal.

## 4. Sasaran MVP

1. Mencari wallpaper melalui Wallhaven API.
2. Memfilter dan mengurutkan hasil.
3. Menampilkan grid thumbnail yang efisien.
4. Menampilkan detail dan preview wallpaper.
5. Mengunduh gambar resolusi penuh dengan status, progress, dan error yang jelas.
6. Menyimpan favorit dan metadata secara lokal.
7. Menampilkan library lokal.
8. Menyediakan pengaturan API key, direktori download, dan preferensi konten.
9. Menjaga arsitektur cukup modular untuk menambahkan provider di masa depan.

## 5. Bukan sasaran MVP

Bagian ini adalah satu-satunya daftar ruang lingkup yang dirujuk dokumen lain.

- Menerapkan wallpaper atau mengubah pengaturan wallpaper sistem.
- Integrasi dengan Hyprland, Niri, GNOME, KDE, atau compositor/desktop environment tertentu.
- Desktop shell, dock, panel, widget, OSD, lockscreen.
- Sinkronisasi cloud, akun pengguna aplikasi, atau berbagi lintas perangkat.
- Sinkronisasi favorit/koleksi dengan akun Wallhaven (lihat D-008).
- Sistem plugin yang memuat kode pihak ketiga.
- Editor gambar, wallpaper live, atau video wallpaper.
- Mengakali autentikasi, batas akses, atau kebijakan provider.

## 6. Fitur fungsional

### F-01 Discover
- Pengguna dapat melihat daftar wallpaper dari Wallhaven. Tampilan awal tanpa query adalah wallpaper terbaru SFW (perilaku default API, lihat `API_CONTRACTS.md` §2).
- Hasil memakai pagination bernomor (Sebelumnya/Berikutnya, "Halaman X dari Y"); tidak meminta semua halaman sekaligus (D-010).
- Grid memakai thumbnail dari provider.
- UI menampilkan loading, empty state, dan error.

### F-02 Search
- Pengguna memasukkan kata kunci; Enter atau tombol Cari memulai pencarian.
- Query diteruskan apa adanya ke provider sehingga sintaks tag Wallhaven (misalnya `-tag`, `+tag`) berfungsi; MVP tidak menyediakan UI khusus untuk sintaks tersebut.
- Request baru membatalkan atau mengabaikan hasil request lama agar hasil tidak tertukar.

### F-03 Filters and sorting
- Filter MVP (semuanya terverifikasi di snapshot API): kategori, purity, sorting beserta order, resolusi minimum, rasio, warna.
- `topRange` hanya tersedia dan dikirim saat sorting `toplist`.
- Filter yang tidak didukung provider tidak boleh dikirim sebagai parameter buatan.
- Filter aktif tampil jelas di UI (chip atau ringkasan) dan dapat direset sekaligus.

### F-04 Preview/details
- Klik kartu membuka detail wallpaper.
- Tampilkan preview (thumbnail ukuran `large`), ID, resolusi, ukuran file, kategori/purity, tag, warna dominan, sumber, dan URL halaman bila tersedia.
- Gambar resolusi penuh dimuat hanya saat pengguna mengunduh atau meminta tampilan penuh.
- Bila detail gagal dimuat, UI menampilkan error dan opsi mencoba lagi.

### F-05 Download
- Pengguna mengunduh ke direktori download yang dikonfigurasi, atau memilih lokasi lain lewat file picker bila platform mendukung.
- Tampilkan nama file, status, dan progress bila server memberi informasi yang cukup.
- Tangani kegagalan jaringan, ruang disk, izin tulis, dan file yang sudah ada.
- Tidak menimpa file tanpa persetujuan pengguna.
- Nama file aman dan bebas path traversal; ekstensi ditentukan dari tipe gambar yang tervalidasi.
- Download dapat dicoba ulang. Pembatalan wajib bila implementasi download queue sudah dimulai.

### F-06 Favorites
- Favorit bersifat **lokal** (disimpan di SQLite aplikasi) dan tidak terhubung ke akun Wallhaven (D-008).
- Pengguna dapat menambah atau menghapus favorit; favorit bertahan setelah aplikasi ditutup.
- Simpan metadata minimum agar favorit tetap tampil saat offline (lihat `DATA_MODEL.md`). Tandai bila file lokal tidak ada.

### F-07 Local library
- Tampilkan file wallpaper yang dikelola aplikasi.
- Baca metadata lokal terlebih dahulu.
- Bila gambar dipindahkan atau dihapus dari luar aplikasi, tampilkan status "hilang"; jangan crash.
- Penghapusan file memerlukan konfirmasi yang jelas.

### F-08 Settings
- API key Wallhaven (opsional; dibutuhkan untuk NSFW dan untuk akses ke pengaturan akun).
- Preferensi kategori dan purity, dengan konfirmasi untuk non-SFW (lihat `UI_GUIDELINES.md` §6).
- Direktori download.
- Pengaturan thumbnail/cache sederhana bila diperlukan.
- Rahasia tidak masuk log atau repository.

### F-09 Additional providers
- MVP hanya mengimplementasikan Wallhaven.
- Struktur provider memungkinkan penambahan provider baru; provider kedua dikerjakan setelah implementasi Wallhaven dan kontrak umum teruji.

## 7. Persyaratan nonfungsional

- UI tetap responsif selama operasi jaringan dan file.
- Grid tidak mengunduh gambar penuh untuk setiap kartu.
- Error jaringan tidak menyebabkan crash.
- Database lokal dapat dibuka ulang setelah aplikasi ditutup normal.
- Operasi file memakai path aman dan menghormati izin pengguna.
- Aplikasi tidak menjalankan command desktop atau mengubah wallpaper sistem.
- Request ke API mengikuti batas rate limit provider (45 request/menit pada snapshot; lihat `API_CONTRACTS.md`).
- Dependensi terkunci lewat `Cargo.lock`.

## 8. Kriteria penerimaan MVP

MVP diterima bila pengguna dapat:
1. membuka aplikasi tanpa API key dan langsung melihat wallpaper SFW terbaru, atau melihat petunjuk jelas bila suatu fitur membutuhkan key;
2. mencari dan menelusuri hasil Wallhaven dengan filter yang terlihat aktif di UI;
3. membuka detail wallpaper;
4. mengunduh wallpaper dan melihat hasil atau error;
5. menandai favorit lalu melihatnya kembali setelah restart;
6. melihat library file lokal yang dikelola aplikasi, termasuk penanda file hilang;
7. mengubah preferensi konten; mengaktifkan sketchy/NSFW membutuhkan konfirmasi, dan NSFW membutuhkan API key;
8. menggunakan aplikasi di Wayland maupun X11 tanpa bergantung pada compositor tertentu.

## 9. Prioritas

P0: fondasi workspace dan konfigurasi dasar, provider Wallhaven, search/grid, preview, download dasar, state loading/error.

P1: SQLite, favorit, library lokal, settings UI, cache thumbnail.

P2: koleksi lokal, download queue lanjutan (retry/cancel), peningkatan aksesibilitas dan performa.

Fitur di luar daftar ini memerlukan keputusan baru di `DECISIONS.md`.
