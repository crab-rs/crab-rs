use std::fmt::format;
use dioxus::prelude::*;
use bitcoin::key::Secp256k1;
use bitcoin::{Address, Network, PrivateKey, PublicKey};
use dioxus_logger::tracing::info;
use std::str::FromStr;
use anyhow::{bail, ensure};
use serde_json::Value;
use sha2::{Digest, Sha256};
use crate::components::common_alg_comp::sha256;
use crate::MyState;
fn gen_btc_address(private_key: &str)->String{
    let secp = Secp256k1::new();

    // 从指定的私钥字符串生成 SecretKey
    let private_key = PrivateKey::from_str(&private_key_to_wif(private_key)).unwrap();

    // 生成公钥
    let public_key = PublicKey::from_private_key(&secp, &private_key);


    // 创建比特币地址
    let address = Address::p2pkh(&public_key, Network::Bitcoin);

    format!("{address}")
}


fn private_key_to_wif(private_key_hex: &str) -> String {
    // 1. 将私钥从16进制转换为字节
    let mut private_key_bytes = hex::decode(private_key_hex).expect("Invalid Hex");

    // 2. 添加 WIF 前缀 (0x80 for mainnet, 0xef for testnet)
    private_key_bytes.insert(0, 0x80);

    // 3. 对扩展后的私钥进行两次 SHA-256 哈希
    let sha256_1 = Sha256::digest(&private_key_bytes);
    let sha256_2 = Sha256::digest(&sha256_1);

    // 4. 取前4个字节作为校验和
    let checksum = &sha256_2[0..4];

    // 5. 将校验和附加到私钥后面
    private_key_bytes.extend_from_slice(checksum);

    // 6. Base58 编码
    bs58::encode(private_key_bytes).into_string()
}

#[component]
pub fn GenBitcoinAddrButton() -> Element {
    let  result_str = use_context::<MyState>().result_str;
    let mut btc_addr = use_context::<MyState>().btc_addr;
    rsx! {
        button {
            class:"button",
            onclick: move |e| {
                let addr = gen_btc_address(&sha256(&result_str.to_string()));
                info!("addr : {addr}");
                btc_addr.set(addr);
            },
            "Generate BTC address"
        }
    }
}
#[component]
pub fn QueryBtcBalanceComp() -> Element {
    let btc_addr = use_context::<MyState>().btc_addr;
    if *btc_addr.read()!=""{
        let r = use_resource(move || async move { query_btc_balance(&btc_addr.read()).await});
        return rsx! {

             pre{
                match &*r.read() {
                    Some(Ok(balance)) => rsx!("btc balance is : {balance}"),
                    Some(Err(e)) => rsx!("Loading balance failed, {e}"),
                    None =>  rsx!("Loading..."),
                }
            }
        }

    }

    None
}

async fn query_btc_balance(address: &str) ->anyhow::Result<f64>{
    ensure!(address!="");

    let url = format!("https://blockchain.info/balance?active={}", address);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    let json: Value = serde_json::from_str(&body)?;

    if let Some(balance) = json[address]["final_balance"].as_u64() {

        Ok((balance as f64) / 100_000_000.0)
    } else {
        bail!("无法获取余额信息");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_key_to_wif() {
        let private_key = "a6ad3e00ddc4d9c08c908f3cb917cfc98b1845d4bc146107904c6c1d855b94b1";
        // let expected_wif = "5J3mBbAH58CpQ3Y5RNJpYXtWY8wWTD2YNrCxPVtXcEwrVamVHoz"; // 预期的WIF格式

        let wif = private_key_to_wif(private_key);

        println!("wif : {}", wif);
    }
    #[test]
    fn test_gen_btc_address() {
        let private_key = "a6ad3e00ddc4d9c08c908f3cb917cfc98b1845d4bc146107904c6c1d855b94b1";

        let address = gen_btc_address(private_key);

        println!("address: {address}");
    }
}
