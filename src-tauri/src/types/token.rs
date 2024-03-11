use serde::Deserialize;
use std::cmp::PartialEq;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Token {
    #[serde(alias = "tokenName")]
    pub name: String,
    #[serde(alias = "tokenSymbol")]
    pub symbol: String,
    #[serde(alias = "tokenDecimal")]
    pub decimal: String,
    #[serde(alias = "contractAddress")]
    pub address: String,
    #[serde(skip_deserializing)]
    pub balance: String,
}

impl Token {
    pub fn new() -> Token {
        Token {
            name: String::new(),
            symbol: String::new(),
            decimal: String::new(),
            address: String::new(),
            balance: String::new(),
        }
    }
}
