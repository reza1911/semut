# Siren

**V2Ray Tanpa Server (Serverless) yang Dioptimalkan untuk Indonesia 🇮🇩**

Saya menggandakan repositori publik dari URL berikut: [Changli](https://github.com/Mayumiwandi/Changli).

---

## 🔧 Fitur

- ✅ **Multi-Protokol**
  - Vmess
  - Trojan
  - Vless
  - Shadowsocks

- ✅ **Domain over HTTPS (DoH)**
  Mengenkripsi permintaan DNS untuk meningkatkan privasi dan keamanan.

---

## 🌐 Endpoint

| Endpoint | Deskripsi                                |
|----------|------------------------------------------|
| `/`      | Halaman utama                            |
| `/link`  | Membuat akun 4 protocol otomatis         |
| `/sub`   | Memilih server dan protokol sendiri      |

---

## 🚀 Panduan Deploy

Dapat dengan mudah di-deploy menggunakan GitHub Actions bersama Cloudflare Workers.

### ⚙️ CI/CD via GitHub Actions

1. **Buat Namespace KV**

   - Masuk ke Cloudflare Dashboard → Workers → KV
   - Buat namespace baru dengan nama `AIO`

2. **Konfigurasi `wrangler.toml`**

   Tambahkan konfigurasi berikut ke dalam file `wrangler.toml` Anda:

   ```toml
   [[kv_namespaces]]
   binding = "JATIM"
   id = "ID_NAMESPACE_KV_ANDA"
   ```

3. **Buat API Token**

   - [Buat API Token](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/) dengan izin:
     - Workers
     - KV Storage

4. **Atur Secret di GitHub**

   - Masuk ke GitHub → Repositori Anda → Settings → Secrets and variables → Actions
   - Tambahkan secret baru:
     - Nama: `CF`
     - Nilai: Token API Anda

5. **Edit manual UUID dan domain di web/sub.html**

   - domain baris ke 1388
   - UUID baris ke 1390

6. **Aktifkan GitHub Actions**

   - Buka tab **Actions** di repositori GitHub Anda
   - Aktifkan workflow jika diminta

7. **Jalankan Deploy**

   - Push commit baru atau jalankan workflow deployment secara manual

8. **Akses Worker Anda**

   - Buka URL: `https://<SUBDOMAIN-WORKERS-ANDA>.workers.dev`
