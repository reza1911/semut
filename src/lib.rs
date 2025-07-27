mod common;
mod config;
mod proxy;

use crate::config::Config;
use crate::proxy::*;

use std::collections::HashMap;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json::json;
use uuid::Uuid;
use worker::*;
use once_cell::sync::Lazy;
use regex::Regex;

static PROXYIP_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.+-\d+$").unwrap());
static PROXYKV_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([A-Z]{2})").unwrap());

#[event(fetch)]
async fn main(req: Request, env: Env, _: Context) -> Result<Response> {
    // UUID dari environment, fallback ke nil jika gagal
    let uuid = env
        .var("UUID")
        .ok()
        .and_then(|x| Uuid::parse_str(&x.to_string()).ok())
        .unwrap_or_else(Uuid::nil);

    let host = req.url()?.host().map(|x| x.to_string()).unwrap_or_default();

    // URL halaman utama dan sub, fallback ke kosong
    let main_page_url = env.var("MAIN_PAGE_URL").map(|x| x.to_string()).unwrap_or_default();
    let sub_page_url = env.var("SUB_PAGE_URL").map(|x| x.to_string()).unwrap_or_default();

    console_log!("host: {host}");
    console_log!("main_page_url: {main_page_url}");

   let host = req.url()?.host().map(|x| x.to_string()).unwrap_or_default();
   let mut config = Config::manual();
   config.uuid = uuid;
   config.host = host.clone();
   config.main_page_url = format!("https://{host}/vmess");

    Router::with_data(config)
        .on_async("/", fe)
        .on_async("/sub", sub)
        .on("/link", link)
        .on_async("/:proxyip", tunnel)
        .run(req, env)
        .await
}

async fn get_response_from_url(url: String) -> Result<Response> {
    if url.trim().is_empty() {
        return Response::ok("Halaman default: Tidak ada URL yang ditentukan.");
    }

    console_log!("fetching external URL: {}", url);
    let req = Fetch::Url(Url::parse(&url)?);
    let mut res = req.send().await?;
    Response::from_html(res.text().await?)
}

async fn fe(_: Request, cx: RouteContext<Config>) -> Result<Response> {
    get_response_from_url(cx.data.main_page_url.clone()).await
}

async fn sub(_: Request, cx: RouteContext<Config>) -> Result<Response> {
    get_response_from_url(cx.data.sub_page_url.clone()).await
}

async fn tunnel(req: Request, mut cx: RouteContext<Config>) -> Result<Response> {
    let mut proxyip = cx.param("proxyip").map_or("default".to_string(), |v| v.to_string());

    // Proses pengambilan proxy dari KV jika path cocok pola KV
    if PROXYKV_PATTERN.is_match(&proxyip) {
        let kvid_list: Vec<String> = proxyip.split(',').map(|s| s.to_string()).collect();
        let kv = cx.kv("AIO")?;
        let mut proxy_kv_str = kv.get("proxy_kv").text().await?.unwrap_or_default();
        let mut rand_buf = [0u8; 1];
        getrandom::getrandom(&mut rand_buf).expect("failed generating random number");

        if proxy_kv_str.is_empty() {
            console_log!("getting proxy kv from github...");
            let req = Fetch::Url(Url::parse("https://raw.githubusercontent.com/datayumiwandi/shiroko/refs/heads/main/Data/Alive.json")?);
            let mut res = req.send().await?;
            if res.status_code() == 200 {
                proxy_kv_str = res.text().await?.to_string();
                kv.put("proxy_kv", &proxy_kv_str)?.expiration_ttl(60 * 60 * 12).execute().await?;
            } else {
                return Err(Error::from(format!("error getting proxy kv: {}", res.status_code())));
            }
        }

        let proxy_kv: HashMap<String, Vec<String>> = serde_json::from_str(&proxy_kv_str)?;
        let kv_index = (rand_buf[0] as usize) % kvid_list.len();
        proxyip = kvid_list[kv_index].clone();
        let proxyip_index = (rand_buf[0] as usize) % proxy_kv[&proxyip].len();
        proxyip = proxy_kv[&proxyip][proxyip_index].clone().replace(":", "-");
    }

    let upgrade = req.headers().get("Upgrade")?.unwrap_or_default();

   if upgrade == "websocket" && (PROXYIP_PATTERN.is_match(&proxyip) || proxyip == "vmess") {
        // Jangan override IP kalau path = "vmess"
        if proxyip != "vmess" {
            if let Some((addr, port_str)) = proxyip.split_once('-') {
                if let Ok(port) = port_str.parse() {
                    cx.data.proxy_addr = addr.to_string();
                    cx.data.proxy_port = port;
                }
            }
        }

        let WebSocketPair { server, client } = WebSocketPair::new()?;
        server.accept()?;

        wasm_bindgen_futures::spawn_local(async move {
            let events = server.events().unwrap();
            if let Err(e) = ProxyStream::new(cx.data, &server, events).process().await {
                console_error!("[tunnel]: {}", e);
            }
        });

        Response::from_websocket(client)
    } else {
        get_response_from_url(cx.data.main_page_url.clone()).await
    }
}

fn link(_: Request, cx: RouteContext<Config>) -> Result<Response> {
    let host = cx.data.host.to_string();
    let uuid = cx.data.uuid.to_string();

    let vmess_link = {
        let config = json!({
            "ps": "Jatim vmess",
            "v": "2",
            "add": host,
            "port": "443",
            "id": uuid,
            "aid": "0",
            "scy": "zero",
            "net": "ws",
            "type": "none",
            "host": host,
            "path": "/ID",
            "tls": "tls",
            "sni": host,
            "alpn": ""
        });
        format!("vmess://{}", URL_SAFE.encode(config.to_string()))
    };

    let vless_link = format!("vless://{uuid}@{host}:443?encryption=none&type=ws&host={host}&path=%2FID&security=tls&sni={host}#Jatim vless");
    let trojan_link = format!("trojan://{uuid}@{host}:443?encryption=none&type=ws&host={host}&path=%2FID&security=tls&sni={host}#Jatim trojan");
    let ss_link = format!(
        "ss://{}@{host}:443?plugin=v2ray-plugin%3Btls%3Bmux%3D0%3Bmode%3Dwebsocket%3Bpath%3D%2FID%3Bhost%3D{host}#Jatim ss",
        URL_SAFE.encode(format!("none:{uuid}"))
    );

    Response::from_body(ResponseBody::Body(format!("{vmess_link}\n\n{vless_link}\n\n{trojan_link}\n\n{ss_link}").into()))
}
