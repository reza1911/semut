use uuid::Uuid;

/// Struktur konfigurasi utama aplikasi.
pub struct Config {
    pub uuid: Uuid,
    pub host: String,
    pub proxy_addr: String,
    pub proxy_port: u16,
    pub main_page_url: String,
    pub sub_page_url: String,
}

impl Config {
    /// Konfigurasi manual tanpa memuat dari file atau URL.
    /// IP dan port diset langsung lewat variabel,
    /// dan path publik hanya menggunakan "/vmess".
    pub fn manual() -> Self {
        Config {
            uuid: Uuid::new_v4(),
            host: String::from("example.com"),          // Ganti jika punya domain sendiri
            proxy_addr: String::from("4.145.124.60"),   // IP proxy yang disembunyikan
            proxy_port: 443,                            // Port proxy
            main_page_url: String::from("/vmess"),      // Path publik
            sub_page_url: String::new(),                // Kosongkan jika tidak diperlukan
        }
    }
}
