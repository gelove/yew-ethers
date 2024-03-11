use log::debug;
use std::future;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web3::api::Web3;
use web3::futures::StreamExt;
use web3::transports::eip_1193::{Eip1193, Provider};
use yew::prelude::*;

use crate::component::button::Button;
use crate::component::conn_button::ConnButton;
use crate::constant::metamask::MetaMaskState;
use crate::service::wallet;

#[function_component(Connect)]
pub fn connect() -> Html {
    let web3_transport = use_state(get_web3);
    let metamask_state = use_state_eq(|| MetaMaskState::Disconnected);
    let chain_id = use_state_eq(String::new);
    let wallet_addr = use_state_eq(|| String::from(""));

    let btn_disabled = metamask_state.deref().clone() == MetaMaskState::Connected
        || metamask_state.deref().clone() == MetaMaskState::Loading;

    let on_conn = {
        let web3_transport = web3_transport.clone();
        let metamask_state = metamask_state.clone();
        Callback::from(move |_: MouseEvent| {
            let eth = web3_transport.web3.as_ref().unwrap_throw().eth();
            let metamask_state = metamask_state.clone();
            spawn_local(async move {
                metamask_state.set(MetaMaskState::Loading);
                let res = eth.request_accounts().await;
                match res {
                    Ok(_) => metamask_state.set(MetaMaskState::Connected),
                    Err(_) => metamask_state.set(MetaMaskState::Disconnected),
                };
            });
        })
    };

    {
        let chain_id = chain_id.clone();
        let wallet_addr = wallet_addr.clone();
        let metamask_state = metamask_state.clone();
        let w3_ = web3_transport.web3.clone();
        use_effect_with_deps(
            move |_| {
                match w3_ {
                    None => metamask_state.set(MetaMaskState::NoWallet),
                    Some(w3) => {
                        {
                            let w3 = w3.clone();
                            let wallet_addr = wallet_addr.clone();
                            let chain_id = chain_id.clone();
                            let metamask_state = metamask_state.clone();
                            spawn_local(async move {
                                let chain_res = w3.eth().chain_id().await;
                                let accounts = w3.eth().accounts().await;
                                if let Ok(id) = chain_res {
                                    chain_id.set(id.to_string());
                                }
                                if let Ok(accounts) = accounts {
                                    if !accounts.is_empty() {
                                        wallet_addr.set(accounts[0].to_string());
                                        metamask_state.set(MetaMaskState::Connected);
                                    } else {
                                        wallet_addr.set("Unavailable".to_string());
                                        metamask_state.set(MetaMaskState::Disconnected);
                                    }
                                }
                            });
                        }

                        {
                            let w3 = w3.clone();
                            let wallet_addr = wallet_addr.clone();
                            let metamask_state = metamask_state.clone();
                            spawn_local(async move {
                                w3.transport()
                                    .accounts_changed_stream()
                                    .for_each(|addresses| {
                                        debug!("addresses, {:?}", addresses);
                                        if !addresses.is_empty() {
                                            wallet_addr.set(addresses[0].to_string());
                                        } else {
                                            wallet_addr.set("Unavailable".to_string());
                                            metamask_state.set(MetaMaskState::Disconnected);
                                        }
                                        future::ready(())
                                    })
                                    .await;
                            });
                        }

                        {
                            let w3 = w3.clone();
                            spawn_local(async move {
                                w3.transport()
                                    .chain_changed_stream()
                                    .for_each(|id| {
                                        debug!("chain_id, {:?}", id);
                                        chain_id.set(id.to_string());
                                        future::ready(())
                                    })
                                    .await;
                            });
                        }
                    }
                };
                || ()
            },
            (),
        );
    }
 
    let on_click = {
        Callback::from(move |_: MouseEvent| {
            spawn_local(async move {
                // wallet::connect_provider().await;
                wallet::connect_wallet().await;
            });
        })
    };

    html! {
        <>
            <Button onclick={on_click.clone()}>{"登录"}</Button>
            <ConnButton metamask_state={metamask_state.deref().clone()} disabled={btn_disabled} handler={on_conn.clone()} wallet_addr={wallet_addr.deref().clone()} />
        </>
    }
}
