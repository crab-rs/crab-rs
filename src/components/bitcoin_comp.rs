use std::fmt::format;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use std::str::FromStr;
use anyhow::{bail, ensure};
use serde_json::Value;
use crate::MyState;
use rust_utils::*;

#[component]
pub fn GenBitcoinAddrButton() -> Element {
    let  result_str = use_context::<MyState>().result_str;
    let mut btc_addr = use_context::<MyState>().btc_addr;
    rsx! {
        button {
            class:"button",
            onclick: move |e| {
                let addr = rust_utils::bitcoin_wallet::gen_btc_address(&sha256(&result_str.to_string()));
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

        let r = use_resource(move || async move { rust_utils::bitcoin_wallet::query_btc_balance(&btc_addr.read()).await});
        rsx! {

             pre{
                match &*r.read() {
                    Some(Ok(balance)) => rsx!("btc balance is : {balance}"),
                    Some(Err(e)) => rsx!("Loading balance failed, {e}"),
                    None =>  rsx!("Loading..."),
                }
            }
        }


}
