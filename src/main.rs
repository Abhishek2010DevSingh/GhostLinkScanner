use clap::Parser;
use ghost_link_scanner::scan::scan_base_url;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use url::Url;

#[derive(Parser, Debug)]
#[command(about = "A ghost link scanner for detecting broken links.")]
struct Args {
    #[arg(value_parser = Url::parse)]
    url: Url,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting scan for URL: {}", args.url);

    scan_base_url(&args.url).await
}
