use std::future;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::component::button::Button;
use crate::component::conn_button::ConnButton;
use crate::constant::metamask::MetaMaskState;
use crate::service::wallet;
use crate::types::WalletConnect;

#[function_component(Connect)]
pub fn connect() -> Html {
    let on_click = {
        Callback::from(move |_: MouseEvent| {
            let ctx = use_context::<WalletConnect>().unwrap();
            // let eth = web3_transport.web3.as_ref().unwrap_throw().eth();
            // let metamask_state = metamask_state.clone();
            spawn_local(async move {
                log::debug!("connect");
                // wallet::connect_wallet().await;
                // web3::send_tx().await;

                // if let Some(client) = ctx.client {
                //     let url = wallet::get_url(client).await.unwrap_or_default();
                //     wallet::new_qr(url).await;
                // }
                // let url = web3::get_url(ctx.client).await.unwrap_or_default();
                // wallet::new_qr(url).await;
            });
        })
    };

    html! {
        <>
        <Button onclick={on_click.clone()}>{"登录"}</Button>
        // <ConnButton metamask_state={metamask_state.deref().clone()} disabled={btn_disabled} handler={on_conn.clone()} wallet_addr={wallet_addr.deref().clone()} />
        </>
    }
}
