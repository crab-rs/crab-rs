use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::{MyState};
use rust_utils::*;

#[derive(Clone, PartialEq, Debug)]
enum AlgKind {
    Sha256,
    Md5,
    Base64,
    Repeat10,
    Repeat20,
    AddSuffix,
}

#[component]
pub fn AlgButtonList() -> Element {
    rsx!{
        div {
            class:"button-group3",
            AlgButton {alg_kind: AlgKind::Md5 }
            AlgButton { alg_kind: AlgKind::AddSuffix }
            AlgButton { alg_kind: AlgKind::Base64 }
            AlgButton { alg_kind: AlgKind::Repeat10 }
            AlgButton { alg_kind: AlgKind::Repeat20 }
            AlgButton { alg_kind: AlgKind::Sha256 }

        }
    }
}

#[component]
pub fn AlgButton(alg_kind: AlgKind) -> Element {
    let mut result_str = use_context::<MyState>().result_str;
    let mut seed_str = use_context::<MyState>().seed_str;

    rsx! {
        button {
            class: "button  outline primary",
            onclick: move |e| {
                info!("clicked  :{alg_kind:?}");

                if seed_str.read().is_empty(){
                    seed_str.set(result_str.to_string());
                }

                match alg_kind{
                    AlgKind::Sha256 => {
                        result_str.set(sha256(&result_str.to_string()));
                    }
                    AlgKind::Md5 => {
                        result_str.set(md5(&result_str.to_string()));
                    }
                    AlgKind::Base64 => {
                        result_str.set(base64(&result_str.to_string()));
                    }
                    AlgKind::Repeat10 => {
                         result_str.set(repeat10(&result_str.to_string()));
                    }
                    AlgKind::Repeat20 => {
                        result_str.set(repeat20(&result_str.to_string()));
                    }
                    AlgKind::AddSuffix => {
                        result_str.set(add_seed_to_suffix(&result_str.to_string(), &seed_str.to_string()));
                    }
                }
            },
            "{alg_kind:?}"
        }
    }
}