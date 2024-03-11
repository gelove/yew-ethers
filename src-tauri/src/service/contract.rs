use eyre::Result;
use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::{Address, H160, U256};
use web3::Web3;

use crate::service::web3::format_units;

pub async fn get_contract_demo(w3: &Web3<Http>) -> Result<()> {
    dotenv::dotenv().ok();

    // let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY").unwrap()).await?;
    // let w3 = web3::Web3::new(websocket);

    let mut accounts = w3.eth().accounts().await?;

    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);

    for account in accounts {
        let balance = w3.eth().balance(account, None).await?;
        println!(
            "Eth balance of {:?}: {:?}",
            account,
            format_units(balance, 18)?
        );
    }

    let aave_addr = Address::from_str("0x42447d5f59d5bf78a82c34663474922bdf278162").unwrap();
    let token_contract =
        Contract::from_json(w3.eth(), aave_addr, include_bytes!("../../abi/ERC20.json")).unwrap();

    let token_name: String = token_contract
        .query("name", (), None, Options::default(), None)
        .await
        .unwrap();

    let total_supply: U256 = token_contract
        .query("totalSupply", (), None, Options::default(), None)
        .await
        .unwrap();

    println!("Token name: {}, total supply: {}", token_name, total_supply);

    let token_decimal: u128 = token_contract
        .query("decimals", (), None, Options::default(), None)
        .await
        .unwrap();

    let is_paused: bool = token_contract
        .query("paused", (), None, Options::default(), None)
        .await
        .unwrap();
    let symbol: String = token_contract
        .query("symbol", (), None, Options::default(), None)
        .await
        .unwrap();

    let random_address = Address::from_str("0xC2d1266205aa5c80984a3D56d3FFbC23C971FB05").unwrap();
    let balance: U256 = token_contract
        .query("balanceOf", random_address, None, Options::default(), None)
        .await
        .unwrap();

    println!(
        "
        Token name: {}
        Token Symbol: {}
        Token decimals: {}
        Total supply: {}
        isPaused: {}
        balanceOf Random Address {}",
        token_name,
        symbol,
        token_decimal,
        format_units(total_supply, 18)?,
        is_paused,
        format_units(balance, 18)?
    );

    Ok(())
}
