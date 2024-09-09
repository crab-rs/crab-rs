#![allow(non_snake_case)]


use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use std::str::FromStr;



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


// All of our routes will be a variant of this Route enum
#[derive(Routable, PartialEq, Clone)]
enum Route {
    // if the current location is "/home", render the Home component
    #[route("/home")]
    Home {},
    // if the current location is "/blog", render the Blog component
    #[route("/blog/:id")]
    Blog { id: i32 },

    //  if the current location doesn't match any of the above routes, render the NotFound component
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

#[component]
fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {segments:?}" }
    }
}

fn Home() -> Element {
    rsx! {
        h2{
            "Home"
        }
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        h2{
            "Blog"
        }
    }
}


fn App() -> Element {
    //MyState usage: let  result_str = use_context::<MyState>().result_str;
    use_context_provider(|| MyState::default());

    //see style: https://jenil.github.io/chota/#buttons
    rsx! {
        //define css here to compatible with ios
         style { {include_str!("../public/chota.min.css")} }
         style { {include_str!("../public/main.css")} }

        h2{
            "hello crab.rs"
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
