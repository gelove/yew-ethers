use ethers::abi::AbiDecode;
use ethers::prelude::*;
use ethers::utils::hex::ToHex;
use futures::stream::StreamExt;
use log::debug;
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::component::button::Button;
use crate::utils::sign;

abigen!(
    IUniswapV2Factory,
    r#"[
        createPair(address tokenA, address tokenB) external returns (address pair) 
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    IUniswapV2Router02,
    r#"[
        addLiquidity(address tokenA, address tokenB, uint amountADesired, uint amountBDesired, uint amountAMin, uint amountBMin, address to, uint deadline)
        addLiquidityETH(address token, uint amountTokenDesired, uint amountTokenMin, uint amountETHMin, address to, uint deadline)
        getAmountsOut(uint amountIn, address[] memory path) public view returns (uint[] memory amounts)
        swapExactTokensForTokens(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts)
        swapExactTokensForTokensSupportingFeeOnTransferTokens(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts)
        swapExactETHForTokensSupportingFeeOnTransferTokens(uint amountOutMin, address[] calldata path, address to, uint deadline) external payable
        swapExactTokensForETHSupportingFeeOnTransferTokens(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external
        swapETHForExactTokens(uint amountOut, address[] calldata path, address to, uint deadline) external  payable returns (uint[] memory amounts)
        swapTokensForExactTokens(uint amountOut, uint amountInMax, address[] calldata path, address to, uint deadline])
    ]"#,
);

const AFTER_TAX: f32 = 0.95;

#[function_component(Presale)]
pub fn presale() -> Html {
    async fn watch() {
        let endpoint = "wss://eth-mainnet.g.alchemy.com/v2/y8cmG9BNP7UeJVYwtvKeVoxkfqMIye0f";
        let provider = Provider::connect(endpoint)
            .await
            .expect_throw("could not instantiate WebSocket Provider");

        let uniswap_router_v2 = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
        let uniswap_router_v3 = "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45";
        let params = json!([
            "alchemy_filteredNewFullPendingTransactions",
            json!({ "toAddress": uniswap_router_v2 })
        ]);
        debug!("Constructed Params:{:?}", params);
        // alchemy_pendingTransactions or alchemy_filteredNewFullPendingTransactions
        // subscribe pending transactions  监听待执行的交易
        let id: U256 = provider
            .request("eth_subscribe", params)
            .await
            .unwrap_throw();
        debug!("Sent request with id: {:?}", id);
        let mut stream: SubscriptionStream<'_, ethers::providers::Ws, Transaction> =
            SubscriptionStream::new(id, &provider)
                .map(Into::into)
                .unwrap_throw();
        let method_add_liquidity = sign::hex_to_u8("e8e33700");
        let method_add_liquidity_eth = sign::hex_to_u8("f305d719");
        let method_swap_exact_tokens_for_tokens = sign::hex_to_u8("38ed1739");
        let method_swap_tokens_for_exact_tokens = sign::hex_to_u8("8803dbee");
        debug!(
            "method_add_liquidity: {:?} {:?}",
            method_add_liquidity, method_add_liquidity_eth
        );

        // if &method_add_liquidity[..] == [232, 227, 55, 0] {
        //     debug!("method_add_liquidity");
        // }
        // if &method_add_liquidity_eth[..] ==  [243, 5, 215, 25] {
        //     debug!("method_add_liquidity_eth");
        // }
        while let Some(tx) = stream.next().await {
            // debug!("tx: {:?}", tx);
            if tx.input.is_empty() {
                continue;
            }
            let method = &tx.input[..4];
            let gas_price = tx.gas_price;
            if method == method_add_liquidity {
                debug!("tx method == method_add_liquidity");
                let decoded = AddLiquidityCall::decode(&tx.input)
                    .expect_throw("AddLiquidityCall::decode failed");
                let token_a = decoded.token_a.encode_hex::<String>();
                debug!("tx token_a => {:?}", token_a);
                let token_b = decoded.token_b.encode_hex::<String>();
                debug!("tx token_b => {:?}", token_b);
            }
            if method == method_add_liquidity_eth {
                debug!("tx method == method_add_liquidity_eth");
                let decoded = AddLiquidityETHCall::decode(&tx.input)
                    .expect_throw("AddLiquidityCall::decode failed");
                let token = decoded.token.encode_hex::<String>();
                debug!("tx token => {:?}", token);
            }

            // 估算需用的gas
            // let gas = await router.estimateGas.swapExactETHForTokensSupportingFeeOnTransferTokens

            // gas_price-1 比添加流动性的gas_prices少1wei
            // 贿赂矿工，使自己在同gas下优先，避免自己被夹

            // let input = &tx.input.to_string();
            // debug!("tx.input: {:?}", input);
            // let method = &input[2..10];
            // debug!("tx method: {:?}", method);
            if tx.from.to_string() != "contract creator" {
                continue;
            }
            debug!("value {:?}", tx.value);
            debug!("transaction_type {:?}", tx.transaction_type);
        }
        debug!("SubscriptionStream finished");
    }

    let onclick = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            spawn_local(async move {
                watch().await;
            })
        })
    };

    html! {
        <div class="flex flex-col">
            <input type="text" name="token" />
            <input type="text" name="amount" />
            <input type="text" name="price_limit" />
            <Button {onclick}>{"抢购"}</Button>
        </div>
    }
}

#[test]
fn decode_test() {
    let tx_input = "0xf305d719000000000000000000000000d6ea6addbe4654eaa6292e1d13d62e3ff8744dfb00000000000000000000000000000000000000000000000016461aff1080800000000000000000000000000000000000000000000000000016461aff108080000000000000000000000000000000000000000000000000001bc16d674ec800000000000000000000000000001b67f3457a2ab6369373121b5d3e5e5f2c818f2800000000000000000000000000000000000000000000000000000000631e00d8";
    let calldata: Bytes = tx_input.parse().expect_throw("parse error");
    let decoded = AddLiquidityETHCall::decode(&calldata).expect_throw("decode failed");
    let token = decoded.token.encode_hex::<String>();
    let to = decoded.to.encode_hex::<String>(); // 添加流动性的做市商(此交易发起人)的地址
    println!("{} {}", token, to);
}
