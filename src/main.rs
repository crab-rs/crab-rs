#![allow(non_snake_case)]


use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use std::str::FromStr;
use rust_utils::data::IData;
use crab_rs::comps::*;

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
            .launch(App);
    }

    #[cfg(not(target_os = "ios"))]
    launch(App);
}


