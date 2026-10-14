# Instruksi Wajib untuk Coding Agent

Ikuti dokumen ini sebelum mengubah kode. Ruang lingkup produk ditetapkan satu kali di `docs/PRD.md` (§5) dan `docs/DECISIONS.md`; dokumen lain merujuk ke sana.

## 1. Cara bekerja

1. Baca `README.md` dan dokumen yang relevan sebelum membuat perubahan.
2. Periksa struktur repo, `Cargo.toml`, dan workspace yang sudah ada. Pertahankan pekerjaan pengguna.
3. Kerjakan hanya fase aktif dari `docs/IMPLEMENTATION_PLAN.md`.
4. Buat perubahan kecil, mudah direview, dan sesuai pola yang sudah ada.
5. Setelah perubahan, jalankan `cargo fmt --all --check`, `cargo check --workspace`, dan test yang relevan. Bila suatu perintah tidak dapat dijalankan, jelaskan alasannya dan jangan mengklaim lulus.
6. Perbarui dokumentasi bila perilaku, kontrak, atau struktur berubah.
7. Akhiri pekerjaan dengan laporan: fase, file yang berubah, perilaku yang dibuat, perintah yang dijalankan beserta hasilnya, kriteria yang terpenuhi, dan pekerjaan yang belum selesai.

## 2. Ruang lingkup

Aplikasi ini adalah **wallpaper browser dan manager**. Kerjakan fitur MVP di `docs/PRD.md` §6 sesuai fase aktif.

Hal berikut membutuhkan keputusan baru di `docs/DECISIONS.md` sebelum dikerjakan:

- menerapkan wallpaper ke desktop, integrasi compositor/desktop environment, daemon, shell, dock, panel, OSD, atau lockscreen (lihat D-002);
- fitur akun/login buatan sendiri;
- provider kedua sebelum abstraksi Wallhaven teruji;
- layanan berbayar;
- sistem plugin dinamis;
- scraping HTML sebagai pengganti API resmi.

Kontrol UI hanya ditampilkan bila fiturnya sudah berfungsi.

## 3. Sumber kebenaran

- Bila dokumen bertentangan, ikuti urutan prioritas di `README.md`; bila masih ambigu, berhenti dan tanyakan.
- Keputusan berstatus **Accepted** dan **Proposed** di `docs/DECISIONS.md` sama-sama berlaku sebagai default. Perubahan atas keduanya dicatat sebagai entri baru.
- Fakta API Wallhaven berasal dari snapshot di `docs/API_CONTRACTS.md` §2. Verifikasi ulang ke https://wallhaven.cc/help/api bila snapshot lebih tua dari 90 hari, atau bila respons nyata berbeda dari snapshot, lalu perbarui snapshot beserta tanggalnya.

## 4. API eksternal

- Gunakan nama parameter, enum, dan field yang tercatat di snapshot atau dokumentasi resmi. Bila tidak tercatat di sana, verifikasi dulu.
- Anggap respons eksternal tidak tepercaya dan dapat berubah tanpa pemberitahuan: buat model serde hanya untuk field yang dipakai, abaikan field tak dikenal, dan beri `Option` pada field yang bisa hilang.
- Bila suatu kategori memerlukan API key, tampilkan petunjuk konfigurasi yang jelas.

## 5. Dependensi

- Tambahkan crate hanya bila diperlukan, jelaskan alasannya, dan pastikan cocok dengan versi Rust/Dioxus yang dikunci di repo.
- Periksa versi Dioxus di `Cargo.toml` dan baca dokumentasi untuk versi itu.
- Pilih satu crate per tugas.
- Pilihan teknologi dicatat di D-012 pada Fase 0.
- Jangan menjalankan perintah yang menghapus data pengguna atau mengubah sistem.

## 6. Aturan Rust

- Pada jalur runtime yang bisa gagal (input, jaringan, file, database), gunakan `Result` dan tipe error yang jelas; hindari `unwrap()`/`expect()`.
- Pesan error untuk pengguna harus bisa dipahami; detail teknis masuk ke log tanpa rahasia.
- Jalankan request jaringan, operasi file besar, dan query lambat di luar jalur render UI.
- Pisahkan UI dari akses API dan penyimpanan. Crate `core` tidak boleh bergantung pada Dioxus.
- Simpan file gambar di disk dan metadata di SQLite.
- Lewati download bila file valid sudah ada, kecuali pengguna memintanya ulang.
- Validasi dan bersihkan nama file sebelum dipakai sebagai path.
- Hindari global mutable state tanpa kebutuhan kuat.

## 7. Aturan UI

- Setiap layar data punya state loading, sukses, kosong, dan error (lihat `docs/UI_GUIDELINES.md`).
- Aksi async memberi umpan balik.
- Grid memakai thumbnail; gambar resolusi penuh dimuat hanya atas permintaan.
- Tampilkan konten sensitif hanya sesuai `docs/SECURITY_AND_CONTENT.md`.
- Sampaikan status dengan teks atau ikon, tidak hanya dengan warna.

## 8. Data dan konfigurasi

- API key, token, kredensial, dan data pribadi tidak masuk repo, fixture, test, atau log.
- Konfigurasi dan data ditulis ke direktori Linux standar (config/data/cache), tidak ke direktori repo.
- Kegagalan database atau file ditampilkan dengan aman dan tidak menghapus data pengguna secara otomatis.
- Perubahan skema database memakai migrasi bernomor.

## 9. Bila menemukan ambiguitas

- Pilih perilaku paling sederhana dan aman yang sesuai dokumen.
- Bila keputusan memengaruhi arsitektur, penyimpanan, keamanan, atau kompatibilitas, berhenti dan minta konfirmasi.
- Catat keputusan baru di `docs/DECISIONS.md`.

## 10. Definisi selesai

Tugas selesai bila:

- kriteria penerimaan tugas terpenuhi;
- format, check, dan test relevan dijalankan;
- error path utama ditangani;
- dokumentasi diperbarui bila perlu;
- tidak ada fitur di luar ruang lingkup;
- laporan akhir jujur tentang apa yang diuji dan tidak diuji.
