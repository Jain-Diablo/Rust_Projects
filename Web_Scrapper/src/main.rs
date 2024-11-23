use std::vec;

use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use log::{error, info};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ScrapeRequest {
    url: String,
}

#[derive(Serialize)]
struct ScrapeResponse {
    title: Option<String>,
    meta_description: Option<String>,
    headings: Vec<String>,
    links: Vec<String>,
    images: Vec<String>,
}

#[post("/scrape")]
async fn scrape(data: web::Json<ScrapeRequest>) -> impl Responder {
    let url = &data.url;

    // Scrape the website
    match scrape_website(url).await {
        Ok(res) => {
            info!("Scraping successful for: {}", url);
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            error!("Error scraping URL {}: {}", url, err);
            HttpResponse::InternalServerError().json(ScrapeResponse {
                title: None,
                meta_description: Some(format!("Error: {}", err)),
                headings: vec![],
                links: vec![],
                images: vec![],
            })
        }
    }
}

async fn scrape_website(url: &str) -> Result<ScrapeResponse, String> {
    // Fetch the webpage
    let client = Client::new();
    let response = client.get(url).send().await;

    // Check if fetching was successful
    let body = match response {
        Ok(resp) => {
            if resp.status().is_success() {
                info!("Successfully fetched URL: {}", url);
                resp.text()
                    .await
                    .map_err(|e| format!("Failed to read body: {}", e))?
            } else {
                error!(
                    "Failed to fetch URL: {} with status: {}",
                    url,
                    resp.status()
                );
                return Err(format!(
                    "Failed to fetch the webpage with status: {}",
                    resp.status()
                ));
            }
        }
        Err(e) => {
            error!("Error fetching URL: {}. Error: {}", url, e);
            return Err(format!("Error fetching URL: {}", e));
        }
    };

    // Parse the HTML document
    let document = Html::parse_document(&body);

    // Extract title
    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|el| el.inner_html());

    // Extract meta description
    let meta_selector = Selector::parse("meta[name=\"description\"]").unwrap();
    let meta_description = document
        .select(&meta_selector)
        .next()
        .and_then(|el| el.value().attr("content").map(String::from));

    // Extract headings
    let heading_selector = Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();
    let headings: Vec<String> = document
        .select(&heading_selector)
        .map(|el| el.inner_html().trim().to_string())
        .collect();

    // Extract links
    let link_selector = Selector::parse("a").unwrap();
    let links: Vec<String> = document
        .select(&link_selector)
        .filter_map(|el| el.value().attr("href").map(String::from))
        .collect();

    let image_selector = Selector::parse("img").unwrap();
    let images: Vec<String> = document
        .select(&image_selector)
        .filter_map(|el| el.value().attr("src").map(String::from))
        .collect();

    // Return the extracted data
    Ok(ScrapeResponse {
        title,
        meta_description,
        headings,
        links,
        images,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging for debugging
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // Enabling CORS for all origins
            .service(scrape) // Registering the scrape service
    })
    .bind("127.0.0.1:6969")? // Binding to localhost:6969
    .run()
    .await
}
