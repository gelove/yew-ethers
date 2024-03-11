use eyre::Result;
use gloo_net::http::Request;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::str::FromStr;
use web3::{
    contract::{Contract, Options},
    transports::eip_1193::Eip1193,
    types::{Address, U256},
    Web3,
};

use crate::types::Token;

#[derive(Clone, Debug, Deserialize)]
pub struct TxResponse {
    pub message: String,
    pub result: Vec<Token>,
    pub status: String,
}

/// 获取账户的所有代币交易记录
/// http://api.etherscan.io/api?module=account&action=tokentx&address=0x4e83362442b8d1bec281594cea3050c8eb01311c&startblock=0&endblock=999999999&sort=asc&apikey=FWIFYX812T9MP4CE7YMYRCHXQ4K9VAF7QV&page=1&offset=100
/**
 {
    "blockNumber":"15617636",
    "timeStamp":"1664196371",
    "hash":"0xed23f05743e15aead9261a6e28c4f226b7b6cdeee0946d04eaf5a4b76bb5b0e1",
    "nonce":"72",
    "blockHash":"0xfc435e702daf8e5a7a40ac625b50e3653162dd16606257ab7843ffd51f5fe454",
    "from":"0xec5ee3714a9c0be3def303136391011f604284e2",
    "contractAddress":"0x64e39084fce774b6e892e5a4da5a9032d7436871",
    "to":"0x4e83362442b8d1bec281594cea3050c8eb01311c",
    "value":"16229347000000000000000000",
    "tokenName":"AV DAO",
    "tokenSymbol":"Avault.fi",
    "tokenDecimal":"18",
    "transactionIndex":"150",
    "gas":"3398122",
    "gasPrice":"6000000000",
    "gasUsed":"3398122",
    "cumulativeGasUsed":"22215689",
    "input":"deprecated",
    "confirmations":"11875"
}
 */
pub async fn get_user_balances(w3: &Web3<Eip1193>, account: Address) -> Result<Vec<Token>> {
    // http://api.etherscan.io/api?module=account&action=tokentx&address=0x4e83362442b8d1bec281594cea3050c8eb01311c&apikey=FWIFYX812T9MP4CE7YMYRCHXQ4K9VAF7QV
    // http://api-goerli.etherscan.io/api?module=account&action=tokentx&address=0x4e83362442b8d1bec281594cea3050c8eb01311c&apikey=FWIFYX812T9MP4CE7YMYRCHXQ4K9VAF7QV
    let url = format!("http://api-goerli.etherscan.io/api?module=account&action=tokentx&address={}&apikey=FWIFYX812T9MP4CE7YMYRCHXQ4K9VAF7QV", "0x38B48a6A8E7b6c1f9064553c4EA2Cc532D3D29Fc");
    // debug!("fetch url => {:?}", url);
    // todo json 解析错误 不支持嵌套 struct ?
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

    let utils_addr = Address::from_str("0x2e5E9F116e8c1B166Bb5c66e98CCC5c5913C0E6f")?;
    let utils_contract =
        Contract::from_json(w3.eth(), utils_addr, include_bytes!("../../abi/Utils.json"))?;
    // 调用合约方法返回所有代币余额
    // tokenBalances(accounts, token_addresses)
    let balances: Vec<U256> = utils_contract
        .query(
            "tokenBalances",
            (vec![account], token_addresses),
            None,
            Options::default(),
            None,
        )
        .await?;
    for (i, v) in balances.iter().enumerate() {
        tokens[i].balance = format_units(v, tokens[i].decimal.parse::<u32>()?)?
    }

    Ok(tokens)
}

// U256转为f64(损失精度)
pub fn wei_to_eth(wei: U256) -> f64 {
    let res = wei.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

// U256转为浮点数字符串
pub fn format_units<T: Into<U256>>(amount: T, decimal: u32) -> Result<String> {
    let amount = amount.into();
    let amount_integer = amount / U256::from(10_u128.pow(decimal));
    let amount_decimals = amount % U256::from(10_u128.pow(decimal));
    Ok(format!(
        "{}.{:0width$}",
        amount_integer,
        amount_decimals.as_u128(),
        width = decimal as usize
    ))
}

// 字符串转为U256
pub fn parse_units<S: ToString>(amount: S, decimal: u32) -> Result<U256> {
    let num: Decimal = amount.to_string().parse()?;
    let multiplier: Decimal = 10u64.pow(decimal).into();
    let val = num
        .checked_mul(multiplier)
        .ok_or(rust_decimal::Error::ExceedsMaximumPossibleValue)?;
    let u256_n: U256 = U256::from_dec_str(&val.round().to_string())?;
    Ok(u256_n)
}

// Determining if an address is a valid Smart Contract address
pub async fn is_contract(w3: &Web3<Eip1193>, addr: Address) -> bool {
    match w3.eth().code(addr, None).await {
        Ok(code) => {
            // code should nonempty
            code != web3::types::Bytes::from([])
        }
        _ => {
            println!("Unable to retrieve code, skipping.");
            false
        }
    }
}

// 获取用户余额
pub async fn balance_of(w3: &Web3<Eip1193>, token: Address, account: Address) -> Result<U256> {
    let token_contract =
        Contract::from_json(w3.eth(), token, include_bytes!("../../abi/Token.json"))?;
    // Make sure to specify the expected return type, to prevent ambiguous compiler errors about `Detokenize` missing for `()`
    let balance: U256 = token_contract
        .query("balanceOf", (account,), None, Options::default(), None)
        .await?;
    Ok(balance)
}

// 获取代币的小数位数
pub async fn decimals(w3: &Web3<Eip1193>, token: Address) -> Result<u8> {
    let token_contract =
        Contract::from_json(w3.eth(), token, include_bytes!("../../abi/Token.json"))?;
    let decimal: u8 = token_contract
        .query("decimals", (), None, Options::default(), None)
        .await?;
    Ok(decimal)
}
