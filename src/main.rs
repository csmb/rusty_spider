use anyhow::{Context, Result};
use image::ImageFormat;
use scraper::{Html, Selector};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::sync::Mutex;
use tokio::time::sleep;
use url::{Url, Origin};
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<()> {
    // Get URL from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <url>", args[0]);
        eprintln!("Example: {} https://example.com", args[0]);
        std::process::exit(1);
    }

    let start_url = &args[1];
    let base_url = Url::parse(start_url).context("Failed to parse URL")?;
    let base_origin = base_url.origin();

    println!("Starting crawler for {}", start_url);
    println!("Images will be saved to the 'downloads' directory");

    // Create downloads directory
    fs::create_dir_all("downloads").await?;

    // Shared state for tracking visited URLs and downloaded images
    let visited_urls = Arc::new(Mutex::new(std::collections::HashSet::new()));
    let downloaded_images = Arc::new(Mutex::new(std::collections::HashSet::new()));
    let client = Arc::new(reqwest::Client::new());

    // Start crawling from the initial URL
    crawl_url(
        base_url,
        base_origin,
        visited_urls.clone(),
        downloaded_images.clone(),
        client.clone(),
    ).await?;

    // Print summary
    let visited = visited_urls.lock().await;
    let downloaded = downloaded_images.lock().await;
    println!("\nCrawling completed!");
    println!("Pages visited: {}", visited.len());
    println!("Images downloaded: {}", downloaded.len());

    Ok(())
}

async fn crawl_url(
    url: Url,
    base_origin: Origin,
    visited_urls: Arc<Mutex<std::collections::HashSet<String>>>,
    downloaded_images: Arc<Mutex<std::collections::HashSet<String>>>,
    client: Arc<reqwest::Client>,
) -> Result<()> {
    // Skip if we've already visited this URL
    {
        let mut visited = visited_urls.lock().await;
        if !visited.insert(url.to_string()) {
            return Ok(());
        }
    }

    println!("Crawling: {}", url);

    // Add a small delay between requests to be respectful to the server
    sleep(Duration::from_millis(500)).await;

    // Fetch the page content
    let response = client.get(url.as_str()).send().await?;
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    // Download images
    let img_selector = Selector::parse("img").unwrap();
    for img in document.select(&img_selector) {
        if let Some(src) = img.value().attr("src") {
            if let Ok(img_url) = url.join(src) {
                // Only process images from the same origin
                if img_url.origin() == base_origin {
                    let mut downloaded = downloaded_images.lock().await;
                    if downloaded.insert(img_url.to_string()) {
                        download_image(&client, img_url).await?;
                    }
                }
            }
        }
    }

    // Find and follow links
    let link_selector = Selector::parse("a").unwrap();
    let mut futures = Vec::new();
    
    for link in document.select(&link_selector) {
        if let Some(href) = link.value().attr("href") {
            if let Ok(link_url) = url.join(href) {
                // Only follow links from the same origin
                if link_url.origin() == base_origin {
                    futures.push(crawl_url(
                        link_url,
                        base_origin.clone(),
                        visited_urls.clone(),
                        downloaded_images.clone(),
                        client.clone(),
                    ));
                }
            }
        }
    }

    // Wait for all child crawls to complete
    join_all(futures).await;

    Ok(())
}

async fn download_image(client: &Arc<reqwest::Client>, url: Url) -> Result<()> {
    println!("Downloading: {}", url);
    
    let response = client.get(url.as_str()).send().await?;
    let bytes = response.bytes().await?;
    
    // Try to determine image format from content
    let format = image::guess_format(&bytes)?;
    
    // Create filename from URL
    let filename = url.path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("image");
    
    let extension = match format {
        ImageFormat::Jpeg => "jpg",
        ImageFormat::Gif => "gif",
        _ => return Ok(()), // Skip non-jpg/gif images
    };
    
    let path = Path::new("downloads").join(format!("{}.{}", filename, extension));
    
    // Save the image
    fs::write(&path, &bytes).await?;
    
    Ok(())
}
