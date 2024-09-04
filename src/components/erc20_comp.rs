use std::fmt::format;
use dioxus::prelude::*;

use crate::components::common_alg_comp::sha256;
use crate::MyState;
use ethereum_types::Address;
use secp256k1::{PublicKey, SecretKey};
use sha3::{Digest, Keccak256};
use std::str::FromStr;
use anyhow::{bail, ensure};
use dioxus_logger::tracing::info;
use serde_json::Value;


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
            "Generate ERC20 address"
        }
    }
}
#[component]
pub fn QueryEthBalanceComp() -> Element {
    let erc20_addr = use_context::<MyState>().erc20_addr;

        let r = use_resource(move || async move { query_eth_balance(&erc20_addr.read()).await});
        return rsx! {

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
        return rsx! {

             pre{
                match &*r.read() {
                    Some(Ok(balance)) => rsx!("usdt balance is : {balance}"),
                    Some(Err(e)) => rsx!("Loading balance failed, {e}"),
                    None =>  rsx!("Loading..."),
                }
            }
        }

}

async fn query_eth_balance(address: &str) ->anyhow::Result<f64>{
    ensure!(address!="");
    let url = format!("https://api.etherscan.io/api?module=account&action=balance&address={}&tag=latest&apikey=Y2CGJ2P89DWWQ7Q2435X5TUTIUYB946V7P", address);
    let resp = reqwest::get(&url).await?.json::<Value>().await?;
    let balance = resp["result"].as_str().unwrap().parse::<f64>()? / 1e18;
    println!("ETH Balance: {}", balance);

    Ok(balance)
}
async fn query_usdt_balance(address: &str) ->anyhow::Result<f64>{
    ensure!(address!="");
    let url = format!("https://api.etherscan.io/api?module=account&action=tokenbalance&contractaddress=0xdac17f958d2ee523a2206206994597c13d831ec7&address={}&tag=latest&apikey=Y2CGJ2P89DWWQ7Q2435X5TUTIUYB946V7P", address);
    let resp = reqwest::get(&url).await?.json::<Value>().await?;
    info!("usdt balance : {:?}", resp);
    let balance = resp["result"].as_str().unwrap().parse::<f64>()? / 1e6;
    Ok(balance)
}



fn generate_erc20_address(private_key_hex: &str) -> String {
    // 从十六进制字符串解析私钥
    let secret_key = SecretKey::from_str(private_key_hex).unwrap();


    // 生成公钥
    let secp = secp256k1::Secp256k1::new();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // 序列化公钥
    let public_key = public_key.serialize_uncompressed();

    // 计算 Keccak-256 哈希
    let mut hasher = Keccak256::new();
    hasher.update(&public_key[1..]);
    let hash = hasher.finalize();

    // 取哈希的后 20 字节作为地址
    format!("{:?}", Address::from_slice(&hash[12..]))
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
