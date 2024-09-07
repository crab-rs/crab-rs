use std::fmt::format;
use dioxus::prelude::*;

use crate::MyState;
use std::str::FromStr;
use anyhow::{bail, ensure};
use dioxus_logger::tracing::info;
use serde_json::Value;
use rust_utils::erc20_wallet::*;
use rust_utils::sha256;

#[component]
pub fn GenErc20AddrButton() -> Element {
    let result_str = use_context::<MyState>().result_str;
    let mut erc20_addr = use_context::<MyState>().erc20_addr;
    rsx! {
        button {
            class:"button",
            onclick: move |e| {
                let addr = generate_erc20_address(&sha256(&result_str.to_string()));
                info!("addr : {addr}");
                // result_str.set(addr.to_string());
                erc20_addr.set(addr.to_string());
            },
            "Generate ERC20 addr"
        }
    }
}
#[component]
pub fn QueryEthBalanceComp() -> Element {
    let erc20_addr = use_context::<MyState>().erc20_addr;

        let r = use_resource(move || async move { query_eth_balance(&erc20_addr.read()).await});
        rsx! {

             pre{
                match &*r.read() {
                    Some(Ok(balance)) => rsx!("eth balance is : {balance}"),
                    Some(Err(e)) => rsx!("Loading balance failed, {e}"),
                    None =>  rsx!("Loading..."),
                }
            }
        }


}
#[component]
pub fn QueryUsdtBalanceComp() -> Element {
    let erc20_addr = use_context::<MyState>().erc20_addr;
        let r = use_resource(move || async move { query_usdt_balance(&erc20_addr.read()).await});
        rsx! {

             pre{
                match &*r.read() {
                    Some(Ok(balance)) => rsx!("usdt balance is : {balance}"),
                    Some(Err(e)) => rsx!("Loading balance failed, {e}"),
                    None =>  rsx!("Loading..."),
                }
            }
        }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_erc20_address() {
        let input_string = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"; // 替换为您想要用作种子的字符串
        let erc20_address = generate_erc20_address(input_string);
        println!("ERC20 Address: {:?}", erc20_address);
    }
}
