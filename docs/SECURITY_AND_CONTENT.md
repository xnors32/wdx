# Keamanan, API Key, dan Konten

## 1. API key

- API key Wallhaven adalah rahasia pengguna.
- Jangan menaruh key di source code, contoh config dengan nilai asli, screenshot, test fixture, atau log.
- Kirim key lewat header `X-API-Key`, bukan query string (`?apikey=`), agar tidak masuk URL, log, atau riwayat. Pastikan fitur logging/tracing HTTP tidak mencetak header autentikasi.
- UI memakai field masked untuk input key.
- Sumber key: variabel lingkungan `WALLHAVEN_API_KEY` (pengembangan, Fase 1–5) dan penyimpanan lokal dari Settings (Fase 6).
- Penyimpanan lokal: file konfigurasi terpisah dari preferensi non-rahasia, dengan permission `0600` (hanya pengguna) dan direktori induk yang tidak dapat ditulis orang lain. Verifikasi permission saat dibaca; bila lebih longgar, perbaiki atau peringatkan.
- Keyring desktop (Secret Service) adalah peningkatan masa depan (lihat `ROADMAP.md`); jangan menambah dependensi keyring tanpa keputusan baru.
- Tampilkan key hanya dalam bentuk tersamar (mis. beberapa karakter terakhir) dan sediakan aksi hapus key.
- Jangan mengirim key ke host selain `wallhaven.cc`.

## 2. Konten dan purity

- Default: SFW (`purity=100`).
- Pilihan purity sesuai snapshot API: SFW, Sketchy, NSFW. NSFW membutuhkan API key yang valid; Sketchy tidak.
- Filter diterapkan pada request provider (parameter `purity`), bukan hanya menyembunyikan item setelah diterima.
- Mengaktifkan Sketchy/NSFW membutuhkan konfirmasi pengguna (lihat `UI_GUIDELINES.md` §6).
- Bila key ditolak (`401`), tampilkan petunjuk dan jangan melewati pembatasan akses.
- Jangan bypass autentikasi, pembatasan akses, atau kebijakan layanan.
- Blur thumbnail adalah lapisan tambahan, bukan pengganti kontrol konten.
- Provider tambahan membutuhkan evaluasi kebijakan dan legalitas terpisah sebelum integrasi.

## 3. Keamanan jaringan

- Allowlist host untuk request dan pemuatan gambar: `wallhaven.cc`, `w.wallhaven.cc`, `th.wallhaven.cc` (dari snapshot `API_CONTRACTS.md` §2). Request ke host lain ditolak.
- Hanya `https`. Tolak scheme lain kecuali ada alasan eksplisit dan keputusan tercatat.
- Redirect: batasi jumlahnya, dan hanya ikuti bila tujuan juga berada di allowlist dan memakai `https`. Jangan mengirim header autentikasi ke host di luar `wallhaven.cc`.
- Download: tetapkan batas ukuran maksimum yang dapat dikonfigurasi (nilai awal yang wajar, mis. 100 MiB), dan tangani respons tanpa `Content-Length` dengan menghitung byte yang diterima.
- Jangan memercayai `Content-Type`; validasi tipe gambar dari byte awal file (signature) dan tentukan ekstensi dari hasil validasi.
- Validasi URL eksternal sebelum membuka di browser (hanya `https` dan host yang diharapkan, atau konfirmasi pengguna).
- Atur Content-Security-Policy WebView seketat mungkin yang tetap bekerja pada versi Dioxus terpilih. Minimal: batasi `img-src` ke `'self'`, `data:`, dan host thumbnail/gambar di allowlist, serta larang skrip dari sumber eksternal. Verifikasi bahwa kebijakan tidak merusak mekanisme komunikasi bawaan Dioxus, lalu catat hasilnya.
- Nonaktifkan devtools WebView dan batasi navigasi WebView ke konten aplikasi pada build rilis.
- Hormati rate limit: seluruh request API melewati satu limiter.

## 4. Keamanan file

- Jangan memakai nama file dari server sebagai path; bentuk nama sendiri (lihat `DATA_MODEL.md` §5).
- Bersihkan nama file dan cegah path traversal (`..`, separator, path absolut); tolak ID di luar `[a-z0-9]+`.
- Jangan menimpa file tanpa persetujuan pengguna.
- Unduh ke file sementara dan finalisasi setelah verifikasi sukses; hapus file sementara bila gagal.
- Jangan menghapus direktori atau file pengguna secara rekursif tanpa tindakan eksplisit; pembersihan cache hanya menyentuh direktori cache aplikasi.
- Periksa izin filesystem dan laporkan kegagalan dengan jelas.

## 5. Privasi dan logging

- Log error teknis secukupnya, tanpa API key, token, atau header rahasia.
- Tidak ada telemetry atau pengiriman statistik penggunaan pada MVP.
- Jangan menyimpan riwayat pencarian kecuali fitur dan kebijakannya disetujui.

## 6. Lisensi dan atribusi

- Simpan URL sumber dan provider ID bila tersedia.
- Wallhaven menyatakan semua gambar tetap milik pemilik aslinya; jangan menyatakan gambar bebas lisensi hanya karena dapat diunduh.
- Hormati ketentuan penggunaan provider dan hak cipta gambar.
- Tampilkan catatan singkat bahwa pengguna bertanggung jawab memastikan penggunaan gambar sesuai lisensi yang berlaku (mis. di halaman Pengaturan atau Tentang).
