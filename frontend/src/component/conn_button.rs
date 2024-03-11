use crate::component::button::Button;
use crate::constant::metamask::MetaMaskState;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ConnButtonProps {
    pub disabled: bool,
    pub metamask_state: MetaMaskState,
    pub handler: Callback<MouseEvent>,
    pub wallet_addr: String,
}

#[function_component(ConnButton)]
pub fn conn_button(props: &ConnButtonProps) -> Html {
    let ConnButtonProps {
        disabled,
        wallet_addr,
        metamask_state,
        handler,
    } = props;

    html! {
        if *metamask_state == MetaMaskState::NoWallet {
            <p>{"Please, install a ethereum compatible wallet"}</p>
        } else {
            <Button onclick={handler} loading={*metamask_state == MetaMaskState::Loading} disabled={*disabled}>
                <div class="flex items-center">
                    <img src="asset/image/metamask.svg" alt="MetaMask Icon" width="32" height="32" />
                    if *metamask_state == MetaMaskState::Disconnected {
                        <p class="ml-2">{"Connect to MetaMask"}</p>
                    } else {
                        <p class="ml-2">{wallet_addr}</p>
                    }
                </div>
            </Button>
        }
    }
}
