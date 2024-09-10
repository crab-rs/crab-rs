use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crab_rs::comps::App;
use crab_rs::Route;

#[cfg(feature = "dioxus-ssr")]
fn main() {
    // 创建一个新的 VirtualDom
    let mut app = VirtualDom::new(App);

    app.rebuild_in_place();


    let html = dioxus_ssr::render(&app);
    // 输出 HTML（在实际应用中，你可能想要将其写入文件或发送到客户端）
    println!("Generated HTML:\n{}", html);
}

