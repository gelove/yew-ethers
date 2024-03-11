#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod service;
mod types;

use eyre::Result;
use once_cell::sync::Lazy;
use service::web3;
use std::{convert::Into, sync::Arc};
use tauri::{Manager, Window};
use tokio::sync::Mutex;
use walletconnect::{Client, Metadata, Transaction, Uri};

static CLIENT: Lazy<Arc<Mutex<Client>>> = Lazy::new(|| {
    let client = Client::new(
        "WalletConnect",
        Metadata {
            description: "Rust WalletConnect".into(),
            url: "https://alin.app".parse().expect("parse url error"),
            icons: vec!["https://avatars.githubusercontent.com/u/15058512?s=96&v=4"
                .parse()
                .expect("parse icons error")],
            name: "Rust WalletConnect".into(),
        },
    )
    .expect("The walletconnect service should initialize safely");
    Arc::new(Mutex::new(client))
});

pub fn init_client() {
    // let mut client = CLIENT.lock().await?;
    // theres also "add_detected_transports()" in the docs?
    // client.add_u2f_usb_hid_platform_transports();
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// Create the command:
// This command must be async so that it doesn't run on the main thread.
#[tauri::command]
async fn close_splashscreen(window: Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
async fn connect_wallet(window: Window) {
    let client = Arc::clone(&CLIENT);
    connect(window, client).await;
}

#[tauri::command]
async fn send_tx(window: Window) {
    let client = Arc::clone(&CLIENT);
    _send_tx(client).await;
}

// #[tauri::command]
// async fn tauri(window: Window) {
//     let client = Arc::clone(&CLIENT);
//     web3::get_balances(client).await;
// }

#[tauri::command]
async fn multi_send(window: Window) {
    // airdrop().await;
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![close_splashscreen, connect_wallet, send_tx])
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                // let window = app.get_window("main").unwrap();
                // window.open_devtools();
                // std::thread::spawn(move || {
                //     std::thread::sleep(std::time::Duration::from_secs(10));
                //     window.close_devtools();
                // });
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn connect(window: Window, client: Arc<Mutex<Client>>) -> Result<()> {
    let client = client.lock().await;

    // 将扫码地址输出到前端并由js处理为二维码显示
    let callback = |url: Uri| {
        // let url: &str = url.as_ref();
        // println!("{url}");
        window
            .emit(
                "send_qr",
                Payload {
                    message: url.to_string(),
                },
            )
            .unwrap();
    };
    // qr::print_with_url 在终端显示二维码 需打开'qr'特征功能
    let (accounts, chain_id) = client.ensure_session(callback).await?;

    println!("Connected chain({}) accounts:", chain_id);
    for account in &accounts {
        println!(" - {:?}", account);
    }
    // if (accounts.is_empty()) {}

    Ok(())
}

async fn _send_tx(client: Arc<Mutex<Client>>) -> Result<()> {
    let client = client.lock().await;
    let (accounts, chain_id) = client.accounts()?;

    let tx = client
        .send_transaction(Transaction {
            from: accounts[0],
            to: Some("391313794D97F2a0e75E6a88349De72352fc50cC".parse()?),
            value: 1_000_000_000u128.into(),
            ..Transaction::default()
        })
        .await?;

    println!(
        "Transaction sent:\n  https://goerli.etherscan.io/tx/{:?}",
        tx
    );

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_qr_run() -> Result<()> {
//         // 阻塞等待交易完成
//         let result = futures::executor::block_on(qr_run(qr::print_with_url))?;
//         println!("result => {:?}", result);
//         Ok(())
//     }
// }
