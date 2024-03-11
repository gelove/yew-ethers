use std::env;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use web3::contract::{Contract, Options};
use web3::transports::eip_1193::{Eip1193, Provider};
use web3::types::{Address, H160, U256};


pub async fn get_contract_demo() -> web3::Result<()> {
    dotenv::dotenv().ok();

    // let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY").unwrap_throw()).await?;
    // let web3s = web3::Web3::new(websocket);

    let web3s = match Provider::default() {
        Ok(Some(p)) => Some(web3::Web3::new(Eip1193::new(p))),
        _ => None,
    };
    let web3s = web3s.unwrap_throw();

    let mut accounts = web3s.eth().accounts().await?;

    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap_throw()).unwrap_throw());
    println!("Accounts: {:?}", accounts);

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("Eth balance of {:?}: {}", account, format_units(balance));
    }

    let aave_addr = Address::from_str("0x42447d5f59d5bf78a82c34663474922bdf278162").unwrap_throw();
    let token_contract = Contract::from_json(
        web3s.eth(),
        aave_addr,
        include_bytes!("../../IERC20.json"),
    )
    .unwrap_throw();

    let token_name: String = token_contract
        .query("name", (), None, Options::default(), None)
        .await
        .unwrap_throw();

    let total_supply: U256 = token_contract
        .query("totalSupply", (), None, Options::default(), None)
        .await
        .unwrap_throw();

    println!("Token name: {}, total supply: {}", token_name, total_supply);

    let token_decimal: u128 = token_contract
        .query("decimals", (), None, Options::default(), None)
        .await
        .unwrap_throw();

    let is_paused: bool = token_contract
        .query("paused", (), None, Options::default(), None)
        .await
        .unwrap_throw();
    let symbol: String = token_contract
        .query("symbol", (), None, Options::default(), None)
        .await
        .unwrap_throw();

    let random_address = Address::from_str("0xC2d1266205aa5c80984a3D56d3FFbC23C971FB05").unwrap_throw();
    let balance: U256 = token_contract
        .query("balanceOf", random_address, None, Options::default(), None)
        .await
        .unwrap_throw();
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
        format_units(total_supply),
        is_paused,
        format_units(balance)
    );

    Ok(())
}
