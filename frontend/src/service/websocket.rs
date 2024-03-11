use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use log::{debug, error};
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew_agent::Dispatched;

use crate::services::event_bus::{EventBus, Request};

pub struct WebsocketService {
    pub tx: Sender<String>,
}

impl WebsocketService {
    // "ws://127.0.0.1:8080"
    pub fn new(url: &str) -> Self {
        let ws = WebSocket::open(url).unwrap_throw();

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);
        let mut event_bus = EventBus::dispatcher();

        spawn_local(async move {
            while let Some(s) = in_rx.next().await {
                debug!("got event from channel! {}", s);
                write.send(Message::Text(s)).await.unwrap_throw();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(data)) => {
                        debug!("from websocket: {}", &data);
                        event_bus.send(Request::EventBusMsg(data));
                    }
                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {
                            debug!("from websocket: {}", val);
                            event_bus.send(Request::EventBusMsg(val.into()));
                        }
                    }
                    Err(e) => {
                        error!("ws: {:?}", e)
                    }
                }
            }
            debug!("WebSocket Closed");
        });

        Self { tx: in_tx }
    }
}
