# Catatan Platform (Linux)

Dokumen ini berisi hal yang perlu diverifikasi per lingkungan. Catat hasil verifikasi di bagian "Hasil verifikasi" agar tidak diulang.

## 1. Dependensi sistem

Dioxus Desktop di Linux memakai WebView berbasis WebKitGTK. Daftar paket pasti mengikuti dokumentasi Dioxus untuk versi yang dipilih di D-012. Pada Arch Linux umumnya diperlukan paket seperti `webkit2gtk-4.1`, `gtk3`, dan `xdotool`; verifikasi terhadap dokumentasi versi tersebut sebelum menuliskannya sebagai instruksi build di README.

Crate `wallhaven-core` tidak membutuhkan paket GUI, sehingga dapat dibangun dan diuji di lingkungan minimal (termasuk CI).

## 2. Wayland dan X11

- Uji aplikasi di sesi Wayland dan X11. Aplikasi tidak boleh bergantung pada compositor tertentu.
- Beberapa kombinasi WebKitGTK, driver GPU, dan compositor dapat menimbulkan jendela kosong atau masalah rendering. Workaround yang dikenal (mis. menonaktifkan DMABUF renderer lewat variabel lingkungan WebKit) hanya didokumentasikan sebagai solusi pemecahan masalah bila terbukti diperlukan; jangan dijadikan default tanpa bukti.
- Catat compositor, driver, dan versi WebKitGTK yang dipakai saat menemukan masalah.

## 3. File picker

- Gunakan dialog yang cocok dengan Wayland dan X11 (umumnya lewat xdg-desktop-portal). Pilihan crate dicatat di D-012.
- Bila portal tidak tersedia, tampilkan pesan jelas dan sediakan input path manual pada Pengaturan; jangan crash.

## 4. Membuka URL eksternal

Buka tautan lewat mekanisme sistem (mis. `xdg-open`) setelah validasi scheme dan host (lihat `SECURITY_AND_CONTENT.md` §3).

## 5. Hasil verifikasi

### Fase 0 (2026-10-14)
- Distro: Arch Linux (rolling), kernel `7.2.9-arch1-1`.
- Toolchain: Rust `1.99.0`, Cargo `1.99.0`, Dioxus CLI `0.7.10`.
- Paket grafis terpasang: `gtk3` 3.24.52, `gtk4` 4.24.1, `cairo`, `pango`, paket portal `xdg-desktop-portal` 1.22.1 dan `xdg-desktop-portal-gtk`.
- `cargo check -p wallhaven-core` berhasil (17.72s, dengan dependensi jaringan dan kripto yang terunduh).
- Paket sistem tambahan terpasang: `webkit2gtk-4.1` 2.54.1, `xdotool` 4.20260303.1; linking test binary desktop sekarang berhasil.
- `cargo check --workspace`, `cargo test --workspace`, dan `cargo clippy --workspace --all-targets --all-features -- -D warnings` lulus.
- Aplikasi minimal `wallhaven-desktop` berhasil dibangun dan dijalankan pada Wayland (`DISPLAY=:1`, `WAYLAND_DISPLAY=wayland-1`) dan dengan `GDK_BACKEND=x11`; pengujian interaksi/manual rinci ditunda.
- Workaround DMABUF belum diperlukan bukti.
