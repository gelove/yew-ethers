#![allow(clippy::all)]

use ethers::{prelude::*, providers::Ws};
use log::{debug, error};
use serde_json::{json, value::Value};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn subscribe() {
    // let endpoint = "ws://127.0.0.1:8545";
    let endpoint = "wss://eth-mainnet.g.alchemy.com/v2/y8cmG9BNP7UeJVYwtvKeVoxkfqMIye0f";
    let provider = Provider::new(Ws::connect(endpoint).await.unwrap_throw());
    let provider = Arc::new(provider);
    debug!("Provider connected to `{:?}`", endpoint);

    // subscribe pending transactions
    // 监听待执行的交易
    let mut stream = provider.subscribe_pending_txs().await.unwrap_throw();
    while let Some(tx) = stream.next().await {
        debug!("tx: {:?}", tx);
    }
}

#[wasm_bindgen]
pub async fn subscribe_alchemy() {
    let endpoint = "wss://eth-mainnet.g.alchemy.com/v2/y8cmG9BNP7UeJVYwtvKeVoxkfqMIye0f";
    let provider = Provider::connect(endpoint)
        .await
        .expect_throw("could not instantiate WebSocket Provider");

    let uniswap = "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984";
    // let mut param_map = serde_json::Map::new();
    // param_map.insert("toAddress".to_owned(), Value::String(uniswap.to_owned()));
    // param_map["toAddress"] = json!(uniswap);
    // let params = Value::Array(vec![
    //     Value::String("alchemy_filteredNewFullPendingTransactions".to_owned()),
    //     Value::Object(param_map),
    // ]);
    let params = json!([
        "alchemy_filteredNewFullPendingTransactions",
        json!({ "toAddress": uniswap })
    ]);
    debug!("Constructed Params: {:?}", params);
    // alchemy_pendingTransactions or alchemy_filteredNewFullPendingTransactions
    // subscribe pending transactions  监听待执行的交易
    let id: U256 = provider
        .request("eth_subscribe", params)
        .await
        .unwrap_throw();
    debug!("Sent request with id: {:?}", id);
    // first approach
    // let mut stream: SubscriptionStream<'_, ethers::providers::Ws, Transaction> =
    //     SubscriptionStream::new(id, &provider).map(Into::into).unwrap_throw();
    // while let Some(tx) = stream.next().await {
    //     debug!("tx: {:?}", tx);
    // }
    // second approach
    if let Ok(stream) = SubscriptionStream::new(id, &provider).map(Into::into) {
        let mut stream: SubscriptionStream<'_, Ws, Transaction> = stream;
        while let Some(tx) = stream.next().await {
            debug!("tx: {:?}", tx);
        }
    };
}
