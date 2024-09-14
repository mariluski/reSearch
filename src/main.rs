use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define the URL you want to scrape
    let url = "https://google.com";

    // Create a new HTTP client
    let client = Client::new();

    // Send a GET request
    let response = client.get(url).send().await?;

    // Ensure the request was successful
    if !response.status().is_success() {
        return Err(format!("Failed to fetch URL: {}", url).into());
    }

    // Get the response text (HTML content)
    let body = response.text().await?;

    // Print the HTML content
    println!("{}", body);

    Ok(())
}
