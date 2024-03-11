#![allow(clippy::all)]

use ethers::{
    prelude::*,
    providers::call_raw::{spoof, RawCall},
    utils::parse_ether,
};
use log::debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn pay() {
    let endpoint = "wss://eth-mainnet.g.alchemy.com/v2/y8cmG9BNP7UeJVYwtvKeVoxkfqMIye0f";
    let provider = Provider::connect(endpoint).await.unwrap_throw();

    let adr1: Address = "0x6fC21092DA55B392b045eD78F4732bff3C580e2c"
        .parse()
        .unwrap_throw();
    let adr2: Address = "0x295a70b2de5e3953354a6a8344e616ed314d7251"
        .parse()
        .unwrap_throw();
    let pay_amt = parse_ether(1u64).unwrap_throw();

    // Not enough ether to pay for the transaction
    let tx = TransactionRequest::pay(adr2, pay_amt).from(adr1).into();

    // 测试时重写账户adr1的余额
    // override the sender's balance for the call
    let state = spoof::balance(adr1, pay_amt * 2);
    debug!("state `{:?}`", state);
    provider.call_raw(&tx).state(&state).await.unwrap_throw();
}
