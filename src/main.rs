use ghost_link_scanner::scan::scan_base_url;
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    scan_base_url(&Url::parse("https://github.com/Abhishek2010DevSingh")?).await
}
