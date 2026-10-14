# Fixture Wallhaven

Contoh JSON yang strukturnya mengikuti dokumentasi resmi (snapshot 2026-10-14). ID, nama pengguna, dan nilai lain adalah data rekaan, bukan data nyata. Salin ke direktori test crate `wallhaven-core` untuk menguji mapper tanpa internet.

- `search_page.json` — halaman hasil search biasa (`meta.query` string, `meta.seed` null; satu item sfw dan satu sketchy).
- `search_exact_tag_random.json` — `meta.query` berbentuk objek dan `meta.seed` terisi.
- `wallpaper_detail.json` — respons `/w/<ID>` dengan `tags` dan `uploader`.

Tambahkan fixture untuk error (401, 429, JSON rusak) sesuai kebutuhan test; jangan memasukkan API key asli.
