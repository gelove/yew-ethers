use ethers::prelude::*;
use eyre::Result;

pub async fn transfer(provider: Provider<Http>) -> Result<U256> {
    // Connect to the ethereum network with anvil
    // Provider::<http::Provider> 实现了 try_from
    // let provider = Provider::try_from("http://localhost:8545")?;
    // Get accounts
    let block_number = provider.get_block_number().await?;
    println!("block_number: {:?}", block_number);

    let accounts = provider.get_accounts().await?;
    println!("accounts: {:?}", accounts);
    let from = accounts[0];
    let to = accounts[1];

    // Get initial balance
    let balance_before = provider.get_balance(from, None).await?;

    // Make transaction requests from accounts
    let tx = TransactionRequest::new().to(to).value(1).from(from);

    // Make Transaction
    let tx = provider.send_transaction(tx, None).await?;

    println!("TX Hash: {:#?}", tx);

    // Get transaction count by BlockNumber 获取指定区块的nonce
    let nonce = provider
        .get_transaction_count(from, Some(BlockNumber::Latest.into()))
        .await?;
    println!("nonce: {} ", nonce);

    // Get final balance
    let balance_after = provider.get_balance(from, None).await?;

    assert!(balance_after < balance_before);

    println!("Balance before {}", balance_before);
    println!("Balance after {}", balance_after);

    Ok(balance_after)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    use tokio_test::block_on;

    #[test]
    fn test_transfer() -> Result<()> {
        let provider = Provider::try_from("http://localhost:8545")?;
        // 阻塞等待交易完成
        let result = block_on(transfer(provider))?;
        println!("result => {:?}", result);
        Ok(())
    }
}
