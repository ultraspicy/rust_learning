use tokio_postgres::{NoTls, Error};
use rand::Rng;
use dotenv::dotenv;
use std::env;
use reqwest;
use scraper::{Html, Selector};
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let host = env::var("DB_HOST").expect("DB_HOST must be set");
    let user = env::var("DB_USER").expect("DB_USER must be set");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let dbname = env::var("DB_NAME").expect("DB_NAME must be set");
    let connection_string = format!(
        "host={} user={} password={} dbname={}",
        host, user, password, dbname
    );

    println!("Connecting to database...");
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;
    println!("Connected!");

    // Spawn connection in the background since
    // connection.await runs an infinite loop processing I/O and only returns when connection closes
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    match crawl_page("https://www.ferrari.com/en-US").await {
        Ok(page) => {
            println!("{:?}", page);
        }
        Err(e) => {
            println!("Fail to crawl: {}", e);
        }
    }

    // Insert into database
    // client.execute(
    //     "INSERT INTO dummy (data) VALUES ($1)",
    //     &[&random_sentence],
    // ).await?;

    // println!("Successfully inserted: {}", random_sentence);

    Ok(())
}

#[derive(Clone, Debug, Default)]
struct PageData {
    source_url: String,
    title: String,
    content: String,
    status_code: u16,
}

async fn crawl_page(url: &str) -> Result<PageData, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .gzip(true)      // ← Enable gzip decompression
        .brotli(true)    // ← Enable brotli decompression
        .deflate(true)   // ← Enable deflate decompression
        .timeout(Duration::from_secs(30))
        .build()?;
    
    let response = client
        .get(url)
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept-Encoding", "gzip, deflate, br")  // ← Tell server we accept compression
        .send()
        .await?;

    let status_code = response.status().as_u16();
    let html = response.text().await?;
    
    let document = Html::parse_document(&html);
    let title_selector = Selector::parse("title").unwrap();
    
    let title = document
        .select(&title_selector)
        .next()
        .map(|el| el.inner_html())

        .unwrap_or_else(|| "No title".to_string());
    
    Ok(PageData {
        source_url: url.to_string(),
        title,
        content: html,
        status_code,
    })
}

// async fn extract_links(document: &Html, base_url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {

// }