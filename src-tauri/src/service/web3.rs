use crate::types::Token;
use eyre::Result;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use walletconnect::Client;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, U256},
    Web3,
};

#[derive(Clone, Debug, Deserialize)]
pub struct TxResponse {
    pub message: String,
    pub result: Vec<Token>,
    pub status: String,
}

// pub fn get_utils_contract(
//     eth: Eth<Http>,
//     addr: Address,
// ) -> Result<Contract<Http>> {
//     return Contract::from_json(eth, addr, include_bytes!("../abi/Utils.json"))?;
// }

// pub async fn get_balances(
//     client: Arc<Mutex<Client>>,
//     account: Address,
//     tokens: Vec<Address>,
// ) -> Result<Vec<U256>> {
//     let client = client.lock().await;
//     let (accounts, chain_id) = client.accounts()?;
//     let utils_addr = Address::from_str("0x2e5E9F116e8c1B166Bb5c66e98CCC5c5913C0E6f")?;
//     let utils_contract = Contract::from_json(
//         client.eth(), // todo
//         utils_addr,
//         include_bytes!("../../abi/Utils.json"),
//     )
//     .unwrap();
//     // 调用合约方法返回所有代币余额
//     // tokenBalances(accounts, tokens)
//     let balances: Vec<U256> = utils_contract
//         .query(
//             "tokenBalances",
//             (vec![account], tokens),
//             None,
//             Options::default(),
//             None,
//         )
//         .await?;
//     Ok(balances)
// }

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
pub async fn is_contract(w3: &Web3<Http>, addr: Address) -> bool {
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
pub async fn balance_of(w3: &Web3<Http>, token: Address, account: Address) -> Result<U256> {
    let token_contract =
        Contract::from_json(w3.eth(), token, include_bytes!("../../abi/ERC20.json")).unwrap();
    // Make sure to specify the expected return type, to prevent ambiguous compiler errors about `Detokenize` missing for `()`
    let balance: U256 = token_contract
        .query("balanceOf", (account,), None, Options::default(), None)
        .await?;
    Ok(balance)
}

// 获取代币的小数位数
pub async fn decimals(w3: &Web3<Http>, token: Address) -> Result<u8> {
    let token_contract =
        Contract::from_json(w3.eth(), token, include_bytes!("../../abi/ERC20.json")).unwrap();
    // Make sure to specify the expected return type, to prevent ambiguous compiler errors about `Detokenize` missing for `()`
    let decimal: u8 = token_contract
        .query("decimals", (), None, Options::default(), None)
        .await?;
    Ok(decimal)
}
