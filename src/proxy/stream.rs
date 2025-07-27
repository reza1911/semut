use crate::config::Config;
use worker::*;
use futures_util::{StreamExt, SinkExt};
use std::time::Duration;
use gloo_timers::future::TimeoutFuture;

pub struct ProxyStream {
    config: Config,
    ws: WebSocket,
    events: WebSocketEvents,
}

impl ProxyStream {
    pub fn new(config: Config, ws: &WebSocket, events: WebSocketEvents) -> Self {
        Self {
            config,
            ws: ws.clone(),
            events,
        }
    }

    pub async fn process(mut self) -> Result<()> {
        console_log!(
            "[proxy] starting tunnel to {}:{}",
            self.config.proxy_addr,
            self.config.proxy_port
        );

        // Keepalive ping setiap 15 detik
        let ws_keepalive = self.ws.clone();
        wasm_bindgen_futures::spawn_local(async move {
            loop {
                TimeoutFuture::new(15000).await;
                let _ = ws_keepalive.send_with_str("ping");
            }
        });

        while let Some(Ok(ev)) = self.events.next().await {
            match ev {
                WebSocketEvent::Message(msg) => {
                    match msg {
                        Message::Text(txt) => {
                            console_log!("[proxy] text received: {}", txt);
                        }
                        Message::Bytes(bytes) => {
                            self.ws.send_with_bytes(&bytes)?;
                        }
                    }
                }
                WebSocketEvent::Close(_) => {
                    console_log!("[proxy] closed");
                    break;
                }
                WebSocketEvent::Error(e) => {
                    console_error!("[proxy] error: {:?}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}
