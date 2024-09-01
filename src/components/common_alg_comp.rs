use dioxus::prelude::*;
use base64::engine::general_purpose;
use sha2::{Digest, Sha256};
use base64::Engine;
use dioxus_logger::tracing::info;
use crate::{MyState};


#[derive(Clone, PartialEq, Debug)]
enum AlgKind {
    Sha256,
    Md5,
    Base64,
    Repeat10,
    Repeat20,
    AddSuffix,
}

fn addSeedToSuffix(s: &str, seed: &str) -> String {
    format!("{}{}", s, seed)
}

fn repeat10(s: &str) -> String {
    format!("{}", s.repeat(10))
}

fn repeat20(s: &str) -> String {
    format!("{}", s.repeat(20))
}

fn md5(s: &str) -> String {
    let digest = md5::compute(s.as_bytes());
    format!("{:x}", digest)
}
fn base64(s: &str) -> String {
    let mut buf = String::new();
    general_purpose::STANDARD. encode_string(s.as_bytes(), &mut buf);
    format!("{}", buf)
}
pub fn sha256(s: &str) -> String {
    let data = s.as_bytes();
    let mut hasher = Sha256::new();

    // Write input data
    hasher.update(data);

    // Read hash digest
    let result = hasher.finalize();

    // Print hash in hexadecimal format
    format!("{:x}", result)
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
                        result_str.set(addSeedToSuffix(&result_str.to_string(), &seed_str.to_string()));
                    }
                }
            },
            "{alg_kind:?}"
        }
    }
}