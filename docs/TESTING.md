# Testing dan Quality Gate

## 1. Perintah dasar

Jalankan dari root repo:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Bila `--all-features` tidak sesuai karena fitur tertentu memerlukan sistem eksternal, jalankan konfigurasi yang valid dan jelaskan alasannya. Crate `wallhaven-core` dapat diuji sendiri (`cargo test -p wallhaven-core`) tanpa dependensi GUI. Jangan mengklaim suatu perintah berhasil bila tidak dijalankan.

Opsional namun disarankan sebelum rilis: `cargo audit` atau `cargo deny check` untuk memeriksa kerentanan dan lisensi dependensi.

## 2. Unit tests

Prioritaskan:
- mapping DTO Wallhaven ke model domain (search dan detail);
- parsing field opsional dan `meta.query` yang polimorfik (string/null/objek);
- normalisasi parameter: bit purity/kategori, penghapusan NSFW tanpa key, `topRange` hanya untuk `toplist`, seed hanya untuk `random`;
- validasi filter terhadap daftar nilai yang terverifikasi (mis. daftar warna);
- pembentukan nama file aman dan penanganan tabrakan nama;
- pemetaan error HTTP/provider (401, 429, 5xx, JSON rusak);
- rate limiter (tidak melebihi 45/menit pada jam simulasi);
- state download (transisi status yang valid);
- validasi konfigurasi purity.

## 3. Test provider

- Gunakan fixture di `docs/fixtures/wallhaven/` (salin ke direktori test crate). Fixture adalah contoh berbasis dokumentasi resmi, bukan data nyata.
- Unit test tidak membutuhkan internet.
- Untuk menguji client HTTP, gunakan server mock lokal (mis. `wiremock` atau `httpmock`; pilihan dicatat di D-012).
- Integration test live API disediakan terpisah (mis. di-`#[ignore]`) dan bukan syarat test lokal.
- Jangan memasukkan API key asli ke test atau fixture.
- Jangan bergantung pada urutan hasil API live.

## 4. Test storage

- Gunakan database sementara.
- Uji insert/read/delete favorit.
- Uji migrasi dari skema sebelumnya bila ada, dan bahwa migrasi gagal tidak menghapus data.
- Uji path file yang hilang.
- Pastikan operasi yang gagal tidak membuat metadata tampak sukses.

## 5. Test download

Uji atau simulasikan dengan server mock:
- respons sukses;
- HTTP error;
- timeout;
- koneksi terputus di tengah;
- respons tanpa `Content-Length`;
- respons melebihi batas ukuran;
- isi bukan gambar valid (cek byte awal) meski `Content-Type` mengaku gambar;
- redirect ke host di luar allowlist;
- file tujuan sudah ada;
- direktori tidak bisa ditulis;
- nama file berisi karakter tidak aman;
- pembatalan bila didukung.

Jangan mengunduh file besar dari internet dalam test unit.

## 6. Test UI/manual

Checklist:
- aplikasi terbuka tanpa crash di Wayland dan X11;
- loading dan error terlihat; hasil kosong terlihat;
- pencarian cepat berturut-turut tidak menampilkan hasil lama;
- thumbnail gagal tidak merusak grid;
- detail dengan metadata kosong tetap berfungsi;
- progress download berubah dengan benar;
- favorit bertahan setelah restart;
- file yang hilang ditandai;
- preferensi SFW dipatuhi; Sketchy/NSFW membutuhkan konfirmasi, dan NSFW membutuhkan key;
- navigasi keyboard dasar bekerja (Enter mencari, Esc menutup detail, fokus terlihat);
- tidak ada kontrol untuk fitur di luar ruang lingkup `PRD.md` §5.

## 7. Definisi lulus

Fase belum lulus bila:
- kode tidak dikompilasi;
- test yang diwajibkan gagal tanpa penjelasan;
- ada jalur error utama yang menyebabkan crash;
- rahasia tercetak ke log atau URL;
- file pengguna dapat tertimpa atau dihapus tanpa konfirmasi.
