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
    /// Konfigurasi manual default tanpa mengambil dari env.
    /// UUID dan host akan dioverride di `lib.rs`.
    pub fn manual() -> Self {
        Config {
            uuid: Uuid::nil(), // akan diisi ulang di lib.rs
            host: String::new(), // akan diisi ulang di lib.rs
            proxy_addr: String::from("172.232.234.62"), // IP proxy disembunyikan
            proxy_port: 443,
            main_page_url: String::from("/vmess"),
            sub_page_url: String::new(),
        }
    }
}
