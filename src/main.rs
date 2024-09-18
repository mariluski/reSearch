use reqwest;
use tokio;
use web_view::{self, Content, WebViewBuilder};
use select::document::Document;
use select::predicate::Name;

#[tokio::main]
async fn main() {
    let res: reqwest::Response = reqwest::get("https://google.com")
        .await
        .expect("Failed to send request");
    let body: String = res.text()
        .await
        .expect("Failed to read response text");
    let raw_html: String = body.trim().to_string();
    println!("{}", raw_html);

    let document = Document::from_read(raw_html.as_bytes()).unwrap();
    let title = "huh";
    let title_string = format!("ReSearch - {}", title);
    let web_view = WebViewBuilder::new()
        .title(&title_string) // Set the window title
        .content(Content::Html(&raw_html)) // Set the initial HTML content
        .size(800, 600) // Set the window size
        .resizable(true) // Allow the window to be resized
        .user_data(()) // Specify user data
        .invoke_handler(|_webview, _arg| Ok(())); // Specify an invoke handler

web_view.run().unwrap(); // Run the event loop
}