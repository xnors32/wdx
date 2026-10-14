# Roadmap Pasca-MVP

Daftar ide, bukan komitmen. Setiap item membutuhkan keputusan di `DECISIONS.md` sebelum dikerjakan.

## Kandidat dekat
- Koleksi lokal (P2): grup wallpaper buatan pengguna.
- Download queue lanjutan: antrean, retry, dan cancel.
- Cache thumbnail dengan batas ukuran dan pembersihan aman (bila belum selesai di Fase 5).
- Penyimpanan API key lewat keyring desktop (Secret Service).
- Peningkatan aksesibilitas dan performa grid (virtualization).

## Kandidat menengah
- Membaca koleksi akun Wallhaven secara read-only (endpoint `/collections`, membutuhkan API key).
- Penjelajahan tag (endpoint `/tag/<ID>`) dan pencarian serupa (`like:<ID>`).
- Simpan pencarian/filter favorit (dengan kebijakan privasi riwayat yang jelas).
- Ekspor/impor favorit lokal.

## Membutuhkan keputusan arsitektur
- Menerapkan wallpaper ke desktop: adapter opsional di luar `wallhaven-core`, tanpa dependensi wajib pada compositor tertentu (lihat D-002).
- Provider kedua: butuh evaluasi kebijakan dan legalitas (lihat `SECURITY_AND_CONTENT.md` §2).
- Paket distribusi (mis. PKGBUILD untuk Arch).
