use eyre::{eyre, Context, ContextCompat, Result};
use gloo::dialogs::alert;
use log::debug;
use std::ops::{Add, Deref};
use std::str::FromStr;
use thiserror::Error;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web3::contract::{Contract, Options};
use web3::transports::eip_1193::Eip1193;
use web3::types::{Address, U256};
use web3::Web3;
use yew::prelude::*;

use crate::component::button::Button;
use crate::component::token_select::TokenSelect;
use crate::types::Token;
use crate::service::editor::get_content;
use crate::service::web3::{get_user_balances, parse_units, Web3Provider};

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
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

#[function_component(Airdrop)]
pub fn airdrop() -> Html {
    let provider_state = use_state(Web3Provider::new);
    let token_state = use_state_eq(Token::new);
    let tokens_state: UseStateHandle<Vec<Token>> = use_state_eq(Vec::new);

    {
        let provider = provider_state.web3.clone();
        let token_state = token_state.clone();
        let tokens_state = tokens_state.clone();
        use_effect_with_deps(
            move |_| {
                match provider {
                    None => {
                        alert("Please sign with your wallet");
                    }
                    Some(w3) => {
                        spawn_local(async move {
                            // expect_throw("accounts error")
                            let accounts = match w3.eth().accounts().await {
                                Ok(v) => v,
                                Err(_) => {
                                    alert("Please sign with your wallet");
                                    return;
                                }
                            };
                            if accounts.is_empty() {
                                alert("Please sign with your wallet");
                                return;
                            }
                            let tokens = get_user_balances(w3.deref(), accounts[0])
                                .await
                                .expect_throw("get_user_balances error");
                            debug!("tokens => {:?}", tokens);
                            if !tokens.is_empty() {
                                token_state.set(tokens[0].clone());
                            }
                            tokens_state.set(tokens);
                        });
                    }
                };
                || ()
            },
            (),
        );
    }

    async fn estimate_gas(
        w3: &Web3<Eip1193>,
        token: Address,
        recipients: Vec<Address>,
        amounts: Vec<U256>,
    ) -> Result<()> {
        let utils_addr = Address::from_str("0x2e5E9F116e8c1B166Bb5c66e98CCC5c5913C0E6f")?;
        let airdrop_contract =
            Contract::from_json(w3.eth(), utils_addr, include_bytes!("../abi/Utils.json"))?;
        let accounts = w3.eth().accounts().await?;
        if accounts.is_empty() {
            alert("Please sign with your wallet");
            return Err(eyre!(AirdropError::IsEmpty));
        }
        let gas_estimate = airdrop_contract
            .estimate_gas(
                "airdrop",
                (token, recipients, amounts),
                accounts[0],
                Options {
                    // amount of ETH we will put into the swap (10**18/20 => 0.05 eth).
                    value: Some(U256::exp10(18).checked_div(20.into()).unwrap()),
                    // gas: Some(500_000.into()),
                    ..Default::default()
                },
            )
            .await?;
        debug!("gas_estimate => {:?}", gas_estimate);
        Ok(())
    }

    async fn allowance(w3: &Web3<Eip1193>, token: Address) -> Result<U256> {
        let utils_addr = Address::from_str("0x2e5E9F116e8c1B166Bb5c66e98CCC5c5913C0E6f")?;
        let token_contract =
            Contract::from_json(w3.eth(), token, include_bytes!("../abi/Token.json"))?;
        let accounts = w3.eth().accounts().await?;
        if accounts.is_empty() {
            alert("Please sign with your wallet");
            return Err(eyre!(AirdropError::IsEmpty));
        }
        let owner = accounts[0];
        let allowance: U256 = token_contract
            .query(
                "allowance",
                (owner, utils_addr),
                None,
                Options::default(),
                None,
            )
            .await?;
        debug!("allowance: {:?}", allowance);
        Ok(allowance)
    }

    // 判断 allowance 是否足够, 不足则 approve
    async fn approve(
        w3: &Web3<Eip1193>,
        token: Address,
        spender: Address,
        amount: U256,
    ) -> Result<()> {
        let token_contract =
            Contract::from_json(w3.eth(), token, include_bytes!("../abi/Token.json"))?;
        let accounts = w3.eth().accounts().await?;
        let receipt = token_contract
            .call_with_confirmations(
                "approve",
                (spender, amount),
                accounts[0],
                Options::default(),
                3,
            )
            .await?;
        debug!("receipt: {:?}", receipt);
        debug!("approve finished");
        Ok(())
    }

    /**
    0x36906395a28c44fed185E808d113b53e7029921E, 100
    0x391313794D97F2a0e75E6a88349De72352fc50cC, 200
    0x20acc5427a362E6eB18D7824664075Ea166136c5, 300
        */
    async fn multi_send(
        w3: &Web3<Eip1193>,
        token: Address,
        recipients: Vec<Address>,
        amounts: Vec<U256>,
    ) -> Result<()> {
        let utils_addr = Address::from_str("0x2e5E9F116e8c1B166Bb5c66e98CCC5c5913C0E6f")?;
        let utils_contract =
            Contract::from_json(w3.eth(), utils_addr, include_bytes!("../abi/Utils.json"))?;
        let accounts = w3.eth().accounts().await?;
        let receipt = utils_contract
            .call_with_confirmations(
                "airdrop",
                (token, recipients, amounts),
                accounts[0],
                Options::default(),
                3,
            )
            .await?;
        debug!("receipt: {:?}", receipt);
        Ok(())
    }

    fn process(text: String, token_decimal: u32) -> Result<(Vec<Address>, Vec<U256>, u32, U256)> {
        let blank = String::new();
        if text == blank {
            return Err(eyre!(AirdropError::IsEmpty));
        }
        let lines = text.lines();
        let mut count: u32 = 0;
        let mut total_amount = U256::zero();
        let mut recipients: Vec<Address> = Vec::new();
        let mut amounts: Vec<U256> = Vec::new();
        for line in lines {
            count += 1;
            let mut args = line.split(',');
            let recipient = args
                .next()
                .with_context(|| format!("第{}行输入账号有误", count))?
                .trim();
            let recipient = Address::from_str(recipient)
                .with_context(|| format!("第{}行输入账号有误", count))?;
            recipients.push(recipient);
            let amount = args
                .next()
                .with_context(|| format!("第{}行输入金额有误", count))?
                .trim();

            let amount = parse_units(amount, token_decimal)?;
            amounts.push(amount);
            total_amount = total_amount.add(amount);
        }
        Ok((recipients, amounts, count, total_amount))
    }

    let onclick = {
        let token_state = token_state.clone();
        let provider_state = provider_state.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let provider = provider_state.deref().clone();
            let w3 = match provider.web3 {
                Some(v) => v,
                None => {
                    alert("provider不得为空");
                    return;
                }
            };
            debug!("w3 {:?}", w3);

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

            let text = get_content();
            debug!("get_content => {}", text);
            let (recipients, amounts, count, total_amount) = match process(text, token_decimal) {
                Ok(v) => v,
                Err(err) => {
                    alert(&err.to_string());
                    return;
                }
            };

            debug!(
                "count => {:?}, recipients => {:#?}, amounts => {:#?} ",
                count, recipients, amounts,
            );

            spawn_local(async move {
                let _allowance = allowance(w3.deref(), token_address)
                    .await
                    .expect_throw("get allowance error");
                if _allowance.lt(&total_amount) {
                    // 授信
                    let utils_addr =
                        Address::from_str("0x2e5E9F116e8c1B166Bb5c66e98CCC5c5913C0E6f")
                            .expect_throw("utils_addr error");
                    approve(w3.deref(), token_address, utils_addr, total_amount).await;
                    return;
                }

                let res = multi_send(w3.deref(), token_address, recipients, amounts).await;
            })
        })
    };

    let onchange = {
        let token_state = token_state.clone();
        let tokens_state = tokens_state.clone();
        Callback::from(move |value: String| {
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
            <TokenSelect name="代币" options={tokens_state.deref().clone()} onchange={onchange.clone()}/>
            <iframe id="editor" width="100%" height="320" src="/asset/js/editor/index.html" style="padding:0;margin:0" />
            <Button {onclick}>{"空投"}</Button>
        </div>
    }
}
