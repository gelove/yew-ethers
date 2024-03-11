use crate::component::button::Button;
use ethers::{prelude::*, providers::Ws};
use futures::{future, pin_mut};
use futures::{sink::SinkExt, stream::StreamExt};
use log::{debug, error, info};
use serde_json::{json, value::Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(Presale)]
pub fn presale() -> Html {
    let onclick = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            spawn_local(async move {
                let endpoint =
                    "wss://maximum-dimensional-hexagon.bsc.discover.quiknode.pro/fe817de3a123037c66e356ac64918d113d52507f/";
                let provider = Provider::connect(endpoint)
                    .await
                    .expect_throw("could not instantiate WebSocket Provider");

                let uniswap = "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984";
                // let params = json!([
                //     "alchemy_filteredNewFullPendingTransactions",
                //     json!({ "toAddress": uniswap })
                // ]);
                let params = json!([
                    "eth_newPendingTransactionFilter",
                    json!({ "toAddress": uniswap })
                ]);
                debug!("Constructed Params:{:?}", params);
                // alchemy_pendingTransactions or alchemy_filteredNewFullPendingTransactions
                // subscribe pending transactions  监听待执行的交易
                let id: U256 = provider
                    .request("eth_subscribe", params)
                    .await
                    .unwrap_throw();
                debug!("Sent request with id: {:?}", id);
                let mut stream: SubscriptionStream<'_, ethers::providers::Ws, Transaction> =
                    SubscriptionStream::new(id, &provider)
                        .map(Into::into)
                        .unwrap_throw();
                while let Some(tx) = stream.next().await {
                    debug!("tx: {:?}", tx);
                    if tx.from.to_string() != "creator" {
                        continue;
                    }
                    debug!("value {:?}", tx.value);
                    debug!("input {:?}", tx.input);
                    debug!("transaction_type {:?}", tx.transaction_type);
                }
            });
        })
    };

    html! {
        <Button {onclick}>{"抢购"}</Button>
    }
}
