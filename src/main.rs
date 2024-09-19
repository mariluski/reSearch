#[tokio::main]
async fn main() {
    let url = "https://google.com";
    let res = reqwest::get(url)
        .await
        .expect("Failed to send request");

    let final_url = res.url().clone();
    let web_view = web_view::builder()
        .title("ReSearch") // Set the window title
        .size(800, 600) // Set the window size
        .resizable(true) // Allow the window to be resized
        .user_data(()) // Specify user data
        .invoke_handler(|_webview, _arg| Ok(())); // Specify an invoke handler

    let web_view = web_view.build().unwrap();
    web_view.load_html("<html><body><iframe src=\"" + &final_url.to_string() + "\" width=\"100%\" height=\"100%\"></iframe></body></html>").unwrap();
    web_view.run().unwrap(); // Run the event loop
}