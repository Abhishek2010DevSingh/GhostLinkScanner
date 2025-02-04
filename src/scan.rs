use anyhow::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;
use tokio::task::JoinSet;
use tracing::{error, info, warn};
use url::Url;

pub async fn scan_base_url(base_url: &Url) -> Result<()> {
    let document = get_html(base_url).await?;
    let selector =
        Selector::parse("a").map_err(|e| anyhow::anyhow!("Failed to parse selector: {}", e))?;
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .context("Failed to build HTTP client")?;

    let mut tasks = JoinSet::new();

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            if let Ok(absolute_url) = base_url.join(link) {
                let client = client.clone();
                tasks.spawn(async move {
                    scan_url(client, &absolute_url).await;
                });
            } else {
                warn!("Skipping invalid URL: {}", link);
            }
        }
    }

    while let Some(res) = tasks.join_next().await {
        if let Err(e) = res {
            error!("Task failed: {:?}", e);
        }
    }

    Ok(())
}

async fn scan_url(client: Client, url: &Url) {
    info!("Scanning {}", url);
    match client.get(url.as_str()).send().await {
        Ok(res) if res.status().is_success() => info!("✅ OK: {}", url),
        Ok(res) => warn!("❌ Dead: {} (Status: {})", url, res.status()),
        Err(e) => error!("❌ Request Failed: {} (Error: {})", url, e),
    }
}

async fn get_html(url: &Url) -> Result<Html> {
    let response = reqwest::get(url.as_str())
        .await
        .with_context(|| format!("Failed to fetch URL: {}", url))?;
    let body = response
        .text()
        .await
        .context("Failed to read response body")?;
    Ok(Html::parse_document(&body))
}
