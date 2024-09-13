use anyhow::Result;
use play_cache::render_html_in_browser;

#[tokio::main]
async fn main() -> Result<()> {
    let html = render_html_in_browser("https://zhouzhipeng.com").await?;
    println!("{}", html);

    Ok(())
}