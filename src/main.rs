#![allow(non_snake_case)]

mod components;

use std::str::FromStr;
use base64::Engine;
use base64::engine::general_purpose;
use bitcoin::{Address, Network, PrivateKey, PublicKey};
use bitcoin::secp256k1::Secp256k1;
use dioxus::mobile::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use sha2::{Digest, Sha256};
use crate::components::bitcoin_comp::{GenBitcoinAddrButton, QueryBtcBalanceComp};
use crate::components::common_alg_comp::{AlgButton, AlgButtonList};
use crate::components::erc20_comp::{GenErc20AddrButton, QueryEthBalanceComp, QueryUsdtBalanceComp};
use crate::components::misc_comp::{ResultLabel, NumButton, SetSeedButton};


fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    #[cfg(target_os = "ios")]
    {
        use dioxus::mobile::{Config, LogicalSize, WindowBuilder};
        LaunchBuilder::mobile()
            .with_cfg(
                Config::new().with_window(
                    WindowBuilder::new()
                        .with_inner_size(LogicalSize::new(390.0, 1000.0)),
                ),
            )
            .launch(crate::App);
    }

    #[cfg(not(target_os = "ios"))]
    launch(App);
}

fn App() -> Element {
    use_context_provider(|| MyState::default());

    //see style: https://jenil.github.io/chota/#buttons
    rsx! {
         style { {include_str!("../assets/chota.min.css")} }
         style { {include_str!("../assets/main.css")} }
        div {
            div {
                class:"button-group",
                for i in 0..10 {
                    NumButton { num: i }
                }

                 // SetSeedButton{}
            }
            hr {}

            AlgButtonList{}

            hr {}

            div{
                class:"button-group2",
                GenBitcoinAddrButton{}
                GenErc20AddrButton{}
            }


            hr {}
            ResultLabel{prefix: "Input: "}

            hr {}

            QueryBtcBalanceComp{}
            QueryEthBalanceComp{}
            QueryUsdtBalanceComp{}


        }

    }
}




#[derive(Default, Clone)]
pub struct MyState {
    pub btc_addr: Signal<String>,
    pub erc20_addr: Signal<String>,
    pub result_str: Signal<String>,
    pub seed_str: Signal<String>,
}
