use ethers::types::Address;

#[derive(Clone, Debug, PartialEq)]
pub struct WalletConnect {
    pub account: Option<Address>,
    pub chain_id: u64,
}

impl WalletConnect {
    pub fn new() -> Self {
        WalletConnect {
            account: None,
            chain_id: 0,
        }
    }
}
