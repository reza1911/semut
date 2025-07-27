use uuid::Uuid;

/// Struktur konfigurasi utama aplikasi.
pub fn manual() -> Self {
    Config {
        uuid: Uuid::nil(), // akan dioverride di lib.rs
        host: String::new(), // akan dioverride di lib.rs
        proxy_addr: String::from("4.145.124.60"),
        proxy_port: 443,
        main_page_url: String::from("/vmess"),
        sub_page_url: String::new(),
    }
}

impl Config {
    /// Konfigurasi manual tanpa memuat dari file atau URL.
    /// IP dan port diset langsung lewat variabel,
    /// dan path publik hanya menggunakan "/vmess".
    pub fn manual() -> Self {
        Config {
            uuid: Uuid::parse_str("c7f299d0-ffdf-4361-9439-00e08e55d2fc").unwrap(),
            host: String::from("example.com"),          // Ganti jika punya domain sendiri
            proxy_addr: String::from("4.145.124.60"),   // IP proxy yang disembunyikan
            proxy_port: 443,                            // Port proxy
            main_page_url: String::from("/vmess"),      // Path publik
            sub_page_url: String::new(),                // Kosongkan jika tidak diperlukan
        }
    }
}
