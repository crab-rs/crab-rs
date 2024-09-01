use crate::MyState;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn NumButton(num: i32) -> Element {
    let mut result_str = use_context::<MyState>().result_str;
    rsx! {
        button {
            class: "button primary",
            onclick: move |e| {
                info!("clicked  :{num}");
                result_str.set(format!("{}{}",result_str,  num.to_string()));
            },
            "{num}"
        }
    }
}

#[component]
pub fn ResultLabel(prefix: String) -> Element {
    let result_str = use_context::<MyState>().result_str;
    let btc_addr = use_context::<MyState>().btc_addr;
    let erc20_addr = use_context::<MyState>().erc20_addr;
    let len = result_str.read().len();
    rsx! {
        label {
            class:"result-label",
            "{prefix} [{len}]{result_str}"
        }
        if !btc_addr.read().is_empty(){
               label {
            class:"result-label",
            "btc address: {btc_addr}"
            }
        }
        if !erc20_addr.read().is_empty(){
            label {
                class:"result-label",
                "erc20 address: {erc20_addr}"
            }
        }
    }
}

#[component]
pub fn SetSeedButton() -> Element {
    let mut seed_str = use_context::<MyState>().seed_str;
    let result_str = use_context::<MyState>().result_str;
    rsx! {
        button {
            class:"button",
            onclick: move |e| {
                seed_str.set(result_str.to_string());
            },
            "Set seed"
        }
    }
}
