use eyre::Result;
use walletconnect::{qr, Client, Metadata, Transaction};

// Note: 使用了 wasm 不支持的 udp 协议
async fn qr_run() -> Result<()> {
    let client = Client::new(
        "WalletConnect",
        Metadata {
            description: "Rust WalletConnect".into(),
            url: "https://allens.top".parse()?,
            icons: vec!["https://avatars.githubusercontent.com/u/15058512?s=96&v=4".parse()?],
            name: "Rust WalletConnect".into(),
        },
    )?;

    let (accounts, _) = client.ensure_session(qr::print_with_url).await?;

    println!("Connected accounts:");
    for account in &accounts {
        println!(" - {:?}", account);
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_run() -> Result<()> {
        // 阻塞等待交易完成
        let result = futures::executor::block_on(qr_run())?;
        println!("result => {:?}", result);
        Ok(())
    }
}
