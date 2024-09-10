use dioxus::prelude::*;

pub mod comps;
pub mod models;
pub mod api;

use comps::*;

// All of our routes will be a variant of this Route enum
#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    // if the current location is "/home", render the Home component
    #[route("/")]
    Home {},
    // if the current location is "/blog", render the Blog component
    #[route("/blog/:id")]
    Blog { id: i64 },

    //  if the current location doesn't match any of the above routes, render the NotFound component
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

#[derive(Default, Clone)]
pub struct MyState {
    pub btc_addr: Signal<String>,
    pub erc20_addr: Signal<String>,
    pub result_str: Signal<String>,
    pub seed_str: Signal<String>,
}