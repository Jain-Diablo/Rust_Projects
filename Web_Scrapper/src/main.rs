use reqwest::blocking::{Client, Response};
use scraper::{Html, Selector};
use serde::Serialize;
use std::fs::File;
use std::io::Write;

// Struct to store all scraped data
#[derive(Serialize)]
struct WebData {
    title: Option<String>,
    meta_description: Option<String>,
    headings: Vec<String>,
    links: Vec<Link>,
    images: Vec<Image>,
}

// Struct to store links with their text
#[derive(Serialize)]
struct Link {
    text: String,
    href: String,
}

// Struct to store image data
#[derive(Serialize)]
struct Image {
    src: String,
    alt: String,
}

// Main function
fn main() {
    println!("Enter the website URL (e.g., https://example.com): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let url = input.trim();

    let client = Client::new();

    match scrape_website(url, &client,) {
        Ok(data) => {
            // Save the JSON data to a file
            let json_data = serde_json::to_string_pretty(&data).unwrap();
            let mut file = File::create("output.json").unwrap();
            file.write_all(json_data.as_bytes()).unwrap();
            println!("Scraped data has been saved to output.json.");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

// Function to scrape a website
fn scrape_website(
    url: &str,
    client: &Client,
) -> Result<WebData, Box<dyn std::error::Error>> {
    // Recursive scraping with pagination
    let mut all_links = Vec::new();
    let mut all_images = Vec::new();
    let mut current_url = url.to_string();
    let mut first_page_data: Option<WebData> = None;

    loop {
        let response = fetch_page(&current_url, client)?;
        let html = response.text()?;
        let document = Html::parse_document(&html);

        // Extract the title and meta description from the first page only
        if first_page_data.is_none() {
            let title_selector = Selector::parse("title").unwrap();
            let title = document
                .select(&title_selector)
                .next()
                .map(|el| el.inner_html());

            let meta_selector = Selector::parse("meta[name=\"description\"]").unwrap();
            let meta_description = document
                .select(&meta_selector)
                .next()
                .and_then(|el| el.value().attr("content").map(String::from));

            first_page_data = Some(WebData {
                title,
                meta_description,
                headings: Vec::new(),
                links: Vec::new(),
                images: Vec::new(),
            });
        }

        // Extract headings
        let heading_selector = Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();
        let _headings: Vec<String> = document
            .select(&heading_selector)
            .map(|el| el.inner_html().trim().to_string())
            .collect();

        // Extract links
        let link_selector = Selector::parse("a").unwrap();
        let links: Vec<Link> = document
            .select(&link_selector)
            .filter_map(|el| {
                let href = el.value().attr("href")?;
                let text = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                Some(Link {
                    text: if text.is_empty() { "No text".to_string() } else { text },
                    href: href.to_string(),
                })
            })
            .collect();
        all_links.extend(links);

        // Extract images
        let image_selector = Selector::parse("img").unwrap();
        let images: Vec<Image> = document
            .select(&image_selector)
            .filter_map(|el| {
                let src = el.value().attr("src")?;
                let alt = el.value().attr("alt").unwrap_or("").to_string();
                Some(Image {
                    src: src.to_string(),
                    alt,
                })
            })
            .collect();
        all_images.extend(images);

        // Check for next page (pagination)
        let next_page_selector = Selector::parse("a.next, a[rel=\"next\"]").unwrap();
        if let Some(next_page) = document
            .select(&next_page_selector)
            .next()
            .and_then(|el| el.value().attr("href"))
        {
            current_url = next_page.to_string();
        } else {
            break; // No next page, stop pagination
        }
    }

    // Combine all data
    let mut web_data = first_page_data.unwrap();
    web_data.headings = all_links.iter().map(|link| link.text.clone()).collect();
    web_data.links = all_links;
    web_data.images = all_images;

    Ok(web_data)
}

// Fetch a single webpage
fn fetch_page(
    url: &str,
    client: &Client,
) -> Result<Response, reqwest::Error> {
    let request = client.get(url);
    request.send()
}
