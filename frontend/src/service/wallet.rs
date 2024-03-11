use defi_wallet_connect::session::SessionInfo;
use defi_wallet_connect::{Client, Metadata, WCMiddleware};
use defi_wallet_connect::{ClientChannelMessage, ClientChannelMessageType};
use ethers::prelude::Middleware;
use ethers::types::H160;
use eyre::eyre;
use std::fs::File;
use std::io::prelude::*;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen(module = "/asset/js/walletconnect/esm/index.js")]
// #[wasm_bindgen(module = "/asset/js/walletconnect/umd/index.js")]
// extern "C" {
//     #[wasm_bindgen(catch, js_name = connect)]
//     pub async fn connect_provider() -> Result<(), JsValue>;
//     #[wasm_bindgen(catch, js_name = disconnect)]
//     pub async fn disconnect_provider() -> Result<(), JsValue>;
// }

#[wasm_bindgen(module = "/asset/js/web3modal/index.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = connectHandle)]
    pub async fn connect_provider() -> Result<(), JsValue>;
    #[wasm_bindgen(catch, js_name = disconnectHandle)]
    pub async fn disconnect_provider() -> Result<(), JsValue>;
}

#[wasm_bindgen(module = "/asset/js/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = connectWallet)]
    pub async fn connect_wallet();
    #[wasm_bindgen(js_name = newQR)]
    pub async fn new_qr(url: String);
}

fn add_network() {
    // metamask api js
    // await window.ethereum.request({
    //     method: "wallet_addEthereumChain",
    //     params: [
    //         {
    //             chainName: "BSC Testnet",
    //             chainId: "0x61",
    //             nativeCurrency: { name: "tBNB", decimals: 18, symbol: "tBNB" },
    //             rpcUrls: ["https://data-seed-prebsc-1-s1.binance.org:8545"],
    //         },
    //     ],
    // });
}

/// remove session.json to start new session
const G_FILENAME: &str = "session.json";

///  temporary session is stored to session.json
async fn make_client() -> eyre::Result<Client> {
    log::debug!("make_client {}", G_FILENAME);
    let filename = G_FILENAME;
    if let Ok(mut file) = File::open(filename) {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let session: SessionInfo = serde_json::from_str(&contents)?;
        let client = Client::restore(session).await?;
        log::debug!("restored client");
        Ok(client)
    } else {
        let client = Client::new(
            Metadata {
                description: "Defi WalletConnect example.".into(),
                url: "http://localhost:8080/".parse().expect("url"),
                icons: vec![],
                name: "Defi WalletConnect Web3 Example".into(),
            },
            None,
        )
        .await?;
        log::debug!("created client");
        Ok(client)
    }
}

pub fn write_session_to_file(info: &SessionInfo, filename: &str) -> eyre::Result<()> {
    let mut file = std::fs::File::create(filename)?;
    let buffer = serde_json::to_string(&info)?;
    // write buffer to file
    file.write_all(buffer.as_bytes())?;
    Ok(())
}

async fn eth_sign(client: Client, address: Vec<H160>) -> eyre::Result<()> {
    let middleware = WCMiddleware::new(client);
    // Note that `sign` on ethers middleware translate to `eth_sign` JSON-RPC method
    // which in Metamask docs is marked as "(insecure and unadvised to use)"
    // and some wallets may reject it
    let sig2 = middleware
        .sign("world".as_bytes().to_vec(), &address[0])
        .await;
    match sig2 {
        Ok(value) => log::debug!("sig2: {:?}", value),
        Err(_error) => log::debug!("not erorr, eth_sign not supported in the wallet"),
    }
    Ok(())
}

pub async fn get_url(client: Client) -> eyre::Result<String> {
    let url = client.get_connection_string().await?;
    Ok(url)
}

pub async fn listen() -> eyre::Result<()> {
    let filename = G_FILENAME;

    let mut client = make_client().await?;

    client
        .run_callback(Box::new(
            move |message: ClientChannelMessage| -> eyre::Result<()> {
                match message.state {
                    ClientChannelMessageType::Connected => {
                        log::debug!("Connected");
                        if let Some(info) = message.session {
                            log::debug!("session info: {:?}", info);
                            write_session_to_file(&info, filename)
                        } else {
                            Err(eyre!("no session info"))
                        }
                    }
                    ClientChannelMessageType::Disconnected => {
                        log::debug!("Disconnected");
                        if let Some(info) = message.session {
                            log::debug!("session info: {:?}", info);
                            Ok(())
                        } else {
                            Err(eyre!("no session info"))
                        }
                    }
                    ClientChannelMessageType::Connecting => {
                        log::debug!("Connecting");
                        if let Some(info) = &message.session {
                            info.uri().print_qr_uri();
                            write_session_to_file(info, filename)
                        } else {
                            Err(eyre!("no session info"))
                        }
                    }
                    ClientChannelMessageType::Updated => {
                        log::debug!("Updated");
                        if let Some(info) = &message.session {
                            write_session_to_file(info, filename)
                        } else {
                            Err(eyre!("no session info"))
                        }
                    }
                }
            },
        ))
        .await?;

    // qrcode display
    log::debug!(
        "connection string = {}",
        client.get_connection_string().await?
    );

    let (address, chain_id) = client.ensure_session().await?;
    log::debug!("address: {:?}", address);
    log::debug!("chain_id: {}", chain_id);

    // personal_sign is signing with document
    let sig1 = client.personal_sign("hello", &address[0]).await?;
    log::debug!("sig1: {:?}", sig1);

    // eth_sign  is signing directly with hash of message
    // because it's not secure and not recommended to use it
    // metamask and etc. will reject it, so that is not an error
    eth_sign(client, address).await?;

    Ok(())
}
