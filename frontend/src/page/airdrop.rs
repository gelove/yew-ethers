use ethers::prelude::*;
use gloo::dialogs::alert;
use std::ops::Deref;
use std::str::FromStr;
use thiserror::Error;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use crate::component::base_info::BaseInfo;
use crate::component::button::Button;
use crate::component::token_select::TokenSelect;
use crate::service::editor;
use crate::types::Token;

#[derive(Error, Debug)]
pub enum AirdropError {
    #[error("Account is unavailable")]
    UnavailableAccount,
    #[error("数据不得为空")]
    IsEmpty,
    #[error("第`{0}`行输入收款账号有误")]
    InvalidRecipient(u32),
    #[error("第`{0}`行输入金额有误")]
    InvalidAmount(u32),
    #[error("disconnected")]
    Disconnect(#[from] std::io::Error),
    #[error("invalid param (expected {expected:?}, found {found:?})")]
    InvalidParam { expected: String, found: String },
    #[error("unknown error")]
    Unknown,
}

#[function_component(Airdrop)]
pub fn airdrop() -> Html {
    // let provider_state = use_state(Web3Provider::new);
    let token_state = use_state_eq(Token::new);
    let tokens_state: UseStateHandle<Vec<Token>> = use_state_eq(Vec::new);

    {
        let token_state = token_state.clone();
        let tokens_state = tokens_state.clone();
        use_effect_with_deps(move |_| || (), ());
    }

    let onclick = {
        let token_state = token_state.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            // let provider = provider_state.deref().clone();
            // let w3 = match provider.web3 {
            //     Some(v) => v,
            //     None => {
            //         alert("provider不得为空");
            //         return;
            //     }
            // };
            // log::debug!("w3 {:?}", w3);

            let token = token_state.deref().clone();
            // token address like 0x38B48a6A8E7b6c1f9064553c4EA2Cc532D3D29Fc
            let token_address = match Address::from_str(&token.address) {
                Ok(v) => v,
                Err(_) => {
                    alert("代币合约地址非法(应以`0x`开始)");
                    return;
                }
            };
            let token_decimal = match token.decimal.parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    alert("代币小数位数非法");
                    return;
                }
            };

            let text = editor::get_content();
            log::debug!("get_content => {}", text);
            // let (recipients, amounts, count, total_amount) = match process(text, token_decimal) {
            //     Ok(v) => v,
            //     Err(err) => {
            //         alert(&err.to_string());
            //         return;
            //     }
            // };

            // log::debug!(
            //     "count => {:?}, recipients => {:#?}, amounts => {:#?} ",
            //     count, recipients, amounts,
            // );

            spawn_local(async move {})
        })
    };

    let onchange = {
        let token_state = token_state.clone();
        let tokens_state = tokens_state.clone();
        Callback::from(move |(name, value): (AttrValue, String)| {
            log::debug!("name, value => {}, {}", name, value);
            let list = tokens_state.deref().clone();
            for v in list.iter() {
                if v.address == value {
                    token_state.set(v.clone());
                }
            }
        })
    };

    html! {
        <div class="flex flex-col">
            <BaseInfo name="alice" age={8} icon="heh" />
            <TokenSelect name="代币" options={tokens_state.deref().clone()} onchange={onchange.clone()} />
            <iframe id="editor" width="100%" height="320" src="/asset/js/editor/index.html" style="padding:0;margin:0" />
            <Button {onclick}>{"空投"}</Button>
        </div>
    }
}
