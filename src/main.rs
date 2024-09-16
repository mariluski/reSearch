use reqwest::Client;
use std::error::Error;
use web_view::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define the URL you want to scrape
    let url = "https://www.w3schools.com/html/tryit.asp?filename=tryhtml_default";

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

    web_view::builder()
    .title("Webview Example")
    .content(Content::Html(body))
    .size(800, 600)
    .resizable(true)
    .user_data(())
    .invoke_handler(|_webview: &mut WebView<()>, _arg: &str| Ok(()))
    .run()
    .unwrap();

    Ok(())
}