use anyhow::{Context, Result};
use prettytable::{row, Table};
use reqwest::Client;
use scraper::{Html, Selector};
use std::sync::LazyLock;
use std::sync::Mutex;
use std::time::Duration;
use tokio::task::JoinSet;
use tracing::{error, info, warn};
use url::Url;

static DEAD_LINKS: LazyLock<Mutex<Vec<(Url, String)>>> = LazyLock::new(|| Mutex::new(Vec::new()));

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

    let dead_links = DEAD_LINKS.lock().unwrap();
    if !dead_links.is_empty() {
        let mut table = Table::new();
        table.add_row(row!["#", "URL", "Status"]);

        for (index, (url, status)) in dead_links.iter().enumerate() {
            table.add_row(row![index + 1, url.as_str(), status]);
        }

        println!("\nDead Links Found:");
        table.printstd();
    } else {
        info!("No dead links found.");
    }

    Ok(())
}

async fn scan_url(client: Client, url: &Url) {
    info!("Scanning {}", url);
    match client.get(url.as_str()).send().await {
        Ok(res) if res.status().is_success() => info!("✅ OK: {}", url),
        Ok(res) => {
            let status = format!("Dead (Status: {})", res.status());
            warn!("❌ {}", status);
            DEAD_LINKS.lock().unwrap().push((url.clone(), status));
        }
        Err(e) => {
            error!("Request Failed (Error: {})", e);
        }
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
