use ethers::types::Address;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::utils::to_js;

/// 调用 tauri Event
#[wasm_bindgen(
    inline_js = "export function invoke_tauri(cmd, args = {}) { console.log('args', args); return window.__TAURI__._invoke(cmd, args, window.__TAURI_INVOKE_KEY__) }"
)]
extern "C" {
    async fn invoke_tauri(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn invoke<M: Serialize + Deserialize<'static>>(args: InvokeCommand<M>) -> JsValue {
    invoke_tauri("tauri".into(), to_js(&args).unwrap()).await
}

#[derive(Serialize, Deserialize)]
pub struct InvokeCommand<M> {
    #[serde(rename = "__tauriModule")]
    pub tauri_module: String,
    pub message: M,
}

pub trait TauriMessage<M>
where
    M: Serialize + Deserialize<'static>,
{
    fn event(&self) -> M;
    fn module(&self) -> String;
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    GetAppVersion,
    TokenBalances(Address, Vec<Address>),
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub event: Event,
}

impl TauriMessage<Event> for Message {
    fn event(&self) -> Event {
        self.event.clone()
    }

    fn module(&self) -> String {
        "Event".into()
    }
}

impl From<Event> for InvokeCommand<Message> {
    fn from(event: Event) -> Self {
        let msg = Message { event };
        InvokeCommand {
            tauri_module: msg.module().into(),
            message: msg,
        }
    }
}

#[test]
fn tauri_invoke() -> () {
    // args = {"__tauriModule": "Event", "message": "getEventVersion"} ???
    let args = InvokeCommand::from(Event::GetAppVersion);
    wasm_bindgen_futures::spawn_local(async move {
        let result = invoke(args).await;
        log::info!("{}", result.as_string().unwrap())
    })
}
