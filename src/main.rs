
use reqwest::Client;
use std::error::Error;
use wry::{
    webview::WebView,
    window::WindowBuilder,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use urlencoding::encode;

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

    // Print the HTML content (optional)
    println!("{}", body);

    // Create and run the webview
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Simple HTML Viewer")
        .with_inner_size(wry::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)?;

    let webview = WebView::new(window)?;
    webview.load_html(&body)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(_) => println!("Application has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Window close requested");
                *control_flow = ControlFlow::Exit;
            },
            _ => (),
        }
    });

    Ok(())
}