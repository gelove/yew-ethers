use std::str::FromStr;

use ethers::types::Address;
use eyre::Result;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use crate::service::tauri::{invoke, InvokeCommand, TauriMessage, Event};
use crate::types::Token;

#[derive(Clone, Debug, Deserialize)]
pub struct TxResponse {
    pub message: String,
    pub result: Vec<Token>,
    pub status: String,
}

// 获取用户所有的ERC20代币
pub async fn get_user_tokens(account: &str) -> Result<(Vec<Token>, Vec<Address>)> {
    // http://api.etherscan.io/api?module=account&action=tokentx&address=0x4e83362442b8d1bec281594cea3050c8eb01311c&apikey=FWIFYX812T9MP4CE7YMYRCHXQ4K9VAF7QV
    // http://api-goerli.etherscan.io/api?module=account&action=tokentx&address=0x4e83362442b8d1bec281594cea3050c8eb01311c&apikey=FWIFYX812T9MP4CE7YMYRCHXQ4K9VAF7QV
    let url = format!("http://api-goerli.etherscan.io/api?module=account&action=tokentx&address={}&apikey=FWIFYX812T9MP4CE7YMYRCHXQ4K9VAF7QV", account);
    // debug!("url => {:?}", url);
    let fetched: TxResponse = Request::get(&url).send().await?.json().await?;
    // debug!("fetched.result => {:?}", fetched.result);
    let mut list: Vec<String> = Vec::new();
    let mut token_addresses: Vec<Address> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();
    for v in fetched.result.iter() {
        if !list.contains(&v.address) {
            list.push(v.address.clone());
            token_addresses.push(Address::from_str(&v.address)?);
            tokens.push(v.clone());
        }
    }

    Ok((tokens, token_addresses))
}


pub async fn get_balances(account: &str) -> Result<()> {
    let (tokens, token_addresses) = get_user_tokens(account).await?;
    let account = Address::from_str(account)?;
    let args = InvokeCommand::from(Event::TokenBalances(account, token_addresses));
    let result = invoke(args).await;
    log::info!("{}", result.as_string().unwrap());
    Ok(())
}
