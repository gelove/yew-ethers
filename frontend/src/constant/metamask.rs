#[derive(Clone, Debug, PartialEq)]
pub enum MetaMaskState {
    Connected,
    Disconnected,
    NoWallet,
    Loading,
}
