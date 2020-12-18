use tokio;
use hyper::{Body, Client, Method, Request};
use hyper::body::HttpBody as _;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Client::new();

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"ping": "pong"}"#))?;

    let mut resp = cli.request(req).await?;

    let body = resp.body_mut().data().await;

    println!("Response: {:?}", body);

    Ok(())
}
